use crate::app::context::AppContext;
use crate::app::state::{AppState, PopupType};
use crate::ui::theme_apply::parse_color;
use crate::config::localization::t;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

// ─────────────────────────────────────────────────────────────────────────────
// Menu definitions — fully matches norton_commander_features.md sections 1–5
// ─────────────────────────────────────────────────────────────────────────────

/// Returns the menu item labels for a given top-level menu index.
/// - 0 = Left   (panel config for the left panel)
/// - 1 = Files  (file operations)
/// - 2 = Commands
/// - 3 = Options
/// - 4 = Right  (panel config for the right panel)
pub fn get_menu_items(menu_idx: usize) -> Vec<String> {
    match menu_idx {
        // ── Left (mirrors Right exactly, just different drive shortcut) ───────
        0 => vec![
            format!(" {:<25}Ctrl+1 ", t("menu_brief")),
            format!(" {:<25}Ctrl+2 ", t("menu_medium")),
            format!(" {:<25}Ctrl+3 ", t("menu_full")),
            format!(" {:<25}Ctrl+4 ", t("menu_wide")),
            format!(" {:<25}Ctrl+5 ", t("menu_detailed")),
            format!(" {:<25}Ctrl+6 ", t("menu_descriptions")),
            format!(" {:<25}Ctrl+7 ", t("menu_file_owners")),
            format!(" {:<25}Ctrl+8 ", t("menu_file_links")),
            format!(" {:<25}Ctrl+9 ", t("menu_alt_full")),
            " ───────────────────────────────── ".to_string(),
            format!(" {:<25}Ctrl+L ", t("menu_info_panel")),
            format!(" {:<25}Ctrl+Q ", t("menu_quick_view")),
            " ───────────────────────────────── ".to_string(),
            format!(" {:<25}Ctrl+F12", t("menu_sort_modes")),
            format!(" {:<25}Ctrl+N ", t("menu_show_long_names")),
            format!(" {:<25}Ctrl+F1 ", t("menu_panel_on_off")),
            format!(" {:<25}Ctrl+R ", t("menu_re_read")),
            format!(" {:<25}Alt+F1 ", t("menu_change_drive")),
        ],
        // ── Files ─────────────────────────────────────────────────────────────
        1 => vec![
            format!(" {:<25}    F3 ", t("menu_view")),
            format!(" {:<25}    F4 ", t("menu_edit")),
            format!(" {:<25}    F5 ", t("menu_copy")),
            format!(" {:<25}    F6 ", t("menu_rename_move")),
            format!(" {:<25}Alt+F6 ", t("menu_link")),
            format!(" {:<25}    F7 ", t("menu_make_folder")),
            format!(" {:<25}    F8 ", t("menu_delete")),
            format!(" {:<25}Alt+Del", t("menu_wipe")),
            " ───────────────────────────────── ".to_string(),
            format!(" {:<25}Shf+F1 ", t("menu_add_to_archive")),
            format!(" {:<25}Shf+F2 ", t("menu_extract_files")),
            format!(" {:<25}Shf+F3 ", t("menu_archive_commands")),
            " ───────────────────────────────── ".to_string(),
            format!(" {:<25}Ctrl+A ", t("menu_file_attributes")),
            format!(" {:<25}Ctrl+G ", t("menu_apply_command")),
            format!(" {:<25}Ctrl+Z ", t("menu_describe_files")),
            " ───────────────────────────────── ".to_string(),
            format!(" {:<25} Gray+ ", t("menu_select_group")),
            format!(" {:<25} Gray- ", t("menu_unselect_group")),
            format!(" {:<25} Gray* ", t("menu_invert_selection")),
            format!(" {:<25}Ctrl+M ", t("menu_restore_selection")),
        ],
        // ── Commands ──────────────────────────────────────────────────────────
        2 => vec![
            format!(" {:<25}Alt+F7 ", t("menu_find_file")),
            format!(" {:<25}Alt+F8 ", t("menu_history")),
            format!(" {:<25}Alt+F11", t("menu_file_view_hist")),
            format!(" {:<25}Alt+F12", t("menu_folders_hist")),
            " ───────────────────────────────── ".to_string(),
            format!(" {:<25}Ctrl+U ", t("menu_swap_panels")),
            format!(" {:<25}Ctrl+O ", t("menu_panels_on_off")),
            format!(" {:<25}       ", t("menu_compare_folders")),
            " ───────────────────────────────── ".to_string(),
            format!(" {:<25}       ", t("menu_edit_user_menu")),
            format!(" {:<25}       ", t("menu_file_associations")),
            format!(" {:<25}       ", t("menu_folder_shortcuts")),
            format!(" {:<25}Ctrl+I ", t("menu_file_panel_filter")),
            " ───────────────────────────────── ".to_string(),
            format!(" {:<25}   F11 ", t("menu_plugin_commands")),
            format!(" {:<25}   F12 ", t("menu_screens_list")),
            format!(" {:<25}Ctrl+W ", t("menu_task_list")),
            format!(" {:<25}       ", t("menu_hotplug_devices")),
        ],
        // ── Options ───────────────────────────────────────────────────────────
        3 => vec![
            format!(" {:<25}       ", t("menu_configuration")),
            " ───────────────────────────────── ".to_string(),
            format!(" {:<25}Shf+F9 ", t("menu_save_setup")),
        ],
        // ── Right (mirrors Left) ──────────────────────────────────────────────
        4 => vec![
            format!(" {:<25}Ctrl+1 ", t("menu_brief")),
            format!(" {:<25}Ctrl+2 ", t("menu_medium")),
            format!(" {:<25}Ctrl+3 ", t("menu_full")),
            format!(" {:<25}Ctrl+4 ", t("menu_wide")),
            format!(" {:<25}Ctrl+5 ", t("menu_detailed")),
            format!(" {:<25}Ctrl+6 ", t("menu_descriptions")),
            format!(" {:<25}Ctrl+7 ", t("menu_file_owners")),
            format!(" {:<25}Ctrl+8 ", t("menu_file_links")),
            format!(" {:<25}Ctrl+9 ", t("menu_alt_full")),
            " ───────────────────────────────── ".to_string(),
            format!(" {:<25}Ctrl+L ", t("menu_info_panel")),
            format!(" {:<25}Ctrl+Q ", t("menu_quick_view")),
            " ───────────────────────────────── ".to_string(),
            format!(" {:<25}Ctrl+F12", t("menu_sort_modes")),
            format!(" {:<25}Ctrl+N ", t("menu_show_long_names")),
            format!(" {:<25}Ctrl+F2 ", t("menu_panel_on_off")),
            format!(" {:<25}Ctrl+R ", t("menu_re_read")),
            format!(" {:<25}Alt+F2 ", t("menu_change_drive")),
        ],
        _ => vec![],
    }
}

/// Returns the display labels for the top-level menu bar.
pub fn get_menu_titles() -> Vec<String> {
    vec![
        format!("  {}  ", t("menu_left")),
        format!("  {}  ", t("menu_files")),
        format!("  {}  ", t("menu_commands")),
        format!("  {}  ", t("menu_options")),
        format!("  {}  ", t("menu_right")),
    ]
}

// ─────────────────────────────────────────────────────────────────────────────
// Renderer
// ─────────────────────────────────────────────────────────────────────────────

pub fn render_menu(f: &mut Frame, area: Rect, context: &AppContext, state: &AppState) {
    let theme = &context.config.theme;

    let active_menu_idx = if let Some(PopupType::Menu {
        active_menu_idx, ..
    }) = state.active_popup
    {
        Some(active_menu_idx)
    } else {
        None
    };

    let mut spans = Vec::new();
    let titles = get_menu_titles();
    for (i, title) in titles.iter().enumerate() {
        let is_active = Some(i) == active_menu_idx;
        let style = if is_active {
            Style::default()
                .bg(parse_color(&theme.selection_bg))
                .fg(parse_color(&theme.selection_fg))
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(parse_color(&theme.panel_fg))
                .add_modifier(Modifier::BOLD)
        };
        spans.push(Span::styled(title.clone(), style));
    }

    let menu_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(20), Constraint::Length(12)])
        .split(area);

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line).style(Style::default().bg(parse_color("DarkGray")));
    f.render_widget(paragraph, menu_chunks[0]);

    if context.config.settings.interface_clock {
        let time_str = chrono::Local::now().format(" %H:%M:%S ").to_string();
        let clock_para = Paragraph::new(time_str)
            .style(
                Style::default()
                    .bg(parse_color("DarkGray"))
                    .fg(parse_color(&theme.panel_fg)),
            )
            .alignment(ratatui::layout::Alignment::Right);
        f.render_widget(clock_para, menu_chunks[1]);
    } else {
        let empty_para = Paragraph::new("").style(Style::default().bg(parse_color("DarkGray")));
        f.render_widget(empty_para, menu_chunks[1]);
    }
}
