use super::centered_rect;
use crate::app::state::PopupType;
use crate::ui::theme_apply::parse_color;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

pub fn render_info_popup(
    f: &mut Frame,
    popup: &PopupType,
    theme: &crate::config::theme::Theme,
    size: Rect,
) -> bool {
    match popup {
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
            true
        }
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
            true
        }
        _ => false,
    }
}
