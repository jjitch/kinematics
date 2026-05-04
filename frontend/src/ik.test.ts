import { describe, it, expect } from "vitest";
import {
  convergenceStatus,
  STATUS_COLOR,
} from "./ik.js";
import type { SolveResult } from "./ik.js";

describe("convergenceStatus", () => {
  it("returns converged when converged flag is true", () => {
    const r: SolveResult = { converged: true, iterations: 5, residual: 1e-6 };
    expect(convergenceStatus(r)).toBe("converged");
  });

  it("returns iterating when residual is within 10x tolerance", () => {
    const r: SolveResult = { converged: false, iterations: 50, residual: 5e-4 };
    expect(convergenceStatus(r, 1e-4)).toBe("iterating");
  });

  it("returns failed when residual is far above tolerance", () => {
    const r: SolveResult = { converged: false, iterations: 50, residual: 0.5 };
    expect(convergenceStatus(r)).toBe("failed");
  });

  it("uses default tolerance of 1e-4", () => {
    const r: SolveResult = { converged: false, iterations: 50, residual: 5e-5 };
    expect(convergenceStatus(r)).toBe("iterating");
  });
});

describe("STATUS_COLOR", () => {
  it("has a color for each status", () => {
    expect(STATUS_COLOR.converged).toBeTruthy();
    expect(STATUS_COLOR.iterating).toBeTruthy();
    expect(STATUS_COLOR.failed).toBeTruthy();
  });

  it("converged is green-ish", () => {
    expect(STATUS_COLOR.converged).toMatch(/^#[0-9a-f]{6}$/i);
  });
});
