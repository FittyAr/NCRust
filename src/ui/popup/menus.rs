use super::{centered_rect, centered_rect_in};
use crate::app::state::{ActivePanel, PopupType, SortField};
use crate::ui::theme_apply::parse_color;
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Row, Table},
};

pub fn render_menu_popup(
    f: &mut Frame,
    popup: &PopupType,
    theme: &crate::config::theme::Theme,
    size: Rect,
    left_rect: Rect,
    right_rect: Rect,
    active_panel: ActivePanel,
) -> bool {
    match popup {
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
            true
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
            true
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
            true
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
            true
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
            true
        }
        PopupType::ContextMenu { items, cursor_idx } => {
            let panel_rect = match active_panel {
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
            true
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
                .border_style(Style::default().fg(ratatui::style::Color::Yellow))
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
                    Style::default().fg(ratatui::style::Color::DarkGray),
                ));
                lines.push(Line::from(""));
                lines.push(hint);

                let paragraph =
                    Paragraph::new(lines).style(Style::default().fg(parse_color(&theme.popup_fg)));
                f.render_widget(paragraph, inner);
            }
            true
        }
        _ => false,
    }
}
