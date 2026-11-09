# OpenRailWorks

Independent, open-source reimplementation of the **Train Simulator Classic** (RailWorks) runtime — inspired by [openOMSI](https://openomsi.org/). Your purchased routes, rolling stock, and scenarios stay on disk in the original installation; **this project ships no game content**.

> **Requires Train Simulator Classic.** Point OpenRailWorks at your existing RailWorks folder (Steam). The installation directory is never modified.

## Status (v0.0.1)

Early development. Today you get:

- **`orw-catalog`** — read-only scan of `Content/Routes`, `.ap` package counts, benchmark route hints (Ringbahn Berlin, Südast).
- **`orw-engine`** — shared performance SLO defaults (60 FPS @ 1080p Vanilla) and wgpu dependency for upcoming rendering work.
- Cross-platform **Rust** workspace (Windows, Linux, macOS) with CI builds.

Driving, rendering Berlin, or full Lua compatibility is **not** in this release yet. See [docs/ROADMAP.md](docs/ROADMAP.md).

## Quick start

```bash
# Build (needs Rust stable — see rust-toolchain.toml)
cargo build --release -p orw-catalog

# Windows example
set RAILWORKS_PATH=C:\Program Files (x86)\Steam\steamapps\common\RailWorks
orw-catalog doctor
orw-catalog routes
orw-catalog benchmarks

# JSON for tooling / CI on your machine
orw-catalog routes --json
```

### Config file (optional)

`~/.config/openrailworks/config.toml` on Linux/macOS, or `%APPDATA%\openrailworks\config.toml` on Windows:

```toml
railworks_path = "/path/to/RailWorks"
```

Environment variable **`RAILWORKS_PATH`** overrides the file. CLI flag **`--railworks-path`** overrides both.

## Platforms

| OS | Engine | TSC content |
|----|--------|-------------|
| Windows | Native | Typical Steam `...\common\RailWorks` |
| Linux | Native Vulkan (wgpu) | Path to Windows install (Proton prefix, mounted NTFS, etc.) |
| macOS | Native Metal (wgpu) | Path to Windows install on external / network volume |

## Building

```bash
./scripts/build-linux.sh
# scripts/build-macos.sh on macOS
# scripts\build-windows.cmd on Windows
```

## Legal

- License: [AGPL-3.0-only](LICENSE)
- Trademarks: [TRADEMARKS.md](TRADEMARKS.md)
- Contributing: [CONTRIBUTING.md](CONTRIBUTING.md)

Train Simulator Classic, RailWorks, and Dovetail Games are not affiliated with this project.

## Benchmark routes

Performance gates for the engine (see [docs/BENCHMARKS.md](docs/BENCHMARKS.md)):

- **Ringbahn Berlin** — urban density, night FPS on Tempelhof–Südkreuz in the original TSC.
- **Südast** — southern branch, same SLO.

Use `orw-catalog benchmarks` after installing those DLCs.
