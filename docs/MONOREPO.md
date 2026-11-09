# Monorepo layout

All OpenRailWorks code lives in **one** Git repository (this repo). Crates are not published separately in early phases.

```text
openrailworks/                 # repository root (= this monorepo)
├── Cargo.toml                 # workspace manifest
├── crates/
│   ├── orw-core/              # install path, route catalog (library)
│   ├── orw-catalog-cli/       # `orw-catalog` binary
│   └── orw-engine/            # simulation / wgpu (library)
├── docs/                      # roadmap, benchmarks, legal, formats
├── scripts/                   # build + GitHub bootstrap
└── .github/                   # CI, issue templates
```

## Commands (from repo root)

| Task | Command |
|------|---------|
| Build all | `cargo build --release --workspace` |
| Test all | `cargo test --workspace` |
| CLI only | `cargo build --release -p orw-catalog` |
| Run catalog | `cargo run -p orw-catalog -- doctor` |

## Adding a crate

1. `cargo new crates/orw-<name> --lib` (or `--bin`)
2. Add the member to root `Cargo.toml` `[workspace.members]`
3. Extend CI if new artifacts should be uploaded

Future crates (not yet present): `orw-vfs`, `orw-lua`, `orw-launcher`, `orw-render`.

## GitHub

Primary remote should be **GitHub** (`github`). Cursor Origin (`origin`) may mirror the same history during migration. See [GITHUB.md](GITHUB.md).
