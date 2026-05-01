#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "==> Building WASM..."
wasm-pack build "$REPO_ROOT/crates/wasm" \
    --target web \
    --out-dir "$REPO_ROOT/frontend/pkg" \
    --release

echo "==> Installing frontend dependencies..."
npm ci --prefix "$REPO_ROOT/frontend"

echo "==> Building frontend..."
npm run build --prefix "$REPO_ROOT/frontend"

echo "==> Done. Output: frontend/dist/"
