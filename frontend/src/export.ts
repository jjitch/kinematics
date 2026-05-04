import type { Chain, FkResult } from "./kinematics.js";

export function exportPoseJson(chain: Chain, fk: FkResult): string {
  return JSON.stringify({ chain, transforms: fk.transforms }, null, 2);
}

export function exportTrajectoryCSV(
  keyframes: Array<{ time: number; jointValues: Record<number, number> }>,
  jointIds: number[],
): string {
  const header = ["time", ...jointIds.map((id) => `j${id}`)].join(",");
  const rows = keyframes.map((kf) => {
    const vals = jointIds.map((id) => (kf.jointValues[id] ?? 0).toFixed(6));
    return [kf.time.toFixed(3), ...vals].join(",");
  });
  return [header, ...rows].join("\n");
}

export function downloadText(
  filename: string,
  content: string,
  mimeType = "text/plain",
): void {
  const blob = new Blob([content], { type: mimeType });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  a.click();
  URL.revokeObjectURL(url);
}
