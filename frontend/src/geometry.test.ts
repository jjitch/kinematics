import { describe, it, expect } from "vitest";
import { parseMeshJson, meshDataToGeometry } from "./geometry";

const triangleJson = JSON.stringify({
  positions: [0, 0, 0, 1, 0, 0, 0, 1, 0],
  normals: [0, 0, 1, 0, 0, 1, 0, 0, 1],
  indices: [0, 1, 2],
});

describe("parseMeshJson", () => {
  it("parses positions, normals, and indices", () => {
    const data = parseMeshJson(triangleJson);
    expect(data.positions).toHaveLength(9);
    expect(data.normals).toHaveLength(9);
    expect(data.indices).toEqual([0, 1, 2]);
  });

  it("throws on invalid JSON", () => {
    expect(() => parseMeshJson("not json")).toThrow();
  });
});

describe("meshDataToGeometry", () => {
  it("returns a BufferGeometry with correct vertex count", () => {
    const data = parseMeshJson(triangleJson);
    const geo = meshDataToGeometry(data);
    expect(geo.getAttribute("position").count).toBe(3);
    expect(geo.getAttribute("normal").count).toBe(3);
  });

  it("sets the index buffer", () => {
    const data = parseMeshJson(triangleJson);
    const geo = meshDataToGeometry(data);
    expect(geo.getIndex()).not.toBeNull();
    expect(geo.getIndex()!.count).toBe(3);
  });

  it("stores float32 position values", () => {
    const data = { positions: [1, 2, 3, 4, 5, 6, 7, 8, 9], normals: [0, 0, 1, 0, 0, 1, 0, 0, 1], indices: [0, 1, 2] };
    const geo = meshDataToGeometry(data);
    const arr = (geo.getAttribute("position") as { array: Float32Array }).array;
    expect(arr[0]).toBeCloseTo(1);
    expect(arr[1]).toBeCloseTo(2);
    expect(arr[2]).toBeCloseTo(3);
  });
});
