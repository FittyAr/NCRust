# NCRust User Guide

This guide covers installation, configuration settings, styling themes, keyboard shortcuts, and file associations for **NCRust**.

---

## 🚀 Installation & Quick Start

### Build Requirements
Ensure you have the Rust compiler toolchain installed:
```bash
cargo build --release
```
The compiled executable will be located in `target/release/ncrust` (or `ncrust.exe` on Windows).

### Dual-Mode Startup
You can launch NCRust inside your current terminal session directly:
```bash
ncrust
```
To force NCRust to launch in its own standalone optimized terminal window, use the launcher scripts (`run.bat` on Windows or `run.sh` on Unix) or double-click the executables where standalone detection is configured.

---

## ⌨️ Keyboard Shortcuts Cheatsheet

NCRust supports multiple presets (Vim, Classic Norton, Modern). Below is the default **Norton** cheatsheet:

### Navigation & Panel Control
| Key | Action |
| :--- | :--- |
| `Tab` | Switch focus between the Left and Right panels. |
| `Up / Down` | Move file selection cursor. |
| `PageUp / PageDown` | Scroll the active file panel list by one page. |
| `Home / End` | Jump to the top or bottom of the panel list. |
| `Ctrl+U` | Swap Left and Right panel directories. |
| `Ctrl+H` | Toggle visibility of hidden files. |
| `Ctrl+R` | Force reload / refresh the current directories. |
| `Ctrl+\` | Open the Directory Hotlist bookmarks. |

### File Actions (F-Keys Bar)
| Key | Action |
| :--- | :--- |
| `F1` | Show Keybindings Help overlay. |
| `F2` | Open Top Menu Bar (Files, Commands, Options). |
| `F3` | Open internal Viewer (text or hex dump mode). |
| `F4` | Open internal Editor. |
| `F5` | Copy selected / tagged items to passive panel. |
| `F6` | Rename or Move selected / tagged items to passive panel. |
| `F7` | Create a new Directory (MkDir). |
| `F8` | Delete selected / tagged items. |
| `F9` | Save Setup configuration. |
| `F10` | Quit NCRust safely. |

### Selection
| Key | Action |
| :--- | :--- |
| `Insert` / `Space` | Toggle selection/tag on the current item. |
| `+` (Keypad) | Select a group of files matching a wildcard pattern (e.g. `*.rs`). |
| `-` (Keypad) | Deselect a group of files matching a wildcard pattern. |
| `*` (Keypad) | Invert selection state on the active panel list. |

---

## ⚙️ Settings Configuration Dialog (`F2 -> Options -> Configuration`)

The Configuration menu is divided into interactive tabbed pages:

### Tab 0: System
* **Delete to Recycle Bin:** Toggles moving files to system trash instead of permanent hard delete.
* **Use system copy routine:** Delegates copy routines to OS system methods.
* **Sorting collation:** Choose natural/linguistic file sorting.
* **Treat digits as numbers:** If enabled, `file2` sorts before `file10`.
* **Case sensitive sort:** Enable to sort uppercase names separate from lowercase.

### Tab 1: Panels
* **Show hidden and system files:** Show or hide system files.
* **Select folders:** If disabled, bulk tags apply to files only.
* **Sort folder names by extension:** Apply sort rules to folder extensions.
* **Show ".." in root folders:** Toggle parent directory navigators in disk roots.
* **InfoPanel settings:** Customize computer and user name layouts.

### Tab 2: Interface
* **Clock:** Displays a clock in the top-right corner.
* **Show key bar:** Toggle visibility of the F1-F10 shortcuts line at the bottom.
* **Always show the menu bar:** Always keep the top file menu visible.
* **Show total copy/delete progress indicator:** Show progress bars for background operations.
* **Keybindings preset:** Choose between `"norton"`, `"vim"`, or `"modern"` profiles.

### Tab 4: Language & Plugins
* **Main language:** Change translation tables (e.g., English, Spanish).
* **Scan symbolic links:** Toggle following folder symlinks recursively.

### Tab 5: Editor/Viewer
* **Use external editor / viewer:** Map F3/F4 to system commands (e.g., `vim`, `notepad`).
* **Tab size:** Set spaces count mapping to tab hits.
* **Show line numbers:** Displays row indexes in the editor.
* **Autodetect code page:** Automatically detect text encoding collations.

---

## 🎨 Themes & Custom Styling

Themes are located in `%APPDATA%/ncrust/config/themes/` (Windows) or `~/.config/ncrust/themes/` (Linux/macOS) in TOML format.

Example theme TOML:
```toml
[panel]
border = "Blue"
background = "Black"
file_selected = "Yellow"
file_directory = "Cyan"
file_executable = "Green"

[menu]
background = "Blue"
selected = "White"
```

You can choose your active theme by setting the `theme` key in your general config TOML file.
