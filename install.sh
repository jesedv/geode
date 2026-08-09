#!/usr/bin/env bash
# install.sh — install the geode CLI from source
#
# Usage:
#   curl -sSf https://geode.jesed.dev/install.sh | sh
set -euo pipefail

GEODE_REPO="https://github.com/jesedv/geode.git"
BIN_DIR="${GEODE_BIN_DIR:-$HOME/.cargo/bin}"

bold=$(tput bold 2>/dev/null || true)
reset=$(tput sgr0 2>/dev/null || true)
green=$(tput setaf 2 2>/dev/null || true)
red=$(tput setaf 1 2>/dev/null || true)

info()  { printf "%s*%s %s\n" "$bold" "$reset" "$*"; }
ok()    { printf "%s✓%s %s\n" "$green" "$reset" "$*"; }
err()   { printf "%serror:%s %s\n" "$red" "$reset" "$*" >&2; exit 1; }

usage() {
    cat <<EOF
geode installer

Usage: install.sh [OPTIONS]

Options:
  --to <DIR>    Install binary to DIR (default: ~/.cargo/bin)
  --help        Show this help
EOF
}

while [ $# -gt 0 ]; do
    case "$1" in
        --to)    BIN_DIR="$2"; shift 2 ;;
        --help)  usage; exit 0 ;;
        *)       err "unknown option: $1" ;;
    esac
done

need() {
    command -v "$1" >/dev/null 2>&1 || err "$1 is required but not found. Install Rust: https://rustup.rs"
}

need git
need cargo

info "Installing geode from $GEODE_REPO ..."

TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT

if ! git clone --depth 1 "$GEODE_REPO" "$TMPDIR/geode" 2>/dev/null; then
    err "git clone failed — check your network and try again"
fi

info "Building geode (release, this may take a minute) ..."
if ! cargo build --release --manifest-path "$TMPDIR/geode/Cargo.toml" -p geode-solver 2>&1; then
    err "cargo build failed — check your Rust toolchain"
fi

mkdir -p "$BIN_DIR"
cp "$TMPDIR/geode/target/release/geode" "$BIN_DIR/geode"
chmod +x "$BIN_DIR/geode"

ok "geode installed to $BIN_DIR/geode"
echo ""
echo "  geode --help"
echo "  geode solve \"x^5 - x + 1\""