use crate::config::settings::Settings;

pub fn populate_rows(settings: &Settings, rows: &mut Vec<(String, bool)>) {
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
