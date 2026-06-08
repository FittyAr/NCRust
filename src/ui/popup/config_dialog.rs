use super::centered_rect;
use crate::app::state::PopupType;
use crate::ui::theme_apply::parse_color;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

pub fn render_config_dialog_popup(
    f: &mut Frame,
    popup: &PopupType,
    theme: &crate::config::theme::Theme,
    size: Rect,
) -> bool {
    match popup {
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
                    rows.push((
                        format!(
                            "[{}] Delete to Recycle Bin",
                            if settings.delete_to_recycle_bin {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "[{}] Use system copy routine",
                            if settings.use_system_copy_routine {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "[{}] Copy files opened for writing",
                            if settings.copy_files_opened_for_writing {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "[{}] Scan symbolic links",
                            if settings.scan_symbolic_links {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "[{}] Save commands history",
                            if settings.save_commands_history {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "[{}] Save folders history",
                            if settings.save_folders_history {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "[{}] Save view and edit history",
                            if settings.save_view_and_edit_history {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "[{}] Use Windows registered types",
                            if settings.use_windows_registered_types {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "[{}] Automatic update of environment variables",
                            if settings.automatic_update_env_variables {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push(("Request administrator rights:".to_string(), false));
                    rows.push((
                        format!(
                            "  [{}] For modification",
                            if settings.req_admin_modification {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "  [{}] For reading",
                            if settings.req_admin_reading { "x" } else { " " }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Use additional privileges",
                            if settings.req_admin_use_additional_privileges {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!("Sorting collation: < {} >", settings.sorting_collation),
                        false,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Treat digits as numbers",
                            if settings.treat_digits_as_numbers {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Case sensitive",
                            if settings.case_sensitive_sort {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "[{}] Auto save setup",
                            if settings.auto_save_setup { "x" } else { " " }
                        ),
                        false,
                    ));
                }
                1 => {
                    rows.push((
                        format!(
                            "[{}] Show hidden and system files",
                            if settings.show_hidden { "x" } else { " " }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "[{}] Highlight files",
                            if settings.highlight_files { "x" } else { " " }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "[{}] Select folders",
                            if settings.select_folders { "x" } else { " " }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "[{}] Right click selects files",
                            if settings.right_click_selects_files {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "[{}] Sort folder names by extension",
                            if settings.sort_folder_names_by_extension {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "[{}] Allow reverse sort modes",
                            if settings.sort_reverse { "x" } else { " " }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "Disable automatic panel update if object count exceeds: [ {} ]",
                            settings.disable_panel_update_object_count
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "[{}] Network drives autorefresh",
                            if settings.network_drives_autorefresh {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "[{}] Show column titles",
                            if settings.show_column_titles {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "[{}] Show status line",
                            if settings.show_status_line { "x" } else { " " }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "[{}] Detect volume mount points",
                            if settings.detect_volume_mount_points {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "[{}] Show files total information",
                            if settings.show_files_total_information {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "[{}] Show free size",
                            if settings.show_free_size { "x" } else { " " }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "[{}] Show scrollbar",
                            if settings.show_scrollbar { "x" } else { " " }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "[{}] Show background screens number",
                            if settings.show_background_screens_number {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "[{}] Show sort mode letter",
                            if settings.show_sort_mode_letter {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "[{}] Show \"..\" in root folders",
                            if settings.show_dotdot_in_root_folders {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push(("InfoPanel settings:".to_string(), false));
                    rows.push((
                        format!(
                            "  [{}] Show power status",
                            if settings.infopanel_show_power_status {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Show CD drive parameters",
                            if settings.infopanel_show_cd_drive_parameters {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "  Computer name format: < {} >",
                            settings.infopanel_computer_name_format
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "  User name format: < {} >",
                            settings.infopanel_user_name_format
                        ),
                        false,
                    ));
                    rows.push((
                        "Groups of file masks: [Ins/Del/F4/F7/Ctrl+R]".to_string(),
                        false,
                    ));
                    rows.push((
                        "Edit panel modes: [Ins/Del/F4/Ctrl+Enter]".to_string(),
                        false,
                    ));
                    rows.push(("File descriptions:".to_string(), false));
                    rows.push((
                        format!("  Names: [ {} ]", settings.file_descriptions_list_names),
                        false,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Set \"hidden\" attribute to new lists",
                            if settings.file_descriptions_set_hidden {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Update read only description file",
                            if settings.file_descriptions_update_readonly {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "  Position of new descriptions: [ {} ]",
                            settings.file_descriptions_position
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "  Update mode: < {} >",
                            settings.file_descriptions_update_mode
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Use ANSI code page by default",
                            if settings.file_descriptions_use_ansi {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Save in UTF-8",
                            if settings.file_descriptions_save_utf8 {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        false,
                    ));
                    rows.push((
                        format!(
                            "Folder description list names: [ {} ]",
                            settings.folder_description_list_names
                        ),
                        false,
                    ));
                }
                2 => {
                    rows.push((
                        format!(
                            "[{}] Clock",
                            if settings.interface_clock { "x" } else { " " }
                        ),
                        true,
                    ));
                    rows.push((
                        format!("[{}] Mouse", if settings.mouse_support { "x" } else { " " }),
                        false,
                    ));
                    rows.push((
                        format!(
                            "[{}] Show key bar",
                            if settings.interface_show_key_bar {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "[{}] Always show the menu bar",
                            if settings.interface_always_show_menu_bar {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "Screen saver: [ {} ] minutes",
                            settings.interface_screen_saver_minutes
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "[{}] Show total copy progress indicator",
                            if settings.interface_show_total_copy_progress {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "[{}] Show copying time information",
                            if settings.interface_show_copying_time {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "[{}] Show total delete progress indicator",
                            if settings.interface_show_total_delete_progress {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "[{}] Use Ctrl+PgUp to change drive",
                            if settings.interface_use_ctrl_pgup_change_drive {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "[{}] Use Virtual Terminal for rendering",
                            if settings.interface_use_virtual_terminal {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "[{}] Fullwidth-aware rendering",
                            if settings.interface_fullwidth_aware_rendering {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "[{}] ClearType-friendly redraw",
                            if settings.interface_cleartype_friendly_redraw {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!("Console icon: [ {} ]", settings.interface_console_icon),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Alternate for Administrator",
                            if settings.interface_console_icon_admin_alternate {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    if *editing_value && *cursor_idx == 14 {
                        rows.push((
                            format!("Far window title addons: [ {}◄ ]", edit_buffer),
                            false,
                        ));
                    } else {
                        rows.push((
                            format!(
                                "Far window title addons: [ {} ]",
                                settings.interface_window_title_addons
                            ),
                            true,
                        ));
                    }
                    rows.push(("Dialog settings:".to_string(), true));
                    rows.push((
                        format!(
                            "  [{}] History in dialog edit controls",
                            if settings.dialog_history_in_edit_controls {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Persistent blocks in edit controls",
                            if settings.dialog_persistent_blocks {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Del removes blocks in edit controls",
                            if settings.dialog_del_removes_blocks {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] AutoComplete in edit controls",
                            if settings.dialog_autocomplete {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Backspace deletes unchanged text",
                            if settings.dialog_backspace_deletes_unchanged {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Mouse click outside closes dialog",
                            if settings.dialog_mouse_click_outside_closes {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push(("Menu settings:".to_string(), true));
                    rows.push((
                        format!(
                            "  Left click outside: < {} >",
                            settings.menu_left_click_outside
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  Right click outside: < {} >",
                            settings.menu_right_click_outside
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  Middle click outside: < {} >",
                            settings.menu_middle_click_outside
                        ),
                        true,
                    ));
                    rows.push(("Command line settings:".to_string(), true));
                    rows.push((
                        format!(
                            "  [{}] Persistent blocks",
                            if settings.cmdline_persistent_blocks {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Del removes blocks",
                            if settings.cmdline_del_removes_blocks {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] AutoComplete",
                            if settings.cmdline_autocomplete {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  Set prompt format: [ {} ]",
                            settings.cmdline_prompt_format
                        ),
                        true,
                    ));
                    rows.push((
                        format!("  Use home dir: [ {} ]", settings.cmdline_use_home_dir),
                        true,
                    ));
                    rows.push(("AutoComplete settings:".to_string(), true));
                    rows.push((
                        format!(
                            "  [{}] Show a list",
                            if settings.autocomplete_show_list {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "    [{}] Modal mode",
                            if settings.autocomplete_modal_mode {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Append the first matched item",
                            if settings.autocomplete_append_first {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!("Keybindings preset: < {} >", settings.keybinding_preset),
                        false,
                    ));
                }
                3 => {
                    rows.push((
                        format!(
                            "[{}] Copy",
                            if settings.confirmations.confirm_copy {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "[{}] Move",
                            if settings.confirmations.confirm_move {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
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
                    rows.push((
                        format!(
                            "[{}] Drag and drop",
                            if settings.confirmations.confirm_drag_and_drop {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
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
                    rows.push((
                        format!(
                            "[{}] Delete non-empty folders",
                            if settings.confirmations.confirm_delete_non_empty_folders {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "[{}] Interrupt operation",
                            if settings.confirmations.confirm_interrupt_operation {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "[{}] Disconnect network drive",
                            if settings.confirmations.confirm_disconnect_network_drive {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "[{}] Delete SUBST-disk",
                            if settings.confirmations.confirm_delete_subst_disk {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "[{}] Detach virtual disk",
                            if settings.confirmations.confirm_detach_virtual_disk {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "[{}] HotPlug-device removal",
                            if settings.confirmations.confirm_hotplug_removal {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "[{}] Reload edited file",
                            if settings.confirmations.confirm_reload_edited_file {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "[{}] Clear history list",
                            if settings.confirmations.confirm_clear_history_list {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
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
                    rows.push((format!("Main language: < {} >", settings.language), false));
                    rows.push((
                        "Plugins configuration: [ArcLite | EMenu | HlfViewer | NetBox]".to_string(),
                        true,
                    ));
                    rows.push(("Plugins manager settings:".to_string(), true));
                    rows.push((
                        format!(
                            "  [{}] OEM plugins support",
                            if settings.plugins_manager_oem_support {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Scan symbolic links",
                            if settings.plugins_manager_scan_symlinks {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push(("  Plugin selection:".to_string(), true));
                    rows.push((
                        format!(
                            "    [{}] File processing",
                            if settings.plugins_manager_file_processing {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "      [{}] Show standard association",
                            if settings.plugins_manager_show_standard_association {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "        [{}] Even if only one plugin",
                            if settings.plugins_manager_even_if_one_found {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "    [{}] Search results (SetFindList)",
                            if settings.plugins_manager_search_results {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "    [{}] Prefix processing",
                            if settings.plugins_manager_prefix_processing {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                }
                5 => {
                    rows.push((
                        format!(
                            "[{}] Use external editor for F4 instead of Alt+F4",
                            if settings.editor_use_external {
                                "x"
                            } else {
                                " "
                            }
                        ),
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
                    rows.push((
                        format!("  Expand tabs: < {} >", settings.editor_expand_tabs),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Persistent blocks",
                            if settings.editor_persistent_blocks {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Cursor beyond end of line",
                            if settings.editor_cursor_beyond_eol {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Del removes blocks",
                            if settings.editor_del_removes_blocks {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Select found",
                            if settings.editor_select_found {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Auto indent",
                            if settings.editor_auto_indent {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Cursor at the end",
                            if settings.editor_cursor_at_end {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!("  Tab size: [ {} ]", settings.editor_tab_size),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Show scrollbar",
                            if settings.editor_show_scrollbar {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Show white space",
                            if settings.editor_show_white_space {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Show line numbers",
                            if settings.editor_show_line_numbers {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Save file position",
                            if settings.editor_save_file_position {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Save bookmarks",
                            if settings.editor_save_bookmarks {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Allow editing files opened for writing",
                            if settings.editor_allow_editing_opened_writing {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Lock editing of read-only files",
                            if settings.editor_lock_editing_readonly {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Warn when opening read-only files",
                            if settings.editor_warn_opening_readonly {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Autodetect code page",
                            if settings.editor_autodetect_codepage {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  Default code page: < {} >",
                            settings.editor_default_codepage
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "[{}] Use external viewer for F3 instead of Alt+F3",
                            if settings.viewer_use_external {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    if *editing_value && *cursor_idx == 22 {
                        rows.push((format!("Viewer command: [ {}◄ ]", edit_buffer), false));
                    } else {
                        rows.push((
                            format!("Viewer command: [ {} ]", settings.viewer_command),
                            true,
                        ));
                    }
                    rows.push(("Internal viewer:".to_string(), true));
                    rows.push((
                        format!(
                            "  [{}] Persistent selection",
                            if settings.viewer_persistent_selection {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Show scrolling arrows",
                            if settings.viewer_show_scrolling_arrows {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!("  Tab size: [ {} ]", settings.viewer_tab_size),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Visible '\\0'",
                            if settings.viewer_visible_zero {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Show scrollbar",
                            if settings.viewer_show_scrollbar {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Save file position",
                            if settings.viewer_save_file_position {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Save view mode",
                            if settings.viewer_save_view_mode {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Save file code page",
                            if settings.viewer_save_file_codepage {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Save wrap mode",
                            if settings.viewer_save_wrap_mode {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Save bookmarks",
                            if settings.viewer_save_bookmarks {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Detect dump view mode",
                            if settings.viewer_detect_dump_view_mode {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  Maximum line width: [ {} ]",
                            settings.viewer_max_line_width
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  [{}] Autodetect code page",
                            if settings.viewer_autodetect_codepage {
                                "x"
                            } else {
                                " "
                            }
                        ),
                        true,
                    ));
                    rows.push((
                        format!(
                            "  Default code page: < {} >",
                            settings.viewer_default_codepage
                        ),
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
            true
        }
        _ => false,
    }
}
