# ashell — Agent Guide

## Overview

GPUI-based desktop terminal client (SSH + local). Rust desktop app using GPUI (Zed UI framework) + gpui-component (Longbridge) + alacritty_terminal.

## Build & Run

```bash
cargo run --release          # run (debug builds likely fail, use --release)
cargo build --release        # build
```

- `--release` is **required** — debug builds of GPUI typically fail or freeze.
- Rust edition **2024**, MSRV **1.85.0** (per `Cargo.toml`).

## macOS Packaging

```bash
./scripts/package-macos-app.sh
```

CI builds `.app` bundles manually in `.github/workflows/release.yml` (the script doesn't support `--target`).

## Config

- `~/.config/ashell/sessions.json` — JSON, permissions 0o600 on unix.
- Config dir permissions set to 0o700.
- Temporary SFTP files go to `~/.config/ashell/tmp/`.

## Logging

- `~/.config/ashell/log/ashell-YYYY-MM-DD-HH-MM.log`, rotated every minute, last 6 kept.
- Log level via `RUST_LOG` env var (default: `info`).
- Stdout logging only in debug builds (`cfg!(debug_assertions)`).

## i18n

- `rust-i18n` with `locales/en.yml` and `locales/zh-CN.yml`, fallback `"en"`.
- Hot-swappable at runtime.

## Key Dependencies

| Dep         | Source                                           |
|-------------|--------------------------------------------------|
| gpui        | `https://github.com/zed-industries/zed`          |
| gpui-component | `https://github.com/longbridge/gpui-component` (rev pinned) |
| alacritty_terminal | Zed's fork at `https://github.com/zed-industries/alacritty` |
| russh / russh-keys / russh-sftp | SSH + SFTP          |

## Architecture

Entry: `src/main.rs` → `app::startup::open_main_window` → `Ashell::new`

```
src/
├── main.rs          # platform entry, keybinding setup
├── app/             # UI: Ashell state, theme, startup, dialogs
├── backend/         # local.rs, ssh.rs — shell backends
├── session/         # config.rs — sessions.json model + persistence
├── sftp/            # SFTP operations
├── system/          # Local system metrics (sysinfo)
└── terminal/        # alacritty_terminal wrapper, input, rendering
```

- `#![windows_subsystem = "windows"]` at crate root.
- Tokio runtime created per `Ashell` instance (not global).
- macOS: syncs login shell env vars via `SHELL -l -c 'env -0'`.
- macOS: `on_reopen` handler to recreate window on dock click.
- Window bounds persisted on close, restored on launch.

## Platform Build Dependencies (Linux)

```bash
sudo apt-get install build-essential pkg-config cmake \
  libfontconfig1-dev libfreetype6-dev \
  libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
  libxkbcommon-dev libxkbcommon-x11-dev libwayland-dev \
  libgl1-mesa-dev libegl1-mesa-dev libgtk-3-dev libudev-dev
```

## Embedded Assets

- Fonts: `assets/fonts/MapleMono-NF-CN-{Regular,Bold}.ttf`
- Themes: 4 JSON themes in `assets/themes/` (loaded at startup)
- Icons: `assets/icons/ashell.{png,ico,icns}`

## Testing

**No tests exist in this repository.** No test framework is configured.

## CI

`.github/workflows/release.yml` — builds all platforms on tag push `v*` or manual dispatch.
`.github/workflows/build-windows.yml` — Windows-only manual dispatch.

## Notes

- No `rustfmt` config, no `clippy` config, no pre-commit hooks.
- No existing instruction files (CLAUDE.md, opencode.json, etc.).
