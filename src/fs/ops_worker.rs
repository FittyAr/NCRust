use crate::fs::archive::{compress_zip, extract_archive};
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct ProgressUpdate {
    /// Name of the file currently being copied/moved
    pub current_file: String,
    /// Number of files fully copied so far
    pub files_copied: usize,
    /// Total number of files to copy
    pub total_files: usize,
    /// Total number of bytes copied so far across all files
    pub bytes_copied: u64,
    /// Total bytes to copy across all files
    pub total_bytes: u64,
    /// Detailed error message if the task fails
    pub error: Option<String>,
}

fn copy_symlink(src: &Path, dst: &Path) -> Result<()> {
    let target = fs::read_link(src)?;
    #[cfg(target_os = "windows")]
    {
        let resolved_target = if target.is_relative() {
            src.parent()
                .map(|p| p.join(&target))
                .unwrap_or_else(|| target.clone())
        } else {
            target.clone()
        };
        if resolved_target.is_dir() {
            std::os::windows::fs::symlink_dir(&target, dst)?;
        } else {
            std::os::windows::fs::symlink_file(&target, dst)?;
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::os::unix::fs::symlink(&target, dst)?;
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn run_as_admin_copy(src: &Path, dst: &Path) -> Result<()> {
    use std::process::Command;
    let src_str = src.to_string_lossy().replace('"', "\\\"");
    let dst_str = dst.to_string_lossy().replace('"', "\\\"");
    let ps_arg = format!(
        "Start-Process powershell -ArgumentList '-NoProfile -Command Copy-Item -Path \\\"{}\\\" -Destination \\\"{}\\\" -Force' -Verb RunAs -WindowStyle Hidden -Wait",
        src_str, dst_str
    );
    let status = Command::new("powershell")
        .args(&["-NoProfile", "-Command", &ps_arg])
        .status()?;
    if status.success() {
        Ok(())
    } else {
        anyhow::bail!("Failed to copy as administrator")
    }
}

#[cfg(not(target_os = "windows"))]
fn run_as_admin_copy(src: &Path, dst: &Path) -> Result<()> {
    use std::process::Command;
    let status = Command::new("sudo")
        .arg("cp")
        .arg("-p")
        .arg(src)
        .arg(dst)
        .status()?;
    if status.success() {
        Ok(())
    } else {
        anyhow::bail!("Failed to copy as administrator via sudo")
    }
}

/// Spawns a background task to copy multiple source files/directories to a destination directory.
/// Returns a channel receiver for real-time progress updates.
pub fn spawn_copy_task(
    sources: Vec<PathBuf>,
    destination_dir: PathBuf,
    settings: crate::config::settings::Settings,
) -> mpsc::Receiver<ProgressUpdate> {
    let (tx, rx) = mpsc::channel(100);

    tokio::spawn(async move {
        let mut total_files = 0;
        let mut total_bytes = 0;
        let mut file_mappings = Vec::new(); // (src, dst, is_symlink)
        let mut dirs_to_create = Vec::new();

        // 1. Gather all directories to create and files to copy
        for src in &sources {
            let is_sym = src.is_symlink();
            if src.is_dir() && (!is_sym || settings.scan_symbolic_links) {
                if let Some(folder_name) = src.file_name() {
                    let base_dst = destination_dir.join(folder_name);
                    dirs_to_create.push(base_dst.clone());

                    let mut dirs_to_visit = vec![src.clone()];
                    while let Some(dir) = dirs_to_visit.pop() {
                        if let Ok(entries) = fs::read_dir(&dir) {
                            for entry in entries.flatten() {
                                let path = entry.path();
                                let entry_is_sym = path.is_symlink();
                                if path.is_dir() && (!entry_is_sym || settings.scan_symbolic_links)
                                {
                                    dirs_to_visit.push(path.clone());
                                    if let Ok(rel) = path.strip_prefix(src) {
                                        let dst_dir = base_dst.join(rel);
                                        dirs_to_create.push(dst_dir);
                                    }
                                } else {
                                    total_files += 1;
                                    if !entry_is_sym {
                                        if let Ok(meta) = entry.metadata() {
                                            total_bytes += meta.len();
                                        }
                                    }
                                    if let Ok(rel) = path.strip_prefix(src) {
                                        let dst_path = base_dst.join(rel);
                                        file_mappings.push((path, dst_path, entry_is_sym));
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                total_files += 1;
                if !is_sym {
                    if let Ok(meta) = src.metadata() {
                        total_bytes += meta.len();
                    }
                }
                if let Some(file_name) = src.file_name() {
                    let dst_path = destination_dir.join(file_name);
                    file_mappings.push((src.clone(), dst_path, is_sym));
                }
            }
        }

        // 2. Create the target folder structures
        for dir in dirs_to_create {
            let res = fs::create_dir_all(&dir);
            if res.is_err() {
                let admin_res = if settings.req_admin_modification {
                    #[cfg(target_os = "windows")]
                    {
                        use std::process::Command;
                        let dir_str = dir.to_string_lossy().replace('"', "\\\"");
                        let ps_arg = format!(
                            "Start-Process powershell -ArgumentList '-NoProfile -Command New-Item -ItemType Directory -Path \\\"{}\\\" -Force' -Verb RunAs -WindowStyle Hidden -Wait",
                            dir_str
                        );
                        Command::new("powershell")
                            .args(&["-NoProfile", "-Command", &ps_arg])
                            .status()
                            .map(|s| s.success())
                            .unwrap_or(false)
                    }
                    #[cfg(not(target_os = "windows"))]
                    {
                        use std::process::Command;
                        Command::new("sudo")
                            .arg("mkdir")
                            .arg("-p")
                            .arg(&dir)
                            .status()
                            .map(|s| s.success())
                            .unwrap_or(false)
                    }
                } else {
                    false
                };

                if !admin_res {
                    let e = res.err().unwrap();
                    let _ = tx
                        .send(ProgressUpdate {
                            current_file: dir.to_string_lossy().into_owned(),
                            files_copied: 0,
                            total_files,
                            bytes_copied: 0,
                            total_bytes,
                            error: Some(format!("Failed to create folder {:?}: {}", dir, e)),
                        })
                        .await;
                    return;
                }
            }
        }

        // 3. Copy files block by block
        let mut files_copied = 0;
        let mut bytes_copied = 0;

        // In case there were only empty folders, trigger a finish
        if file_mappings.is_empty() {
            let _ = tx
                .send(ProgressUpdate {
                    current_file: "Completed".to_string(),
                    files_copied: total_files,
                    total_files,
                    bytes_copied: total_bytes,
                    total_bytes,
                    error: None,
                })
                .await;
            return;
        }

        for (src, dst, is_sym) in file_mappings {
            let file_name = src
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();

            // Send starting file notification
            let _ = tx
                .send(ProgressUpdate {
                    current_file: file_name.clone(),
                    files_copied,
                    total_files,
                    bytes_copied,
                    total_bytes,
                    error: None,
                })
                .await;

            if let Some(parent) = dst.parent() {
                let _ = fs::create_dir_all(parent);
            }

            if is_sym {
                let mut res = copy_symlink(&src, &dst);
                if res.is_err() && settings.req_admin_modification {
                    res = run_as_admin_copy(&src, &dst);
                }
                match res {
                    Ok(_) => {
                        files_copied += 1;
                    }
                    Err(e) => {
                        let _ = tx
                            .send(ProgressUpdate {
                                current_file: file_name,
                                files_copied,
                                total_files,
                                bytes_copied,
                                total_bytes,
                                error: Some(format!("Error copying symlink {:?}: {}", src, e)),
                            })
                            .await;
                        return;
                    }
                }
            } else if settings.use_system_copy_routine {
                let mut res: anyhow::Result<()> =
                    std::fs::copy(&src, &dst).map(|_| ()).map_err(|e| e.into());
                if res.is_err() && settings.req_admin_modification {
                    res = run_as_admin_copy(&src, &dst);
                }
                match res {
                    Ok(_) => {
                        if let Ok(meta) = src.metadata() {
                            bytes_copied += meta.len();
                        }
                        files_copied += 1;
                        let _ = tx
                            .send(ProgressUpdate {
                                current_file: file_name,
                                files_copied,
                                total_files,
                                bytes_copied,
                                total_bytes,
                                error: None,
                            })
                            .await;
                    }
                    Err(e) => {
                        let _ = tx
                            .send(ProgressUpdate {
                                current_file: file_name,
                                files_copied,
                                total_files,
                                bytes_copied,
                                total_bytes,
                                error: Some(format!("Error copying file {:?}: {}", src, e)),
                            })
                            .await;
                        return;
                    }
                }
            } else {
                let mut res = copy_file_buffered(
                    &src,
                    &dst,
                    &tx,
                    &mut bytes_copied,
                    &file_name,
                    files_copied,
                    total_files,
                    total_bytes,
                    settings.copy_files_opened_for_writing,
                )
                .await;
                if res.is_err() && settings.req_admin_modification {
                    res = run_as_admin_copy(&src, &dst);
                    if res.is_ok() {
                        if let Ok(meta) = src.metadata() {
                            bytes_copied += meta.len();
                        }
                    }
                }
                match res {
                    Ok(_) => {
                        files_copied += 1;
                    }
                    Err(e) => {
                        let _ = tx
                            .send(ProgressUpdate {
                                current_file: file_name,
                                files_copied,
                                total_files,
                                bytes_copied,
                                total_bytes,
                                error: Some(format!("Error copying file {:?}: {}", src, e)),
                            })
                            .await;
                        return;
                    }
                }
            }
        }

        // 4. Send final completion update
        let _ = tx
            .send(ProgressUpdate {
                current_file: "Completed".to_string(),
                files_copied,
                total_files,
                bytes_copied,
                total_bytes,
                error: None,
            })
            .await;
    });

    rx
}

/// Copies a single file in chunks to allow cancellation or smooth progress updates.
#[allow(clippy::too_many_arguments)]
async fn copy_file_buffered(
    src: &Path,
    dst: &Path,
    tx: &mpsc::Sender<ProgressUpdate>,
    global_bytes_copied: &mut u64,
    file_name: &str,
    files_copied: usize,
    total_files: usize,
    total_bytes: u64,
    copy_files_opened_for_writing: bool,
) -> Result<()> {
    use std::io::{Read, Write};
    let mut src_file = if copy_files_opened_for_writing {
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::fs::OpenOptionsExt;
            std::fs::OpenOptions::new()
                .read(true)
                .share_mode(7) // FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE
                .open(src)?
        }
        #[cfg(not(target_os = "windows"))]
        {
            fs::File::open(src)?
        }
    } else {
        fs::File::open(src)?
    };
    let mut dst_file = fs::File::create(dst)?;

    let mut buffer = vec![0; 64 * 1024]; // 64 KB buffer size
    loop {
        let bytes_read = src_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        dst_file.write_all(&buffer[..bytes_read])?;
        *global_bytes_copied += bytes_read as u64;

        // Stream current status update
        let _ = tx
            .send(ProgressUpdate {
                current_file: file_name.to_string(),
                files_copied,
                total_files,
                bytes_copied: *global_bytes_copied,
                total_bytes,
                error: None,
            })
            .await;
    }
    Ok(())
}

pub fn spawn_extract_task(
    archive_path: PathBuf,
    destination_dir: PathBuf,
) -> mpsc::Receiver<ProgressUpdate> {
    let (tx, rx) = mpsc::channel(100);
    tokio::task::spawn_blocking(move || {
        if let Err(e) = extract_archive(&archive_path, &destination_dir, &tx) {
            let _ = tx.blocking_send(ProgressUpdate {
                current_file: archive_path.to_string_lossy().into_owned(),
                files_copied: 0,
                total_files: 0,
                bytes_copied: 0,
                total_bytes: 0,
                error: Some(format!("Extraction failed: {}", e)),
            });
        } else {
            let _ = tx.blocking_send(ProgressUpdate {
                current_file: "Completed".to_string(),
                files_copied: 1,
                total_files: 1,
                bytes_copied: 0,
                total_bytes: 0,
                error: None,
            });
        }
    });
    rx
}

pub fn spawn_compress_task(
    sources: Vec<PathBuf>,
    dest_archive: PathBuf,
) -> mpsc::Receiver<ProgressUpdate> {
    let (tx, rx) = mpsc::channel(100);
    tokio::task::spawn_blocking(move || {
        if let Err(e) = compress_zip(sources, &dest_archive, &tx) {
            let _ = tx.blocking_send(ProgressUpdate {
                current_file: dest_archive.to_string_lossy().into_owned(),
                files_copied: 0,
                total_files: 0,
                bytes_copied: 0,
                total_bytes: 0,
                error: Some(format!("Compression failed: {}", e)),
            });
        } else {
            let _ = tx.blocking_send(ProgressUpdate {
                current_file: "Completed".to_string(),
                files_copied: 1,
                total_files: 1,
                bytes_copied: 0,
                total_bytes: 0,
                error: None,
            });
        }
    });
    rx
}

/// Spawns a background task that securely wipes each file in `targets`.
/// Uses the same progress channel pattern as `spawn_copy_task`.
pub fn spawn_wipe_task(targets: Vec<PathBuf>) -> mpsc::Receiver<ProgressUpdate> {
    let (tx, rx) = mpsc::channel(64);
    let total = targets.len();

    tokio::task::spawn_blocking(move || {
        for (idx, path) in targets.iter().enumerate() {
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| path.to_string_lossy().into_owned());

            let _ = tx.blocking_send(ProgressUpdate {
                current_file: name.clone(),
                files_copied: idx,
                total_files: total,
                bytes_copied: 0,
                total_bytes: 0,
                error: None,
            });

            if let Err(e) = crate::fs::wipe::wipe_file(path) {
                let _ = tx.blocking_send(ProgressUpdate {
                    current_file: "Completed".to_string(),
                    files_copied: idx,
                    total_files: total,
                    bytes_copied: 0,
                    total_bytes: 0,
                    error: Some(format!("Wipe failed for {:?}: {}", path, e)),
                });
                return;
            }
        }

        let _ = tx.blocking_send(ProgressUpdate {
            current_file: "Completed".to_string(),
            files_copied: total,
            total_files: total,
            bytes_copied: 0,
            total_bytes: 0,
            error: None,
        });
    });

    rx
}
