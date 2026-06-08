use crate::config::settings::Settings;

pub fn handle_row(
    cursor_idx: usize,
    settings: &mut Settings,
    _editing_value: &mut bool,
    _edit_buffer: &mut String,
) {
    match cursor_idx {
        0 => settings.show_hidden = !settings.show_hidden,
        1 => settings.highlight_files = !settings.highlight_files,
        2 => settings.select_folders = !settings.select_folders,
        3 => {
            settings.right_click_selects_files = !settings.right_click_selects_files;
        }
        4 => {
            settings.sort_folder_names_by_extension = !settings.sort_folder_names_by_extension;
        }
        5 => settings.sort_reverse = !settings.sort_reverse,
        6 => {
            settings.disable_panel_update_object_count =
                match settings.disable_panel_update_object_count {
                    0 => 100,
                    100 => 1000,
                    1000 => 10000,
                    _ => 0,
                };
        }
        7 => {
            settings.network_drives_autorefresh = !settings.network_drives_autorefresh;
        }
        8 => settings.show_column_titles = !settings.show_column_titles,
        9 => settings.show_status_line = !settings.show_status_line,
        10 => {
            settings.detect_volume_mount_points = !settings.detect_volume_mount_points;
        }
        11 => {
            settings.show_files_total_information = !settings.show_files_total_information;
        }
        12 => settings.show_free_size = !settings.show_free_size,
        13 => settings.show_scrollbar = !settings.show_scrollbar,
        14 => {
            settings.show_background_screens_number = !settings.show_background_screens_number;
        }
        15 => settings.show_sort_mode_letter = !settings.show_sort_mode_letter,
        16 => {
            settings.show_dotdot_in_root_folders = !settings.show_dotdot_in_root_folders;
        }
        18 => {
            settings.infopanel_show_power_status = !settings.infopanel_show_power_status;
        }
        19 => {
            settings.infopanel_show_cd_drive_parameters =
                !settings.infopanel_show_cd_drive_parameters;
        }
        20 => {
            settings.infopanel_computer_name_format =
                match settings.infopanel_computer_name_format.as_str() {
                    "Physical NetBIOS" => "DNS name".to_string(),
                    _ => "Physical NetBIOS".to_string(),
                };
        }
        21 => {
            settings.infopanel_user_name_format = match settings.infopanel_user_name_format.as_str()
            {
                "Logon name" => "UPN".to_string(),
                _ => "Logon name".to_string(),
            };
        }
        25 => {
            settings.file_descriptions_list_names =
                match settings.file_descriptions_list_names.as_str() {
                    "Descript.ion,Files.bbs" => "descript.ion".to_string(),
                    "descript.ion" => "files.bbs".to_string(),
                    _ => "Descript.ion,Files.bbs".to_string(),
                };
        }
        26 => {
            settings.file_descriptions_set_hidden = !settings.file_descriptions_set_hidden;
        }
        27 => {
            settings.file_descriptions_update_readonly =
                !settings.file_descriptions_update_readonly;
        }
        28 => {
            settings.file_descriptions_position = match settings.file_descriptions_position {
                0 => 1,
                1 => 2,
                _ => 0,
            };
        }
        29 => {
            settings.file_descriptions_update_mode =
                match settings.file_descriptions_update_mode.as_str() {
                    "Do not update" => "Update if displayed".to_string(),
                    "Update if displayed" => "Always update".to_string(),
                    _ => "Do not update".to_string(),
                };
        }
        30 => {
            settings.file_descriptions_use_ansi = !settings.file_descriptions_use_ansi;
        }
        31 => {
            settings.file_descriptions_save_utf8 = !settings.file_descriptions_save_utf8;
        }
        32 => {
            settings.folder_description_list_names =
                match settings.folder_description_list_names.as_str() {
                    "DirInfo,File_Id.diz,Descript.ion,ReadMe.*,Read.Me" => {
                        "DirInfo,File_Id.diz".to_string()
                    }
                    _ => "DirInfo,File_Id.diz,Descript.ion,ReadMe.*,Read.Me".to_string(),
                };
        }
        _ => {}
    }
}
