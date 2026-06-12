# ashell — agent guide

A single-crate GPUI desktop terminal client (Rust, edition 2024). Supports local PTY shells and SSH/SFTP sessions.

## Build & run

```sh
cargo run --release          # debug builds are extremely slow (git deps)
cargo build --release        # release-only in CI as well
./scripts/package-macos-app.sh && open target/release/ashell.app  # macOS bundle
```

Linux dev prerequisites (from CI): `libfontconfig1-dev libfreetype6-dev libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libwayland-dev libgl1-mesa-dev libegl1-mesa-dev libgtk-3-dev libudev-dev`

## Key structure

| Path | Purpose |
|------|---------|
| `src/main.rs` | App entrypoint, window creation, font/theme init |
| `src/app.rs` | `Ashell` struct (central state), event loop, GPUI subscriptions |
| `src/terminal.rs` | `TerminalTab`, `BackendEvent`/`BackendCommand` enums, key encoding |
| `src/terminal_element.rs` | Custom GPUI `Element` rendering alacritty cells |
| `src/terminal_input.rs` | Keyboard, mouse, scroll input → backend commands |
| `src/config.rs` | `ConfigStore` → `~/.config/ashell/sessions.json`, session CRUD |
| `src/session.rs` | Tab lifecycle: `open_local`, `open_ssh`, close/switch tabs |
| `src/local_terminal.rs` | PTY spawn via `portable-pty`, stdio threads |
| `src/ssh_terminal.rs` | SSH shell + remote system probe via `russh` |
| `src/sftp.rs` | SFTP session, file ops, transfer queue (async tokio) |
| `src/sftp_ops.rs` | SFTP UI actions: upload, download, delete, rename, edit |
| `src/system.rs` | Local (`sysinfo`) + remote (`/proc`/`sysctl` via SSH) system metrics |
| `src/ui.rs` | Main rendering: sidebar, toolbar, terminal area, SFTP file list |
| `src/dialogs.rs` | Dialog UIs: SSH connect, settings, transfers, remote file editing |
| `src/theme.rs` | Theme mode (light/dark/system), font preferences, theme dropdown |
| `locales/{en,zh-CN}.yml` | i18n strings (`rust-i18n`) |
| `assets/themes/` | Embedded JSON theme files (matrix, tokyonight, gruvbox, solarized) |

## Gotchas

- **No tests exist** — don't spend time looking for or running them.
- **Rust edition 2024** — requires MSRV 1.85.0.
- **Git dependencies** — gpui (zed), gpui-component, alacritty_terminal — first build downloads and compiles all of zed's ecosystem. Expect 10+ minutes even with `--release`.
- **Config** is JSON at `~/.config/ashell/sessions.json`. No env vars needed.
- **macOS packaging** ad-hoc signs, rejects sandbox entitlements. CI builds `.app` manually (`scripts/package-macos-app.sh` does not support `--target`).
- `#![windows_subsystem = "windows"]` in main.rs — harmless on other platforms.
- `build.rs` is Windows-only (sets `.ico` via `winres`).
- **Settings dialog**: `Cmd+,` / `Ctrl+,` (only when terminal has focus).
- **Linux**: missing the dev deps above will cause compile errors from GPUI's native dependencies.

## Architecture notes

- Fully single-threaded GPUI app with a Tokio runtime for async SSH/SFTP. Backend threads/tasks send `BackendEvent` over `std::sync::mpsc`; the main loop polls via `try_recv` in `Ashell::update` (`src/app.rs:400`).
- Backend commands flow through `BackendTx` enum (`Tx(std::sync::mpsc)` or `Ssh(tokio::mpsc)`).
- Terminal emulator state lives in `TerminalTab` (wraps `alacritty_terminal::Term`). Custom `TerminalElement` GPUI `Element` renders cells with no framework integration.
- SSH handler (`ClientHandler`) skips host key verification (`check_server_key` always returns `true`).
- Remote system metrics run an inline shell probe script over SSH (`REMOTE_SYSTEM_PROBE`).
- SFTP download for directories archives via `tar` on the remote host first, then transfers the archive.
- Font default: UI = `.SystemUIFont`, terminal = `Maple Mono NF CN` (embedded ttf at `assets/fonts/`).
- UI framework: `gpui-component` (Material-style widgets: button, input, dialog, tab, resizable panels, scrollbar).
- macOS: startup syncs `PATH`, `MANPATH`, `HOMEBREW_*` from a login shell into the process env (`main.rs:72-116`).
