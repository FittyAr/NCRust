use crate::app::context::AppContext;
use crate::app::state::{AppState, PopupType};
use crate::keybindings::Action;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle(
    state: &mut AppState,
    key: KeyEvent,
    context: &mut AppContext,
) -> Result<Option<Action>, ()> {
    if let Some(PopupType::SaveSetupConfirm) = state.active_popup {
        match key.code {
            KeyCode::Enter => {
                match context.config.save() {
                    Ok(_) => {
                        state.active_popup = Some(PopupType::Info(
                            "Configuration saved successfully.".to_string(),
                        ));
                    }
                    Err(e) => {
                        state.active_popup =
                            Some(PopupType::Error(format!("Failed to save setup: {}", e)));
                    }
                }
                return Ok(None);
            }
            KeyCode::Esc => {
                state.active_popup = None;
                return Ok(None);
            }
            _ => {}
        }
        Ok(None)
    } else {
        Err(())
    }
}
