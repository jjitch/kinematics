import init, {
  generate_box,
  generate_sphere,
  generate_cylinder,
} from "../pkg/kinematics_wasm.js";
import { Renderer } from "./renderer.js";

async function main(): Promise<void> {
  await init();

  const canvas = document.getElementById("viewport") as HTMLCanvasElement;
  const renderer = new Renderer(canvas, { showFps: true });

  // Populate a small demo scene from WASM geometry
  const boxId = renderer.scene.addMesh(generate_box(1.5, 1.5, 1.5), {
    color: 0x4488ff,
  });
  const sphereId = renderer.scene.addMesh(generate_sphere(0.8, 24, 12), {
    color: 0xff8844,
  });
  renderer.scene.getMesh(sphereId)?.position.set(2.8, 0, 0);

  const cylId = renderer.scene.addMesh(generate_cylinder(0.5, 2.0, 16), {
    color: 0x44cc88,
  });
  renderer.scene.getMesh(cylId)?.position.set(-2.8, 0, 0);

  renderer.fitToScene();
  renderer.start();

  renderer.onSelect((id) => {
    document.title = id !== null ? `Selected: ${id}` : "Kinematics";
  });

  // Press H to reset camera to home
  window.addEventListener("keydown", (e) => {
    if (e.key === "h" || e.key === "H") renderer.resetCamera();
  });

  console.log("Scene ready. Objects:", boxId, sphereId, cylId);
}

main().catch(console.error);
