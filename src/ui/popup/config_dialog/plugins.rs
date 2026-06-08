use crate::config::settings::Settings;

pub fn populate_rows(settings: &Settings, rows: &mut Vec<(String, bool)>) {
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
