use crate::config::settings::Settings;

pub fn populate_rows(settings: &Settings, rows: &mut Vec<(String, bool)>) {
    rows.push((format!("Theme: < {} >", settings.theme), false));
    rows.push((
        "Color groups: [ Panel | Dialog | Menu | clock | ... ]".to_string(),
        true,
    ));
    rows.push((
        "Files highlighting: [ +H | +S | +D | <exec> | <arc> | <temp> ]".to_string(),
        true,
    ));
}
