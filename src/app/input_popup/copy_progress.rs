use crate::app::context::AppContext;
use crate::app::state::{AppState, PopupType};
use crate::keybindings::Action;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle(
    state: &mut AppState,
    key: KeyEvent,
    context: &mut AppContext,
) -> Result<Option<Action>, ()> {
    if let Some(PopupType::CopyProgress { .. }) = state.active_popup {
        if key.code == KeyCode::Esc {
            // Drop channel to signal abort to tokio background thread
            state.progress_rx = None;
            state.active_popup = None;
            state.refresh_both_panels(context.config.settings.show_hidden);
            return Ok(None);
        }
        Err(())
    } else {
        Err(())
    }
}
