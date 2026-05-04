import { describe, it, expect } from "vitest";
import { exportPoseJson, exportTrajectoryCSV } from "./export.js";
import type { Chain } from "./kinematics.js";
import type { FkResult } from "./kinematics.js";

const sampleChain: Chain = {
  bodies: [{ id: 0, name: "A", local_transform: { translation: [0, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1] }, mesh_id: null }],
  joints: [],
};

const sampleFk: FkResult = {
  transforms: {
    "0": { translation: [1, 2, 3], rotation: [0, 0, 0, 1], scale: [1, 1, 1] },
  },
};

describe("exportPoseJson", () => {
  it("returns valid JSON string", () => {
    const json = exportPoseJson(sampleChain, sampleFk);
    expect(() => JSON.parse(json)).not.toThrow();
  });

  it("includes chain bodies", () => {
    const v = JSON.parse(exportPoseJson(sampleChain, sampleFk));
    expect(v.chain.bodies).toHaveLength(1);
  });

  it("includes transforms map", () => {
    const v = JSON.parse(exportPoseJson(sampleChain, sampleFk));
    expect(v.transforms["0"].translation).toEqual([1, 2, 3]);
  });
});

describe("exportTrajectoryCSV", () => {
  const keyframes = [
    { time: 0, jointValues: { 0: 0.0, 1: 0.0 } },
    { time: 1, jointValues: { 0: 0.5, 1: -0.3 } },
    { time: 2, jointValues: { 0: 1.0, 1: 0.5 } },
  ];

  it("first row is a header", () => {
    const csv = exportTrajectoryCSV(keyframes, [0, 1]);
    const header = csv.split("\n")[0];
    expect(header).toBe("time,j0,j1");
  });

  it("has one data row per keyframe", () => {
    const csv = exportTrajectoryCSV(keyframes, [0, 1]);
    const lines = csv.split("\n");
    expect(lines).toHaveLength(4); // header + 3 rows
  });

  it("data rows contain correct values", () => {
    const csv = exportTrajectoryCSV(keyframes, [0, 1]);
    const row1 = csv.split("\n")[1];
    expect(row1.startsWith("0.000")).toBe(true);
    expect(row1).toContain("0.000000");
  });

  it("handles empty keyframes", () => {
    const csv = exportTrajectoryCSV([], [0, 1]);
    expect(csv).toBe("time,j0,j1");
  });

  it("handles single joint", () => {
    const csv = exportTrajectoryCSV([{ time: 0, jointValues: { 5: 1.23 } }], [5]);
    const [header, row] = csv.split("\n");
    expect(header).toBe("time,j5");
    expect(row).toContain("1.230000");
  });
});
