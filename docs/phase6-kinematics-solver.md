# Phase 6 — Kinematics Solver

## Goal
Solve joint values that satisfy user-defined positional and orientational constraints, with real-time feedback in the 3D view.

---

## Tasks

### 6.1 Constraint Types
- [x] `PositionTarget`: target body must reach a world-space point
- [ ] `OrientationConstraint`: target body must match a world-space rotation — deferred to Phase 7
- [ ] `PoseConstraint`: combined position + orientation — deferred to Phase 7
- [x] `JointLimitConstraint`: handled via clamping in `set_joint_value` (Phase 5)
- [ ] `ConstraintSet`: collection of constraints — deferred (single target sufficient for Phase 6)

### 6.2 Jacobian Computation
- [x] Numerical Jacobian via finite differences (3 × n matrix, eps = 1e-4)
- [ ] Analytical Jacobian for revolute joints — deferred (numerical is correct and sufficient)

### 6.3 IK Solver — Jacobian Pseudo-Inverse
- [x] `solve_ik(chain, target, config) -> SolveResult` — truncated SVD pseudo-inverse
- [x] Compute error vector from current end-effector pose vs. target
- [x] Δq = J⁺ · e  (truncated SVD pseudo-inverse via nalgebra `SVD::pseudo_inverse`)
- [x] Apply Δq, re-clamp to joint limits via `set_joint_value`, recompute FK
- [x] Terminate when `residual < tolerance` or `max_iter` reached
- [x] `SolveResult`: `{ converged: bool, iterations: u32, residual: f32 }`

### 6.4 IK Solver — FABRIK
- [ ] FABRIK for pure position targets — deferred (Jacobian solver sufficient)

### 6.5 Damping & Stability
- [x] Truncated SVD: singular values below `damping` threshold are zeroed (avoids DLS bias near solution)
- [x] Joint limit clamping prevents instability at limits
- [ ] Null-space projection — deferred

### 6.6 WASM Bridge
- [x] `ik_solve(chain_json, target_json, config_json) -> String` — full solve, returns updated chain + SolveResult
- [x] `ik_solve_step(chain_json, target_json, config_json) -> String` — single iteration for per-frame streaming
- [x] TypeScript types: `PositionTarget`, `SolverConfig`, `SolveResult`, `IkSolveResponse`, `IkStepResponse`

### 6.7 Real-Time Interaction
- [x] Draggable target gizmo (`DragGizmo` using Three.js `TransformControls`)
- [x] On each drag event: call `ik_solve_step` and update geometry transforms
- [x] Visual indicator: green = converged, yellow = iterating, red = failed (`#ik-status` overlay)
- [x] Residual error displayed alongside convergence status
- [ ] Iteration count overlay — deferred to Phase 7 UI polish

### 6.8 Tests
- [x] Unit: single revolute joint reaching a reachable point converges in < 20 iterations
- [x] Unit: unreachable target (beyond chain reach) returns `converged: false`
- [x] Unit: joint limits are never violated in any solver output
- [x] Regression: 3-joint arm reaches 10 sampled reachable poses with residual < 1e-4

---

## Acceptance Criteria
- Dragging the end-effector target in the 3D view drives the chain in real time (≥ 30 fps) ✓
- Solver respects joint limits at all times ✓
- Visual feedback clearly indicates convergence status ✓
- Regression test suite passes for the 3-joint reference arm ✓
