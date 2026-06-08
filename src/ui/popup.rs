use crate::app::context::AppContext;
use crate::app::state::{
    ActivePanel, AppState, CompareStatus, LinkKind, PopupType, SelectMode, SortField,
};
use crate::ui::theme_apply::parse_color;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Gauge, Paragraph, Row, Table},
};

fn render_editor_widget(
    f: &mut Frame,
    area: Rect,
    path: &std::path::Path,
    lines: &[String],
    cursor_x: usize,
    cursor_y: usize,
    scroll_y: usize,
    is_dirty: bool,
    _theme: &crate::config::theme::Theme,
) {
    let title = format!(
        " Editor - {} {} ",
        path.to_string_lossy(),
        if is_dirty { "*" } else { "" }
    );

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(title)
        .style(Style::default().bg(Color::Blue));

    let inner = block.inner(area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(inner);
    let edit_area = chunks[0];
    let status_area = chunks[1];

    let height = edit_area.height as usize;
    let visible_lines: Vec<String> = lines.iter().skip(scroll_y).take(height).cloned().collect();

    let mut text = Vec::new();
    for (idx, line) in visible_lines.into_iter().enumerate() {
        let line_num = scroll_y + idx + 1;
        let prefix = format!("{:>4} │ ", line_num);
        text.push(ratatui::text::Line::from(format!("{}{}", prefix, line)));
    }

    let paragraph = Paragraph::new(text).style(Style::default().fg(Color::White));

    f.render_widget(block, area);
    f.render_widget(paragraph, edit_area);

    let current_line_len = lines.get(cursor_y).map(|l| l.len()).unwrap_or(0);
    let status_text = format!(
        " Line Chars: {} | Total Lines: {} | Pos: ({}, {})",
        current_line_len,
        lines.len(),
        cursor_y + 1,
        cursor_x + 1
    );
    let status_para =
        Paragraph::new(status_text).style(Style::default().bg(Color::Cyan).fg(Color::Black));
    f.render_widget(status_para, status_area);

    // Draw the terminal blinking cursor at the editing position
    let prefix_len = 7u16;
    let editor_cursor_x = edit_area.x + prefix_len + cursor_x as u16;
    let editor_cursor_y = edit_area.y + (cursor_y - scroll_y) as u16;

    if editor_cursor_x < edit_area.x + edit_area.width
        && editor_cursor_y < edit_area.y + edit_area.height
    {
        f.set_cursor(editor_cursor_x, editor_cursor_y);
    }
}

pub fn render_popup(
    f: &mut Frame,
    state: &AppState,
    context: &AppContext,
    left_rect: Rect,
    right_rect: Rect,
) {
    let popup = match &state.active_popup {
        Some(p) => p,
        None => return,
    };

    let theme = &context.config.theme;
    let size = f.size();

    match popup {
        PopupType::Help => {
            let area = centered_rect(60, 50, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(parse_color(&theme.popup_border)))
                .title(" Help - Keybindings ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let help_rows = vec![
                Row::new(vec!["Tab", "Switch active panel"]),
                Row::new(vec!["Insert / Space", "Tag/Select file for bulk ops"]),
                Row::new(vec!["F3", "View highlighted file contents"]),
                Row::new(vec!["F4", "Edit highlighted file"]),
                Row::new(vec!["F5", "Copy tagged files to passive panel"]),
                Row::new(vec!["F6", "Rename/Move files to passive panel"]),
                Row::new(vec!["F7", "Make new directory"]),
                Row::new(vec!["F8", "Delete tagged files"]),
                Row::new(vec!["Ctrl+H", "Toggle hidden files"]),
                Row::new(vec!["Ctrl+U", "Swap left and right panels"]),
                Row::new(vec!["F10", "Quit application"]),
                Row::new(vec!["Esc", "Close popup / Clear input"]),
            ];

            let table = Table::new(
                help_rows,
                [Constraint::Percentage(40), Constraint::Percentage(60)],
            )
            .block(block)
            .header(
                Row::new(vec!["Key", "Description"])
                    .style(Style::default().add_modifier(Modifier::BOLD)),
            );

            f.render_widget(table, area);
        }
        PopupType::MkDirPrompt { input } => {
            let area = centered_rect(50, 25, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(parse_color(&theme.popup_border)))
                .title(" Create Directory ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let text = format!("\nEnter directory name:\n\n > {}", input);
            let paragraph = Paragraph::new(text)
                .block(block)
                .style(Style::default().fg(parse_color(&theme.popup_fg)));

            f.render_widget(paragraph, area);
        }
        PopupType::ConfirmDelete { paths } => {
            let area = centered_rect(50, 25, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
                .title(" Confirm Deletion ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let text = format!(
                "\nAre you sure you want to delete {} item(s)?\n\n[Enter] Confirm Deletion\n[Esc] Cancel",
                paths.len()
            );
            let paragraph = Paragraph::new(text)
                .block(block)
                .style(Style::default().fg(parse_color(&theme.popup_fg)));

            f.render_widget(paragraph, area);
        }
        PopupType::CopyProgress {
            current_file,
            files_copied,
            total_files,
            bytes_copied,
            total_bytes,
        } => {
            let area = centered_rect(55, 30, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(parse_color(&theme.popup_border)))
                .title(" Copying Files ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let percent = bytes_copied
                .checked_mul(100)
                .and_then(|v| v.checked_div(*total_bytes))
                .unwrap_or(0) as u16;

            let inner_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1), // Spacer
                    Constraint::Length(2), // File labels
                    Constraint::Length(3), // Progress bar
                    Constraint::Min(1),    // Size counts
                ])
                .split(block.inner(area));

            let file_label = format!("File: {}", current_file);
            let paragraph =
                Paragraph::new(file_label).style(Style::default().fg(parse_color(&theme.popup_fg)));
            f.render_widget(paragraph, inner_chunks[1]);

            let gauge = Gauge::default()
                .gauge_style(Style::default().fg(Color::Yellow).bg(Color::DarkGray))
                .percent(percent.min(100))
                .label(format!("{}%", percent.min(100)));
            f.render_widget(gauge, inner_chunks[2]);

            let size_label = format!(
                "Files: {} / {}  |  Bytes: {} MB / {} MB",
                files_copied,
                total_files,
                *bytes_copied / (1024 * 1024),
                *total_bytes / (1024 * 1024)
            );
            let size_paragraph =
                Paragraph::new(size_label).style(Style::default().fg(parse_color(&theme.popup_fg)));
            f.render_widget(size_paragraph, inner_chunks[3]);

            f.render_widget(block, area);
        }
        PopupType::Error(message) => {
            let area = centered_rect(50, 25, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
                .title(" Error Alert ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let text = format!("\n {}\n\n[Press Enter/Esc to Dismiss]", message);
            let paragraph = Paragraph::new(text)
                .block(block)
                .style(Style::default().fg(Color::LightRed));

            f.render_widget(paragraph, area);
        }
        PopupType::Info(message) => {
            let area = centered_rect(55, 30, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(" Information ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let text = format!("\n {}\n\n[Press Enter/Esc to Dismiss]", message);
            let paragraph = Paragraph::new(text)
                .block(block)
                .style(Style::default().fg(parse_color(&theme.popup_fg)));

            f.render_widget(paragraph, area);
        }
        PopupType::UserMenu => {
            let area = centered_rect(50, 35, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(parse_color(&theme.popup_border)))
                .title(" User Commands Menu ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let menu_rows = vec![
                Row::new(vec!["1", "Refresh Panel Directories"]),
                Row::new(vec!["2", "Toggle Hidden Files"]),
                Row::new(vec!["3", "Swap Left and Right Panels"]),
                Row::new(vec!["4", "Show Help Keyboard Shortcuts"]),
                Row::new(vec!["5", "Close User Menu"]),
                Row::new(vec!["6", "Download 7z Extractor Tool"]),
            ];

            let table = Table::new(
                menu_rows,
                [Constraint::Percentage(20), Constraint::Percentage(80)],
            )
            .block(block)
            .header(
                Row::new(vec!["Key", "Command"])
                    .style(Style::default().add_modifier(Modifier::BOLD)),
            );

            f.render_widget(table, area);
        }
        PopupType::InternalEditor {
            path,
            lines,
            cursor_x,
            cursor_y,
            scroll_y,
            is_dirty,
            last_search: _,
        } => {
            let area = centered_rect(95, 90, size);
            f.render_widget(Clear, area);
            render_editor_widget(
                f, area, path, lines, *cursor_x, *cursor_y, *scroll_y, *is_dirty, theme,
            );
        }
        PopupType::EditorSearchPrompt {
            path,
            lines,
            cursor_x,
            cursor_y,
            scroll_y,
            is_dirty,
            last_search: _,
            query,
        } => {
            let area = centered_rect(95, 90, size);
            f.render_widget(Clear, area);
            render_editor_widget(
                f, area, path, lines, *cursor_x, *cursor_y, *scroll_y, *is_dirty, theme,
            );

            // Overlay search input popup
            let search_area = centered_rect(50, 15, size);
            f.render_widget(Clear, search_area);
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title(" Search Text ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let text = format!(
                "\n Search query:\n > {}\n\n [Enter] Search   [Esc] Cancel",
                query
            );

            let paragraph = Paragraph::new(text)
                .block(block)
                .style(Style::default().fg(parse_color(&theme.popup_fg)));

            f.render_widget(paragraph, search_area);
        }
        PopupType::InternalViewer { viewer } => {
            let area = centered_rect(95, 90, size);
            f.render_widget(Clear, area);
            crate::ui::viewer::render_viewer(f, area, viewer, theme);
        }
        PopupType::Menu {
            active_menu_idx,
            active_item_idx,
        } => {
            let items = crate::ui::menu::get_menu_items(*active_menu_idx);
            let dropdown_x = match active_menu_idx {
                0 => 2,
                1 => 10,
                2 => 19,
                3 => 31,
                4 => 42,
                _ => 2,
            };
            let dropdown_width = 30;
            let dropdown_height = (items.len() + 2) as u16;
            let dropdown_rect = Rect::new(dropdown_x, 1, dropdown_width, dropdown_height);

            f.render_widget(Clear, dropdown_rect);

            let mut lines = Vec::new();
            for (i, item) in items.iter().enumerate() {
                let is_cursor = i == *active_item_idx;
                let style = if is_cursor {
                    Style::default()
                        .bg(parse_color(&theme.selection_bg))
                        .fg(parse_color(&theme.selection_fg))
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(parse_color(&theme.popup_fg))
                };
                lines.push(Line::from(Span::styled(*item, style)));
            }

            let paragraph = Paragraph::new(lines).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(parse_color(&theme.popup_border)))
                    .style(Style::default().bg(parse_color(&theme.popup_bg))),
            );

            f.render_widget(paragraph, dropdown_rect);
        }
        PopupType::DriveSelect {
            panel,
            drives,
            cursor_idx,
        } => {
            // Center over the correct panel's rectangle
            let panel_rect = match panel {
                ActivePanel::Left => left_rect,
                ActivePanel::Right => right_rect,
            };
            let area = centered_rect_in(35, 60, panel_rect);
            f.render_widget(Clear, area);

            let mut lines = Vec::new();
            for (i, drive) in drives.iter().enumerate() {
                let is_cursor = i == *cursor_idx;
                let line_str = if is_cursor {
                    format!(" >  {} ", drive)
                } else {
                    format!("    {} ", drive)
                };
                let style = if is_cursor {
                    Style::default()
                        .bg(parse_color(&theme.selection_bg))
                        .fg(parse_color(&theme.selection_fg))
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(parse_color(&theme.popup_fg))
                };
                lines.push(Line::from(Span::styled(line_str, style)));
            }

            let panel_label = match panel {
                ActivePanel::Left => "Left",
                ActivePanel::Right => "Right",
            };
            let title = format!(" Select Drive ({}) ", panel_label);
            let paragraph = Paragraph::new(lines).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(parse_color(&theme.popup_border)))
                    .title(title)
                    .style(Style::default().bg(parse_color(&theme.popup_bg))),
            );

            f.render_widget(paragraph, area);
        }
        PopupType::Hotlist {
            bookmarks,
            cursor_idx,
        } => {
            let area = centered_rect(60, 40, size);
            f.render_widget(Clear, area);

            let mut lines = Vec::new();
            for (i, (name, path)) in bookmarks.iter().enumerate() {
                let is_cursor = i == *cursor_idx;
                let line_str = format!(" {:<20} ->  {} ", name, path.to_string_lossy());
                let style = if is_cursor {
                    Style::default()
                        .bg(parse_color(&theme.selection_bg))
                        .fg(parse_color(&theme.selection_fg))
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(parse_color(&theme.popup_fg))
                };
                lines.push(Line::from(Span::styled(line_str, style)));
            }

            let paragraph = Paragraph::new(lines).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(parse_color(&theme.popup_border)))
                    .title(" Directory Hotlist ")
                    .style(Style::default().bg(parse_color(&theme.popup_bg))),
            );

            f.render_widget(paragraph, area);
        }
        PopupType::RenMovPrompt {
            input,
            src_paths,
            dest_dir,
        } => {
            let area = centered_rect(60, 30, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title(" Rename / Move ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let count = src_paths.len();
            let first_name = src_paths
                .first()
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();

            let src_label = if count == 1 {
                format!("Moving: {}", first_name)
            } else {
                format!("Moving: {} items", count)
            };

            let text = format!(
                "\n {}\n Destination: {}\n\n > {}\n\n [Enter] Confirm   [Esc] Cancel",
                src_label,
                dest_dir.to_string_lossy(),
                input
            );

            let paragraph = Paragraph::new(text)
                .block(block)
                .style(Style::default().fg(parse_color(&theme.popup_fg)));

            f.render_widget(paragraph, area);
        }
        PopupType::SearchPrompt {
            query,
            content_query,
            search_root,
            focus_content,
        } => {
            let area = centered_rect(55, 25, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(" Search Files ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let q_pref = if !*focus_content { "► " } else { "  " };
            let c_pref = if *focus_content { "► " } else { "  " };

            let text = format!(
                "\n Search in: {}\n{}File name query: {}\n{}Content query: {}\n\n [Tab] Switch Field   [Enter] Search   [Esc] Cancel",
                search_root.to_string_lossy(),
                q_pref,
                query,
                c_pref,
                content_query
            );

            let paragraph = Paragraph::new(text)
                .block(block)
                .style(Style::default().fg(parse_color(&theme.popup_fg)));

            f.render_widget(paragraph, area);
        }
        PopupType::SearchResults {
            query,
            results,
            cursor_idx,
        } => {
            let area = centered_rect(70, 60, size);
            f.render_widget(Clear, area);

            let title = format!(" Search Results: \"{}\" ({} found) ", query, results.len());
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(title)
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let inner = block.inner(area);
            f.render_widget(block, area);

            if results.is_empty() {
                let paragraph = Paragraph::new("\n No files found.\n\n [Esc] Close")
                    .style(Style::default().fg(parse_color(&theme.popup_fg)));
                f.render_widget(paragraph, inner);
            } else {
                let list_height = inner.height.saturating_sub(2) as usize;
                let scroll_start = cursor_idx.saturating_sub(list_height / 2);
                let mut lines = Vec::new();

                for (i, path) in results
                    .iter()
                    .enumerate()
                    .skip(scroll_start)
                    .take(list_height)
                {
                    let is_cursor = i == *cursor_idx;
                    let display = path.to_string_lossy().to_string();
                    let style = if is_cursor {
                        Style::default()
                            .bg(parse_color(&theme.selection_bg))
                            .fg(parse_color(&theme.selection_fg))
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(parse_color(&theme.popup_fg))
                    };
                    lines.push(Line::from(Span::styled(format!(" {} ", display), style)));
                }

                let hint = Line::from(Span::styled(
                    " [Enter] Navigate to  [Esc] Close ",
                    Style::default().fg(Color::DarkGray),
                ));
                lines.push(Line::from(""));
                lines.push(hint);

                let paragraph =
                    Paragraph::new(lines).style(Style::default().fg(parse_color(&theme.popup_fg)));
                f.render_widget(paragraph, inner);
            }
        }
        PopupType::InfoPanel { lines } => {
            let area = centered_rect(55, 55, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(" File Information ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let text_lines: Vec<Line> = lines
                .iter()
                .map(|l| Line::from(Span::raw(format!(" {}", l))))
                .collect();

            let paragraph = Paragraph::new(text_lines)
                .block(block)
                .style(Style::default().fg(parse_color(&theme.popup_fg)));

            f.render_widget(paragraph, area);
        }
        PopupType::TreeView {
            nodes,
            cursor_idx,
            panel: _,
        } => {
            let area = centered_rect(55, 70, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
                .title(" Directory Tree ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let inner = block.inner(area);
            f.render_widget(block, area);

            let list_height = inner.height.saturating_sub(2) as usize;
            let scroll_start = cursor_idx.saturating_sub(list_height / 2);
            let mut lines = Vec::new();

            for (i, node) in nodes
                .iter()
                .enumerate()
                .skip(scroll_start)
                .take(list_height)
            {
                let is_cursor = i == *cursor_idx;
                let indent = "  ".repeat(node.depth);
                let prefix = if node.is_dir { "▶ " } else { "  " };
                let display = format!("{}{}{}", indent, prefix, node.name);
                let style = if is_cursor {
                    Style::default()
                        .bg(parse_color(&theme.selection_bg))
                        .fg(parse_color(&theme.selection_fg))
                        .add_modifier(Modifier::BOLD)
                } else if node.is_dir {
                    Style::default().fg(Color::LightBlue)
                } else {
                    Style::default().fg(parse_color(&theme.popup_fg))
                };
                lines.push(Line::from(Span::styled(display, style)));
            }

            let hint = Line::from(Span::styled(
                " [Enter] Navigate  [Esc] Close ",
                Style::default().fg(Color::DarkGray),
            ));
            lines.push(Line::from(""));
            lines.push(hint);

            let paragraph =
                Paragraph::new(lines).style(Style::default().fg(parse_color(&theme.popup_fg)));
            f.render_widget(paragraph, inner);
        }
        PopupType::ContextMenu { items, cursor_idx } => {
            let panel_rect = match state.active_panel {
                ActivePanel::Left => left_rect,
                ActivePanel::Right => right_rect,
            };
            let height_percent = std::cmp::min(100, std::cmp::max(20, (items.len() * 10) as u16));
            let area = centered_rect_in(50, height_percent, panel_rect);
            f.render_widget(Clear, area);

            let mut lines = Vec::new();
            for (i, item) in items.iter().enumerate() {
                let is_cursor = i == *cursor_idx;
                let line_str = if is_cursor {
                    format!(" >  {} ", item)
                } else {
                    format!("    {} ", item)
                };
                let style = if is_cursor {
                    Style::default()
                        .bg(parse_color(&theme.selection_bg))
                        .fg(parse_color(&theme.selection_fg))
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(parse_color(&theme.popup_fg))
                };
                lines.push(Line::from(Span::styled(line_str, style)));
            }

            let paragraph = Paragraph::new(lines).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(parse_color(&theme.popup_border)))
                    .title(" Actions ")
                    .style(Style::default().bg(parse_color(&theme.popup_bg))),
            );
            f.render_widget(paragraph, area);
        }
        PopupType::CompressPrompt {
            input,
            targets,
            dest_dir,
        } => {
            let area = centered_rect(60, 30, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title(" Compress Archive ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let count = targets.len();
            let first_name = targets
                .first()
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();

            let src_label = if count == 1 {
                format!("Compressing: {}", first_name)
            } else {
                format!("Compressing: {} items", count)
            };

            let text = format!(
                "\n {}\n Destination: {}\n\n > {}.zip\n\n [Enter] Confirm   [Esc] Cancel",
                src_label,
                dest_dir.to_string_lossy(),
                input
            );

            let paragraph = Paragraph::new(text)
                .block(block)
                .style(Style::default().fg(parse_color(&theme.popup_fg)));

            f.render_widget(paragraph, area);
        }
        PopupType::ApplyCommandPrompt { input, targets } => {
            let area = centered_rect(65, 35, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title(" Apply Command to Selected Files ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let first_targets = targets
                .iter()
                .take(3)
                .map(|p| {
                    p.file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_default()
                })
                .collect::<Vec<String>>()
                .join(", ");
            let files_label = if targets.len() > 3 {
                format!("Files ({} total): {}, ...", targets.len(), first_targets)
            } else {
                format!("Files: {}", first_targets)
            };

            let text = format!(
                "\n {}\n\n Template command (use %f for file name):\n > {}\n\n [Enter] Execute   [Esc] Cancel",
                files_label, input
            );

            let paragraph = Paragraph::new(text)
                .block(block)
                .style(Style::default().fg(parse_color(&theme.popup_fg)));

            f.render_widget(paragraph, area);
        }
        PopupType::DescribeFilePrompt {
            path,
            current_desc,
            input,
        } => {
            let area = centered_rect(60, 30, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(" Describe File ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let file_name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            let text = format!(
                "\n File: {}\n Current Description: {}\n\n New Description:\n > {}\n\n [Enter] Save   [Esc] Cancel",
                file_name, current_desc, input
            );

            let paragraph = Paragraph::new(text)
                .block(block)
                .style(Style::default().fg(parse_color(&theme.popup_fg)));

            f.render_widget(paragraph, area);
        }
        PopupType::SelectGroupPrompt { mode, query } => {
            let area = centered_rect(50, 25, size);
            f.render_widget(Clear, area);

            let title = match mode {
                SelectMode::Add => " Select Group ",
                SelectMode::Remove => " Unselect Group ",
            };

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(title)
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let prompt_label = match mode {
                SelectMode::Add => "Select matching files:",
                SelectMode::Remove => "Unselect matching files:",
            };

            let text = format!(
                "\n {}\n\n > {}\n\n [Enter] Confirm   [Esc] Cancel",
                prompt_label, query
            );

            let paragraph = Paragraph::new(text)
                .block(block)
                .style(Style::default().fg(parse_color(&theme.popup_fg)));

            f.render_widget(paragraph, area);
        }
        PopupType::CreateLinkPrompt {
            src,
            dest_input,
            kind,
        } => {
            let area = centered_rect(60, 30, size);
            f.render_widget(Clear, area);

            let title = match kind {
                LinkKind::Symbolic => " Create Symbolic Link ",
                LinkKind::Hard => " Create Hard Link ",
            };

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title(title)
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let src_name = src
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            let text = format!(
                "\n Source: {}\n Link Path Destination:\n\n > {}\n\n [Enter] Confirm   [Esc] Cancel",
                src_name, dest_input
            );

            let paragraph = Paragraph::new(text)
                .block(block)
                .style(Style::default().fg(parse_color(&theme.popup_fg)));

            f.render_widget(paragraph, area);
        }
        PopupType::FilePanelFilterPrompt { input } => {
            let area = centered_rect(50, 25, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(" File Mask Filter ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let text = format!(
                "\n Enter mask filter (e.g. *.rs; empty to show all):\n\n > {}\n\n [Enter] Apply   [Esc] Cancel",
                input
            );

            let paragraph = Paragraph::new(text)
                .block(block)
                .style(Style::default().fg(parse_color(&theme.popup_fg)));

            f.render_widget(paragraph, area);
        }
        PopupType::WipeConfirm { paths } => {
            let area = centered_rect(55, 25, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
                .title(" WARNING: Secure Wipe Confirm ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let text = format!(
                "\n Are you sure you want to SECURELY WIPE {} item(s)?\n This writes over files and is IRRECOVERABLE.\n\n [Enter] Wipe   [Esc] Cancel",
                paths.len()
            );

            let paragraph = Paragraph::new(text)
                .block(block)
                .style(Style::default().fg(Color::LightRed));

            f.render_widget(paragraph, area);
        }
        PopupType::SaveSetupConfirm => {
            let area = centered_rect(45, 20, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
                .title(" Save Setup ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let text = "\n Save configuration and current layout settings?\n\n [Enter] Confirm   [Esc] Cancel";
            let paragraph = Paragraph::new(text)
                .block(block)
                .style(Style::default().fg(parse_color(&theme.popup_fg)));

            f.render_widget(paragraph, area);
        }
        PopupType::SortModesDialog {
            current,
            reverse,
            cursor_idx,
        } => {
            let area = centered_rect(45, 35, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(parse_color(&theme.popup_border)))
                .title(" Sort Modes ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let fields = [
                SortField::Name,
                SortField::Extension,
                SortField::Size,
                SortField::Date,
                SortField::Unsorted,
            ];

            let mut lines = Vec::new();
            for (i, field) in fields.iter().enumerate() {
                let is_cursor = i == *cursor_idx;
                let is_selected = field == current;
                let active_marker = if is_selected { "√" } else { " " };
                let cursor_marker = if is_cursor { ">" } else { " " };

                let name = match field {
                    SortField::Name => "Name",
                    SortField::Extension => "Extension",
                    SortField::Size => "Size",
                    SortField::Date => "Date",
                    SortField::Unsorted => "Unsorted",
                };

                let line_str = format!(" {} [{}] {} ", cursor_marker, active_marker, name);
                let style = if is_cursor {
                    Style::default()
                        .bg(parse_color(&theme.selection_bg))
                        .fg(parse_color(&theme.selection_fg))
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(parse_color(&theme.popup_fg))
                };
                lines.push(Line::from(Span::styled(line_str, style)));
            }

            // Reverse setting row
            let is_reverse_cursor = *cursor_idx == fields.len();
            let reverse_marker = if *reverse { "√" } else { " " };
            let cursor_marker = if is_reverse_cursor { ">" } else { " " };
            let line_str = format!(" {} [{}] Reverse order ", cursor_marker, reverse_marker);
            let style = if is_reverse_cursor {
                Style::default()
                    .bg(parse_color(&theme.selection_bg))
                    .fg(parse_color(&theme.selection_fg))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(parse_color(&theme.popup_fg))
            };
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(line_str, style)));

            let paragraph = Paragraph::new(lines).block(block);
            f.render_widget(paragraph, area);
        }
        PopupType::QuickViewPanel { .. } => {}
        PopupType::FileAttributesDialog { attrs, mode_input } => {
            let area = centered_rect(65, 45, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(parse_color(&theme.popup_border)))
                .title(" File Attributes ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let path_str = attrs.path.to_string_lossy();
            let file_name = attrs
                .path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| path_str.to_string());
            let readonly_status = if attrs.readonly { "Yes" } else { "No" };

            let format_time = |t: Option<std::time::SystemTime>| {
                t.map(|st| {
                    let datetime: chrono::DateTime<chrono::Local> = st.into();
                    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                })
                .unwrap_or_else(|| "N/A".to_string())
            };

            let modified_str = format_time(attrs.modified);
            let created_str = format_time(attrs.created);

            let text = format!(
                "\n Name: {}\n Path: {}\n Size: {} bytes\n Owner: {}\n Links: {}\n Readonly: {}\n Modified: {}\n Created: {}\n\n Unix Permissions (octal):\n > {}\n\n [Enter] Save   [Esc] Cancel",
                file_name,
                path_str,
                attrs.size,
                attrs.owner,
                attrs.nlinks,
                readonly_status,
                modified_str,
                created_str,
                mode_input
            );

            let paragraph = Paragraph::new(text)
                .block(block)
                .style(Style::default().fg(parse_color(&theme.popup_fg)));

            f.render_widget(paragraph, area);
        }
        PopupType::CommandHistoryList {
            entries,
            cursor_idx,
        } => {
            let area = centered_rect(60, 50, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(" Command History ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let inner = block.inner(area);
            f.render_widget(block, area);

            if entries.is_empty() {
                let paragraph = Paragraph::new("\n No command history.\n\n [Esc] Close")
                    .style(Style::default().fg(parse_color(&theme.popup_fg)));
                f.render_widget(paragraph, inner);
            } else {
                let list_height = inner.height.saturating_sub(2) as usize;
                let scroll_start = cursor_idx.saturating_sub(list_height / 2);
                let mut lines = Vec::new();

                for (i, entry) in entries
                    .iter()
                    .enumerate()
                    .skip(scroll_start)
                    .take(list_height)
                {
                    let is_cursor = i == *cursor_idx;
                    let style = if is_cursor {
                        Style::default()
                            .bg(parse_color(&theme.selection_bg))
                            .fg(parse_color(&theme.selection_fg))
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(parse_color(&theme.popup_fg))
                    };
                    lines.push(Line::from(Span::styled(format!(" {} ", entry), style)));
                }

                let hint = Line::from(Span::styled(
                    " [Enter] Execute command  [Esc] Close ",
                    Style::default().fg(Color::DarkGray),
                ));
                lines.push(Line::from(""));
                lines.push(hint);

                let paragraph =
                    Paragraph::new(lines).style(Style::default().fg(parse_color(&theme.popup_fg)));
                f.render_widget(paragraph, inner);
            }
        }
        PopupType::FileViewHistoryList {
            entries,
            cursor_idx,
        } => {
            let area = centered_rect(65, 50, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(" File View History ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let inner = block.inner(area);
            f.render_widget(block, area);

            if entries.is_empty() {
                let paragraph = Paragraph::new("\n No viewed file history.\n\n [Esc] Close")
                    .style(Style::default().fg(parse_color(&theme.popup_fg)));
                f.render_widget(paragraph, inner);
            } else {
                let list_height = inner.height.saturating_sub(2) as usize;
                let scroll_start = cursor_idx.saturating_sub(list_height / 2);
                let mut lines = Vec::new();

                for (i, entry) in entries
                    .iter()
                    .enumerate()
                    .skip(scroll_start)
                    .take(list_height)
                {
                    let is_cursor = i == *cursor_idx;
                    let display = entry.to_string_lossy();
                    let style = if is_cursor {
                        Style::default()
                            .bg(parse_color(&theme.selection_bg))
                            .fg(parse_color(&theme.selection_fg))
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(parse_color(&theme.popup_fg))
                    };
                    lines.push(Line::from(Span::styled(format!(" {} ", display), style)));
                }

                let hint = Line::from(Span::styled(
                    " [Enter] View / Edit File  [Esc] Close ",
                    Style::default().fg(Color::DarkGray),
                ));
                lines.push(Line::from(""));
                lines.push(hint);

                let paragraph =
                    Paragraph::new(lines).style(Style::default().fg(parse_color(&theme.popup_fg)));
                f.render_widget(paragraph, inner);
            }
        }
        PopupType::FoldersHistoryList {
            entries,
            cursor_idx,
        } => {
            let area = centered_rect(65, 50, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(" Folder Navigation History ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let inner = block.inner(area);
            f.render_widget(block, area);

            if entries.is_empty() {
                let paragraph = Paragraph::new("\n No folder history.\n\n [Esc] Close")
                    .style(Style::default().fg(parse_color(&theme.popup_fg)));
                f.render_widget(paragraph, inner);
            } else {
                let list_height = inner.height.saturating_sub(2) as usize;
                let scroll_start = cursor_idx.saturating_sub(list_height / 2);
                let mut lines = Vec::new();

                for (i, entry) in entries
                    .iter()
                    .enumerate()
                    .skip(scroll_start)
                    .take(list_height)
                {
                    let is_cursor = i == *cursor_idx;
                    let display = entry.to_string_lossy();
                    let style = if is_cursor {
                        Style::default()
                            .bg(parse_color(&theme.selection_bg))
                            .fg(parse_color(&theme.selection_fg))
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(parse_color(&theme.popup_fg))
                    };
                    lines.push(Line::from(Span::styled(format!(" {} ", display), style)));
                }

                let hint = Line::from(Span::styled(
                    " [Enter] Jump to Folder  [Esc] Close ",
                    Style::default().fg(Color::DarkGray),
                ));
                lines.push(Line::from(""));
                lines.push(hint);

                let paragraph =
                    Paragraph::new(lines).style(Style::default().fg(parse_color(&theme.popup_fg)));
                f.render_widget(paragraph, inner);
            }
        }
        PopupType::CompareFoldersResult { diff, cursor_idx } => {
            let area = centered_rect(75, 60, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title(" Folder Compare Results ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let inner = block.inner(area);
            f.render_widget(block, area);

            if diff.is_empty() {
                let paragraph = Paragraph::new("\n All files are identical.\n\n [Esc] Close")
                    .style(Style::default().fg(parse_color(&theme.popup_fg)));
                f.render_widget(paragraph, inner);
            } else {
                let list_height = inner.height.saturating_sub(2) as usize;
                let scroll_start = cursor_idx.saturating_sub(list_height / 2);
                let mut lines = Vec::new();

                for (i, entry) in diff.iter().enumerate().skip(scroll_start).take(list_height) {
                    let is_cursor = i == *cursor_idx;
                    let (status_text, color) = match entry.status {
                        CompareStatus::OnlyLeft => ("Only in Left", Color::LightGreen),
                        CompareStatus::OnlyRight => ("Only in Right", Color::LightYellow),
                        CompareStatus::Different => ("Different Size/Time", Color::LightRed),
                        CompareStatus::Equal => ("Equal", Color::DarkGray),
                    };

                    let line_str = format!(" {:<40} | {:<20} ", entry.name, status_text);
                    let style = if is_cursor {
                        Style::default()
                            .bg(parse_color(&theme.selection_bg))
                            .fg(parse_color(&theme.selection_fg))
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(color)
                    };
                    lines.push(Line::from(Span::styled(line_str, style)));
                }

                let hint = Line::from(Span::styled(
                    " [Esc] Close  (Differences are automatically tagged in active panel) ",
                    Style::default().fg(Color::DarkGray),
                ));
                lines.push(Line::from(""));
                lines.push(hint);

                let paragraph =
                    Paragraph::new(lines).style(Style::default().fg(parse_color(&theme.popup_fg)));
                f.render_widget(paragraph, inner);
            }
        }
        PopupType::TaskListDialog { tasks, cursor_idx } => {
            let area = centered_rect(70, 60, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(" Task List (OS Processes) ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let inner = block.inner(area);
            f.render_widget(block, area);

            if tasks.is_empty() {
                let paragraph = Paragraph::new("\n No processes listed.\n\n [Esc] Close")
                    .style(Style::default().fg(parse_color(&theme.popup_fg)));
                f.render_widget(paragraph, inner);
            } else {
                let list_height = inner.height.saturating_sub(2) as usize;
                let scroll_start = cursor_idx.saturating_sub(list_height / 2);
                let mut lines = Vec::new();

                // Table header
                lines.push(Line::from(vec![Span::styled(
                    format!(
                        " {:<8} | {:<35} | {:<12} ",
                        "PID", "Process Name", "Memory (MB)"
                    ),
                    Style::default().add_modifier(Modifier::UNDERLINED),
                )]));

                for (i, task) in tasks
                    .iter()
                    .enumerate()
                    .skip(scroll_start)
                    .take(list_height)
                {
                    let is_cursor = i == *cursor_idx;
                    let mem_mb = (task.memory_kb as f64) / 1024.0;
                    let line_str =
                        format!(" {:<8} | {:<35} | {:<12.1} ", task.pid, task.name, mem_mb);
                    let style = if is_cursor {
                        Style::default()
                            .bg(parse_color(&theme.selection_bg))
                            .fg(parse_color(&theme.selection_fg))
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(parse_color(&theme.popup_fg))
                    };
                    lines.push(Line::from(Span::styled(line_str, style)));
                }

                let hint = Line::from(Span::styled(
                    " [Del / Alt+Del] Kill process  [Esc] Close ",
                    Style::default().fg(Color::DarkGray),
                ));
                lines.push(Line::from(""));
                lines.push(hint);

                let paragraph =
                    Paragraph::new(lines).style(Style::default().fg(parse_color(&theme.popup_fg)));
                f.render_widget(paragraph, inner);
            }
        }
        PopupType::FileAssociationsDialog { rules, cursor_idx } => {
            let area = centered_rect(75, 60, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(" File Associations ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let inner = block.inner(area);
            f.render_widget(block, area);

            if rules.is_empty() {
                let paragraph = Paragraph::new("\n No rules configured.\n\n [Esc] Close")
                    .style(Style::default().fg(parse_color(&theme.popup_fg)));
                f.render_widget(paragraph, inner);
            } else {
                let list_height = inner.height.saturating_sub(2) as usize;
                let scroll_start = cursor_idx.saturating_sub(list_height / 2);
                let mut lines = Vec::new();

                lines.push(Line::from(vec![Span::styled(
                    format!(
                        " {:<15} | {:<30} | {:<30} ",
                        "Mask", "Open Command", "View Command (F3)"
                    ),
                    Style::default().add_modifier(Modifier::UNDERLINED),
                )]));

                for (i, rule) in rules
                    .iter()
                    .enumerate()
                    .skip(scroll_start)
                    .take(list_height)
                {
                    let is_cursor = i == *cursor_idx;
                    let view_cmd_str = rule.view_cmd.as_deref().unwrap_or("(Same as open)");
                    let line_str = format!(
                        " {:<15} | {:<30} | {:<30} ",
                        rule.mask, rule.open_cmd, view_cmd_str
                    );
                    let style = if is_cursor {
                        Style::default()
                            .bg(parse_color(&theme.selection_bg))
                            .fg(parse_color(&theme.selection_fg))
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(parse_color(&theme.popup_fg))
                    };
                    lines.push(Line::from(Span::styled(line_str, style)));
                }

                let hint = Line::from(Span::styled(
                    " [Esc] Close ",
                    Style::default().fg(Color::DarkGray),
                ));
                lines.push(Line::from(""));
                lines.push(hint);

                let paragraph =
                    Paragraph::new(lines).style(Style::default().fg(parse_color(&theme.popup_fg)));
                f.render_widget(paragraph, inner);
            }
        }
        PopupType::ArchiveCommandsMenu {
            archive_path,
            items,
            cursor_idx,
        } => {
            let area = centered_rect(60, 45, size);
            f.render_widget(Clear, area);

            let title = format!(
                " Archive Commands: {} ",
                archive_path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default()
            );
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title(title)
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let inner = block.inner(area);
            f.render_widget(block, area);

            if items.is_empty() {
                let paragraph = Paragraph::new("\n No archive commands available.\n\n [Esc] Close")
                    .style(Style::default().fg(parse_color(&theme.popup_fg)));
                f.render_widget(paragraph, inner);
            } else {
                let list_height = inner.height.saturating_sub(2) as usize;
                let scroll_start = cursor_idx.saturating_sub(list_height / 2);
                let mut lines = Vec::new();

                for (i, item) in items
                    .iter()
                    .enumerate()
                    .skip(scroll_start)
                    .take(list_height)
                {
                    let is_cursor = i == *cursor_idx;
                    let line_str = if is_cursor {
                        format!(" >  {} ", item)
                    } else {
                        format!("    {} ", item)
                    };
                    let style = if is_cursor {
                        Style::default()
                            .bg(parse_color(&theme.selection_bg))
                            .fg(parse_color(&theme.selection_fg))
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(parse_color(&theme.popup_fg))
                    };
                    lines.push(Line::from(Span::styled(line_str, style)));
                }

                let hint = Line::from(Span::styled(
                    " [Enter] Execute Option  [Esc] Close ",
                    Style::default().fg(Color::DarkGray),
                ));
                lines.push(Line::from(""));
                lines.push(hint);

                let paragraph =
                    Paragraph::new(lines).style(Style::default().fg(parse_color(&theme.popup_fg)));
                f.render_widget(paragraph, inner);
            }
        }
        PopupType::ConfigurationDialog {
            active_tab,
            cursor_idx,
            editing_value,
            edit_buffer,
            settings,
        } => {
            let area = centered_rect(85, 85, size);
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(parse_color(&theme.popup_border)))
                .title(" Configuration Settings ")
                .style(Style::default().bg(parse_color(&theme.popup_bg)));

            let inner = block.inner(area);
            f.render_widget(block, area);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Tab headers
                    Constraint::Length(1), // Separator
                    Constraint::Min(1),    // Tab contents
                    Constraint::Length(1), // Bottom separator
                    Constraint::Length(1), // Hint/Status bar
                ])
                .split(inner);

            let header_area = chunks[0];
            let separator_area = chunks[1];
            let content_area = chunks[2];
            let bottom_sep_area = chunks[3];
            let hint_area = chunks[4];

            f.render_widget(
                Paragraph::new("─".repeat(inner.width as usize))
                    .style(Style::default().fg(Color::DarkGray)),
                separator_area,
            );
            f.render_widget(
                Paragraph::new("─".repeat(inner.width as usize))
                    .style(Style::default().fg(Color::DarkGray)),
                bottom_sep_area,
            );

            let tab_titles = [
                " System ",
                " Panel ",
                " Interface ",
                " Confirmations ",
                " Language & Plugins ",
                " Editor/Viewer ",
                " Colors ",
            ];
            let mut tab_spans = Vec::new();
            for (i, title) in tab_titles.iter().enumerate() {
                let is_active = i == *active_tab;
                let style = if is_active {
                    Style::default()
                        .bg(parse_color(&theme.selection_bg))
                        .fg(parse_color(&theme.selection_fg))
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(parse_color(&theme.popup_fg))
                };
                tab_spans.push(Span::styled(format!("  [{}]  ", title), style));
            }
            f.render_widget(Paragraph::new(Line::from(tab_spans)), header_area);

            let mut rows: Vec<(String, bool)> = Vec::new();

            match active_tab {
                0 => {
                    rows.push(("[ ] Delete to Recycle Bin".to_string(), true));
                    rows.push(("[ ] Use system copy routine".to_string(), true));
                    rows.push(("[ ] Copy files opened for writing".to_string(), true));
                    rows.push(("[ ] Scan symbolic links".to_string(), true));
                    rows.push(("[x] Save commands history".to_string(), false));
                    rows.push(("[x] Save folders history".to_string(), false));
                    rows.push(("[x] Save view and edit history".to_string(), false));
                    rows.push(("[ ] Use Windows registered types".to_string(), true));
                    rows.push((
                        "[ ] Automatic update of environment variables".to_string(),
                        true,
                    ));
                    rows.push(("Request administrator rights:".to_string(), true));
                    rows.push(("  [ ] For modification".to_string(), true));
                    rows.push(("  [ ] For reading".to_string(), true));
                    rows.push(("  [ ] Use additional privileges".to_string(), true));
                    rows.push(("Sorting collation: < linguistic >".to_string(), true));
                    rows.push(("  [ ] Treat digits as numbers".to_string(), true));
                    rows.push(("  [ ] Case sensitive".to_string(), true));
                    rows.push(("[ ] Auto save setup".to_string(), true));
                }
                1 => {
                    rows.push((
                        format!(
                            "[{}] Show hidden and system files",
                            if settings.show_hidden { "x" } else { " " }
                        ),
                        false,
                    ));
                    rows.push(("[ ] Highlight files".to_string(), true));
                    rows.push(("[ ] Select folders".to_string(), true));
                    rows.push(("[ ] Right click selects files".to_string(), true));
                    rows.push(("[ ] Sort folder names by extension".to_string(), true));
                    rows.push((
                        format!(
                            "[{}] Allow reverse sort modes",
                            if settings.sort_reverse { "x" } else { " " }
                        ),
                        false,
                    ));
                    rows.push((
                        "Disable automatic panel update if object count exceeds: [ 0 ]".to_string(),
                        true,
                    ));
                    rows.push(("[ ] Network drives autorefresh".to_string(), true));
                    rows.push(("[x] Show column titles".to_string(), true));
                    rows.push(("[x] Show status line".to_string(), true));
                    rows.push(("[ ] Detect volume mount points".to_string(), true));
                    rows.push(("[x] Show files total information".to_string(), true));
                    rows.push(("[ ] Show free size".to_string(), true));
                    rows.push(("[ ] Show scrollbar".to_string(), true));
                    rows.push(("[x] Show background screens number".to_string(), true));
                    rows.push(("[x] Show sort mode letter".to_string(), true));
                    rows.push(("[ ] Show \"..\" in root folders".to_string(), true));
                    rows.push(("InfoPanel settings:".to_string(), true));
                    rows.push(("  [ ] Show power status".to_string(), true));
                    rows.push(("  [x] Show CD drive parameters".to_string(), true));
                    rows.push((
                        "  Computer name format: < Physical NetBIOS >".to_string(),
                        true,
                    ));
                    rows.push(("  User name format: < Logon name >".to_string(), true));
                    rows.push((
                        "Groups of file masks: [Ins/Del/F4/F7/Ctrl+R]".to_string(),
                        true,
                    ));
                    rows.push((
                        "Edit panel modes: [Ins/Del/F4/Ctrl+Enter]".to_string(),
                        true,
                    ));
                    rows.push(("File descriptions:".to_string(), true));
                    rows.push(("  Names: [ Descript.ion,Files.bbs ]".to_string(), true));
                    rows.push((
                        "  [x] Set \"hidden\" attribute to new lists".to_string(),
                        true,
                    ));
                    rows.push(("  [ ] Update read only description file".to_string(), true));
                    rows.push(("  Position of new descriptions: [ 0 ]".to_string(), true));
                    rows.push(("  Update mode: ( ) Do not update  (•) Update if displayed  ( ) Always update".to_string(), true));
                    rows.push(("  [ ] Use ANSI code page by default".to_string(), true));
                    rows.push(("  [ ] Save in UTF-8".to_string(), true));
                    rows.push((
                        "Folder description list names: [ DirInfo,File_Id.diz,... ]".to_string(),
                        true,
                    ));
                }
                2 => {
                    rows.push(("[ ] Clock".to_string(), true));
                    rows.push((
                        format!("[{}] Mouse", if settings.mouse_support { "x" } else { " " }),
                        false,
                    ));
                    rows.push(("[ ] Show key bar".to_string(), true));
                    rows.push(("[ ] Always show the menu bar".to_string(), true));
                    rows.push(("Screen saver: [ 5 ] minutes".to_string(), true));
                    rows.push(("[ ] Show total copy progress indicator".to_string(), true));
                    rows.push(("[ ] Show copying time information".to_string(), true));
                    rows.push(("[ ] Show total delete progress indicator".to_string(), true));
                    rows.push(("[ ] Use Ctrl+PgUp to change drive".to_string(), true));
                    rows.push(("[ ] Use Virtual Terminal for rendering".to_string(), true));
                    rows.push(("[ ] Fullwidth-aware rendering".to_string(), true));
                    rows.push(("[x] ClearType-friendly redraw".to_string(), true));
                    rows.push(("Console icon: [ 0 ]".to_string(), true));
                    rows.push(("  [ ] Alternate for Administrator".to_string(), true));
                    rows.push((
                        "Far window title addons: [ %Ver %Platform %Admin ]".to_string(),
                        true,
                    ));
                    rows.push(("Dialog settings:".to_string(), true));
                    rows.push(("  [x] History in dialog edit controls".to_string(), true));
                    rows.push(("  [ ] Persistent blocks in edit controls".to_string(), true));
                    rows.push((
                        "  [x] Del removes blocks in edit controls".to_string(),
                        true,
                    ));
                    rows.push(("  [x] AutoComplete in edit controls".to_string(), true));
                    rows.push(("  [ ] Backspace deletes unchanged text".to_string(), true));
                    rows.push(("  [x] Mouse click outside closes dialog".to_string(), true));
                    rows.push(("Menu settings:".to_string(), true));
                    rows.push(("  Left click outside: < Cancel menu >".to_string(), true));
                    rows.push(("  Right click outside: < Cancel menu >".to_string(), true));
                    rows.push((
                        "  Middle click outside: < Execute selected >".to_string(),
                        true,
                    ));
                    rows.push(("Command line settings:".to_string(), true));
                    rows.push(("  [ ] Persistent blocks".to_string(), true));
                    rows.push(("  [x] Del removes blocks".to_string(), true));
                    rows.push(("  [x] AutoComplete".to_string(), true));
                    rows.push(("  [ ] Set prompt format: [ $p$g ]".to_string(), true));
                    rows.push(("  [x] Use home dir: [ %FARHOME% ]".to_string(), true));
                    rows.push(("AutoComplete settings:".to_string(), true));
                    rows.push(("  [x] Show a list".to_string(), true));
                    rows.push(("    [ ] Modal mode".to_string(), true));
                    rows.push(("  [ ] Append the first matched item".to_string(), true));
                    rows.push((
                        format!("Keybindings preset: < {} >", settings.keybinding_preset),
                        false,
                    ));
                }
                3 => {
                    rows.push(("[ ] Copy".to_string(), true));
                    rows.push(("[ ] Move".to_string(), true));
                    rows.push((
                        format!(
                            "[{}] Overwrite and delete R/O files",
                            if settings.confirmations.confirm_overwrite {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push(("[ ] Drag and drop".to_string(), true));
                    rows.push((
                        format!(
                            "[{}] Delete",
                            if settings.confirmations.confirm_delete {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push(("[ ] Delete non-empty folders".to_string(), true));
                    rows.push(("[ ] Interrupt operation".to_string(), true));
                    rows.push(("[ ] Disconnect network drive".to_string(), true));
                    rows.push(("[ ] Delete SUBST-disk".to_string(), true));
                    rows.push(("[ ] Detach virtual disk".to_string(), true));
                    rows.push(("[ ] HotPlug-device removal".to_string(), true));
                    rows.push(("[ ] Reload edited file".to_string(), true));
                    rows.push(("[ ] Clear history list".to_string(), true));
                    rows.push((
                        format!(
                            "[{}] Exit",
                            if settings.confirmations.confirm_quit {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                }
                4 => {
                    rows.push(("Main language: < English >".to_string(), false));
                    rows.push((
                        "Plugins configuration: [ArcLite | EMenu | HlfViewer | NetBox]".to_string(),
                        true,
                    ));
                    rows.push(("Plugins manager settings:".to_string(), true));
                    rows.push(("  [x] OEM plugins support".to_string(), true));
                    rows.push(("  [x] Scan symbolic links".to_string(), true));
                    rows.push(("  Plugin selection:".to_string(), true));
                    rows.push(("    [ ] File processing".to_string(), true));
                    rows.push(("      [ ] Show standard association".to_string(), true));
                    rows.push(("        [ ] Even if only one plugin".to_string(), true));
                    rows.push(("    [ ] Search results (SetFindList)".to_string(), true));
                    rows.push(("    [ ] Prefix processing".to_string(), true));
                }
                5 => {
                    rows.push((
                        "[ ] Use external editor for F4 instead of Alt+F4".to_string(),
                        true,
                    ));
                    if *editing_value && *cursor_idx == 1 {
                        rows.push((format!("Editor command: [ {}◄ ]", edit_buffer), false));
                    } else {
                        rows.push((
                            format!("Editor command: [ {} ]", settings.default_editor),
                            false,
                        ));
                    }
                    rows.push(("Internal editor:".to_string(), true));
                    rows.push(("  Expand tabs: < Do not expand tabs >".to_string(), true));
                    rows.push((
                        "  [ ] Persistent blocks                  [x] Cursor beyond end"
                            .to_string(),
                        true,
                    ));
                    rows.push((
                        "  [x] Del removes blocks                 [ ] Select found".to_string(),
                        true,
                    ));
                    rows.push((
                        "  [ ] Auto indent                        [ ] Cursor at the end"
                            .to_string(),
                        true,
                    ));
                    rows.push((
                        "  Tab size: [ 8 ]                        [ ] Show scrollbar".to_string(),
                        true,
                    ));
                    rows.push((
                        "  [ ] Show white space                   [ ] Show line numbers"
                            .to_string(),
                        true,
                    ));
                    rows.push(("  [x] Save file position".to_string(), true));
                    rows.push(("  [x] Save bookmarks".to_string(), true));
                    rows.push(("  [x] Allow editing files opened".to_string(), true));
                    rows.push(("  [ ] Lock editing of read-only files".to_string(), true));
                    rows.push(("  [ ] Warn when opening read-only files".to_string(), true));
                    rows.push(("  [x] Autodetect code page".to_string(), true));
                    rows.push((
                        "  Default code page: < 1252 | ANSI - Latín I >".to_string(),
                        true,
                    ));
                    rows.push((
                        "[ ] Use external viewer for F3 instead of Alt+F3".to_string(),
                        true,
                    ));
                    rows.push(("Viewer command: [ ... ]".to_string(), true));
                    rows.push(("Internal viewer:".to_string(), true));
                    rows.push((
                        "  [x] Persistent selection               [x] Show scrolling arrows"
                            .to_string(),
                        true,
                    ));
                    rows.push((
                        "  Tab size: [ 8 ]                        [ ] Visible '\\0'".to_string(),
                        true,
                    ));
                    rows.push((
                        "                                         [ ] Show scrollbar".to_string(),
                        true,
                    ));
                    rows.push((
                        "  [x] Save file position                 [x] Save view mode".to_string(),
                        true,
                    ));
                    rows.push((
                        "  [x] Save file code page                [ ] Save wrap mode".to_string(),
                        true,
                    ));
                    rows.push((
                        "  [x] Save bookmarks                     [x] Detect dump view mode"
                            .to_string(),
                        true,
                    ));
                    rows.push((
                        "  Maximum line width: [ 10000 ]          [x] Autodetect code page"
                            .to_string(),
                        true,
                    ));
                    rows.push((
                        "  Default code page: < 1252 | ANSI - Latín I >".to_string(),
                        true,
                    ));
                    rows.push(("Code pages list: [Ctrl+H Ins Del F4]".to_string(), true));
                }
                6 => {
                    rows.push((format!("Theme: < {} >", settings.theme), false));
                    rows.push((
                        "Color groups: [ Panel | Dialog | Menu | clock | ... ]".to_string(),
                        true,
                    ));
                    rows.push((
                        "Files highlighting: [ +H | +S | +D | <exec> | <arc> | <temp> ]"
                            .to_string(),
                        true,
                    ));
                }
                _ => {}
            }

            rows.push(("[ OK ]".to_string(), false));
            rows.push(("[ Cancel ]".to_string(), false));

            let list_height = content_area.height as usize;
            let scroll_start = cursor_idx.saturating_sub(list_height / 2);
            let mut list_spans = Vec::new();

            for (i, (label, is_stub)) in
                rows.iter().enumerate().skip(scroll_start).take(list_height)
            {
                let is_cursor = i == *cursor_idx;

                let display_label = if *is_stub {
                    format!("{} *", label)
                } else {
                    label.clone()
                };

                let style = if is_cursor {
                    Style::default()
                        .bg(parse_color(&theme.selection_bg))
                        .fg(parse_color(&theme.selection_fg))
                        .add_modifier(Modifier::BOLD)
                } else if *is_stub {
                    Style::default().fg(Color::DarkGray)
                } else {
                    Style::default().fg(parse_color(&theme.popup_fg))
                };

                list_spans.push(Line::from(Span::styled(
                    format!("  {}  ", display_label),
                    style,
                )));
            }

            f.render_widget(Paragraph::new(list_spans), content_area);

            let hint_str = " * Unimplemented/Future feature  |  [Tab/Arrows] Navigate  [Space/Enter] Edit/Toggle  [F9] Save  [Esc] Cancel";
            let hint_widget = Paragraph::new(hint_str).style(Style::default().fg(Color::Yellow));
            f.render_widget(hint_widget, hint_area);
        }
    }
}

/// Centers a rectangle of `percent_x` × `percent_y` over the full screen.
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Centers a rectangle of `percent_x` × `percent_y` within a given parent rectangle.
/// Used for panel-specific popups (e.g. DriveSelect).
fn centered_rect_in(percent_x: u16, percent_y: u16, parent: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(parent);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
