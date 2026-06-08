use crate::app::context::AppContext;
use crate::app::state::{AppState, PopupType};
use crate::keybindings::Action;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle(
    state: &mut AppState,
    key: KeyEvent,
    _context: &mut AppContext,
) -> Result<Option<Action>, ()> {
    if let Some(PopupType::InternalViewer { mut viewer }) = state.active_popup.clone() {
        match key.code {
            KeyCode::Esc | KeyCode::F(10) => {
                state.active_popup = None;
                return Ok(None);
            }
            KeyCode::Up => {
                viewer.scroll_up(1);
            }
            KeyCode::Down => {
                viewer.scroll_down(1);
            }
            KeyCode::PageUp => {
                viewer.scroll_up(18);
            }
            KeyCode::PageDown => {
                viewer.scroll_down(18);
            }
            KeyCode::F(2) => {
                viewer.toggle_mode();
            }
            _ => {}
        }
        state.active_popup = Some(PopupType::InternalViewer { viewer });
        Ok(None)
    } else {
        Err(())
    }
}
