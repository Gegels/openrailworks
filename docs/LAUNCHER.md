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

## Pages (openOMSI-style)

| Page | Purpose |
|------|---------|
| **Start** | Installation status, route count, benchmark hints |
| **Strecken** | Search, favourites, route selection |
| **Fahren** | Free Roam / scenario (stub), **Start** → engine session |
| **Grafik / Kamera / Anzeige / Allgemein** | Settings tabs; changes apply **live** to the GameCam panel |
| **Über** | Version, GitHub link |

### GameCam preview (right panel)

On **Start**, **Fahren**, and all graphics-related settings pages, the launcher shows a **wgpu preview** that mirrors in-game camera/lighting:

- **Grafikmodus** — Vanilla / Vanilla+ / Enhanced (lighting and shadows)
- **Tageszeit** — sky gradient and fog
- **Sichtweite** — distance fog
- **Kamera** — FOV, Führerstand / Chase / Oben, orbit with mouse drag
- **Anzeige** — aspect ratio from resolution, render scale shrinks the preview viewport

Settings are written to `config.toml` as you change sliders (no separate save required).

Configuration is stored in `~/.config/openrailworks/config.toml` (see [config.example.toml](config.example.toml)).

## Engine coupling

**Fahrt starten** builds a [`LaunchSession`](../crates/orw-engine/src/session.rs) and calls `orw-engine::launch_session`. Until the wgpu route viewer ships, the launcher shows a clear “engine not ready” message (expected in early Phase 1).

## Updates (planned)

Stable/Beta channels map to GitHub Releases on `develop` / pre-release tags. Auto-update UI is stubbed in settings until the release pipeline attaches `orw-launcher` artifacts (see CI).
