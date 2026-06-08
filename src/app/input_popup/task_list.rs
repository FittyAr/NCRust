use crate::app::context::AppContext;
use crate::app::state::{AppState, PopupType};
use crate::app::sys_helpers::kill_process;
use crate::keybindings::Action;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle(
    state: &mut AppState,
    key: KeyEvent,
    _context: &mut AppContext,
) -> Result<Option<Action>, ()> {
    if let Some(PopupType::TaskListDialog {
        mut tasks,
        mut cursor_idx,
    }) = state.active_popup.clone()
    {
        match key.code {
            KeyCode::Esc => {
                state.active_popup = None;
                return Ok(None);
            }
            KeyCode::Up => {
                if cursor_idx > 0 {
                    cursor_idx -= 1;
                }
            }
            KeyCode::Down => {
                if !tasks.is_empty() && cursor_idx < tasks.len().saturating_sub(1) {
                    cursor_idx += 1;
                }
            }
            KeyCode::Delete | KeyCode::Char('k') => {
                if let Some(task) = tasks.get(cursor_idx) {
                    let pid = task.pid;
                    match kill_process(pid) {
                        Ok(_) => {
                            tasks.remove(cursor_idx);
                            if cursor_idx >= tasks.len() && cursor_idx > 0 {
                                cursor_idx = tasks.len().saturating_sub(1);
                            }
                        }
                        Err(e) => {
                            state.active_popup =
                                Some(PopupType::Error(format!("Failed to kill process: {}", e)));
                            return Ok(None);
                        }
                    }
                }
            }
            _ => {}
        }
        state.active_popup = Some(PopupType::TaskListDialog { tasks, cursor_idx });
        return Ok(None);
    }
    Err(())
}
