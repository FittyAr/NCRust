use crate::app::context::AppContext;
use crate::app::state::{AppState, LinkKind, PopupType};
use crate::keybindings::Action;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle(
    state: &mut AppState,
    key: KeyEvent,
    context: &mut AppContext,
) -> Result<Option<Action>, ()> {
    if let Some(PopupType::CreateLinkPrompt {
        src,
        dest_input,
        kind,
    }) = state.active_popup.clone()
    {
        match key.code {
            KeyCode::Char('s') | KeyCode::Char('h') => {
                let new_kind = match key.code {
                    KeyCode::Char('s') => LinkKind::Symbolic,
                    _ => LinkKind::Hard,
                };
                state.active_popup = Some(PopupType::CreateLinkPrompt {
                    src,
                    dest_input,
                    kind: new_kind,
                });
                return Ok(None);
            }
            KeyCode::Char(c) => {
                let mut new_input = dest_input;
                new_input.push(c);
                state.active_popup = Some(PopupType::CreateLinkPrompt {
                    src,
                    dest_input: new_input,
                    kind,
                });
                return Ok(None);
            }
            KeyCode::Backspace => {
                let mut new_input = dest_input;
                new_input.pop();
                state.active_popup = Some(PopupType::CreateLinkPrompt {
                    src,
                    dest_input: new_input,
                    kind,
                });
                return Ok(None);
            }
            KeyCode::Enter => {
                let dest = state.get_passive_panel().current_path.join(&dest_input);
                state.active_popup = None;
                let result = match kind {
                    LinkKind::Symbolic => crate::fs::create_symlink(&src, &dest),
                    LinkKind::Hard => crate::fs::create_hardlink(&src, &dest),
                };
                if let Err(e) = result {
                    state.active_popup = Some(PopupType::Error(format!("Link failed: {}", e)));
                } else {
                    state.refresh_both_panels(context.config.settings.show_hidden);
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
