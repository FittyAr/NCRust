use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub panel_bg: String,
    pub panel_fg: String,
    pub panel_border: String,
    pub selection_bg: String,
    pub selection_fg: String,
    pub marked_fg: String,
    pub header_bg: String,
    pub header_fg: String,
    pub cli_bg: String,
    pub cli_fg: String,
    pub fkey_num_fg: String,
    pub fkey_text_fg: String,
    pub fkey_bg: String,
    pub popup_bg: String,
    pub popup_fg: String,
    pub popup_border: String,
}

impl Default for Theme {
    fn default() -> Self {
        // Modern slate/dark-mode theme as default
        Self {
            name: "slate".to_string(),
            panel_bg: "Reset".to_string(),
            panel_fg: "White".to_string(),
            panel_border: "DarkGray".to_string(),
            selection_bg: "Blue".to_string(),
            selection_fg: "White".to_string(),
            marked_fg: "Yellow".to_string(),
            header_bg: "Reset".to_string(),
            header_fg: "Cyan".to_string(),
            cli_bg: "Reset".to_string(),
            cli_fg: "White".to_string(),
            fkey_num_fg: "White".to_string(),
            fkey_text_fg: "Black".to_string(),
            fkey_bg: "Cyan".to_string(),
            popup_bg: "Black".to_string(),
            popup_fg: "White".to_string(),
            popup_border: "DarkGray".to_string(),
        }
    }
}

impl Theme {
    /// Generates the classic Norton Commander blue/cyan interface colors.
    pub fn classic_blue() -> Self {
        Self {
            name: "classic_blue".to_string(),
            panel_bg: "#0000AA".to_string(),
            panel_fg: "#AAAAAA".to_string(),
            panel_border: "#55FFFF".to_string(),
            selection_bg: "#00AAAA".to_string(),
            selection_fg: "#000000".to_string(),
            marked_fg: "#FFFF55".to_string(),
            header_bg: "#0000AA".to_string(),
            header_fg: "#55FFFF".to_string(),
            cli_bg: "#000000".to_string(),
            cli_fg: "#AAAAAA".to_string(),
            fkey_num_fg: "#FFFFFF".to_string(),
            fkey_text_fg: "#000000".to_string(),
            fkey_bg: "#00AAAA".to_string(),
            popup_bg: "#AAAAAA".to_string(),
            popup_fg: "#000000".to_string(),
            popup_border: "#000000".to_string(),
        }
    }
}
