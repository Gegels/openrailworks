# OpenRailWorks

[![CI](https://github.com/Gegels/openrailworks/actions/workflows/ci.yml/badge.svg)](https://github.com/Gegels/openrailworks/actions/workflows/ci.yml)

**GitHub monorepo:** [github.com/Gegels/openrailworks](https://github.com/Gegels/openrailworks) — all crates and docs in one repo ([layout](docs/MONOREPO.md), [GitHub setup](docs/GITHUB.md)).

Independent, open-source reimplementation of the **Train Simulator Classic** (RailWorks) runtime — inspired by [openOMSI](https://openomsi.org/). Your purchased routes, rolling stock, and scenarios stay on disk in the original installation; **this project ships no game content**.

> **Requires Train Simulator Classic.** Point OpenRailWorks at your existing RailWorks folder (Steam). The installation directory is never modified.

## Status (v0.0.1)

Early development. Today you get:

- **`orw-launcher`** — OpenOMSI-style desktop UI (Strecken, Fahren, Einstellungen) plus CLI (`doctor`, `routes`, `ap-count`). See [docs/LAUNCHER.md](docs/LAUNCHER.md).
- **`orw-catalog`** — read-only scan of `Content/Routes`, `.ap` package counts, benchmark route hints (Ringbahn Berlin, Südast).
- **`orw-vfs`** — unified read path for loose files and `.ap` (ZIP) archives under your installation.
- **`orw-engine`** — `LaunchSession` model, performance SLO (60 FPS @ 1080p Vanilla), wgpu dependency; interactive runtime next.
- Cross-platform **Rust** workspace (Windows, Linux, macOS) with CI builds.

Driving, rendering Berlin, or full Lua compatibility is **not** in this release yet. See [docs/ROADMAP.md](docs/ROADMAP.md).

Neuer **Cloud-Agent**? Lies [AGENTS.md](AGENTS.md), Branch-Modell [docs/GIT_WORKFLOW.md](docs/GIT_WORKFLOW.md), [docs/CLOUD_AGENT_HANDOFF.md](docs/CLOUD_AGENT_HANDOFF.md).

**Branches:** `develop` = Entwicklung · `release` = freigegeben (PR von dir).

## Quick start

```bash
# Build (needs Rust stable — see rust-toolchain.toml)
cargo build --release -p orw-launcher

# or diagnostics-only CLI:
cargo build --release -p orw-catalog

# Windows example
set RAILWORKS_PATH=C:\Program Files (x86)\Steam\steamapps\common\RailWorks
orw-launcher
# or: orw-catalog doctor
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
