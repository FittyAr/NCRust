pub mod exec;
pub mod fs_ops;
pub mod navigation;
pub mod ui_settings;

pub use exec::execute_shell_command;

use crate::app::context::AppContext;
use crate::app::state::AppState;
use crate::keybindings::Action;
use crate::terminal::TerminalBackend;
use anyhow::Result;

/// Dispatches actions to their respective state changes.
pub async fn handle_action(
    state: &mut AppState,
    action: Action,
    context: &mut AppContext,
    terminal_backend: &mut TerminalBackend,
) -> Result<()> {
    if navigation::handle_navigation_action(state, &action, context) {
        return Ok(());
    }
    if fs_ops::handle_fs_action(state, &action, context, terminal_backend) {
        return Ok(());
    }
    if ui_settings::handle_ui_settings_action(state, &action, context) {
        return Ok(());
    }
    Ok(())
}
