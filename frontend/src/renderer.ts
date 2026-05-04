import * as THREE from "three";
import { OrbitControls } from "three/addons/controls/OrbitControls.js";
import { KinematicsScene, ObjectId } from "./scene.js";

export interface RendererOptions {
  showFps?: boolean;
}

export class Renderer {
  readonly scene: KinematicsScene;
  private webgl: THREE.WebGLRenderer;
  private camera: THREE.PerspectiveCamera;
  private controls: OrbitControls;
  private axesHelper: THREE.AxesHelper;
  private gridHelper: THREE.GridHelper;
  private raycaster = new THREE.Raycaster();
  private pointer = new THREE.Vector2();
  private animId = 0;
  private onSelectCb: ((id: ObjectId | null) => void) | null = null;

  // FPS overlay
  private fpsEl: HTMLElement | null = null;
  private frameCount = 0;
  private lastFpsTime = 0;

  constructor(canvas: HTMLCanvasElement, opts: RendererOptions = {}) {
    this.scene = new KinematicsScene();

    this.webgl = new THREE.WebGLRenderer({ canvas, antialias: true });
    this.webgl.setPixelRatio(window.devicePixelRatio);
    this.webgl.setSize(canvas.clientWidth, canvas.clientHeight, false);
    this.webgl.shadowMap.enabled = true;
    this.webgl.shadowMap.type = THREE.PCFSoftShadowMap;

    this.camera = new THREE.PerspectiveCamera(
      60,
      canvas.clientWidth / canvas.clientHeight,
      0.01,
      1000,
    );
    this.camera.position.set(3, 3, 5);

    this.controls = new OrbitControls(this.camera, canvas);
    this.controls.enableDamping = true;
    this.controls.dampingFactor = 0.08;

    // Lighting: ambient + directional sun
    this.scene.three.add(new THREE.AmbientLight(0xffffff, 0.4));
    const sun = new THREE.DirectionalLight(0xffffff, 1.2);
    sun.position.set(5, 10, 7);
    sun.castShadow = true;
    sun.shadow.mapSize.set(1024, 1024);
    this.scene.three.add(sun);

    // Axes (XYZ corners) and ground grid
    this.axesHelper = new THREE.AxesHelper(1);
    this.scene.three.add(this.axesHelper);
    this.gridHelper = new THREE.GridHelper(10, 10, 0x888888, 0x444444);
    this.scene.three.add(this.gridHelper);

    if (opts.showFps) {
      this.fpsEl = document.createElement("div");
      Object.assign(this.fpsEl.style, {
        position: "fixed",
        top: "8px",
        right: "8px",
        color: "#0f0",
        fontFamily: "monospace",
        fontSize: "12px",
        pointerEvents: "none",
      });
      document.body.appendChild(this.fpsEl);
    }

    window.addEventListener("resize", this.onResize);
    canvas.addEventListener("click", this.onClick);
  }

  onSelect(cb: (id: ObjectId | null) => void): void {
    this.onSelectCb = cb;
  }

  /** Auto-frames the camera to fit all visible mesh objects. */
  fitToScene(): void {
    const box = new THREE.Box3();
    for (const id of this.scene.getObjectIds()) {
      const m = this.scene.getMesh(id);
      if (m) box.expandByObject(m);
    }
    if (box.isEmpty()) return;
    const size = box.getSize(new THREE.Vector3()).length();
    const center = box.getCenter(new THREE.Vector3());
    this.controls.target.copy(center);
    this.camera.position
      .copy(center)
      .addScaledVector(new THREE.Vector3(1, 0.8, 1).normalize(), size * 1.5);
    this.camera.near = size / 100;
    this.camera.far = size * 100;
    this.camera.updateProjectionMatrix();
    this.controls.update();
  }

  /** Keyboard shortcut: reset camera to home position. */
  resetCamera(): void {
    this.camera.position.set(3, 3, 5);
    this.controls.target.set(0, 0, 0);
    this.controls.update();
  }

  setAxesVisible(v: boolean): void {
    this.axesHelper.visible = v;
  }

  setGridVisible(v: boolean): void {
    this.gridHelper.visible = v;
  }

  start(): void {
    this.lastFpsTime = performance.now();
    const loop = (): void => {
      this.animId = requestAnimationFrame(loop);
      this.controls.update();
      this.webgl.render(this.scene.three, this.camera);
      if (this.fpsEl) this.tickFps();
    };
    loop();
  }

  stop(): void {
    cancelAnimationFrame(this.animId);
  }

  dispose(): void {
    this.stop();
    window.removeEventListener("resize", this.onResize);
    this.webgl.domElement.removeEventListener("click", this.onClick);
    this.controls.dispose();
    this.webgl.dispose();
    this.fpsEl?.remove();
  }

  private tickFps(): void {
    this.frameCount++;
    const now = performance.now();
    if (now - this.lastFpsTime >= 1000) {
      this.fpsEl!.textContent = `${this.frameCount} fps`;
      this.frameCount = 0;
      this.lastFpsTime = now;
    }
  }

  private onResize = (): void => {
    const c = this.webgl.domElement;
    const w = c.clientWidth;
    const h = c.clientHeight;
    this.webgl.setSize(w, h, false);
    this.camera.aspect = w / h;
    this.camera.updateProjectionMatrix();
  };

  private onClick = (event: MouseEvent): void => {
    const c = this.webgl.domElement;
    const rect = c.getBoundingClientRect();
    this.pointer.set(
      ((event.clientX - rect.left) / rect.width) * 2 - 1,
      -((event.clientY - rect.top) / rect.height) * 2 + 1,
    );
    this.raycaster.setFromCamera(this.pointer, this.camera);

    const meshes = this.scene
      .getObjectIds()
      .map((id) => this.scene.getMesh(id)!)
      .filter(Boolean);
    const hits = this.raycaster.intersectObjects(meshes);

    if (hits.length > 0) {
      const id = this.scene.findMeshId(hits[0].object as THREE.Mesh);
      if (id !== undefined) {
        this.scene.setSelected(id);
        console.log("Selected object:", id);
        this.onSelectCb?.(id);
        return;
      }
    }
    this.scene.setSelected(null);
    this.onSelectCb?.(null);
  };
}
