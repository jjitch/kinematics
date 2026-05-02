use kinematics_core::hello_message;
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
}
