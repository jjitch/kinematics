import { describe, it, expect } from "vitest";
import { KinematicsScene } from "./scene";

const triangleMesh = JSON.stringify({
  positions: [0, 0, 0, 1, 0, 0, 0, 1, 0],
  normals: [0, 0, 1, 0, 0, 1, 0, 0, 1],
  indices: [0, 1, 2],
});

describe("KinematicsScene", () => {
  it("starts with zero meshes", () => {
    const s = new KinematicsScene();
    expect(s.meshCount).toBe(0);
  });

  it("addMesh returns a unique string id", () => {
    const s = new KinematicsScene();
    const a = s.addMesh(triangleMesh);
    const b = s.addMesh(triangleMesh);
    expect(typeof a).toBe("string");
    expect(a).not.toBe(b);
  });

  it("addMesh increments meshCount", () => {
    const s = new KinematicsScene();
    s.addMesh(triangleMesh);
    expect(s.meshCount).toBe(1);
    s.addMesh(triangleMesh);
    expect(s.meshCount).toBe(2);
  });

  it("getMesh returns the THREE.Mesh for a valid id", () => {
    const s = new KinematicsScene();
    const id = s.addMesh(triangleMesh);
    expect(s.getMesh(id)).toBeDefined();
  });

  it("getMesh returns undefined for unknown id", () => {
    const s = new KinematicsScene();
    expect(s.getMesh("nope")).toBeUndefined();
  });

  it("removeMesh removes the mesh and returns true", () => {
    const s = new KinematicsScene();
    const id = s.addMesh(triangleMesh);
    expect(s.removeMesh(id)).toBe(true);
    expect(s.meshCount).toBe(0);
    expect(s.getMesh(id)).toBeUndefined();
  });

  it("removeMesh returns false for unknown id", () => {
    const s = new KinematicsScene();
    expect(s.removeMesh("ghost")).toBe(false);
  });

  it("clear removes all meshes", () => {
    const s = new KinematicsScene();
    s.addMesh(triangleMesh);
    s.addMesh(triangleMesh);
    s.clear();
    expect(s.meshCount).toBe(0);
  });

  it("getObjectIds returns all current ids", () => {
    const s = new KinematicsScene();
    const a = s.addMesh(triangleMesh);
    const b = s.addMesh(triangleMesh);
    expect(s.getObjectIds()).toContain(a);
    expect(s.getObjectIds()).toContain(b);
  });

  it("selectedId starts null", () => {
    const s = new KinematicsScene();
    expect(s.selectedId).toBeNull();
  });

  it("setSelected updates selectedId", () => {
    const s = new KinematicsScene();
    const id = s.addMesh(triangleMesh);
    s.setSelected(id);
    expect(s.selectedId).toBe(id);
  });

  it("setSelected(null) clears selection", () => {
    const s = new KinematicsScene();
    const id = s.addMesh(triangleMesh);
    s.setSelected(id);
    s.setSelected(null);
    expect(s.selectedId).toBeNull();
  });

  it("findMeshId retrieves id from THREE.Mesh reference", () => {
    const s = new KinematicsScene();
    const id = s.addMesh(triangleMesh);
    const mesh = s.getMesh(id)!;
    expect(s.findMeshId(mesh)).toBe(id);
  });
});
