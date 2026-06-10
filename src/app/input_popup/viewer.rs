use crate::app::context::AppContext;
use crate::app::state::{AppState, PopupType, Screen};
use crate::keybindings::Action;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle(
    state: &mut AppState,
    key: KeyEvent,
    _context: &mut AppContext,
) -> Result<Option<Action>, ()> {
    if let Some(popup) = state.active_popup.clone() {
        match popup {
            PopupType::ViewerSearchPrompt { mut query } => {
                match key.code {
                    KeyCode::Esc => {
                        state.active_popup = None;
                        return Ok(None);
                    }
                    KeyCode::Enter => {
                        state.active_popup = None;
                        if !query.is_empty() {
                            if let Some(Screen::Viewer(vw)) =
                                state.screens.get_mut(state.active_screen_idx)
                            {
                                vw.last_search = Some(query.clone());
                                if vw.mode == crate::ui::viewer::ViewerMode::Text {
                                    // simple downward search from current line
                                    if let Some(found_idx) = vw
                                        .lines
                                        .iter()
                                        .enumerate()
                                        .skip(vw.scroll)
                                        .find(|(_, l)| {
                                            l.to_lowercase().contains(&query.to_lowercase())
                                        })
                                        .map(|(i, _)| i)
                                    {
                                        vw.scroll = found_idx;
                                    } else if let Some(found_idx) = vw
                                        .lines
                                        .iter()
                                        .enumerate()
                                        .take(vw.scroll)
                                        .find(|(_, l)| {
                                            l.to_lowercase().contains(&query.to_lowercase())
                                        })
                                        .map(|(i, _)| i)
                                    {
                                        vw.scroll = found_idx;
                                    }
                                }
                            }
                        }
                        return Ok(None);
                    }
                    KeyCode::Backspace => {
                        query.pop();
                        state.active_popup = Some(PopupType::ViewerSearchPrompt { query });
                        return Ok(None);
                    }
                    KeyCode::Char(c) => {
                        query.push(c);
                        state.active_popup = Some(PopupType::ViewerSearchPrompt { query });
                        return Ok(None);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    Err(())
}
