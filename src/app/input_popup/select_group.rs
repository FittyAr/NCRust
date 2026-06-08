use crate::app::context::AppContext;
use crate::app::state::{AppState, PopupType, SelectMode};
use crate::keybindings::Action;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle(
    state: &mut AppState,
    key: KeyEvent,
    _context: &mut AppContext,
) -> Result<Option<Action>, ()> {
    if let Some(PopupType::SelectGroupPrompt { mode, query }) = state.active_popup.clone() {
        match key.code {
            KeyCode::Char(c) => {
                let mut new_q = query;
                new_q.push(c);
                state.active_popup = Some(PopupType::SelectGroupPrompt { mode, query: new_q });
                return Ok(None);
            }
            KeyCode::Backspace => {
                let mut new_q = query;
                new_q.pop();
                state.active_popup = Some(PopupType::SelectGroupPrompt { mode, query: new_q });
                return Ok(None);
            }
            KeyCode::Enter => {
                state.active_popup = None;
                match mode {
                    SelectMode::Add => state.get_active_panel_mut().select_group(&query),
                    SelectMode::Remove => state.get_active_panel_mut().unselect_group(&query),
                }
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
