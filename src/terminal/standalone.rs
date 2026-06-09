use anyhow::Result;
use std::env;
use std::process::Command;

/// Checks if the `--standalone` argument was provided.
/// If it is present, it launches a new independent terminal window running
/// the same executable (without the `--standalone` flag) and returns `true`.
/// If the flag is not present, returns `false`.
pub fn check_and_launch_standalone() -> Result<bool> {
    let args: Vec<String> = env::args().collect();
    
    if let Some(pos) = args.iter().position(|x| x == "--standalone") {
        let current_exe = env::current_exe()?;
        
        let mut new_args = args.clone();
        new_args.remove(pos);
        if !new_args.is_empty() {
            new_args.remove(0); // remove the executable path itself
        }

        #[cfg(target_os = "windows")]
        {
            let mut cmd = Command::new("cmd.exe");
            cmd.arg("/c").arg("start").arg("NCRust").arg(&current_exe);
            for arg in new_args {
                cmd.arg(arg);
            }
            cmd.spawn()?;
        }

        #[cfg(target_os = "linux")]
        {
            let terminals = [
                "x-terminal-emulator",
                "gnome-terminal",
                "konsole",
                "xfce4-terminal",
                "alacritty",
                "kitty",
                "xterm",
            ];
            
            let mut spawned = false;
            for term in terminals {
                let mut cmd = Command::new(term);
                cmd.arg("-e").arg(&current_exe);
                for arg in &new_args {
                    cmd.arg(arg);
                }
                
                if cmd.spawn().is_ok() {
                    spawned = true;
                    break;
                }
            }
            
            if !spawned {
                return Ok(false); // Fallback to running in current terminal
            }
        }

        #[cfg(target_os = "macos")]
        {
            let mut cmd = Command::new("open");
            cmd.arg("-a").arg("Terminal").arg(&current_exe);
            for arg in new_args {
                cmd.arg("--args").arg(arg);
            }
            cmd.spawn()?;
        }

        return Ok(true);
    }
    
    Ok(false)
}
