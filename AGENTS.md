# Cloud Agent instructions

## Repository

- **Canonical GitHub:** https://github.com/Gegels/openrailworks (monorepo root)
- Rust workspace: `cargo test --workspace`, CLI: `cargo run -p orw-catalog -- doctor`
- Never commit TSC/DLC assets; read-only `RAILWORKS_PATH` on the developer machine.

## Git remotes in agent VMs

| Remote | Role |
|--------|------|
| `origin` | Primary push target when GitHub auth works (`Gegels/openrailworks`) |
| `cursor` | Cursor Origin mirror (optional) |

Push script: `./scripts/push-github.sh` (uses `GH_TOKEN` or SSH deploy key in `.deploy/`).

## Benchmark routes

Ringbahn Berlin, Südast — see `docs/BENCHMARKS.md`.
