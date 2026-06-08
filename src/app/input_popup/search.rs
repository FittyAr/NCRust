use crate::app::context::AppContext;
use crate::app::state::{AppState, PopupType};
use crate::app::sys_helpers::search_files_recursive;
use crate::keybindings::Action;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle(
    state: &mut AppState,
    key: KeyEvent,
    context: &mut AppContext,
) -> Result<Option<Action>, ()> {
    let popup = state.active_popup.clone();
    if let Some(p) = popup {
        match p {
            PopupType::SearchPrompt {
                query,
                content_query,
                search_root,
                focus_content,
            } => {
                match key.code {
                    KeyCode::Tab | KeyCode::Up | KeyCode::Down => {
                        state.active_popup = Some(PopupType::SearchPrompt {
                            query,
                            content_query,
                            search_root,
                            focus_content: !focus_content,
                        });
                        return Ok(None);
                    }
                    KeyCode::Char(c) => {
                        let mut new_query = query;
                        let mut new_content = content_query;
                        if focus_content {
                            new_content.push(c);
                        } else {
                            new_query.push(c);
                        }
                        state.active_popup = Some(PopupType::SearchPrompt {
                            query: new_query,
                            content_query: new_content,
                            search_root,
                            focus_content,
                        });
                        return Ok(None);
                    }
                    KeyCode::Backspace => {
                        let mut new_query = query;
                        let mut new_content = content_query;
                        if focus_content {
                            new_content.pop();
                        } else {
                            new_query.pop();
                        }
                        state.active_popup = Some(PopupType::SearchPrompt {
                            query: new_query,
                            content_query: new_content,
                            search_root,
                            focus_content,
                        });
                        return Ok(None);
                    }
                    KeyCode::Enter => {
                        let q = query.clone();
                        let c_q = content_query.clone();
                        if !q.is_empty() || !c_q.is_empty() {
                            let results = search_files_recursive(
                                &search_root,
                                &q,
                                if c_q.is_empty() { None } else { Some(&c_q) },
                            );
                            state.active_popup = Some(PopupType::SearchResults {
                                query: if q.is_empty() { c_q } else { q },
                                results,
                                cursor_idx: 0,
                            });
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
            }
            PopupType::SearchResults {
                query,
                results,
                cursor_idx,
            } => {
                match key.code {
                    KeyCode::Esc => {
                        state.active_popup = None;
                        return Ok(None);
                    }
                    KeyCode::Up => {
                        if !results.is_empty() {
                            let new_idx = if cursor_idx > 0 {
                                cursor_idx - 1
                            } else {
                                results.len() - 1
                            };
                            state.active_popup = Some(PopupType::SearchResults {
                                query,
                                results,
                                cursor_idx: new_idx,
                            });
                        }
                        return Ok(None);
                    }
                    KeyCode::Down => {
                        if !results.is_empty() {
                            let new_idx = if cursor_idx < results.len() - 1 {
                                cursor_idx + 1
                            } else {
                                0
                            };
                            state.active_popup = Some(PopupType::SearchResults {
                                query,
                                results,
                                cursor_idx: new_idx,
                            });
                        }
                        return Ok(None);
                    }
                    KeyCode::Enter => {
                        if let Some(result_path) = results.get(cursor_idx) {
                            // Navigate the active panel to the directory containing the result
                            let target_dir = if result_path.is_dir() {
                                result_path.clone()
                            } else {
                                result_path
                                    .parent()
                                    .map(|p| p.to_path_buf())
                                    .unwrap_or_else(|| result_path.clone())
                            };
                            let panel = state.get_active_panel_mut();
                            panel.current_path = target_dir;
                            panel.cursor_index = 0;
                            panel.selected_paths.clear();
                            state.active_popup = None;
                            state.refresh_both_panels(context.config.settings.show_hidden);
                        }
                        return Ok(None);
                    }
                    _ => {}
                }
                Err(())
            }
            _ => Err(()),
        }
    } else {
        Err(())
    }
}
