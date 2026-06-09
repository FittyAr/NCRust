use crate::config::settings::Settings;
use crate::config::localization::t;
use super::RowType;

pub fn populate_rows(settings: &Settings, rows: &mut Vec<(String, RowType)>) {
    rows.push(("Appearance & Theme".to_string(), RowType::Title));
    rows.push((format!("{}: < {} >", t("col_theme"), settings.theme), RowType::Setting(0)));
    rows.push((
        t("col_groups"),
        RowType::Setting(1),
    ));
    rows.push((
        t("col_highlighting"),
        RowType::Setting(2),
    ));
}
