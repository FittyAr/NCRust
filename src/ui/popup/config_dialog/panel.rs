use crate::config::settings::Settings;

pub fn populate_rows(settings: &Settings, rows: &mut Vec<(String, bool)>) {
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
