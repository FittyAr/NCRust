use crate::app::context::AppContext;
use crate::app::state::{AppState, PopupType};
use crate::keybindings::Action;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle(
    state: &mut AppState,
    key: KeyEvent,
    _context: &mut AppContext,
) -> Result<Option<Action>, ()> {
    if let Some(PopupType::FilePanelFilterPrompt { input }) = state.active_popup.clone() {
        match key.code {
            KeyCode::Char(c) => {
                let mut new_input = input;
                new_input.push(c);
                state.active_popup = Some(PopupType::FilePanelFilterPrompt { input: new_input });
                return Ok(None);
            }
            KeyCode::Backspace => {
                let mut new_input = input;
                new_input.pop();
                state.active_popup = Some(PopupType::FilePanelFilterPrompt { input: new_input });
                return Ok(None);
            }
            KeyCode::Enter => {
                let mask = input.trim().to_string();
                state.active_popup = None;
                let panel = state.get_active_panel_mut();
                panel.filter_mask = if mask.is_empty() { None } else { Some(mask) };
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
