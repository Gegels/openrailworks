# Contributing to OpenRailWorks

Thank you for helping build an openOMSI-style runtime for Train Simulator Classic.

## Principles

- **No game content** in pull requests (no DLC assets, `.ap` dumps, or DTG binaries).
- **Read-only** access to the user's RailWorks folder; never patch the original install.
- Small, reviewable changes per subsystem (catalog, streaming, Lua, signals, …).

## Development setup

```bash
git clone <your-fork>
cd openrailworks
rustup toolchain install stable
cargo test --workspace
cargo run -p orw-catalog -- doctor --railworks-path /path/to/RailWorks
```

## Compatibility matrix

When you fix or test a route, add a row to [docs/COMPAT_MATRIX.md](docs/COMPAT_MATRIX.md)
or attach `orw-catalog routes --json` output in the issue (no copyrighted assets).

## Issues

Use the bug template and include:

- OpenRailWorks version / git SHA
- Route and scenario (if applicable)
- TSC build
- Steps, logs, and whether the problem reproduces in **original TSC**

## Branches

- **`develop`** — all changes land here first.
- **`release`** — only via approved promotion PR from `develop` (see [docs/GIT_WORKFLOW.md](docs/GIT_WORKFLOW.md)).
- Tags `v*` on `release` trigger GitHub Release artifacts.

## License

Contributions are licensed under **AGPL-3.0-only**, same as the project.
