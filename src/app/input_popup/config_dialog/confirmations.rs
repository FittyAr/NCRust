use crate::config::settings::Settings;

pub fn handle_row(
    cursor_idx: usize,
    settings: &mut Settings,
    _editing_value: &mut bool,
    _edit_buffer: &mut String,
) -> Option<crate::app::state::PopupType> {
    match cursor_idx {
        0 => {
            settings.confirmations.confirm_copy = !settings.confirmations.confirm_copy;
        }
        1 => {
            settings.confirmations.confirm_move = !settings.confirmations.confirm_move;
        }
        2 => {
            settings.confirmations.confirm_overwrite = !settings.confirmations.confirm_overwrite;
        }
        3 => {
            settings.confirmations.confirm_drag_and_drop =
                !settings.confirmations.confirm_drag_and_drop;
        }
        4 => {
            settings.confirmations.confirm_delete = !settings.confirmations.confirm_delete;
        }
        5 => {
            settings.confirmations.confirm_delete_non_empty_folders =
                !settings.confirmations.confirm_delete_non_empty_folders;
        }
        6 => {
            settings.confirmations.confirm_interrupt_operation =
                !settings.confirmations.confirm_interrupt_operation;
        }
        7 => {
            settings.confirmations.confirm_disconnect_network_drive =
                !settings.confirmations.confirm_disconnect_network_drive;
        }
        8 => {
            settings.confirmations.confirm_delete_subst_disk =
                !settings.confirmations.confirm_delete_subst_disk;
        }
        9 => {
            settings.confirmations.confirm_detach_virtual_disk =
                !settings.confirmations.confirm_detach_virtual_disk;
        }
        10 => {
            settings.confirmations.confirm_hotplug_removal =
                !settings.confirmations.confirm_hotplug_removal;
        }
        11 => {
            settings.confirmations.confirm_reload_edited_file =
                !settings.confirmations.confirm_reload_edited_file;
        }
        12 => {
            settings.confirmations.confirm_clear_history_list =
                !settings.confirmations.confirm_clear_history_list;
        }
        13 => {
            settings.confirmations.confirm_quit = !settings.confirmations.confirm_quit;
        }
        _ => {}
    }
    None
}
