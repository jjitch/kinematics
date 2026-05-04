import * as THREE from "three";
import type { Chain, FkResult, Pose } from "./kinematics.js";

export class ChainViz {
  private boneLines = new Map<number, THREE.Line>();
  private scene: THREE.Scene;

  constructor(scene: THREE.Scene) {
    this.scene = scene;
  }

  rebuildBones(chain: Chain): void {
    this.boneLines.forEach((line) => {
      line.geometry.dispose();
      (line.material as THREE.Material).dispose();
      this.scene.remove(line);
    });
    this.boneLines.clear();

    for (const joint of chain.joints) {
      const geo = new THREE.BufferGeometry().setFromPoints([
        new THREE.Vector3(0, 0, 0),
        new THREE.Vector3(0, 0, 0),
      ]);
      const mat = new THREE.LineBasicMaterial({ color: 0xffffff });
      const line = new THREE.Line(geo, mat);
      this.scene.add(line);
      this.boneLines.set(joint.id, line);
    }
  }

  updateFromFk(chain: Chain, fk: FkResult): void {
    for (const joint of chain.joints) {
      const line = this.boneLines.get(joint.id);
      if (!line) continue;
      const p = fk.transforms[joint.parent_body.toString()];
      const c = fk.transforms[joint.child_body.toString()];
      if (!p || !c) continue;
      line.geometry.setFromPoints([
        new THREE.Vector3(...p.translation),
        new THREE.Vector3(...c.translation),
      ]);
    }
  }

  applyFkToMeshes(meshMap: Map<string, THREE.Object3D>, fk: FkResult): void {
    for (const [id, pose] of Object.entries(fk.transforms)) {
      const obj = meshMap.get(id);
      if (obj) applyPose(obj, pose);
    }
  }

  dispose(): void {
    this.boneLines.forEach((line) => {
      line.geometry.dispose();
      (line.material as THREE.Material).dispose();
      this.scene.remove(line);
    });
    this.boneLines.clear();
  }

  getBoneCount(): number {
    return this.boneLines.size;
  }
}

function applyPose(obj: THREE.Object3D, pose: Pose): void {
  obj.position.set(...pose.translation);
  obj.quaternion.set(
    pose.rotation[0],
    pose.rotation[1],
    pose.rotation[2],
    pose.rotation[3],
  );
  obj.scale.set(...pose.scale);
}
