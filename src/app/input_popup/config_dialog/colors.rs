use crate::config::settings::Settings;

pub fn handle_row(
    cursor_idx: usize,
    settings: &mut Settings,
    _editing_value: &mut bool,
    _edit_buffer: &mut String,
) {
    match cursor_idx {
        0 => {
            settings.theme = match settings.theme.as_str() {
                "slate" => "classic_blue".to_string(),
                _ => "slate".to_string(),
            };
        }
        _ => {}
    }
}
