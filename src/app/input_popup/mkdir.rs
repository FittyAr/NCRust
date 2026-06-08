use crate::app::context::AppContext;
use crate::app::state::{AppState, PopupType};
use crate::keybindings::Action;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle(
    state: &mut AppState,
    key: KeyEvent,
    context: &mut AppContext,
) -> Result<Option<Action>, ()> {
    if let Some(PopupType::MkDirPrompt { ref input }) = state.active_popup {
        match key.code {
            KeyCode::Char(c) => {
                let mut new_input = input.clone();
                new_input.push(c);
                state.active_popup = Some(PopupType::MkDirPrompt { input: new_input });
                return Ok(None);
            }
            KeyCode::Backspace => {
                let mut new_input = input.clone();
                new_input.pop();
                state.active_popup = Some(PopupType::MkDirPrompt { input: new_input });
                return Ok(None);
            }
            KeyCode::Enter => {
                if !input.is_empty() {
                    let path = state.get_active_panel().current_path.join(input);
                    if let Err(e) = crate::fs::create_directory(&path) {
                        state.active_popup =
                            Some(PopupType::Error(format!("Directory error: {}", e)));
                    } else {
                        state.active_popup = None;
                        state.refresh_both_panels(context.config.settings.show_hidden);
                    }
                } else {
                    state.active_popup = None;
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
