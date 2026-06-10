use super::RowType;
use crate::config::localization::t;
use crate::config::settings::Settings;

pub fn populate_rows(settings: &Settings, rows: &mut Vec<(String, RowType)>) {
    rows.push(("File Operations".to_string(), RowType::Title));
    rows.push((
        format!(
            "[{}] {}",
            if settings.confirmations.confirm_copy {
                "x"
            } else {
                " "
            },
            t("conf_copy")
        ),
        RowType::Setting(0),
    ));
    rows.push((
        format!(
            "[{}] {}",
            if settings.confirmations.confirm_move {
                "x"
            } else {
                " "
            },
            t("conf_move")
        ),
        RowType::Setting(1),
    ));
    rows.push((
        format!(
            "[{}] {}",
            if settings.confirmations.confirm_overwrite {
                "x"
            } else {
                " "
            },
            t("conf_overwrite")
        ),
        RowType::Setting(2),
    ));
    rows.push((
        format!(
            "[{}] {}",
            if settings.confirmations.confirm_drag_and_drop {
                "x"
            } else {
                " "
            },
            t("conf_drag_drop")
        ),
        RowType::Setting(3),
    ));
    rows.push((
        format!(
            "[{}] {}",
            if settings.confirmations.confirm_delete {
                "x"
            } else {
                " "
            },
            t("conf_delete")
        ),
        RowType::Setting(4),
    ));
    rows.push((
        format!(
            "[{}] {}",
            if settings.confirmations.confirm_delete_non_empty_folders {
                "x"
            } else {
                " "
            },
            t("conf_delete_non_empty")
        ),
        RowType::Setting(5),
    ));

    rows.push(("Drives & System".to_string(), RowType::Title));
    rows.push((
        format!(
            "[{}] {}",
            if settings.confirmations.confirm_interrupt_operation {
                "x"
            } else {
                " "
            },
            t("conf_interrupt")
        ),
        RowType::Setting(6),
    ));
    rows.push((
        format!(
            "[{}] {}",
            if settings.confirmations.confirm_disconnect_network_drive {
                "x"
            } else {
                " "
            },
            t("conf_disconnect")
        ),
        RowType::Setting(7),
    ));
    rows.push((
        format!(
            "[{}] {}",
            if settings.confirmations.confirm_delete_subst_disk {
                "x"
            } else {
                " "
            },
            t("conf_delete_subst")
        ),
        RowType::Setting(8),
    ));
    rows.push((
        format!(
            "[{}] {}",
            if settings.confirmations.confirm_detach_virtual_disk {
                "x"
            } else {
                " "
            },
            t("conf_detach_vdisk")
        ),
        RowType::Setting(9),
    ));
    rows.push((
        format!(
            "[{}] {}",
            if settings.confirmations.confirm_hotplug_removal {
                "x"
            } else {
                " "
            },
            t("conf_hotplug")
        ),
        RowType::Setting(10),
    ));

    rows.push(("General".to_string(), RowType::Title));
    rows.push((
        format!(
            "[{}] {}",
            if settings.confirmations.confirm_reload_edited_file {
                "x"
            } else {
                " "
            },
            t("conf_reload")
        ),
        RowType::Setting(11),
    ));
    rows.push((
        format!(
            "[{}] {}",
            if settings.confirmations.confirm_clear_history_list {
                "x"
            } else {
                " "
            },
            t("conf_clear_history")
        ),
        RowType::Setting(12),
    ));
    rows.push((
        format!(
            "[{}] {}",
            if settings.confirmations.confirm_quit {
                "x"
            } else {
                " "
            },
            t("conf_exit")
        ),
        RowType::Setting(13),
    ));
}
