use crate::config::settings::Settings;

pub fn handle_row(
    cursor_idx: usize,
    settings: &mut Settings,
    _editing_value: &mut bool,
    _edit_buffer: &mut String,
) {
    match cursor_idx {
        0 => {
            settings.language = match settings.language.as_str() {
                "English" => "Spanish".to_string(),
                _ => "English".to_string(),
            };
        }
        3 => {
            settings.plugins_manager_oem_support = !settings.plugins_manager_oem_support;
        }
        4 => {
            settings.plugins_manager_scan_symlinks = !settings.plugins_manager_scan_symlinks;
        }
        6 => {
            settings.plugins_manager_file_processing = !settings.plugins_manager_file_processing;
        }
        7 => {
            settings.plugins_manager_show_standard_association =
                !settings.plugins_manager_show_standard_association;
        }
        8 => {
            settings.plugins_manager_even_if_one_found =
                !settings.plugins_manager_even_if_one_found;
        }
        9 => {
            settings.plugins_manager_search_results = !settings.plugins_manager_search_results;
        }
        10 => {
            settings.plugins_manager_prefix_processing =
                !settings.plugins_manager_prefix_processing;
        }
        _ => {}
    }
}
