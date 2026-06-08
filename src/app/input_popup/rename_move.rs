use crate::app::context::AppContext;
use crate::app::state::{AppState, PopupType};
use crate::keybindings::Action;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle(
    state: &mut AppState,
    key: KeyEvent,
    context: &mut AppContext,
) -> Result<Option<Action>, ()> {
    if let Some(PopupType::RenMovPrompt {
        input,
        src_paths,
        dest_dir,
    }) = state.active_popup.clone()
    {
        match key.code {
            KeyCode::Char(c) => {
                let mut new_input = input;
                new_input.push(c);
                state.active_popup = Some(PopupType::RenMovPrompt {
                    input: new_input,
                    src_paths,
                    dest_dir,
                });
                return Ok(None);
            }
            KeyCode::Backspace => {
                let mut new_input = input;
                new_input.pop();
                state.active_popup = Some(PopupType::RenMovPrompt {
                    input: new_input,
                    src_paths,
                    dest_dir,
                });
                return Ok(None);
            }
            KeyCode::Enter => {
                state.active_popup = None;

                if src_paths.len() == 1 {
                    // Single item: use the input string as the new filename
                    let dst = dest_dir.join(&input);
                    if let Err(e) = crate::fs::rename_or_move_sync(
                        &src_paths[0],
                        &dst,
                        context.config.settings.req_admin_modification,
                    ) {
                        state.active_popup = Some(PopupType::Error(format!("Move failed: {}", e)));
                    }
                } else {
                    // Multiple items: move all into dest_dir (ignore input as filename)
                    for src in &src_paths {
                        if let Some(fname) = src.file_name() {
                            let dst = dest_dir.join(fname);
                            if let Err(e) = crate::fs::rename_or_move_sync(
                                src,
                                &dst,
                                context.config.settings.req_admin_modification,
                            ) {
                                state.active_popup =
                                    Some(PopupType::Error(format!("Move failed: {}", e)));
                                break;
                            }
                        }
                    }
                }

                state.get_active_panel_mut().selected_paths.clear();
                state.refresh_both_panels(context.config.settings.show_hidden);
                return Ok(None);
            }
            KeyCode::Esc => {
                state.active_popup = None;
                return Ok(None);
            }
            _ => {}
        }
        Err(())
    } else {
        Err(())
    }
}
