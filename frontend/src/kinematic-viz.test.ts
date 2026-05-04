import { describe, it, expect } from "vitest";
import * as THREE from "three";
import { ChainViz } from "./kinematic-viz.js";
import type { Chain } from "./kinematics.js";

function twoBodyChain(): Chain {
  return {
    bodies: [
      {
        id: 0,
        name: "A",
        local_transform: { translation: [0, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1] },
        mesh_id: null,
      },
      {
        id: 1,
        name: "B",
        local_transform: { translation: [1, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1] },
        mesh_id: null,
      },
    ],
    joints: [
      {
        id: 0,
        parent_body: 0,
        child_body: 1,
        joint_type: { Revolute: { axis: [0, 1, 0] } },
        rest_transform: { translation: [0, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1] },
        min: -Math.PI,
        max: Math.PI,
        value: 0,
      },
    ],
  };
}

describe("ChainViz", () => {
  it("getBoneCount is 0 initially", () => {
    const viz = new ChainViz(new THREE.Scene());
    expect(viz.getBoneCount()).toBe(0);
  });

  it("rebuildBones creates one line per joint", () => {
    const viz = new ChainViz(new THREE.Scene());
    viz.rebuildBones(twoBodyChain());
    expect(viz.getBoneCount()).toBe(1);
  });

  it("rebuildBones adds lines to scene", () => {
    const scene = new THREE.Scene();
    const viz = new ChainViz(scene);
    const before = scene.children.length;
    viz.rebuildBones(twoBodyChain());
    expect(scene.children.length).toBe(before + 1);
  });

  it("rebuildBones replaces existing lines", () => {
    const scene = new THREE.Scene();
    const viz = new ChainViz(scene);
    viz.rebuildBones(twoBodyChain());
    const countAfterFirst = scene.children.length;
    viz.rebuildBones(twoBodyChain());
    expect(scene.children.length).toBe(countAfterFirst);
    expect(viz.getBoneCount()).toBe(1);
  });

  it("dispose removes lines from scene and resets count", () => {
    const scene = new THREE.Scene();
    const viz = new ChainViz(scene);
    viz.rebuildBones(twoBodyChain());
    const countAfterBuild = scene.children.length;
    viz.dispose();
    expect(scene.children.length).toBe(countAfterBuild - 1);
    expect(viz.getBoneCount()).toBe(0);
  });

  it("applyFkToMeshes updates mesh position", () => {
    const scene = new THREE.Scene();
    const viz = new ChainViz(scene);
    const mesh = new THREE.Mesh();
    scene.add(mesh);
    const meshMap = new Map<string, THREE.Object3D>([["0", mesh]]);
    const fkResult = {
      transforms: {
        "0": {
          translation: [3, 4, 5] as [number, number, number],
          rotation: [0, 0, 0, 1] as [number, number, number, number],
          scale: [1, 1, 1] as [number, number, number],
        },
      },
    };
    viz.applyFkToMeshes(meshMap, fkResult);
    expect(mesh.position.x).toBeCloseTo(3);
    expect(mesh.position.y).toBeCloseTo(4);
    expect(mesh.position.z).toBeCloseTo(5);
  });
});
