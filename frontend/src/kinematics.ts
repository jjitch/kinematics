export interface Pose {
  translation: [number, number, number];
  rotation: [number, number, number, number]; // [x, y, z, w]
  scale: [number, number, number];
}

export interface Body {
  id: number;
  name: string;
  local_transform: Pose;
  mesh_id: string | null;
}

export type JointType =
  | { Revolute: { axis: [number, number, number] } }
  | { Prismatic: { axis: [number, number, number] } }
  | "Fixed";

export interface Joint {
  id: number;
  parent_body: number;
  child_body: number;
  joint_type: JointType;
  rest_transform: Pose;
  min: number;
  max: number;
  value: number;
}

export interface Chain {
  bodies: Body[];
  joints: Joint[];
  next_body_id?: number;
  next_joint_id?: number;
}

export interface FkResult {
  transforms: Record<string, Pose>;
}

export interface ChainOpResult {
  ok: boolean;
  chain?: Chain;
  id?: number;
  error?: string;
}

export function identityPose(): Pose {
  return { translation: [0, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1] };
}

export function translationPose(x: number, y: number, z: number): Pose {
  return { translation: [x, y, z], rotation: [0, 0, 0, 1], scale: [1, 1, 1] };
}

export function jointTypeName(jt: JointType): string {
  if (jt === "Fixed") return "fixed";
  if ("Revolute" in jt) return "revolute";
  if ("Prismatic" in jt) return "prismatic";
  return "unknown";
}
