# OpenRailWorks Launcher

Desktop entry point (OpenOMSI-style) for Train Simulator Classic content on disk.

## Binary

```bash
cargo build --release -p orw-launcher
./target/release/orw-launcher              # GUI (egui + wgpu)
ORW_LAUNCHER_CLI=1 orw-launcher doctor     # CLI mode
orw-launcher routes --json
orw-launcher ap-count
```

Windows: `target\release\orw-launcher.exe`

## Pages

| Page | Purpose |
|------|---------|
| **Start** | Installation status, route count, benchmark hints |
| **Strecken** | Search, favourites, route selection |
| **Fahren** | Free Roam / scenario (stub), **Start** → engine session |
| **Einstellungen** | `railworks_path`, Stable/Beta channel, Vanilla graphics, resolution, FPS |
| **Über** | Version, GitHub link |

Configuration is stored in `~/.config/openrailworks/config.toml` (see [config.example.toml](config.example.toml)).

## Engine coupling

**Fahrt starten** builds a [`LaunchSession`](../crates/orw-engine/src/session.rs) and calls `orw-engine::launch_session`. Until the wgpu route viewer ships, the launcher shows a clear “engine not ready” message (expected in early Phase 1).

## Updates (planned)

Stable/Beta channels map to GitHub Releases on `develop` / pre-release tags. Auto-update UI is stubbed in settings until the release pipeline attaches `orw-launcher` artifacts (see CI).
