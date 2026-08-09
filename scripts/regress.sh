#!/usr/bin/env bash
# regress.sh — verify geode against known exact solutions
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "== geode regression: known exact solutions =="
cargo test --release --manifest-path "$ROOT/Cargo.toml" -- --nocapture 2>&1 | tail -5

echo "== all regressions passed =="
