use serde::{Deserialize, Serialize};

use crate::mesh::Mesh;

/// Flat serialization-friendly representation of a mesh.
/// Positions and normals are stored as `[x, y, z, x, y, z, ...]`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
)]
pub struct MeshData {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub indices: Vec<u32>,
}

impl MeshData {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("MeshData serialization is infallible")
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        rkyv::to_bytes::<rkyv::rancor::Error>(self)
            .expect("MeshData rkyv serialization is infallible")
            .to_vec()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, rkyv::rancor::Error> {
        rkyv::from_bytes::<MeshData, rkyv::rancor::Error>(bytes)
    }
}

impl Mesh {
    /// Converts this mesh into a flat `MeshData` suitable for serialization / WASM transfer.
    pub fn to_mesh_data(&self) -> MeshData {
        MeshData {
            positions: self
                .positions
                .iter()
                .flat_map(|p| p.iter().copied())
                .collect(),
            normals: self
                .normals
                .iter()
                .flat_map(|n| n.iter().copied())
                .collect(),
            indices: self.indices.clone(),
        }
    }

    /// Reconstructs a `Mesh` from a flat `MeshData`.
    pub fn from_mesh_data(data: &MeshData) -> Self {
        let positions = data
            .positions
            .chunks(3)
            .map(|c| [c[0], c[1], c[2]])
            .collect();
        let normals = data.normals.chunks(3).map(|c| [c[0], c[1], c[2]]).collect();
        Mesh {
            positions,
            normals,
            indices: data.indices.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh_gen;

    fn reference_mesh() -> Mesh {
        mesh_gen::box_(2.0, 3.0, 4.0)
    }

    // --- to/from mesh_data ---
    #[test]
    fn mesh_data_round_trip_preserves_positions() {
        let mesh = reference_mesh();
        let restored = Mesh::from_mesh_data(&mesh.to_mesh_data());
        assert_eq!(mesh.positions, restored.positions);
    }

    #[test]
    fn mesh_data_round_trip_preserves_normals() {
        let mesh = reference_mesh();
        let restored = Mesh::from_mesh_data(&mesh.to_mesh_data());
        assert_eq!(mesh.normals, restored.normals);
    }

    #[test]
    fn mesh_data_round_trip_preserves_indices() {
        let mesh = reference_mesh();
        let restored = Mesh::from_mesh_data(&mesh.to_mesh_data());
        assert_eq!(mesh.indices, restored.indices);
    }

    #[test]
    fn positions_are_flattened_correctly() {
        let mesh = reference_mesh();
        let data = mesh.to_mesh_data();
        assert_eq!(data.positions.len(), mesh.positions.len() * 3);
        assert_eq!(data.normals.len(), mesh.normals.len() * 3);
        assert_eq!(data.indices.len(), mesh.indices.len());
    }

    // --- JSON ---
    #[test]
    fn json_round_trip() {
        let data = reference_mesh().to_mesh_data();
        let restored = MeshData::from_json(&data.to_json()).unwrap();
        assert_eq!(data, restored);
    }

    #[test]
    fn json_is_valid_json_string() {
        let json = reference_mesh().to_mesh_data().to_json();
        assert!(json.starts_with('{'));
        assert!(json.ends_with('}'));
    }

    // --- rkyv ---
    #[test]
    fn rkyv_round_trip() {
        let data = reference_mesh().to_mesh_data();
        let restored = MeshData::from_bytes(&data.to_bytes()).unwrap();
        assert_eq!(data, restored);
    }

    #[test]
    fn rkyv_is_smaller_than_json() {
        let data = reference_mesh().to_mesh_data();
        let json_size = data.to_json().len();
        let bin_size = data.to_bytes().len();
        assert!(
            bin_size < json_size,
            "binary ({bin_size}) should be smaller than JSON ({json_size})"
        );
    }
}
