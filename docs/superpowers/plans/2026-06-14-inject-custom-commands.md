# Inject Custom Commands Features Implementation Plan

> **Goal:** Add flat-category custom commands, multi-line command editor, and UI cleanup to the `latest-combined` branch based on upstream code.

**Architecture:** Add `CommandItem/CommandFolder/CommandEntry` types to config, command-tree logic to app/mod.rs, UI rendering to app/ui.rs, editor dialogs to app/dialogs.rs, keyboard execution to terminal/input.rs. Keep all folders flat at root level.

**Tech Stack:** Rust, GPUI, gpui-component, serde

---

### Task 1: Add command types to `src/session/config.rs`

**Files:**
- Modify: `src/session/config.rs`

Add `CommandEntry`, `CommandFolder`, `CommandItem` types after the `Session` impl block, and add `custom_commands` field + accessors to `ConfigFile`/`ConfigStore`.

### Task 2: Add command fields & logic to `src/app/mod.rs`

**Files:**
- Modify: `src/app/mod.rs`

Add `MonitoringTab` enum, `FlatCommandItem` struct, `CommandContextMenu` struct. Add command-related fields to `Ashell` struct. Add `flatten_command_tree`, `remove_tree_item`, `set_tree_item`, `resolve_path`, `push_item_at` functions. Add `add_command_folder`, `add_command_item`, `navigate_up`, `current_children`, `get_item_at_path`, `get_command_at_path`, `rename_node`, `update_command_string`, `delete_item_recursive`, `navigate_into_folder` methods. Add new InputState inits and subscriptions.

### Task 3: Add custom commands UI & monitoring tab switching to `src/app/ui.rs`

**Files:**
- Modify: `src/app/ui.rs`
- Modify: `src/app/dialogs.rs`

Add `render_custom_commands_content` to `ui.rs`. Add `show_custom_command_dialog`, `show_new_folder_dialog`, `show_rename_dialog` to `dialogs.rs`. Update the monitoring tab header in `ui.rs` to include "Remote Files" | "Custom Commands" tab switching with `shared_header`. Remove `render_sidebar_monitoring_panel` from `sidebar()`.

### Task 4: Add command execution to `src/terminal/input.rs`

**Files:**
- Modify: `src/terminal/input.rs`

Add `execute_command_string`, `focus_commands_panel`, `on_commands_key_down`, `remove_custom_command` methods.

### Task 5: Update i18n and verify

**Files:**
- Modify: `locales/en.yml`
- Modify: `locales/zh-CN.yml`

Add command-related locale strings. Run `cargo check` to verify compilation.
