use kinematics_core::chain::{Body, Chain, Joint, JointType};
use kinematics_core::hello_message;
use kinematics_core::ik::{solve_ik, solve_ik_step, PositionTarget, SolverConfig};
use kinematics_core::mesh_data::MeshData;
use kinematics_core::mesh_gen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello() -> String {
    hello_message()
}

#[wasm_bindgen]
pub fn generate_box(width: f32, height: f32, depth: f32) -> String {
    mesh_gen::box_(width, height, depth)
        .to_mesh_data()
        .to_json()
}

#[wasm_bindgen]
pub fn generate_sphere(radius: f32, segments: u32, rings: u32) -> String {
    mesh_gen::sphere(radius, segments, rings)
        .to_mesh_data()
        .to_json()
}

#[wasm_bindgen]
pub fn generate_cylinder(radius: f32, height: f32, segments: u32) -> String {
    mesh_gen::cylinder(radius, height, segments)
        .to_mesh_data()
        .to_json()
}

#[wasm_bindgen]
pub fn generate_cone(radius: f32, height: f32, segments: u32) -> String {
    mesh_gen::cone(radius, height, segments)
        .to_mesh_data()
        .to_json()
}

#[wasm_bindgen]
pub fn generate_plane(width: f32, depth: f32, subdivisions: u32) -> String {
    mesh_gen::plane(width, depth, subdivisions)
        .to_mesh_data()
        .to_json()
}

/// Parses a `MeshData` JSON string and returns a JSON object with the same
/// flat arrays, suitable for uploading to WebGL typed arrays.
/// Returns the same JSON on success, or an error JSON on parse failure.
#[wasm_bindgen]
pub fn mesh_to_typed_arrays(mesh_json: &str) -> String {
    match MeshData::from_json(mesh_json) {
        Ok(data) => data.to_json(),
        Err(e) => format!("{{\"error\":\"{}\"}}", e),
    }
}

/// Returns an empty `Chain` serialised as JSON.
#[wasm_bindgen]
pub fn chain_new() -> String {
    serde_json::to_string(&Chain::new()).unwrap()
}

/// Adds a body with the given name to the chain.
/// Returns `{"ok":true,"chain":<chain_json>,"id":<body_id>}`.
#[wasm_bindgen]
pub fn chain_add_body(chain_json: &str, name: &str) -> String {
    let mut chain: Chain = match serde_json::from_str(chain_json) {
        Ok(c) => c,
        Err(e) => return format!("{{\"ok\":false,\"error\":\"{}\"}}", e),
    };
    let id = chain.add_body(Body::new(name));
    let chain_str = serde_json::to_string(&chain).unwrap();
    format!("{{\"ok\":true,\"chain\":{},\"id\":{}}}", chain_str, id)
}

/// Adds a joint to the chain.
/// `kind` is `"revolute"`, `"prismatic"`, or `"fixed"`.
/// `axis_x/y/z` are ignored for `"fixed"`.
/// Returns `{"ok":true,"chain":<chain_json>,"id":<joint_id>}` or `{"ok":false,"error":"..."}`.
#[allow(clippy::too_many_arguments)]
#[wasm_bindgen]
pub fn chain_add_joint(
    chain_json: &str,
    parent_id: u32,
    child_id: u32,
    kind: &str,
    axis_x: f32,
    axis_y: f32,
    axis_z: f32,
    min: f32,
    max: f32,
) -> String {
    let mut chain: Chain = match serde_json::from_str(chain_json) {
        Ok(c) => c,
        Err(e) => return format!("{{\"ok\":false,\"error\":\"{}\"}}", e),
    };
    let axis = [axis_x, axis_y, axis_z];
    let mut joint = match kind {
        "revolute" => Joint::revolute(parent_id, child_id, axis),
        "prismatic" => Joint::prismatic(parent_id, child_id, axis),
        "fixed" => Joint {
            joint_type: JointType::Fixed,
            ..Joint::revolute(parent_id, child_id, axis)
        },
        other => {
            return format!(
                "{{\"ok\":false,\"error\":\"unknown joint kind: {}\"}}",
                other
            )
        }
    };
    joint.min = min;
    joint.max = max;
    match chain.add_joint(joint) {
        Ok(id) => {
            let chain_str = serde_json::to_string(&chain).unwrap();
            format!("{{\"ok\":true,\"chain\":{},\"id\":{}}}", chain_str, id)
        }
        Err(e) => format!("{{\"ok\":false,\"error\":\"{:?}\"}}", e),
    }
}

/// Sets a joint value (clamped to limits).
/// Returns updated chain JSON or `{"ok":false,"error":"..."}` if joint not found.
#[wasm_bindgen]
pub fn chain_set_joint_value(chain_json: &str, joint_id: u32, value: f32) -> String {
    let mut chain: Chain = match serde_json::from_str(chain_json) {
        Ok(c) => c,
        Err(e) => return format!("{{\"ok\":false,\"error\":\"{}\"}}", e),
    };
    if chain.set_joint_value(joint_id, value) {
        let chain_str = serde_json::to_string(&chain).unwrap();
        format!("{{\"ok\":true,\"chain\":{}}}", chain_str)
    } else {
        format!(
            "{{\"ok\":false,\"error\":\"joint {} not found\"}}",
            joint_id
        )
    }
}

/// Computes FK for the chain.
/// Returns `{"transforms":{"<body_id>":{"translation":[x,y,z],"rotation":[rx,ry,rz,rw],"scale":[sx,sy,sz]},...}}`.
#[wasm_bindgen]
pub fn chain_compute_fk(chain_json: &str) -> String {
    let chain: Chain = match serde_json::from_str(chain_json) {
        Ok(c) => c,
        Err(e) => return format!("{{\"error\":\"{}\"}}", e),
    };
    let transforms = chain.compute_transforms();
    let map: std::collections::HashMap<String, _> = transforms
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();
    format!(
        "{{\"transforms\":{}}}",
        serde_json::to_string(&map).unwrap()
    )
}

/// Run the full IK solve.
/// `target_json`: `{"body_id":N,"target":[x,y,z]}`
/// `config_json`: `{"max_iter":50,"tolerance":1e-4,"damping":0.01,"step_size":1.0}` (all optional)
/// Returns `{"ok":true,"chain":<chain>,"result":{"converged":bool,"iterations":N,"residual":f}}`
#[wasm_bindgen]
pub fn ik_solve(chain_json: &str, target_json: &str, config_json: &str) -> String {
    let mut chain: Chain = match serde_json::from_str(chain_json) {
        Ok(c) => c,
        Err(e) => return format!("{{\"ok\":false,\"error\":\"{}\"}}", e),
    };
    let target: PositionTarget = match serde_json::from_str(target_json) {
        Ok(t) => t,
        Err(e) => return format!("{{\"ok\":false,\"error\":\"{}\"}}", e),
    };
    let config: SolverConfig = if config_json.is_empty() || config_json == "{}" {
        SolverConfig::default()
    } else {
        match serde_json::from_str(config_json) {
            Ok(c) => c,
            Err(e) => return format!("{{\"ok\":false,\"error\":\"{}\"}}", e),
        }
    };
    let result = solve_ik(&mut chain, &target, &config);
    let chain_str = serde_json::to_string(&chain).unwrap();
    let result_str = serde_json::to_string(&result).unwrap();
    format!(
        "{{\"ok\":true,\"chain\":{},\"result\":{}}}",
        chain_str, result_str
    )
}

/// Run one IK iteration (for per-frame streaming).
/// Returns `{"ok":true,"chain":<chain>,"residual":f}`
#[wasm_bindgen]
pub fn ik_solve_step(chain_json: &str, target_json: &str, config_json: &str) -> String {
    let mut chain: Chain = match serde_json::from_str(chain_json) {
        Ok(c) => c,
        Err(e) => return format!("{{\"ok\":false,\"error\":\"{}\"}}", e),
    };
    let target: PositionTarget = match serde_json::from_str(target_json) {
        Ok(t) => t,
        Err(e) => return format!("{{\"ok\":false,\"error\":\"{}\"}}", e),
    };
    let config: SolverConfig = if config_json.is_empty() || config_json == "{}" {
        SolverConfig::default()
    } else {
        match serde_json::from_str(config_json) {
            Ok(c) => c,
            Err(e) => return format!("{{\"ok\":false,\"error\":\"{}\"}}", e),
        }
    };
    let residual = solve_ik_step(&mut chain, &target, &config);
    let chain_str = serde_json::to_string(&chain).unwrap();
    format!(
        "{{\"ok\":true,\"chain\":{},\"residual\":{}}}",
        chain_str, residual
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use kinematics_core::mesh_data::MeshData;

    #[test]
    fn generate_box_returns_valid_json() {
        let json = generate_box(1.0, 2.0, 3.0);
        let data = MeshData::from_json(&json).expect("generate_box produced invalid JSON");
        assert_eq!(data.positions.len(), 24 * 3);
        assert_eq!(data.indices.len(), 36);
    }

    #[test]
    fn generate_sphere_returns_valid_json() {
        let json = generate_sphere(1.0, 8, 4);
        let data = MeshData::from_json(&json).expect("generate_sphere produced invalid JSON");
        assert_eq!(data.positions.len(), 5 * 9 * 3);
    }

    #[test]
    fn generate_cylinder_returns_valid_json() {
        let json = generate_cylinder(1.0, 2.0, 8);
        let data = MeshData::from_json(&json).expect("generate_cylinder produced invalid JSON");
        assert_eq!(data.indices.len() % 3, 0);
    }

    #[test]
    fn generate_cone_returns_valid_json() {
        let json = generate_cone(1.0, 2.0, 8);
        let data = MeshData::from_json(&json).expect("generate_cone produced invalid JSON");
        assert_eq!(data.indices.len() % 3, 0);
    }

    #[test]
    fn generate_plane_returns_valid_json() {
        let json = generate_plane(1.0, 1.0, 1);
        let data = MeshData::from_json(&json).expect("generate_plane produced invalid JSON");
        assert_eq!(data.positions.len(), 4 * 3);
        assert_eq!(data.indices.len(), 6);
    }

    #[test]
    fn mesh_to_typed_arrays_round_trips_json() {
        let original = generate_box(1.0, 1.0, 1.0);
        let result = mesh_to_typed_arrays(&original);
        let data =
            MeshData::from_json(&result).expect("mesh_to_typed_arrays returned invalid JSON");
        assert_eq!(data.positions.len(), 24 * 3);
    }

    #[test]
    fn mesh_to_typed_arrays_returns_error_json_on_bad_input() {
        let result = mesh_to_typed_arrays("not json");
        assert!(result.contains("error"), "expected error key in: {result}");
    }

    #[test]
    fn chain_new_returns_valid_json() {
        let json = chain_new();
        let chain: Chain = serde_json::from_str(&json).expect("chain_new produced invalid JSON");
        assert_eq!(chain.bodies.len(), 0);
        assert_eq!(chain.joints.len(), 0);
    }

    #[test]
    fn chain_add_body_returns_ok_with_id() {
        let chain_json = chain_new();
        let result = chain_add_body(&chain_json, "root");
        let v: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(v["ok"], true);
        assert_eq!(v["id"], 0);
        let chain: Chain = serde_json::from_str(v["chain"].to_string().as_str()).unwrap();
        assert_eq!(chain.bodies.len(), 1);
        assert_eq!(chain.bodies[0].name, "root");
    }

    fn chain_str(v: &serde_json::Value) -> String {
        v["chain"].to_string()
    }

    #[test]
    fn chain_add_joint_revolute_returns_ok() {
        let cj = chain_new();
        let r1 = chain_add_body(&cj, "A");
        let v1: serde_json::Value = serde_json::from_str(&r1).unwrap();
        let r2 = chain_add_body(&chain_str(&v1), "B");
        let v2: serde_json::Value = serde_json::from_str(&r2).unwrap();
        let parent_id = v1["id"].as_u64().unwrap() as u32;
        let child_id = v2["id"].as_u64().unwrap() as u32;
        let r3 = chain_add_joint(
            &chain_str(&v2),
            parent_id,
            child_id,
            "revolute",
            0.0,
            1.0,
            0.0,
            -std::f32::consts::PI,
            std::f32::consts::PI,
        );
        let v3: serde_json::Value = serde_json::from_str(&r3).unwrap();
        assert_eq!(v3["ok"], true);
        assert_eq!(v3["id"], 0);
    }

    #[test]
    fn chain_add_joint_errors_on_self_loop() {
        let cj = chain_new();
        let r = chain_add_body(&cj, "A");
        let v: serde_json::Value = serde_json::from_str(&r).unwrap();
        let id = v["id"].as_u64().unwrap() as u32;
        let r2 = chain_add_joint(
            &chain_str(&v),
            id,
            id,
            "revolute",
            0.0,
            1.0,
            0.0,
            -std::f32::consts::PI,
            std::f32::consts::PI,
        );
        let v2: serde_json::Value = serde_json::from_str(&r2).unwrap();
        assert_eq!(v2["ok"], false);
    }

    #[test]
    fn chain_set_joint_value_updates_chain() {
        let cj = chain_new();
        let r1 = chain_add_body(&cj, "A");
        let v1: serde_json::Value = serde_json::from_str(&r1).unwrap();
        let r2 = chain_add_body(&chain_str(&v1), "B");
        let v2: serde_json::Value = serde_json::from_str(&r2).unwrap();
        let parent_id = v1["id"].as_u64().unwrap() as u32;
        let child_id = v2["id"].as_u64().unwrap() as u32;
        let r3 = chain_add_joint(
            &chain_str(&v2),
            parent_id,
            child_id,
            "revolute",
            0.0,
            1.0,
            0.0,
            -std::f32::consts::PI,
            std::f32::consts::PI,
        );
        let v3: serde_json::Value = serde_json::from_str(&r3).unwrap();
        let joint_id = v3["id"].as_u64().unwrap() as u32;
        let r4 = chain_set_joint_value(&chain_str(&v3), joint_id, 1.0);
        let v4: serde_json::Value = serde_json::from_str(&r4).unwrap();
        assert_eq!(v4["ok"], true);
        let chain: Chain = serde_json::from_str(&chain_str(&v4)).unwrap();
        assert_eq!(chain.joints[0].value, 1.0);
    }

    #[test]
    fn chain_compute_fk_returns_transforms() {
        let cj = chain_new();
        let r1 = chain_add_body(&cj, "root");
        let v1: serde_json::Value = serde_json::from_str(&r1).unwrap();
        let fk = chain_compute_fk(&chain_str(&v1));
        let v: serde_json::Value = serde_json::from_str(&fk).unwrap();
        assert!(v["transforms"].is_object());
        assert!(v["transforms"]["0"].is_object());
    }

    /// Build a one-joint arm (revolute Y, link length 1 in child frame).
    fn build_one_joint_arm() -> String {
        let cj = chain_new();
        let ra = chain_add_body(&cj, "A");
        let va: serde_json::Value = serde_json::from_str(&ra).unwrap();
        // Body B with local_transform offset: needs full body JSON
        // Use chain_add_body then edit local_transform via serde
        let rb = chain_add_body(&chain_str(&va), "B");
        let vb: serde_json::Value = serde_json::from_str(&rb).unwrap();
        let parent_id = va["id"].as_u64().unwrap() as u32;
        let child_id = vb["id"].as_u64().unwrap() as u32;
        // Set child local_transform to translate(1,0,0) by deserializing and mutating chain
        let mut chain: Chain = serde_json::from_str(chain_str(&vb).as_str()).unwrap();
        chain.bodies[1].local_transform =
            kinematics_core::chain::Pose::from_translation(1.0, 0.0, 0.0);
        let chain_json = serde_json::to_string(&chain).unwrap();
        let rj = chain_add_joint(
            &chain_json,
            parent_id,
            child_id,
            "revolute",
            0.0,
            1.0,
            0.0,
            -std::f32::consts::PI,
            std::f32::consts::PI,
        );
        let vj: serde_json::Value = serde_json::from_str(&rj).unwrap();
        chain_str(&vj)
    }

    #[test]
    fn ik_solve_converges_on_reachable_target() {
        let chain_json = build_one_joint_arm();
        let target = r#"{"body_id":1,"target":[0.0,0.0,-1.0]}"#;
        let result = ik_solve(&chain_json, target, "{}");
        let v: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(v["ok"], true);
        assert_eq!(v["result"]["converged"], true);
        let residual = v["result"]["residual"].as_f64().unwrap();
        assert!(residual < 1e-3, "residual too large: {residual}");
    }

    #[test]
    fn ik_solve_step_returns_updated_chain() {
        let chain_json = build_one_joint_arm();
        let target = r#"{"body_id":1,"target":[0.0,0.0,-1.0]}"#;
        let result = ik_solve_step(&chain_json, target, "{}");
        let v: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(v["ok"], true);
        assert!(v["residual"].as_f64().unwrap() < 2.0);
        assert!(v["chain"].is_object());
    }
}
