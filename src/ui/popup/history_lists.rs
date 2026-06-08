use super::centered_rect;
use crate::app::state::{CompareStatus, PopupType};
use crate::ui::theme_apply::parse_color;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

pub fn render_history_lists_popup(
    f: &mut Frame,
    popup: &PopupType,
    theme: &crate::config::theme::Theme,
    size: Rect,
) -> bool {
    match popup {
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
            true
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
            true
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
            true
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
            true
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
            true
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
            true
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
            true
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
            true
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
            true
        }
        _ => false,
    }
}
