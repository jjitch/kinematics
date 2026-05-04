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
import { SceneTree } from "./scene-tree.js";
import { PropertiesPanel } from "./properties-panel.js";
import { UndoStack } from "./undo.js";
import { Timeline } from "./timeline.js";
import { exportPoseJson, exportTrajectoryCSV, downloadText } from "./export.js";
import { jointTypeName } from "./kinematics.js";
import { convergenceStatus, STATUS_COLOR } from "./ik.js";
import type { Chain, ChainOpResult, FkResult } from "./kinematics.js";
import type { IkStepResponse } from "./ik.js";

const STORAGE_KEY = "kinematics_chain_v1";
const BODY_COLORS = [0x4488ff, 0xff8844, 0x44cc88, 0xcc44cc, 0xffcc44, 0x44ccff];

async function main(): Promise<void> {
  await init();

  // ── State ──────────────────────────────────────────────────────────────────
  let chainJson: string;
  let ikBodyId: number | null = null;
  const bodyMeshIds = new Map<number, string>(); // bodyId → scene ObjectId

  // ── Core rendering ─────────────────────────────────────────────────────────
  const canvas = document.getElementById("viewport") as HTMLCanvasElement;
  const renderer = new Renderer(canvas);
  const viz = new ChainViz(renderer.scene.three);
  const gizmo = new DragGizmo(renderer.scene.three, renderer.camera, canvas);
  gizmo.connectOrbitControls(renderer.controls);
  gizmo.setVisible(false);

  // ── UI components ──────────────────────────────────────────────────────────
  const sceneTree = new SceneTree(document.getElementById("scene-tree-container")!);
  const propsPanel = new PropertiesPanel(document.getElementById("props-container")!);
  const undoStack = new UndoStack();
  const timeline = new Timeline();

  // ── DOM refs ───────────────────────────────────────────────────────────────
  const btnUndo = document.getElementById("btn-undo") as HTMLButtonElement;
  const btnRedo = document.getElementById("btn-redo") as HTMLButtonElement;
  const btnPlayPause = document.getElementById("btn-play-pause")!;
  const timelineTime = document.getElementById("timeline-time")!;
  const timelineScrub = document.getElementById("timeline-scrub") as HTMLInputElement;
  const timelineDuration = document.getElementById("timeline-duration") as HTMLInputElement;
  const kfList = document.getElementById("kf-list")!;
  const loopCheck = document.getElementById("loop-check") as HTMLInputElement;
  const ikBodySelect = document.getElementById("ik-body-select") as HTMLSelectElement;
  const statusFps = document.getElementById("status-fps")!;
  const statusIk = document.getElementById("status-ik")!;
  const statusSel = document.getElementById("status-sel")!;
  const ikOverlayStatus = document.getElementById("ik-status")!;
  const sliderPanel = document.getElementById("slider-panel")!;
  const sliderMap = new Map<number, HTMLInputElement>();

  // ── FPS counter ─────────────────────────────────────────────────────────────
  {
    let fc = 0, last = performance.now();
    const tick = (): void => {
      fc++;
      const now = performance.now();
      if (now - last >= 1000) { statusFps.textContent = String(fc); fc = 0; last = now; }
      requestAnimationFrame(tick);
    };
    requestAnimationFrame(tick);
  }

  // ── FK update (visuals only, no UI rebuild) ────────────────────────────────
  function quickUpdateFk(): void {
    const chain = JSON.parse(chainJson) as Chain;
    const fk = JSON.parse(chain_compute_fk(chainJson)) as FkResult;
    const meshMap = new Map<string, THREE.Object3D>();
    for (const [bodyId, meshId] of bodyMeshIds) {
      const m = renderer.scene.getMesh(meshId);
      if (m) meshMap.set(String(bodyId), m);
    }
    viz.rebuildBones(chain);
    viz.updateFromFk(chain, fk);
    viz.applyFkToMeshes(meshMap, fk);
    if (ikBodyId !== null) {
      const p = fk.transforms[String(ikBodyId)]?.translation;
      if (p) gizmo.setPosition(...p);
    }
  }

  // ── Full apply (structural change, undo/redo) ──────────────────────────────
  function applyChain(json: string): void {
    chainJson = json;
    const chain = JSON.parse(chainJson) as Chain;

    // Remove meshes for deleted bodies
    const liveIds = new Set(chain.bodies.map((b) => b.id));
    for (const [bodyId, meshId] of [...bodyMeshIds]) {
      if (!liveIds.has(bodyId)) {
        renderer.scene.removeMesh(meshId);
        bodyMeshIds.delete(bodyId);
      }
    }
    // Add meshes for new bodies
    chain.bodies.forEach((body, i) => {
      if (!bodyMeshIds.has(body.id)) {
        const id = renderer.scene.addMesh(generate_box(0.5, 0.25, 0.25), {
          color: BODY_COLORS[i % BODY_COLORS.length],
        });
        bodyMeshIds.set(body.id, id);
      }
    });

    quickUpdateFk();
    sceneTree.rebuild(chain);
    propsPanel.show(chain, sceneTree.getSelection());
    buildSliders(chain);
    updateIkBodySelect(chain);
    localStorage.setItem(STORAGE_KEY, chainJson);
  }

  // ── Slider panel ───────────────────────────────────────────────────────────
  function buildSliders(chain: Chain): void {
    sliderPanel.innerHTML = "";
    sliderMap.clear();
    for (const joint of chain.joints) {
      const parentName = chain.bodies.find((b) => b.id === joint.parent_body)?.name ?? "?";
      const childName = chain.bodies.find((b) => b.id === joint.child_body)?.name ?? "?";

      const row = document.createElement("div");
      row.className = "joint-row";

      const label = document.createElement("label");
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

      let dragStartJson = chainJson;
      slider.addEventListener("mousedown", () => { dragStartJson = chainJson; });
      slider.addEventListener("input", () => {
        valueEl.textContent = parseFloat(slider.value).toFixed(3);
        const r = JSON.parse(
          chain_set_joint_value(chainJson, joint.id, parseFloat(slider.value)),
        ) as ChainOpResult;
        if (r.ok) { chainJson = JSON.stringify(r.chain!); quickUpdateFk(); }
      });
      slider.addEventListener("change", () => {
        const finalJson = chainJson;
        undoStack.push(
          UndoStack.chainCmd(`Joint ${joint.id} drag`, dragStartJson, finalJson, applyChain),
        );
      });

      sliderMap.set(joint.id, slider);
      row.appendChild(label);
      row.appendChild(slider);
      row.appendChild(valueEl);
      sliderPanel.appendChild(row);
    }
  }

  function updateSliders(chain: Chain): void {
    for (const joint of chain.joints) {
      const s = sliderMap.get(joint.id);
      if (s) s.value = String(joint.value);
    }
  }

  // ── IK body selector ───────────────────────────────────────────────────────
  function updateIkBodySelect(chain: Chain): void {
    const prev = ikBodySelect.value;
    ikBodySelect.innerHTML = '<option value="">— none —</option>';
    for (const body of chain.bodies) {
      const opt = document.createElement("option");
      opt.value = String(body.id);
      opt.textContent = body.name;
      ikBodySelect.appendChild(opt);
    }
    if (prev && chain.bodies.some((b) => String(b.id) === prev)) {
      ikBodySelect.value = prev;
      ikBodyId = Number(prev);
    } else {
      ikBodyId = null;
      gizmo.setVisible(false);
    }
  }

  ikBodySelect.addEventListener("change", () => {
    ikBodyId = ikBodySelect.value ? Number(ikBodySelect.value) : null;
    gizmo.setVisible(ikBodyId !== null);
    if (ikBodyId !== null) {
      const fk = JSON.parse(chain_compute_fk(chainJson)) as FkResult;
      const p = fk.transforms[String(ikBodyId)]?.translation;
      if (p) gizmo.setPosition(...p);
    }
  });

  // ── IK drag ────────────────────────────────────────────────────────────────
  gizmo.onDrag(() => {
    if (ikBodyId === null) return;
    const target = JSON.stringify({ body_id: ikBodyId, target: gizmo.getPosition() });
    const res = JSON.parse(ik_solve_step(chainJson, target, "{}")) as IkStepResponse;
    if (res.ok) {
      chainJson = JSON.stringify(res.chain!);
      const chain = JSON.parse(chainJson) as Chain;
      quickUpdateFk();
      updateSliders(chain);
      propsPanel.show(chain, sceneTree.getSelection());
      const residual = res.residual ?? Infinity;
      const status = convergenceStatus(
        { converged: residual < 1e-4, iterations: 1, residual },
        1e-4,
      );
      const msg = `${status} (${residual.toExponential(2)})`;
      ikOverlayStatus.textContent = msg;
      (ikOverlayStatus as HTMLElement).style.color = STATUS_COLOR[status];
      statusIk.textContent = msg;
    }
  });

  // ── Undo/redo buttons ──────────────────────────────────────────────────────
  undoStack.onStateChange(() => {
    btnUndo.disabled = !undoStack.canUndo();
    btnRedo.disabled = !undoStack.canRedo();
  });
  document.getElementById("btn-undo")!.addEventListener("click", () => undoStack.undo());
  document.getElementById("btn-redo")!.addEventListener("click", () => undoStack.redo());

  // ── Toolbar ────────────────────────────────────────────────────────────────
  document.getElementById("btn-new")!.addEventListener("click", () => {
    if (!confirm("Start a new chain? Unsaved changes will be lost.")) return;
    for (const meshId of bodyMeshIds.values()) renderer.scene.removeMesh(meshId);
    bodyMeshIds.clear();
    undoStack.clear();
    timeline.clearKeyframes();
    refreshTimelineUi();
    ikBodyId = null;
    gizmo.setVisible(false);
    applyChain(chain_new());
    renderer.fitToScene();
  });

  document.getElementById("btn-save")!.addEventListener("click", () => {
    localStorage.setItem(STORAGE_KEY, chainJson);
  });

  document.getElementById("btn-add-body")!.addEventListener("click", () => {
    const chain = JSON.parse(chainJson) as Chain;
    const name = `Body${chain.bodies.length + 1}`;
    const prevJson = chainJson;
    const r = JSON.parse(chain_add_body(chainJson, name)) as ChainOpResult;
    if (r.ok) {
      undoStack.push(
        UndoStack.chainCmd(`Add ${name}`, prevJson, JSON.stringify(r.chain!), applyChain),
      );
    }
  });

  document.getElementById("btn-add-joint")!.addEventListener("click", () => {
    const sel = sceneTree.getSelection();
    if (!sel || sel.type !== "body") { alert("Select a parent body first."); return; }
    const prevJson = chainJson;
    const chain = JSON.parse(chainJson) as Chain;
    const childName = `Body${chain.bodies.length + 1}`;
    const rb = JSON.parse(chain_add_body(chainJson, childName)) as ChainOpResult;
    if (!rb.ok) return;
    const rj = JSON.parse(
      chain_add_joint(
        JSON.stringify(rb.chain!), sel.id, rb.id!, "revolute", 0, 1, 0, -Math.PI, Math.PI,
      ),
    ) as ChainOpResult;
    if (!rj.ok) return;
    undoStack.push(
      UndoStack.chainCmd(
        `Add joint ${sel.id}→${rb.id}`, prevJson, JSON.stringify(rj.chain!), applyChain,
      ),
    );
  });

  document.getElementById("btn-export-json")!.addEventListener("click", () => {
    const chain = JSON.parse(chainJson) as Chain;
    const fk = JSON.parse(chain_compute_fk(chainJson)) as FkResult;
    downloadText("pose.json", exportPoseJson(chain, fk), "application/json");
  });

  document.getElementById("btn-export-csv")!.addEventListener("click", () => {
    const chain = JSON.parse(chainJson) as Chain;
    const jointIds = chain.joints.map((j) => j.id);
    downloadText(
      "trajectory.csv",
      exportTrajectoryCSV([...timeline.getKeyframes()], jointIds),
      "text/csv",
    );
  });

  // ── Scene tree selection ───────────────────────────────────────────────────
  sceneTree.onSelect((sel) => {
    const chain = JSON.parse(chainJson) as Chain;
    propsPanel.show(chain, sel);
    if (!sel) { statusSel.textContent = "Nothing selected"; return; }
    if (sel.type === "body") {
      statusSel.textContent = `Body: ${chain.bodies.find((b) => b.id === sel.id)?.name ?? sel.id}`;
    } else {
      statusSel.textContent = `Joint: ${sel.id}`;
    }
  });

  // ── Properties panel changes ───────────────────────────────────────────────
  propsPanel.onChange((nextJson) => {
    undoStack.push(UndoStack.chainCmd("Edit property", chainJson, nextJson, applyChain));
  });

  // ── Timeline ───────────────────────────────────────────────────────────────
  function refreshTimelineUi(): void {
    const t = timeline.getCurrentTime();
    timelineTime.textContent = `${t.toFixed(2)}s`;
    timelineScrub.value = String(t);
    const kfs = timeline.getKeyframes();
    kfList.textContent = kfs.length > 0 ? kfs.map((k) => k.time.toFixed(2)).join("  ") : "—";
    btnPlayPause.textContent = timeline.isPlaying() ? "⏸" : "▶";
    btnPlayPause.classList.toggle("active", timeline.isPlaying());
  }

  timeline.onTimeUpdate((_t, vals) => {
    let cur = chainJson;
    for (const [id, val] of Object.entries(vals)) {
      const r = JSON.parse(chain_set_joint_value(cur, Number(id), val)) as ChainOpResult;
      if (r.ok) cur = JSON.stringify(r.chain!);
    }
    chainJson = cur;
    quickUpdateFk();
    updateSliders(JSON.parse(chainJson) as Chain);
    refreshTimelineUi();
  });

  document.getElementById("btn-stop")!.addEventListener("click", () => {
    timeline.stop(); refreshTimelineUi();
  });
  btnPlayPause.addEventListener("click", () => {
    if (timeline.isPlaying()) timeline.pause(); else timeline.play();
    refreshTimelineUi();
  });
  document.getElementById("btn-record")!.addEventListener("click", () => {
    const chain = JSON.parse(chainJson) as Chain;
    const vals: Record<number, number> = {};
    for (const j of chain.joints) vals[j.id] = j.value;
    timeline.recordKeyframe(timeline.getCurrentTime(), vals);
    refreshTimelineUi();
  });

  timelineScrub.max = String(timeline.getDuration());
  timelineScrub.addEventListener("input", () => {
    timeline.setTime(parseFloat(timelineScrub.value));
    refreshTimelineUi();
  });
  timelineDuration.addEventListener("change", () => {
    const d = parseFloat(timelineDuration.value);
    if (d > 0) { timeline.setDuration(d); timelineScrub.max = String(d); }
  });
  loopCheck.addEventListener("change", () => timeline.setLoop(loopCheck.checked));

  // ── Keyboard shortcuts ─────────────────────────────────────────────────────
  window.addEventListener("keydown", (e) => {
    const tag = (e.target as HTMLElement).tagName;
    if (tag === "INPUT" || tag === "SELECT") return;
    if (e.key === "f" || e.key === "F") { renderer.fitToScene(); return; }
    if (e.key === " ") {
      e.preventDefault();
      if (timeline.isPlaying()) timeline.pause(); else timeline.play();
      refreshTimelineUi();
      return;
    }
    if (e.ctrlKey && !e.shiftKey && e.key === "z") { e.preventDefault(); undoStack.undo(); return; }
    if (e.ctrlKey && (e.key === "y" || (e.shiftKey && e.key === "Z"))) {
      e.preventDefault(); undoStack.redo();
    }
  });

  // ── Init ───────────────────────────────────────────────────────────────────
  const saved = localStorage.getItem(STORAGE_KEY);
  if (saved) {
    chainJson = saved;
    const chain = JSON.parse(chainJson) as Chain;
    chain.bodies.forEach((body, i) => {
      const id = renderer.scene.addMesh(generate_box(0.5, 0.25, 0.25), {
        color: BODY_COLORS[i % BODY_COLORS.length],
      });
      bodyMeshIds.set(body.id, id);
    });
    quickUpdateFk();
    sceneTree.rebuild(chain);
    propsPanel.show(chain, null);
    buildSliders(chain);
    updateIkBodySelect(chain);
  } else {
    // Default 3-body chain
    chainJson = chain_new();
    const r0 = JSON.parse(chain_add_body(chainJson, "Base")) as ChainOpResult;
    chainJson = JSON.stringify(r0.chain!);
    const r1 = JSON.parse(chain_add_body(chainJson, "Link1")) as ChainOpResult;
    chainJson = JSON.stringify(r1.chain!);
    const r2 = JSON.parse(chain_add_body(chainJson, "Link2")) as ChainOpResult;
    chainJson = JSON.stringify(r2.chain!);
    const rj1 = JSON.parse(
      chain_add_joint(chainJson, r0.id!, r1.id!, "revolute", 0, 1, 0, -Math.PI, Math.PI),
    ) as ChainOpResult;
    chainJson = JSON.stringify(rj1.chain!);
    const rj2 = JSON.parse(
      chain_add_joint(
        chainJson, r1.id!, r2.id!, "revolute", 0, 0, 1, -Math.PI / 2, Math.PI / 2,
      ),
    ) as ChainOpResult;
    chainJson = JSON.stringify(rj2.chain!);
    applyChain(chainJson);
  }

  refreshTimelineUi();
  renderer.fitToScene();
  renderer.start();
}

main().catch(console.error);
