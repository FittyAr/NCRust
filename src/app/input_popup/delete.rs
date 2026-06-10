use crate::app::context::AppContext;
use crate::app::state::{AppState, PopupType};
use crate::keybindings::Action;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle(
    state: &mut AppState,
    key: KeyEvent,
    context: &mut AppContext,
) -> Result<Option<Action>, ()> {
    let popup = state.active_popup.clone();
    if let Some(p) = popup {
        match p {
            PopupType::ConfirmDelete {
                paths,
                cursor_idx: _,
            } => {
                match key.code {
                    KeyCode::Enter => {
                        for path in &paths {
                            if let Err(e) = crate::fs::delete_sync(
                                path,
                                context.config.settings.delete_to_recycle_bin,
                                context.config.settings.req_admin_modification,
                            ) {
                                state.active_popup = Some(PopupType::Error(format!(
                                    "{} {}",
                                    crate::config::localization::t("error_delete_failed"),
                                    e
                                )));
                                return Ok(None);
                            }
                        }
                        state.active_popup = None;
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
            }
            PopupType::WipeConfirm { paths } => {
                match key.code {
                    KeyCode::Enter => {
                        state.active_popup = None;
                        let rx = crate::fs::spawn_wipe_task(paths);
                        state.progress_rx = Some(rx);
                        state.active_popup = Some(PopupType::CopyProgress {
                            current_file: crate::config::localization::t("progress_wiping"),
                            files_copied: 0,
                            total_files: 0,
                            bytes_copied: 0,
                            total_bytes: 0,
                        });
                        return Ok(None);
                    }
                    KeyCode::Esc => {
                        state.active_popup = None;
                        return Ok(None);
                    }
                    _ => {}
                }
                Err(())
            }
            _ => Err(()),
        }
    } else {
        Err(())
    }
}
