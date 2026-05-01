#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "==> Building WASM (dev)..."
wasm-pack build "$REPO_ROOT/crates/wasm" \
    --target web \
    --out-dir "$REPO_ROOT/frontend/pkg" \
    --dev

echo "==> Installing frontend dependencies..."
npm install --prefix "$REPO_ROOT/frontend"

echo "==> Starting Vite dev server..."
npm run dev --prefix "$REPO_ROOT/frontend"
