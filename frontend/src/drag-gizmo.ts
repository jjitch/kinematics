import * as THREE from "three";
import { TransformControls } from "three/addons/controls/TransformControls.js";

export type DragCallback = (x: number, y: number, z: number) => void;

export class DragGizmo {
  private target: THREE.Mesh;
  private controls: TransformControls;
  private onDragCb: DragCallback | null = null;

  constructor(
    scene: THREE.Scene,
    camera: THREE.Camera,
    domElement: HTMLElement,
  ) {
    // Small sphere to show the IK target
    const geo = new THREE.SphereGeometry(0.08, 12, 8);
    const mat = new THREE.MeshBasicMaterial({ color: 0xffff00 });
    this.target = new THREE.Mesh(geo, mat);
    scene.add(this.target);

    this.controls = new TransformControls(camera, domElement);
    this.controls.attach(this.target);
    this.controls.setMode("translate");
    scene.add(this.controls.getHelper());

    this.controls.addEventListener("objectChange", () => {
      const p = this.target.position;
      this.onDragCb?.(p.x, p.y, p.z);
    });
  }

  setPosition(x: number, y: number, z: number): void {
    this.target.position.set(x, y, z);
  }

  getPosition(): [number, number, number] {
    const p = this.target.position;
    return [p.x, p.y, p.z];
  }

  onDrag(cb: DragCallback): void {
    this.onDragCb = cb;
  }

  /** Disable orbit controls while dragging — pass OrbitControls.enabled. */
  connectOrbitControls(orbitControls: { enabled: boolean }): void {
    this.controls.addEventListener("dragging-changed", (e) => {
      orbitControls.enabled = !(e as unknown as { value: boolean }).value;
    });
  }

  setVisible(v: boolean): void {
    this.target.visible = v;
    this.controls.getHelper().visible = v;
  }

  dispose(): void {
    this.controls.dispose();
    this.target.geometry.dispose();
    (this.target.material as THREE.Material).dispose();
  }
}
