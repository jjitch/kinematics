import type { Chain } from "./kinematics.js";
import { jointTypeName } from "./kinematics.js";
import type { Selection } from "./scene-tree.js";

export type PropChangeCallback = (chainJson: string) => void;

export class PropertiesPanel {
  private container: HTMLElement;
  private onChangeCb: PropChangeCallback | null = null;

  constructor(container: HTMLElement) {
    this.container = container;
  }

  onChange(cb: PropChangeCallback): void {
    this.onChangeCb = cb;
  }

  show(chain: Chain, selection: Selection | null): void {
    this.container.innerHTML = "";

    if (!selection) {
      this.container.innerHTML = '<p class="props-empty">Nothing selected</p>';
      return;
    }

    if (selection.type === "body") {
      const body = chain.bodies.find((b) => b.id === selection.id);
      if (!body) return;
      this.renderBodyProps(chain, body.id);
    } else {
      const joint = chain.joints.find((j) => j.id === selection.id);
      if (!joint) return;
      this.renderJointProps(chain, joint.id);
    }
  }

  private renderBodyProps(chain: Chain, bodyId: number): void {
    const body = chain.bodies.find((b) => b.id === bodyId)!;
    const t = body.local_transform;

    this.container.appendChild(this.heading(`Body: ${body.name}`));

    this.container.appendChild(
      this.vec3Row("Position", t.translation, (i, v) => {
        const updated = JSON.parse(JSON.stringify(chain)) as Chain;
        const b = updated.bodies.find((x) => x.id === bodyId)!;
        b.local_transform.translation[i] = v;
        this.onChangeCb?.(JSON.stringify(updated));
      }),
    );
  }

  private renderJointProps(chain: Chain, jointId: number): void {
    const joint = chain.joints.find((j) => j.id === jointId)!;
    const parentName = chain.bodies.find((b) => b.id === joint.parent_body)?.name ?? "?";
    const childName = chain.bodies.find((b) => b.id === joint.child_body)?.name ?? "?";

    this.container.appendChild(
      this.heading(`Joint: ${parentName} → ${childName}`),
    );

    const typeRow = document.createElement("div");
    typeRow.className = "prop-row";
    typeRow.innerHTML = `<span class="prop-label">Type</span><span class="prop-val">${jointTypeName(joint.joint_type)}</span>`;
    this.container.appendChild(typeRow);

    this.container.appendChild(
      this.numRow("Value", joint.value, joint.min, joint.max, (v) => {
        const updated = JSON.parse(JSON.stringify(chain)) as Chain;
        const j = updated.joints.find((x) => x.id === jointId)!;
        j.value = Math.max(j.min, Math.min(j.max, v));
        this.onChangeCb?.(JSON.stringify(updated));
      }),
    );

    this.container.appendChild(
      this.numRow("Min", joint.min, -2 * Math.PI, 0, (v) => {
        const updated = JSON.parse(JSON.stringify(chain)) as Chain;
        updated.joints.find((x) => x.id === jointId)!.min = v;
        this.onChangeCb?.(JSON.stringify(updated));
      }),
    );

    this.container.appendChild(
      this.numRow("Max", joint.max, 0, 2 * Math.PI, (v) => {
        const updated = JSON.parse(JSON.stringify(chain)) as Chain;
        updated.joints.find((x) => x.id === jointId)!.max = v;
        this.onChangeCb?.(JSON.stringify(updated));
      }),
    );
  }

  private heading(text: string): HTMLElement {
    const h = document.createElement("div");
    h.className = "prop-heading";
    h.textContent = text;
    return h;
  }

  private vec3Row(
    label: string,
    vals: [number, number, number],
    onChange: (index: number, value: number) => void,
  ): HTMLElement {
    const row = document.createElement("div");
    row.className = "prop-vec3";
    const lbl = document.createElement("span");
    lbl.className = "prop-label";
    lbl.textContent = label;
    row.appendChild(lbl);
    const coords = ["X", "Y", "Z"] as const;
    coords.forEach((axis, i) => {
      const inp = document.createElement("input");
      inp.type = "number";
      inp.step = "0.1";
      inp.value = vals[i].toFixed(3);
      inp.title = axis;
      inp.addEventListener("change", () => onChange(i, parseFloat(inp.value)));
      row.appendChild(inp);
    });
    return row;
  }

  private numRow(
    label: string,
    value: number,
    min: number,
    max: number,
    onChange: (v: number) => void,
  ): HTMLElement {
    const row = document.createElement("div");
    row.className = "prop-row";
    const lbl = document.createElement("span");
    lbl.className = "prop-label";
    lbl.textContent = label;
    const inp = document.createElement("input");
    inp.type = "number";
    inp.step = "0.01";
    inp.min = String(min);
    inp.max = String(max);
    inp.value = value.toFixed(4);
    inp.addEventListener("change", () => onChange(parseFloat(inp.value)));
    row.appendChild(lbl);
    row.appendChild(inp);
    return row;
  }
}
