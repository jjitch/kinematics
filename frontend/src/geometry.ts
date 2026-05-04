import * as THREE from "three";

export interface MeshData {
  positions: number[];
  normals: number[];
  indices: number[];
}

export function parseMeshJson(json: string): MeshData {
  return JSON.parse(json) as MeshData;
}

export function meshDataToGeometry(data: MeshData): THREE.BufferGeometry {
  const geo = new THREE.BufferGeometry();
  geo.setAttribute("position", new THREE.BufferAttribute(new Float32Array(data.positions), 3));
  geo.setAttribute("normal", new THREE.BufferAttribute(new Float32Array(data.normals), 3));
  geo.setIndex(new THREE.BufferAttribute(new Uint32Array(data.indices), 1));
  return geo;
}
