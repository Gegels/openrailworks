#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
cargo build --release --workspace
cargo test --workspace
echo "Artifacts: target/release/orw-catalog"
