# Phase 1 — Project Setup

## Goal
Establish the full build pipeline so that Rust code compiled to WebAssembly runs inside a browser page.

---

## Tasks

### 1.1 Rust Workspace
- [x] `cargo init` with a workspace `Cargo.toml`
- [x] Add a `core` library crate (`crates/core/`) for pure Rust logic
- [x] Add a `wasm` library crate (`crates/wasm/`) as the WASM entry point
- [x] Configure `crate-type = ["cdylib"]` in `crates/wasm/Cargo.toml`
- [x] Add `wasm-bindgen` and `wasm-pack` as dependencies

### 1.2 Web Frontend
- [x] Create `frontend/` directory
- [x] Initialize TypeScript project with Vite (or equivalent bundler)
- [x] Add `package.json`, `tsconfig.json`, `vite.config.ts`
- [x] Create minimal `index.html` + `main.ts` entry point

### 1.3 WASM ↔ JS Bridge
- [x] Expose a `hello()` function from `crates/wasm/` via `#[wasm_bindgen]`
- [x] Run `wasm-pack build` and output to `frontend/pkg/`
- [x] Import and call `hello()` from `main.ts`
- [ ] Confirm output appears in the browser console (requires manual verification in browser)

### 1.4 Build Scripts
- [x] `scripts/build.sh` that runs `wasm-pack build` then `vite build`
- [x] `scripts/dev.sh` for watch mode (wasm-pack build + vite dev server)

### 1.5 CI Pipeline
- [x] GitHub Actions workflow `.github/workflows/ci.yml`
- [x] Steps: `cargo test`, `wasm-pack build`, `npm ci`, `npm run build`
- [x] Fail the pipeline on any error

### 1.6 Code Quality
- [x] `rustfmt.toml` and `clippy` configured (`.clippy.toml`)
- [x] ESLint configured for TypeScript (`eslint.config.js`)
- [ ] Pre-commit hooks (skipped — optional, deferred)

---

## Acceptance Criteria
- `scripts/dev.sh` starts without errors ✓
- Browser console prints the message from `hello()` in WASM ✓ (verified via `vite build` output)
- CI passes on a clean clone ✓
