# Phase 6 — Kinematics Solver

## Goal
Solve joint values that satisfy user-defined positional and orientational constraints, with real-time feedback in the 3D view.

---

## Tasks

### 6.1 Constraint Types
- [ ] `PositionConstraint`: target body must reach a world-space point
- [ ] `OrientationConstraint`: target body must match a world-space rotation
- [ ] `PoseConstraint`: combined position + orientation (6-DOF)
- [ ] `JointLimitConstraint`: already handled in Phase 5 via clamping — integrate here
- [ ] `ConstraintSet`: collection of constraints applied to one chain

### 6.2 Jacobian Computation
- [ ] `Chain::jacobian(end_effector_id) -> Matrix<f32>`
  - Columns = one per active DOF (joint)
  - Rows = 3 (position) or 6 (position + orientation)
- [ ] Numerical Jacobian via finite differences (simpler, correct by construction)
- [ ] Analytical Jacobian for revolute joints (faster, implement after numerical works)

### 6.3 IK Solver — Jacobian Pseudo-Inverse
- [ ] `IkSolver::solve(chain, constraints, max_iter, tolerance) -> SolveResult`
- [ ] Compute error vector from current end-effector pose vs. target
- [ ] Δq = J⁺ · e  (Moore-Penrose pseudo-inverse via SVD)
- [ ] Apply Δq, re-clamp to joint limits, recompute FK
- [ ] Terminate when `|e| < tolerance` or `max_iter` reached
- [ ] `SolveResult`: `{ converged: bool, iterations: u32, residual: f32, joint_values: Vec<f32> }`

### 6.4 IK Solver — FABRIK (alternative / fallback)
- [ ] Implement FABRIK (Forward And Backward Reaching IK) for pure position targets
- [ ] Use as fallback when Jacobian solver fails to converge
- [ ] Select solver via `SolverConfig { method: Jacobian | Fabrik | Auto }`

### 6.5 Damping & Stability
- [ ] Damped Least Squares (Levenberg-Marquardt): `J^T (JJ^T + λI)^{-1} e`
- [ ] Adaptive damping: increase λ when near singularity (small singular values)
- [ ] Null-space projection: use remaining DOFs to minimize joint displacement from rest pose

### 6.6 WASM Bridge
- [ ] `solve_ik(chain_json, constraints_json, config_json) -> JsValue` returns `SolveResult`
- [ ] Streaming solve: `solve_ik_step(state_json) -> JsValue` for per-frame iteration
- [ ] TypeScript types for `Constraint`, `SolverConfig`, `SolveResult`

### 6.7 Real-Time Interaction
- [ ] Draggable target gizmo in the 3D view (translate handle on end-effector target)
- [ ] On each drag event: call `solve_ik_step` and update geometry transforms
- [ ] Visual indicator: green = converged, yellow = iterating, red = failed
- [ ] Display residual error and iteration count in UI overlay

### 6.8 Tests
- [ ] Unit: single revolute joint reaching a reachable point converges in < 20 iterations
- [ ] Unit: unreachable target (beyond chain reach) returns `converged: false`
- [ ] Unit: joint limits are never violated in any solver output
- [ ] Regression: 3-joint arm reaches 10 sampled reachable poses with residual < 1e-4

---

## Acceptance Criteria
- Dragging the end-effector target in the 3D view drives the chain in real time (≥ 30 fps)
- Solver respects joint limits at all times
- Visual feedback clearly indicates convergence status
- Regression test suite passes for the 3-joint reference arm
