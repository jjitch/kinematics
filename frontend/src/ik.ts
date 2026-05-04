export interface PositionTarget {
  body_id: number;
  target: [number, number, number];
}

export interface SolverConfig {
  max_iter?: number;
  tolerance?: number;
  damping?: number;
  step_size?: number;
}

export interface SolveResult {
  converged: boolean;
  iterations: number;
  residual: number;
}

export interface IkSolveResponse {
  ok: boolean;
  chain?: import("./kinematics.js").Chain;
  result?: SolveResult;
  error?: string;
}

export interface IkStepResponse {
  ok: boolean;
  chain?: import("./kinematics.js").Chain;
  residual?: number;
  error?: string;
}

export type ConvergenceStatus = "converged" | "iterating" | "failed";

export function convergenceStatus(
  result: SolveResult,
  tolerance: number = 1e-4,
): ConvergenceStatus {
  if (result.converged) return "converged";
  if (result.residual < tolerance * 10) return "iterating";
  return "failed";
}

export const STATUS_COLOR: Record<ConvergenceStatus, string> = {
  converged: "#44ff88",
  iterating: "#ffcc44",
  failed: "#ff4444",
};
