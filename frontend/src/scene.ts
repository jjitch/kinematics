import * as THREE from "three";
import { parseMeshJson, meshDataToGeometry } from "./geometry.js";

export type ObjectId = string;

export interface AddMeshOptions {
  color?: number;
  wireframe?: boolean;
  transparent?: boolean;
  opacity?: number;
}

export class KinematicsScene {
  readonly three = new THREE.Scene();
  private objects = new Map<ObjectId, THREE.Mesh>();
  private _nextId = 0;
  private _selectedId: ObjectId | null = null;

  addMesh(json: string, opts: AddMeshOptions = {}): ObjectId {
    const data = parseMeshJson(json);
    const geo = meshDataToGeometry(data);
    const mat = new THREE.MeshPhongMaterial({
      color: opts.color ?? 0x4488ff,
      wireframe: opts.wireframe ?? false,
      transparent: opts.transparent ?? false,
      opacity: opts.opacity ?? 1.0,
    });
    const mesh = new THREE.Mesh(geo, mat);
    const id: ObjectId = String(this._nextId++);
    this.objects.set(id, mesh);
    this.three.add(mesh);
    return id;
  }

  removeMesh(id: ObjectId): boolean {
    const mesh = this.objects.get(id);
    if (!mesh) return false;
    this.three.remove(mesh);
    mesh.geometry.dispose();
    (mesh.material as THREE.Material).dispose();
    this.objects.delete(id);
    if (this._selectedId === id) this._selectedId = null;
    return true;
  }

  clear(): void {
    for (const id of [...this.objects.keys()]) this.removeMesh(id);
  }

  get meshCount(): number {
    return this.objects.size;
  }

  getMesh(id: ObjectId): THREE.Mesh | undefined {
    return this.objects.get(id);
  }

  get selectedId(): ObjectId | null {
    return this._selectedId;
  }

  setSelected(id: ObjectId | null): void {
    if (this._selectedId !== null) {
      const prev = this.objects.get(this._selectedId);
      if (prev) (prev.material as THREE.MeshPhongMaterial).emissive.setHex(0x000000);
    }
    this._selectedId = id;
    if (id !== null) {
      const mesh = this.objects.get(id);
      if (mesh) (mesh.material as THREE.MeshPhongMaterial).emissive.setHex(0x333333);
    }
  }

  getObjectIds(): ObjectId[] {
    return [...this.objects.keys()];
  }

  findMeshId(threeMesh: THREE.Mesh): ObjectId | undefined {
    for (const [id, m] of this.objects) {
      if (m === threeMesh) return id;
    }
    return undefined;
  }
}
