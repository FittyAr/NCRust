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
            "[{}] Use external editor for F4 instead of Alt+F4",
            if settings.editor_use_external {
                "x"
            } else {
                " "
            }
        ),
        true,
    ));
    if editing_value && cursor_idx == 1 {
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
    if editing_value && cursor_idx == 22 {
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
