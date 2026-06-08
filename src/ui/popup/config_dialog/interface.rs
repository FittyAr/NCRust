use crate::config::settings::Settings;

pub fn populate_rows(
    settings: &Settings,
    editing_value: bool,
    cursor_idx: usize,
    edit_buffer: &str,
    rows: &mut Vec<(String, bool)>,
) {
    rows.push((
        format!(
            "[{}] Clock",
            if settings.interface_clock { "x" } else { " " }
        ),
        false,
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
        false,
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
        false,
    ));
    rows.push((
        format!(
            "Screen saver: [ {} ] minutes",
            settings.interface_screen_saver_minutes
        ),
        false,
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
        false,
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
        false,
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
        false,
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
        false,
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
        false,
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
        false,
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
        false,
    ));
    rows.push((
        format!("Console icon: [ {} ]", settings.interface_console_icon),
        false,
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
        false,
    ));
    if editing_value && cursor_idx == 14 {
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
            false,
        ));
    }
    rows.push(("Dialog settings:".to_string(), false));
    rows.push((
        format!(
            "  [{}] History in dialog edit controls",
            if settings.dialog_history_in_edit_controls {
                "x"
            } else {
                " "
            }
        ),
        false,
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
        false,
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
        false,
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
        false,
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
        false,
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
        false,
    ));
    rows.push(("Menu settings:".to_string(), false));
    rows.push((
        format!(
            "  Left click outside: < {} >",
            settings.menu_left_click_outside
        ),
        false,
    ));
    rows.push((
        format!(
            "  Right click outside: < {} >",
            settings.menu_right_click_outside
        ),
        false,
    ));
    rows.push((
        format!(
            "  Middle click outside: < {} >",
            settings.menu_middle_click_outside
        ),
        false,
    ));
    rows.push(("Command line settings:".to_string(), false));
    rows.push((
        format!(
            "  [{}] Persistent blocks",
            if settings.cmdline_persistent_blocks {
                "x"
            } else {
                " "
            }
        ),
        false,
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
        false,
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
        false,
    ));
    rows.push((
        format!(
            "  Set prompt format: [ {} ]",
            settings.cmdline_prompt_format
        ),
        false,
    ));
    rows.push((
        format!("  Use home dir: [ {} ]", settings.cmdline_use_home_dir),
        false,
    ));
    rows.push(("AutoComplete settings:".to_string(), false));
    rows.push((
        format!(
            "  [{}] Show a list",
            if settings.autocomplete_show_list {
                "x"
            } else {
                " "
            }
        ),
        false,
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
        false,
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
        false,
    ));
    rows.push((
        format!("Keybindings preset: < {} >", settings.keybinding_preset),
        false,
    ));
}
