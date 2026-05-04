import * as THREE from "three";
import init, {
  chain_new,
  chain_add_body,
  chain_add_joint,
  chain_set_joint_value,
  chain_compute_fk,
  ik_solve_step,
  generate_box,
} from "../pkg/kinematics_wasm.js";
import { Renderer } from "./renderer.js";
import { ChainViz } from "./kinematic-viz.js";
import { DragGizmo } from "./drag-gizmo.js";
import { jointTypeName } from "./kinematics.js";
import { convergenceStatus, STATUS_COLOR } from "./ik.js";
import type { Chain, ChainOpResult, FkResult } from "./kinematics.js";
import type { IkStepResponse } from "./ik.js";

const STORAGE_KEY = "kinematics_chain_v1";
const IK_BODY_ID = 3; // end-effector (body C = id 3 in the 3-body chain)

async function main(): Promise<void> {
  await init();

  const canvas = document.getElementById("viewport") as HTMLCanvasElement;
  const renderer = new Renderer(canvas, { showFps: true });
  const viz = new ChainViz(renderer.scene.three);

  // Build a 3-body chain: Base –(revolute Y)– Link1 –(revolute Z)– Link2
  let chainJson = chain_new();

  const r0 = JSON.parse(chain_add_body(chainJson, "Base")) as ChainOpResult;
  chainJson = JSON.stringify(r0.chain!);
  const bodyA = r0.id!;

  const r1 = JSON.parse(chain_add_body(chainJson, "Link1")) as ChainOpResult;
  chainJson = JSON.stringify(r1.chain!);
  const bodyB = r1.id!;

  const r2 = JSON.parse(chain_add_body(chainJson, "Link2")) as ChainOpResult;
  chainJson = JSON.stringify(r2.chain!);
  const bodyC = r2.id!;

  const rj1 = JSON.parse(
    chain_add_joint(chainJson, bodyA, bodyB, "revolute", 0, 1, 0, -Math.PI, Math.PI),
  ) as ChainOpResult;
  chainJson = JSON.stringify(rj1.chain!);

  const rj2 = JSON.parse(
    chain_add_joint(chainJson, bodyB, bodyC, "revolute", 0, 0, 1, -Math.PI / 2, Math.PI / 2),
  ) as ChainOpResult;
  chainJson = JSON.stringify(rj2.chain!);

  // Restore saved chain values if available
  const saved = localStorage.getItem(STORAGE_KEY);
  if (saved) chainJson = saved;

  // Attach geometry to each body
  const idA = renderer.scene.addMesh(generate_box(0.8, 0.4, 0.4), { color: 0x4488ff });
  const idB = renderer.scene.addMesh(generate_box(0.6, 0.3, 0.3), { color: 0xff8844 });
  const idC = renderer.scene.addMesh(generate_box(0.4, 0.25, 0.25), { color: 0x44cc88 });

  const bodyMeshMap = new Map<string, THREE.Object3D>([
    [String(bodyA), renderer.scene.getMesh(idA)!],
    [String(bodyB), renderer.scene.getMesh(idB)!],
    [String(bodyC), renderer.scene.getMesh(idC)!],
  ]);

  function updateFk(): void {
    const chain = JSON.parse(chainJson) as Chain;
    const fk = JSON.parse(chain_compute_fk(chainJson)) as FkResult;
    viz.rebuildBones(chain);
    viz.updateFromFk(chain, fk);
    viz.applyFkToMeshes(bodyMeshMap, fk);
    localStorage.setItem(STORAGE_KEY, chainJson);
  }

  updateFk();
  renderer.fitToScene();

  // IK drag gizmo — attach to end-effector (bodyC)
  const gizmo = new DragGizmo(renderer.scene.three, renderer.camera, canvas);
  gizmo.connectOrbitControls(renderer.controls);

  // Place gizmo at the current end-effector position
  const fk0 = JSON.parse(chain_compute_fk(chainJson)) as FkResult;
  const eePos0 = fk0.transforms[String(bodyC)]?.translation ?? [0, 0, 0];
  gizmo.setPosition(...eePos0);

  const statusEl = document.getElementById("ik-status") as HTMLElement;
  const targetJson = () =>
    JSON.stringify({ body_id: IK_BODY_ID, target: gizmo.getPosition() });

  gizmo.onDrag(() => {
    const stepResult = JSON.parse(
      ik_solve_step(chainJson, targetJson(), "{}"),
    ) as IkStepResponse;
    if (stepResult.ok) {
      chainJson = JSON.stringify(stepResult.chain!);
      updateFk();
      updateSliders(JSON.parse(chainJson) as Chain);
      // Update status indicator
      const residual = stepResult.residual ?? Infinity;
      const status = convergenceStatus(
        { converged: residual < 1e-4, iterations: 1, residual },
        1e-4,
      );
      if (statusEl) {
        statusEl.textContent = `${status} (${residual.toExponential(2)})`;
        statusEl.style.color = STATUS_COLOR[status];
      }
    }
  });

  renderer.start();

  // Slider map for live updates from IK
  const sliderMap = new Map<number, HTMLInputElement>();

  function updateSliders(chain: Chain): void {
    for (const joint of chain.joints) {
      const slider = sliderMap.get(joint.id);
      if (slider) slider.value = String(joint.value);
    }
  }

  buildSliders(JSON.parse(chainJson) as Chain, sliderMap, (jointId, value) => {
    const result = JSON.parse(
      chain_set_joint_value(chainJson, jointId, value),
    ) as ChainOpResult;
    if (result.ok) {
      chainJson = JSON.stringify(result.chain!);
      updateFk();
      // Move gizmo to follow the end-effector
      const fk = JSON.parse(chain_compute_fk(chainJson)) as FkResult;
      const p = fk.transforms[String(bodyC)]?.translation;
      if (p) gizmo.setPosition(...p);
    }
  });

  renderer.onSelect((id) => {
    document.title = id !== null ? `Selected: ${id}` : "Kinematics";
  });

  window.addEventListener("keydown", (e) => {
    if (e.key === "h" || e.key === "H") renderer.resetCamera();
  });
}

function buildSliders(
  chain: Chain,
  sliderMap: Map<number, HTMLInputElement>,
  onChange: (jointId: number, value: number) => void,
): void {
  const panel = document.getElementById("slider-panel");
  if (!panel) return;
  panel.innerHTML = "";
  sliderMap.clear();

  for (const joint of chain.joints) {
    const row = document.createElement("div");
    row.className = "joint-row";

    const label = document.createElement("label");
    const parentName = chain.bodies.find((b) => b.id === joint.parent_body)?.name ?? "?";
    const childName = chain.bodies.find((b) => b.id === joint.child_body)?.name ?? "?";
    label.textContent = `${parentName} → ${childName} (${jointTypeName(joint.joint_type)})`;

    const valueEl = document.createElement("span");
    valueEl.className = "joint-value";
    valueEl.textContent = joint.value.toFixed(3);

    const slider = document.createElement("input");
    slider.type = "range";
    slider.min = String(joint.min);
    slider.max = String(joint.max);
    slider.step = "0.01";
    slider.value = String(joint.value);
    slider.addEventListener("input", () => {
      valueEl.textContent = parseFloat(slider.value).toFixed(3);
      onChange(joint.id, parseFloat(slider.value));
    });

    sliderMap.set(joint.id, slider);

    row.appendChild(label);
    row.appendChild(slider);
    row.appendChild(valueEl);
    panel.appendChild(row);
  }
}

main().catch(console.error);
