import * as THREE from "three";
import init, {
  chain_new,
  chain_add_body,
  chain_add_joint,
  chain_set_joint_value,
  chain_compute_fk,
  generate_box,
} from "../pkg/kinematics_wasm.js";
import { Renderer } from "./renderer.js";
import { ChainViz } from "./kinematic-viz.js";
import { jointTypeName } from "./kinematics.js";
import type { Chain, ChainOpResult, FkResult } from "./kinematics.js";

const STORAGE_KEY = "kinematics_chain_v1";

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
  const joint1 = rj1.id!;

  const rj2 = JSON.parse(
    chain_add_joint(chainJson, bodyB, bodyC, "revolute", 0, 0, 1, -Math.PI / 2, Math.PI / 2),
  ) as ChainOpResult;
  chainJson = JSON.stringify(rj2.chain!);
  const joint2 = rj2.id!;

  void joint1;
  void joint2;

  // Restore saved chain values if available
  const saved = localStorage.getItem(STORAGE_KEY);
  if (saved) chainJson = saved;

  // Attach geometry to each body (offset along X so links are visually separated)
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
  renderer.start();

  buildSliders(JSON.parse(chainJson) as Chain, (jointId, value, valueEl) => {
    const result = JSON.parse(
      chain_set_joint_value(chainJson, jointId, value),
    ) as ChainOpResult;
    if (result.ok) {
      chainJson = JSON.stringify(result.chain!);
      const actual = (JSON.parse(chainJson) as Chain).joints.find(
        (j) => j.id === jointId,
      )?.value ?? value;
      valueEl.textContent = actual.toFixed(3);
      updateFk();
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
  onChange: (jointId: number, value: number, valueEl: HTMLElement) => void,
): void {
  const panel = document.getElementById("slider-panel");
  if (!panel) return;
  panel.innerHTML = "";

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
    slider.addEventListener("input", () =>
      onChange(joint.id, parseFloat(slider.value), valueEl),
    );

    row.appendChild(label);
    row.appendChild(slider);
    row.appendChild(valueEl);
    panel.appendChild(row);
  }
}

main().catch(console.error);
