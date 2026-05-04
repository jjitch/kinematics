import { describe, it, expect } from "vitest";
import { identityPose, translationPose, jointTypeName } from "./kinematics.js";
import type { JointType } from "./kinematics.js";

describe("identityPose", () => {
  it("returns zero translation", () => {
    expect(identityPose().translation).toEqual([0, 0, 0]);
  });
  it("returns w=1 quaternion", () => {
    expect(identityPose().rotation).toEqual([0, 0, 0, 1]);
  });
  it("returns unit scale", () => {
    expect(identityPose().scale).toEqual([1, 1, 1]);
  });
});

describe("translationPose", () => {
  it("sets translation", () => {
    expect(translationPose(1, 2, 3).translation).toEqual([1, 2, 3]);
  });
  it("preserves identity rotation", () => {
    expect(translationPose(1, 2, 3).rotation).toEqual([0, 0, 0, 1]);
  });
  it("preserves unit scale", () => {
    expect(translationPose(1, 2, 3).scale).toEqual([1, 1, 1]);
  });
});

describe("jointTypeName", () => {
  it("names revolute", () => {
    const jt: JointType = { Revolute: { axis: [0, 1, 0] } };
    expect(jointTypeName(jt)).toBe("revolute");
  });
  it("names prismatic", () => {
    const jt: JointType = { Prismatic: { axis: [1, 0, 0] } };
    expect(jointTypeName(jt)).toBe("prismatic");
  });
  it("names fixed", () => {
    expect(jointTypeName("Fixed")).toBe("fixed");
  });
});
