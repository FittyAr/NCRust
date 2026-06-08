use crate::app::context::AppContext;
use crate::app::state::{ActivePanel, AppState, PopupType};
use crate::keybindings::Action;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle(
    state: &mut AppState,
    key: KeyEvent,
    context: &mut AppContext,
) -> Result<Option<Action>, ()> {
    if let Some(PopupType::TreeView {
        nodes,
        cursor_idx,
        panel,
    }) = state.active_popup.clone()
    {
        match key.code {
            KeyCode::Esc => {
                state.active_popup = None;
                return Ok(None);
            }
            KeyCode::Up => {
                if !nodes.is_empty() {
                    let new_idx = if cursor_idx > 0 {
                        cursor_idx - 1
                    } else {
                        nodes.len() - 1
                    };
                    state.active_popup = Some(PopupType::TreeView {
                        nodes,
                        cursor_idx: new_idx,
                        panel,
                    });
                }
                return Ok(None);
            }
            KeyCode::Down => {
                if !nodes.is_empty() {
                    let new_idx = if cursor_idx < nodes.len() - 1 {
                        cursor_idx + 1
                    } else {
                        0
                    };
                    state.active_popup = Some(PopupType::TreeView {
                        nodes,
                        cursor_idx: new_idx,
                        panel,
                    });
                }
                return Ok(None);
            }
            KeyCode::Enter => {
                if let Some(node) = nodes.get(cursor_idx) {
                    let target = if node.is_dir {
                        node.path.clone()
                    } else {
                        node.path
                            .parent()
                            .map(|p| p.to_path_buf())
                            .unwrap_or_else(|| node.path.clone())
                    };
                    match panel {
                        ActivePanel::Left => {
                            state.left_panel.current_path = target;
                            state.left_panel.cursor_index = 0;
                            state.left_panel.selected_paths.clear();
                        }
                        ActivePanel::Right => {
                            state.right_panel.current_path = target;
                            state.right_panel.cursor_index = 0;
                            state.right_panel.selected_paths.clear();
                        }
                    }
                    state.active_popup = None;
                    state.refresh_both_panels(context.config.settings.show_hidden);
                }
                return Ok(None);
            }
            _ => {}
        }
        Err(())
    } else {
        Err(())
    }
}
