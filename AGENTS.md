# ashell — agent guide

Single-crate GPUI desktop terminal (Rust edition 2024). Local PTY + SSH/SFTP.

## Build

```sh
cargo run --release             # debug is unusably slow (git deps)
cargo build --release           # CI only builds release
```

Linux deps (from CI): `libfontconfig1-dev libfreetype6-dev libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libxkbcommon-x11-dev libwayland-dev libgl1-mesa-dev libegl1-mesa-dev libgtk-3-dev libudev-dev`

macOS bundle: `./scripts/package-macos-app.sh && open target/release/ashell.app`

## Key files

| Path | Role |
|------|------|
| `src/main.rs` | Entrypoint, window, keybindings, macOS PATH sync |
| `src/app.rs` | `Ashell` central state, event loop, subscription wiring |
| `src/terminal.rs` | `TerminalTab`, `BackendEvent`/`BackendCommand` enums |
| `src/terminal_element.rs` | Custom GPUI `Element` rendering alacritty cells |
| `src/terminal_input.rs` | Keyboard/mouse/scroll → backend commands |
| `src/local_terminal.rs` | PTY spawn via `portable-pty`, stdio threads |
| `src/ssh_terminal.rs` | SSH shell + remote probe via `russh` |
| `src/sftp.rs` / `src/sftp_ops.rs` | SFTP session, file ops, transfer queue (tokio) |
| `src/system.rs` | Local (`sysinfo`) + remote (`/proc`/`sysctl` over SSH) metrics |
| `src/ui.rs` | Main layout: sidebar, toolbar, terminal, SFTP file list |
| `src/dialogs.rs` | Dialogs: SSH connect, settings, transfers, remote edit |
| `src/config.rs` | `~/.config/ashell/sessions.json` CRUD |
| `src/theme.rs` | Theme mode, font prefs, dropdown |
| `locales/{en,zh-CN}.yml` | i18n via `rust-i18n` macro |

## Gotchas

- **No tests.** Don't look for or run them.
- **Git deps are moving targets.** gpui (zed), gpui-component, alacritty_terminal are pinned to revisions but upstream breaks APIs without notice. Common symptom: `overflow_y_scroll()` → renamed to `overflow_y_scrollbar()`. First build compiles half of zed; expect 10+ min even with `--release`.
- **Rust edition 2024** → MSRV 1.85.0.
- **CI:** release workflow (`v*` tags) builds linux-x86_64 (ubuntu-22.04 for glibc compat), macos-aarch64, macos-x86_64 (cross), windows-x86_64. Linux deps must match ubuntu-22.04 names. Windows CI is a separate workflow (manual dispatch only). CI builds `.app` manually — `scripts/package-macos-app.sh` doesn't support `--target`.
- **macOS packaging** ad-hoc signs, rejects sandbox (`codesign --force --deep --sign -`).
- `#![windows_subsystem = "windows"]` — harmless on non-Windows.
- `build.rs` is Windows-only (`.ico` via `winres`).
- **Settings:** `Cmd+,` / `Ctrl+,` only when terminal has focus.
- **Config:** JSON at `~/.config/ashell/sessions.json`. No env vars needed.

## Architecture

- **Single-threaded GPUI** + Tokio runtime for async SSH/SFTP. Backend→main communication via `std::sync::mpsc` (polled with `try_recv` in `Ashell::update`).
- **Backend commands** flow through `BackendTx` enum (`Tx(std::sync::mpsc)` for local, `Ssh(tokio::mpsc)` for SSH).
- **Terminal emulator** uses `alacritty_terminal::Term` inside `TerminalTab`. Custom `TerminalElement` GPUI `Element` renders cells directly (no framework integration).
- **SSH:** `ClientHandler::check_server_key` always returns `true` (no host key verification).
- **Remote metrics** run an inline shell probe script over SSH (`REMOTE_SYSTEM_PROBE` string).
- **SFTP directory download** archives via remote `tar` first, then transfers the archive.
- **Fonts:** UI = `.SystemUIFont`, terminal = `Maple Mono NF CN` (embedded at `assets/fonts/`).
- **macOS startup** syncs `PATH`, `MANPATH`, `HOMEBREW_*` from a login shell into process env (`main.rs:72-116`).
- **UI widgets** from `gpui-component` (Material-style: button, input, dialog, tab, resizable panels, scrollbar).
