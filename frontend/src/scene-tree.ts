import type { Chain } from "./kinematics.js";
import { jointTypeName } from "./kinematics.js";

export type SelectionType = "body" | "joint";
export interface Selection {
  type: SelectionType;
  id: number;
}

export type SelectCallback = (sel: Selection | null) => void;

export class SceneTree {
  private container: HTMLElement;
  private onSelectCb: SelectCallback | null = null;
  private selection: Selection | null = null;

  constructor(container: HTMLElement) {
    this.container = container;
  }

  onSelect(cb: SelectCallback): void {
    this.onSelectCb = cb;
  }

  getSelection(): Selection | null {
    return this.selection;
  }

  rebuild(chain: Chain): void {
    this.container.innerHTML = "";

    for (const body of chain.bodies) {
      const el = document.createElement("div");
      el.className = "tree-item tree-body";
      el.dataset["type"] = "body";
      el.dataset["id"] = String(body.id);
      el.textContent = `⬡ ${body.name}`;
      if (this.selection?.type === "body" && this.selection.id === body.id) {
        el.classList.add("selected");
      }
      el.addEventListener("click", () => this.select({ type: "body", id: body.id }));
      this.container.appendChild(el);

      // Show joints that originate from this body as children
      for (const joint of chain.joints.filter((j) => j.parent_body === body.id)) {
        const childBody = chain.bodies.find((b) => b.id === joint.child_body);
        const jel = document.createElement("div");
        jel.className = "tree-item tree-joint";
        jel.dataset["type"] = "joint";
        jel.dataset["id"] = String(joint.id);
        jel.textContent = `  ↳ ${jointTypeName(joint.joint_type)} → ${childBody?.name ?? "?"}`;
        if (this.selection?.type === "joint" && this.selection.id === joint.id) {
          jel.classList.add("selected");
        }
        jel.addEventListener("click", (e) => {
          e.stopPropagation();
          this.select({ type: "joint", id: joint.id });
        });
        this.container.appendChild(jel);
      }
    }
  }

  private select(sel: Selection): void {
    this.selection = sel;
    this.onSelectCb?.(sel);
  }

  clearSelection(): void {
    this.selection = null;
    this.onSelectCb?.(null);
  }
}
