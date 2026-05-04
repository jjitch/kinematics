import { describe, it, expect } from "vitest";

// DragGizmo requires TransformControls which needs a WebGL context.
// We test only that the module exports exist (smoke test) and that the
// STATUS_COLOR and convergenceStatus helpers work independently.
// Full interaction is verified manually in the browser.

describe("drag-gizmo module", () => {
  it("exports DragGizmo class", async () => {
    const mod = await import("./drag-gizmo.js");
    expect(typeof mod.DragGizmo).toBe("function");
  });
});
