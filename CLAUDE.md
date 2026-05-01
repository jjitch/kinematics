# Development Practices

## 1. Keep docs up to date
- When implementing a phase, update the corresponding `docs/phaseN-*.md` checkboxes as tasks are completed.
- If the implementation deviates from the plan, update the plan to reflect reality — docs are the source of truth for future phases.

## 2. Test first, then implement
- Write the test (or tests) before writing the implementation code.
- For Rust: add `#[cfg(test)]` tests in the same file, or an integration test in `tests/`, before filling in the function body.
- For TypeScript: add a test case before adding the feature.
- A failing test that describes the desired behavior is a valid first commit; the implementation comes next.

## 3. CI must pass before reporting work as done
- Before saying a task is complete, verify that all CI checks pass.
- Locally this means running:
  ```
  cargo fmt --all -- --check
  cargo clippy --all-targets -- -D warnings
  cargo test --all
  bash scripts/build.sh
  ```
- Do not report a task as done if any of the above fail.
