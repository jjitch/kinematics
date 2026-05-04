use std::collections::{HashMap, HashSet, VecDeque};

use serde::{Deserialize, Serialize};

use crate::math::{Quat, Vec3};
use crate::transform::Transform;

pub type BodyId = u32;
pub type JointId = u32;

#[derive(Debug, Clone, PartialEq)]
pub enum ChainError {
    BodyNotFound(BodyId),
    OrphanJoint(JointId),
    SelfLoop(BodyId),
    DuplicateChild(BodyId),
    Cycle,
}

/// Serialisable TRS pose that avoids a nalgebra-serde dependency.
/// `rotation` is stored as `[x, y, z, w]` (quaternion, imaginary first).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pose {
    pub translation: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

impl Pose {
    pub fn identity() -> Self {
        Self {
            translation: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
        }
    }

    pub fn from_translation(x: f32, y: f32, z: f32) -> Self {
        Self {
            translation: [x, y, z],
            ..Self::identity()
        }
    }

    pub fn to_transform(&self) -> Transform {
        let [rx, ry, rz, rw] = self.rotation;
        Transform {
            translation: Vec3::new(
                self.translation[0],
                self.translation[1],
                self.translation[2],
            ),
            rotation: nalgebra::Unit::new_normalize(nalgebra::Quaternion::new(rw, rx, ry, rz)),
            scale: Vec3::new(self.scale[0], self.scale[1], self.scale[2]),
        }
    }
}

impl From<Transform> for Pose {
    fn from(t: Transform) -> Self {
        let c = t.rotation.quaternion().coords; // Vector4 [i, j, k, w]
        Pose {
            translation: t.translation.into(),
            rotation: [c.x, c.y, c.z, c.w],
            scale: t.scale.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JointType {
    Revolute { axis: [f32; 3] },
    Prismatic { axis: [f32; 3] },
    Fixed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body {
    pub id: BodyId,
    pub name: String,
    pub local_transform: Pose,
    pub mesh_id: Option<String>,
}

impl Body {
    pub fn new(name: impl Into<String>) -> Self {
        Body {
            id: 0,
            name: name.into(),
            local_transform: Pose::identity(),
            mesh_id: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Joint {
    pub id: JointId,
    pub parent_body: BodyId,
    pub child_body: BodyId,
    pub joint_type: JointType,
    pub rest_transform: Pose,
    pub min: f32,
    pub max: f32,
    pub value: f32,
}

impl Joint {
    pub fn revolute(parent: BodyId, child: BodyId, axis: [f32; 3]) -> Self {
        Joint {
            id: 0,
            parent_body: parent,
            child_body: child,
            joint_type: JointType::Revolute { axis },
            rest_transform: Pose::identity(),
            min: -std::f32::consts::PI,
            max: std::f32::consts::PI,
            value: 0.0,
        }
    }

    pub fn prismatic(parent: BodyId, child: BodyId, axis: [f32; 3]) -> Self {
        Joint {
            id: 0,
            parent_body: parent,
            child_body: child,
            joint_type: JointType::Prismatic { axis },
            rest_transform: Pose::identity(),
            min: -10.0,
            max: 10.0,
            value: 0.0,
        }
    }

    fn active_transform(&self) -> Transform {
        match &self.joint_type {
            JointType::Revolute { axis } => {
                let unit = nalgebra::Unit::new_normalize(Vec3::new(axis[0], axis[1], axis[2]));
                Transform::new(
                    Vec3::zeros(),
                    Quat::from_axis_angle(&unit, self.value),
                    Vec3::new(1.0, 1.0, 1.0),
                )
            }
            JointType::Prismatic { axis } => {
                let dir = Vec3::new(axis[0], axis[1], axis[2]);
                Transform::new(dir * self.value, Quat::identity(), Vec3::new(1.0, 1.0, 1.0))
            }
            JointType::Fixed => Transform::identity(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Chain {
    pub bodies: Vec<Body>,
    pub joints: Vec<Joint>,
    #[serde(default)]
    next_body_id: u32,
    #[serde(default)]
    next_joint_id: u32,
}

impl Chain {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_body(&mut self, mut body: Body) -> BodyId {
        let id = self.next_body_id;
        self.next_body_id += 1;
        body.id = id;
        self.bodies.push(body);
        id
    }

    pub fn add_joint(&mut self, mut joint: Joint) -> Result<JointId, ChainError> {
        if !self.has_body(joint.parent_body) {
            return Err(ChainError::BodyNotFound(joint.parent_body));
        }
        if !self.has_body(joint.child_body) {
            return Err(ChainError::BodyNotFound(joint.child_body));
        }
        if joint.parent_body == joint.child_body {
            return Err(ChainError::SelfLoop(joint.parent_body));
        }
        if self.joints.iter().any(|j| j.child_body == joint.child_body) {
            return Err(ChainError::DuplicateChild(joint.child_body));
        }
        if self.is_ancestor(joint.child_body, joint.parent_body) {
            return Err(ChainError::Cycle);
        }
        let id = self.next_joint_id;
        self.next_joint_id += 1;
        joint.id = id;
        self.joints.push(joint);
        Ok(id)
    }

    pub fn validate(&self) -> Result<(), ChainError> {
        for joint in &self.joints {
            if !self.has_body(joint.parent_body) {
                return Err(ChainError::BodyNotFound(joint.parent_body));
            }
            if !self.has_body(joint.child_body) {
                return Err(ChainError::BodyNotFound(joint.child_body));
            }
        }
        if self.has_cycle() {
            return Err(ChainError::Cycle);
        }
        Ok(())
    }

    pub fn set_joint_value(&mut self, joint_id: JointId, value: f32) -> bool {
        if let Some(joint) = self.joints.iter_mut().find(|j| j.id == joint_id) {
            joint.value = value.clamp(joint.min, joint.max);
            true
        } else {
            false
        }
    }

    pub fn joint_values(&self) -> Vec<(JointId, f32)> {
        self.joints.iter().map(|j| (j.id, j.value)).collect()
    }

    pub fn compute_transforms(&self) -> HashMap<BodyId, Pose> {
        let children: HashSet<BodyId> = self.joints.iter().map(|j| j.child_body).collect();
        let mut result = HashMap::new();
        let mut queue: VecDeque<(BodyId, crate::transform::Transform)> = self
            .bodies
            .iter()
            .filter(|b| !children.contains(&b.id))
            .map(|b| (b.id, b.local_transform.to_transform()))
            .collect();

        while let Some((body_id, world)) = queue.pop_front() {
            result.insert(body_id, Pose::from(world));
            for joint in &self.joints {
                if joint.parent_body == body_id {
                    let rest = joint.rest_transform.to_transform();
                    let active = joint.active_transform();
                    let child = self
                        .bodies
                        .iter()
                        .find(|b| b.id == joint.child_body)
                        .unwrap();
                    let child_local = child.local_transform.to_transform();
                    let child_world = world.compose(&rest).compose(&active).compose(&child_local);
                    queue.push_back((joint.child_body, child_world));
                }
            }
        }
        result
    }

    pub fn body(&self, id: BodyId) -> Option<&Body> {
        self.bodies.iter().find(|b| b.id == id)
    }

    pub fn joint(&self, id: JointId) -> Option<&Joint> {
        self.joints.iter().find(|j| j.id == id)
    }

    fn has_body(&self, id: BodyId) -> bool {
        self.bodies.iter().any(|b| b.id == id)
    }

    fn is_ancestor(&self, ancestor: BodyId, of: BodyId) -> bool {
        let mut current = of;
        loop {
            match self.joints.iter().find(|j| j.child_body == current) {
                Some(j) if j.parent_body == ancestor => return true,
                Some(j) => current = j.parent_body,
                None => return false,
            }
        }
    }

    fn has_cycle(&self) -> bool {
        let mut vis = HashSet::new();
        let mut stack = HashSet::new();
        for body in &self.bodies {
            if !vis.contains(&body.id) && self.dfs_has_cycle(body.id, &mut vis, &mut stack) {
                return true;
            }
        }
        false
    }

    fn dfs_has_cycle(
        &self,
        id: BodyId,
        vis: &mut HashSet<BodyId>,
        stack: &mut HashSet<BodyId>,
    ) -> bool {
        vis.insert(id);
        stack.insert(id);
        for joint in &self.joints {
            if joint.parent_body == id {
                let child = joint.child_body;
                if stack.contains(&child) {
                    return true;
                }
                if !vis.contains(&child) && self.dfs_has_cycle(child, vis, stack) {
                    return true;
                }
            }
        }
        stack.remove(&id);
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    fn two_body_chain() -> (Chain, BodyId, BodyId) {
        let mut chain = Chain::new();
        let a = chain.add_body(Body::new("root"));
        let b = chain.add_body(Body::new("link1"));
        (chain, a, b)
    }

    // --- add_joint validation ---
    #[test]
    fn add_joint_errors_on_missing_parent() {
        let mut chain = Chain::new();
        let b = chain.add_body(Body::new("b"));
        assert!(chain
            .add_joint(Joint::revolute(99, b, [0.0, 1.0, 0.0]))
            .is_err());
    }

    #[test]
    fn add_joint_errors_on_missing_child() {
        let mut chain = Chain::new();
        let a = chain.add_body(Body::new("a"));
        assert!(chain
            .add_joint(Joint::revolute(a, 99, [0.0, 1.0, 0.0]))
            .is_err());
    }

    #[test]
    fn add_joint_errors_on_self_loop() {
        let mut chain = Chain::new();
        let a = chain.add_body(Body::new("a"));
        let err = chain
            .add_joint(Joint::revolute(a, a, [0.0, 1.0, 0.0]))
            .unwrap_err();
        assert_eq!(err, ChainError::SelfLoop(a));
    }

    #[test]
    fn add_joint_errors_on_duplicate_child() {
        let (mut chain, a, b) = two_body_chain();
        let c = chain.add_body(Body::new("c"));
        chain
            .add_joint(Joint::revolute(a, b, [0.0, 1.0, 0.0]))
            .unwrap();
        let err = chain
            .add_joint(Joint::revolute(c, b, [0.0, 1.0, 0.0]))
            .unwrap_err();
        assert_eq!(err, ChainError::DuplicateChild(b));
    }

    #[test]
    fn add_joint_errors_on_cycle() {
        let (mut chain, a, b) = two_body_chain();
        chain
            .add_joint(Joint::revolute(a, b, [0.0, 1.0, 0.0]))
            .unwrap();
        let err = chain
            .add_joint(Joint::revolute(b, a, [0.0, 1.0, 0.0]))
            .unwrap_err();
        assert_eq!(err, ChainError::Cycle);
    }

    // --- validate ---
    #[test]
    fn validate_empty_chain_is_ok() {
        assert!(Chain::new().validate().is_ok());
    }

    #[test]
    fn validate_linear_chain_is_ok() {
        let (mut chain, a, b) = two_body_chain();
        chain
            .add_joint(Joint::revolute(a, b, [0.0, 1.0, 0.0]))
            .unwrap();
        assert!(chain.validate().is_ok());
    }

    // --- set_joint_value ---
    #[test]
    fn set_joint_value_clamps_to_limits() {
        let (mut chain, a, b) = two_body_chain();
        let jid = chain
            .add_joint(Joint {
                min: -1.0,
                max: 1.0,
                ..Joint::revolute(a, b, [0.0, 1.0, 0.0])
            })
            .unwrap();
        chain.set_joint_value(jid, 999.0);
        assert_abs_diff_eq!(chain.joint(jid).unwrap().value, 1.0);
        chain.set_joint_value(jid, -999.0);
        assert_abs_diff_eq!(chain.joint(jid).unwrap().value, -1.0);
    }

    #[test]
    fn set_joint_value_returns_false_for_unknown_id() {
        let mut chain = Chain::new();
        assert!(!chain.set_joint_value(99, 0.0));
    }

    #[test]
    fn joint_values_returns_all_values() {
        let (mut chain, a, b) = two_body_chain();
        let jid = chain
            .add_joint(Joint::revolute(a, b, [0.0, 1.0, 0.0]))
            .unwrap();
        chain.set_joint_value(jid, 0.5);
        let vals = chain.joint_values();
        assert_eq!(vals.len(), 1);
        assert_abs_diff_eq!(vals[0].1, 0.5);
    }

    // --- FK ---
    #[test]
    fn fk_root_only_is_at_local_transform() {
        let mut chain = Chain::new();
        let a = chain.add_body(Body {
            local_transform: Pose::from_translation(1.0, 2.0, 3.0),
            ..Body::new("root")
        });
        let poses = chain.compute_transforms();
        assert_abs_diff_eq!(poses[&a].translation[0], 1.0, epsilon = 1e-5);
        assert_abs_diff_eq!(poses[&a].translation[1], 2.0, epsilon = 1e-5);
        assert_abs_diff_eq!(poses[&a].translation[2], 3.0, epsilon = 1e-5);
    }

    #[test]
    fn fk_single_revolute_90deg_rotates_child() {
        let (mut chain, a, b) = two_body_chain();
        let jid = chain
            .add_joint(Joint::revolute(a, b, [0.0, 1.0, 0.0]))
            .unwrap();
        chain.set_joint_value(jid, std::f32::consts::FRAC_PI_2);

        let poses = chain.compute_transforms();
        let b_world = poses[&b].to_transform();
        // Revolute Y 90° maps [1,0,0] in child frame to [0,0,-1] in world
        let world_pt = b_world.apply(Vec3::new(1.0, 0.0, 0.0));
        assert_abs_diff_eq!(world_pt.x, 0.0, epsilon = 1e-5);
        assert_abs_diff_eq!(world_pt.y, 0.0, epsilon = 1e-5);
        assert_abs_diff_eq!(world_pt.z, -1.0, epsilon = 1e-5);
    }

    #[test]
    fn fk_three_body_chain_accumulates_translations() {
        let mut chain = Chain::new();
        let a = chain.add_body(Body::new("A"));
        let b = chain.add_body(Body::new("B"));
        // C has a local offset so we can verify child_local is composed correctly
        let c = chain.add_body(Body {
            local_transform: Pose::from_translation(0.0, 1.0, 0.0),
            ..Body::new("C")
        });

        let j1 = chain
            .add_joint(Joint::prismatic(a, b, [1.0, 0.0, 0.0]))
            .unwrap();
        let j2 = chain
            .add_joint(Joint::prismatic(b, c, [1.0, 0.0, 0.0]))
            .unwrap();
        chain.set_joint_value(j1, 2.0);
        chain.set_joint_value(j2, 3.0);

        let poses = chain.compute_transforms();
        // B world = translate(2,0,0)
        assert_abs_diff_eq!(poses[&b].translation[0], 2.0, epsilon = 1e-5);
        assert_abs_diff_eq!(poses[&b].translation[1], 0.0, epsilon = 1e-5);
        // C world = translate(2+3, 0+1, 0) = [5, 1, 0]
        assert_abs_diff_eq!(poses[&c].translation[0], 5.0, epsilon = 1e-5);
        assert_abs_diff_eq!(poses[&c].translation[1], 1.0, epsilon = 1e-5);
    }

    // --- JSON round-trip ---
    #[test]
    fn chain_json_round_trip_preserves_structure() {
        let (mut chain, a, b) = two_body_chain();
        chain
            .add_joint(Joint::revolute(a, b, [0.0, 1.0, 0.0]))
            .unwrap();
        let json = serde_json::to_string(&chain).unwrap();
        let restored: Chain = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.bodies.len(), chain.bodies.len());
        assert_eq!(restored.joints.len(), chain.joints.len());
    }
}
