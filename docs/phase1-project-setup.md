# Phase 1 — Project Setup

## Goal
Establish the full build pipeline so that Rust code compiled to WebAssembly runs inside a browser page.

---

## Tasks

### 1.1 Rust Workspace
- [ ] `cargo init` with a workspace `Cargo.toml`
- [ ] Add a `core` library crate (`crates/core/`) for pure Rust logic
- [ ] Add a `wasm` library crate (`crates/wasm/`) as the WASM entry point
- [ ] Configure `crate-type = ["cdylib"]` in `crates/wasm/Cargo.toml`
- [ ] Add `wasm-bindgen` and `wasm-pack` as dependencies

### 1.2 Web Frontend
- [ ] Create `frontend/` directory
- [ ] Initialize TypeScript project with Vite (or equivalent bundler)
- [ ] Add `package.json`, `tsconfig.json`, `vite.config.ts`
- [ ] Create minimal `index.html` + `main.ts` entry point

### 1.3 WASM ↔ JS Bridge
- [ ] Expose a `hello()` function from `crates/wasm/` via `#[wasm_bindgen]`
- [ ] Run `wasm-pack build` and output to `frontend/pkg/`
- [ ] Import and call `hello()` from `main.ts`
- [ ] Confirm output appears in the browser console

### 1.4 Build Scripts
- [ ] `Makefile` or `scripts/build.sh` that runs `wasm-pack build` then `vite build`
- [ ] `scripts/dev.sh` for watch mode (wasm-pack watch + vite dev server)

### 1.5 CI Pipeline
- [ ] GitHub Actions workflow `.github/workflows/ci.yml`
- [ ] Steps: `cargo test`, `wasm-pack build`, `npm ci`, `npm run build`
- [ ] Fail the pipeline on any error

### 1.6 Code Quality
- [ ] `rustfmt.toml` and `clippy` configured
- [ ] ESLint + Prettier for TypeScript
- [ ] Pre-commit hooks (optional)

---

## Acceptance Criteria
- `scripts/dev.sh` starts without errors
- Browser console prints the message from `hello()` in WASM
- CI passes on a clean clone
