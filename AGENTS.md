# Cloud Agent instructions

Read this file at the start of every Cloud Agent run on this repository.

## Project

**OpenRailWorks** — clean-room reimplementation of the **Train Simulator Classic** (RailWorks) runtime, inspired by [openOMSI](https://openomsi.org/). Multicore, wgpu, Win/Linux/macOS. **Ships no game content.**

- **GitHub:** https://github.com/Gegels/openrailworks (monorepo root)
- **License:** AGPL-3.0-only — see [LICENSE](LICENSE), [TRADEMARKS.md](TRADEMARKS.md)
- **Human handoff (German):** [docs/CLOUD_AGENT_HANDOFF.md](docs/CLOUD_AGENT_HANDOFF.md)

## Current phase

| Done (v0.0.1) | Next (Phase 1) |
|---------------|----------------|
| `orw-core` install + route catalog | `orw-vfs`: read-only `.ap` + loose files |
| CLI `orw-catalog` | `Tracks.bin` / blueprint parsers (stubs → real) |
| `orw-engine` + wgpu dep, SLO 60@1080p | wgpu window or headless CI smoke |
| CI matrix Win/Linux/macOS | First route **display** (no driving yet) |

Full roadmap: [docs/ROADMAP.md](docs/ROADMAP.md).

## Hard rules

1. **Never** commit TSC/Dovetail binaries, DLC assets, or Workshop dumps.
2. **Read-only** access to the user's `RailWorks` path (`RAILWORKS_PATH` / config).
3. Small, testable changes; run `cargo test --workspace` before commit.
4. Performance gates: **Ringbahn Berlin**, **Südast** — [docs/BENCHMARKS.md](docs/BENCHMARKS.md).
5. Update [docs/COMPAT_MATRIX.md](docs/COMPAT_MATRIX.md) when route compat changes.

## Monorepo layout

```text
crates/orw-core       # library: install discovery, routes
crates/orw-catalog-cli  # bin: orw-catalog
crates/orw-engine     # simulation + graphics (early)
docs/                 # ROADMAP, BENCHMARKS, FORMATS, LEGAL
scripts/              # build-linux.sh, push-github.sh
```

Add new crates under `crates/` and list them in root [Cargo.toml](Cargo.toml).

## Commands

```bash
cargo test --workspace
cargo build --release -p orw-catalog
cargo run -p orw-catalog -- doctor --railworks-path /path/to/RailWorks
```

## Git

- Push to GitHub: `./scripts/push-github.sh` (needs `GH_TOKEN` or SSH deploy key).
- User's TSC install is **not** on the cloud VM — use fixture dirs in tests.

## User context (Georg)

- Wants all routes smooth like openOMSI; incremental improvements on GitHub.
- Benchmark routes: Ringbahn Berlin, Südast (night FPS critical on Tempelhof–Südkreuz in original TSC).
