use crate::app::context::AppContext;
use crate::app::state::{AppState, FileAttrsSnapshot, LinkKind, PopupType};
use crate::keybindings::Action;
use crate::terminal::TerminalBackend;

/// Handles filesystem-related actions. Returns `true` if the action was handled.
pub fn handle_fs_action(
    state: &mut AppState,
    action: &Action,
    context: &mut AppContext,
    terminal_backend: &mut TerminalBackend,
) -> bool {
    match action {
        Action::View => {
            let active = state.get_active_panel();
            if let Some(entry) = active
                .entries
                .get(active.cursor_index)
                .filter(|e| !e.is_dir)
            {
                let path = entry.path.clone();
                let entry_name = entry.name.clone();
                state.push_file_view_history(path.clone());

                let rule = crate::config::associations::AssociationsConfig::load()
                    .find_rule(&entry_name)
                    .cloned();

                if let Some(ref r) = rule {
                    let cmd = r.resolve_view_cmd(&path);
                    if let Err(e) = super::exec::execute_external_command(
                        &path,
                        &cmd,
                        context,
                        terminal_backend,
                    ) {
                        state.active_popup =
                            Some(PopupType::Error(format!("Failed to run viewer: {}", e)));
                    }
                } else {
                    let viewer = crate::ui::viewer::ViewerState::load(path);
                    state.active_popup = Some(PopupType::InternalViewer { viewer });
                }
            }
            true
        }
        Action::Edit => {
            let active = state.get_active_panel();
            if let Some(entry) = active
                .entries
                .get(active.cursor_index)
                .filter(|e| !e.is_dir)
            {
                let path = entry.path.clone();
                state.push_file_view_history(path.clone());
                match std::fs::read_to_string(&path) {
                    Ok(content) => {
                        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
                        state.active_popup = Some(PopupType::InternalEditor {
                            path,
                            lines: if lines.is_empty() {
                                vec![String::new()]
                            } else {
                                lines
                            },
                            cursor_x: 0,
                            cursor_y: 0,
                            scroll_y: 0,
                            is_dirty: false,
                            last_search: None,
                        });
                    }
                    Err(e) => {
                        state.active_popup =
                            Some(PopupType::Error(format!("Cannot read file: {}", e)));
                    }
                }
            }
            true
        }
        Action::Copy => {
            let targets = state.get_active_panel().get_targeted_paths();
            if !targets.is_empty() {
                let dest = state.get_passive_panel().current_path.clone();
                let rx = crate::fs::spawn_copy_task(targets, dest, context.config.settings.clone());
                state.progress_rx = Some(rx);
                state.active_popup = Some(PopupType::CopyProgress {
                    current_file: "Initializing...".to_string(),
                    files_copied: 0,
                    total_files: 0,
                    bytes_copied: 0,
                    total_bytes: 0,
                });
            }
            true
        }
        Action::Move => {
            let targets = state.get_active_panel().get_targeted_paths();
            if !targets.is_empty() {
                let dest_dir = state.get_passive_panel().current_path.clone();
                let default_input = targets
                    .first()
                    .and_then(|p| p.file_name())
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();
                state.active_popup = Some(PopupType::RenMovPrompt {
                    input: default_input,
                    src_paths: targets,
                    dest_dir,
                });
            }
            true
        }
        Action::CompressFiles => {
            let targets = state.get_active_panel().get_targeted_paths();
            if !targets.is_empty() {
                let default_name = targets
                    .first()
                    .and_then(|p| p.file_name())
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| "archive".to_string());
                let dest_dir = state.get_passive_panel().current_path.clone();
                state.active_popup = Some(PopupType::CompressPrompt {
                    input: default_name,
                    targets,
                    dest_dir,
                });
            }
            true
        }
        Action::ExtractArchive => {
            let active = state.get_active_panel();
            if let Some(entry) = active
                .entries
                .get(active.cursor_index)
                .filter(|e| !e.is_dir)
            {
                let dest = state.get_passive_panel().current_path.clone();
                let rx = crate::fs::spawn_extract_task(entry.path.clone(), dest);
                state.progress_rx = Some(rx);
                state.active_popup = Some(PopupType::CopyProgress {
                    current_file: "Extracting...".to_string(),
                    files_copied: 0,
                    total_files: 0,
                    bytes_copied: 0,
                    total_bytes: 0,
                });
            }
            true
        }
        Action::MkDir => {
            state.active_popup = Some(PopupType::MkDirPrompt {
                input: String::new(),
            });
            true
        }
        Action::Delete => {
            let targets = state.get_active_panel().get_targeted_paths();
            if !targets.is_empty() {
                state.active_popup = Some(PopupType::ConfirmDelete { paths: targets });
            }
            true
        }
        Action::WipeFile => {
            let targets = state.get_active_panel().get_targeted_paths();
            if !targets.is_empty() {
                state.active_popup = Some(PopupType::WipeConfirm { paths: targets });
            }
            true
        }
        Action::CreateLink => {
            let active = state.get_active_panel();
            if let Some(entry) = active.entries.get(active.cursor_index) {
                if entry.name != ".." {
                    state.active_popup = Some(PopupType::CreateLinkPrompt {
                        src: entry.path.clone(),
                        dest_input: entry.name.clone(),
                        kind: LinkKind::Symbolic,
                    });
                }
            }
            true
        }
        Action::FileAttributes => {
            let active = state.get_active_panel();
            if let Some(entry) = active.entries.get(active.cursor_index) {
                if entry.name != ".." {
                    match crate::fs::read_attrs(&entry.path) {
                        Ok(attrs) => {
                            let mode_octal = format!("{:o}", attrs.mode & 0o7777);
                            state.active_popup = Some(PopupType::FileAttributesDialog {
                                attrs: FileAttrsSnapshot {
                                    path: attrs.path,
                                    readonly: attrs.readonly,
                                    size: attrs.size,
                                    modified: attrs.modified,
                                    created: attrs.created,
                                    owner: attrs.owner,
                                    nlinks: attrs.nlinks,
                                },
                                mode_input: mode_octal,
                            });
                        }
                        Err(e) => {
                            state.active_popup =
                                Some(PopupType::Error(format!("Cannot read attrs: {}", e)));
                        }
                    }
                }
            }
            true
        }
        Action::ApplyCommand => {
            let targets = state.get_active_panel().get_targeted_paths();
            if !targets.is_empty() {
                state.active_popup = Some(PopupType::ApplyCommandPrompt {
                    input: String::new(),
                    targets,
                });
            }
            true
        }
        Action::DescribeFile => {
            let active = state.get_active_panel();
            if let Some(entry) = active.entries.get(active.cursor_index) {
                if entry.name != ".." {
                    let current_desc =
                        crate::fs::read_description(&active.current_path.clone(), &entry.name)
                            .unwrap_or_default();
                    state.active_popup = Some(PopupType::DescribeFilePrompt {
                        path: entry.path.clone(),
                        current_desc: current_desc.clone(),
                        input: current_desc,
                    });
                }
            }
            true
        }
        Action::ArchiveCommands => {
            let active = state.get_active_panel();
            if let Some(entry) = active
                .entries
                .get(active.cursor_index)
                .filter(|e| !e.is_dir)
            {
                state.active_popup = Some(PopupType::ArchiveCommandsMenu {
                    archive_path: entry.path.clone(),
                    items: vec![
                        "1. List contents".to_string(),
                        "2. Test integrity".to_string(),
                        "3. Extract here".to_string(),
                        "4. Extract to other panel".to_string(),
                    ],
                    cursor_idx: 0,
                });
            }
            true
        }
        _ => false,
    }
}
