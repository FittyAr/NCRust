use super::centered_rect;
use crate::app::state::PopupType;
use crate::config::theme::Theme;
use crate::ui::theme_apply::parse_color;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

pub fn render_files_highlighting_popup(
    f: &mut Frame,
    popup: &PopupType,
    theme: &Theme,
    size: Rect,
) -> bool {
    if let PopupType::FilesHighlightingDialog {
        cursor_idx,
        editing,
        edit_buffer,
        rules,
    } = popup
    {
        let area = centered_rect(60, 60, size);
        f.render_widget(Clear, area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(parse_color(&theme.popup_border)))
            .title(" Files Highlighting ")
            .style(Style::default().bg(parse_color(&theme.popup_bg)));

        let inner = block.inner(area);
        f.render_widget(block, area);

        let mut lines = Vec::new();
        let list_height = inner.height as usize;
        let scroll_start = cursor_idx.saturating_sub(list_height / 2);

        for i in scroll_start..scroll_start + list_height {
            if i >= rules.len() {
                break;
            }

            let rule = &rules[i];
            let is_cursor = i == *cursor_idx;

            let color_str = if is_cursor && *editing {
                format!("{}_", edit_buffer)
            } else {
                rule.color.clone()
            };

            let bg_color = if is_cursor {
                parse_color(&theme.selection_bg)
            } else {
                parse_color(&theme.popup_bg)
            };

            // When editing, still try to parse the actual rule color so the item text doesn't disappear
            // or just use White. Let's use the parsed current color or white.
            let fg_color = parse_color(&rule.color);

            let mut style = Style::default().bg(bg_color).fg(fg_color);
            if is_cursor {
                style = style.add_modifier(Modifier::BOLD);
            }

            let line_text = format!(" {:<30} < {:^15} >", rule.mask, color_str);
            lines.push(Line::from(Span::styled(line_text, style)));
        }

        f.render_widget(Paragraph::new(lines), inner);
        return true;
    }
    false
}
