use nalgebra::{DMatrix, DVector};
use serde::{Deserialize, Serialize};

use crate::chain::{BodyId, Chain};
use crate::math::Vec3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionTarget {
    pub body_id: BodyId,
    pub target: [f32; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolverConfig {
    pub max_iter: u32,
    pub tolerance: f32,
    pub damping: f32,
    pub step_size: f32,
}

impl Default for SolverConfig {
    fn default() -> Self {
        SolverConfig {
            max_iter: 50,
            tolerance: 1e-4,
            damping: 0.01,
            step_size: 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolveResult {
    pub converged: bool,
    pub iterations: u32,
    pub residual: f32,
}

/// One Jacobian pseudo-inverse (DLS) iteration.
/// Returns the new residual after applying Δq.
pub fn solve_ik_step(chain: &mut Chain, target: &PositionTarget, config: &SolverConfig) -> f32 {
    let transforms = chain.compute_transforms();
    let Some(ee_pose) = transforms.get(&target.body_id) else {
        return f32::INFINITY;
    };
    let ee_pos = Vec3::new(
        ee_pose.translation[0],
        ee_pose.translation[1],
        ee_pose.translation[2],
    );
    let target_pos = Vec3::new(target.target[0], target.target[1], target.target[2]);
    let error = target_pos - ee_pos;

    let n = chain.joints.len();
    if n == 0 {
        return error.norm();
    }

    // Snapshot joint ids and values before perturbation
    let joint_ids: Vec<_> = chain.joints.iter().map(|j| j.id).collect();
    let joint_vals: Vec<f32> = chain.joints.iter().map(|j| j.value).collect();

    // Numerical Jacobian: 3 × n
    let eps = 1e-4_f32;
    let mut jac = DMatrix::<f32>::zeros(3, n);
    for (col, (&jid, &val)) in joint_ids.iter().zip(joint_vals.iter()).enumerate() {
        chain.set_joint_value(jid, val + eps);
        let perturbed = chain.compute_transforms();
        chain.set_joint_value(jid, val);

        if let Some(pp) = perturbed.get(&target.body_id) {
            let p = Vec3::new(pp.translation[0], pp.translation[1], pp.translation[2]);
            let dp = (p - ee_pos) / eps;
            jac[(0, col)] = dp.x;
            jac[(1, col)] = dp.y;
            jac[(2, col)] = dp.z;
        }
    }

    // Truncated SVD pseudo-inverse: Δq = J⁺ e
    // Singular values below `damping` are treated as zero, avoiding DLS bias
    // that prevents convergence near the solution.
    let e_vec = DVector::<f32>::from_column_slice(error.as_slice());
    let svd = jac.clone().svd(true, true);
    let Ok(j_pinv) = svd.pseudo_inverse(config.damping) else {
        return error.norm();
    };
    let dq = j_pinv * e_vec;

    for (col, &jid) in joint_ids.iter().enumerate() {
        let new_val = joint_vals[col] + dq[col] * config.step_size;
        chain.set_joint_value(jid, new_val);
    }

    // Return residual after update
    let updated = chain.compute_transforms();
    updated
        .get(&target.body_id)
        .map(|p| {
            let pos = Vec3::new(p.translation[0], p.translation[1], p.translation[2]);
            (target_pos - pos).norm()
        })
        .unwrap_or(f32::INFINITY)
}

/// Run the full IK solve loop (up to `config.max_iter` iterations).
pub fn solve_ik(chain: &mut Chain, target: &PositionTarget, config: &SolverConfig) -> SolveResult {
    for iter in 0..config.max_iter {
        let residual = solve_ik_step(chain, target, config);
        if residual < config.tolerance {
            return SolveResult {
                converged: true,
                iterations: iter + 1,
                residual,
            };
        }
    }

    let transforms = chain.compute_transforms();
    let residual = transforms
        .get(&target.body_id)
        .map(|p| {
            let pos = Vec3::new(p.translation[0], p.translation[1], p.translation[2]);
            let t = Vec3::new(target.target[0], target.target[1], target.target[2]);
            (t - pos).norm()
        })
        .unwrap_or(f32::INFINITY);

    SolveResult {
        converged: false,
        iterations: config.max_iter,
        residual,
    }
}

// ---------------------------------------------------------------------------
// helpers for tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chain::{Body, Joint};

    /// Single revolute-Y joint, link length 1 along X in child frame.
    /// End-effector (body B) traces a circle of radius 1 in the XZ plane.
    fn one_joint_arm() -> (Chain, BodyId) {
        let mut chain = Chain::new();
        let a = chain.add_body(Body::new("A"));
        let b = chain.add_body(Body {
            local_transform: crate::chain::Pose::from_translation(1.0, 0.0, 0.0),
            ..Body::new("B")
        });
        chain
            .add_joint(Joint::revolute(a, b, [0.0, 1.0, 0.0]))
            .unwrap();
        (chain, b)
    }

    /// Three revolute-Y joints, each link 1 unit along X in child frame.
    fn three_joint_arm() -> (Chain, BodyId) {
        let mut chain = Chain::new();
        let a = chain.add_body(Body::new("A"));
        let b = chain.add_body(Body {
            local_transform: crate::chain::Pose::from_translation(1.0, 0.0, 0.0),
            ..Body::new("B")
        });
        let c = chain.add_body(Body {
            local_transform: crate::chain::Pose::from_translation(1.0, 0.0, 0.0),
            ..Body::new("C")
        });
        let d = chain.add_body(Body {
            local_transform: crate::chain::Pose::from_translation(1.0, 0.0, 0.0),
            ..Body::new("D")
        });
        chain
            .add_joint(Joint::revolute(a, b, [0.0, 1.0, 0.0]))
            .unwrap();
        chain
            .add_joint(Joint::revolute(b, c, [0.0, 1.0, 0.0]))
            .unwrap();
        chain
            .add_joint(Joint::revolute(c, d, [0.0, 1.0, 0.0]))
            .unwrap();
        (chain, d)
    }

    // --- 6.8 tests ---

    #[test]
    fn single_revolute_reaches_reachable_target() {
        let (mut chain, ee) = one_joint_arm();
        // Target is [0, 0, -1] — achieved at theta = PI/2
        let target = PositionTarget {
            body_id: ee,
            target: [0.0, 0.0, -1.0],
        };
        let config = SolverConfig {
            max_iter: 20,
            ..Default::default()
        };
        let result = solve_ik(&mut chain, &target, &config);
        assert!(
            result.converged,
            "should converge; residual = {}",
            result.residual
        );
        assert!(
            result.iterations <= 20,
            "must converge within 20 iterations, got {}",
            result.iterations
        );
        assert!(
            result.residual < 1e-4,
            "residual too large: {}",
            result.residual
        );
    }

    #[test]
    fn unreachable_target_returns_not_converged() {
        let (mut chain, ee) = one_joint_arm();
        // Arm reach = 1 unit; target is 10 units away
        let target = PositionTarget {
            body_id: ee,
            target: [10.0, 0.0, 0.0],
        };
        let config = SolverConfig::default();
        let result = solve_ik(&mut chain, &target, &config);
        assert!(!result.converged);
        assert!(result.residual > config.tolerance);
    }

    #[test]
    fn joint_limits_never_violated_after_solve() {
        let (mut chain, ee) = three_joint_arm();
        // Set tight limits on all joints
        for joint in chain.joints.iter_mut() {
            joint.min = -1.0;
            joint.max = 1.0;
        }
        let target = PositionTarget {
            body_id: ee,
            target: [2.5, 0.0, 0.0],
        };
        let config = SolverConfig::default();
        solve_ik(&mut chain, &target, &config);
        for joint in &chain.joints {
            assert!(
                joint.value >= joint.min - 1e-6 && joint.value <= joint.max + 1e-6,
                "joint {} value {} out of limits [{}, {}]",
                joint.id,
                joint.value,
                joint.min,
                joint.max
            );
        }
    }

    #[test]
    fn three_joint_arm_reaches_ten_sampled_poses() {
        // Use FK to generate 10 known-reachable targets, then verify IK
        // can reach each from a zeroed starting configuration.
        let seed_configs: [[f32; 3]; 10] = [
            [0.3, 0.0, 0.0],
            [-0.3, 0.3, 0.0],
            [0.5, -0.5, 0.2],
            [-0.8, 0.4, -0.3],
            [1.0, 1.0, -1.0],
            [-1.0, -1.0, 1.0],
            [0.2, 0.8, 0.5],
            [-0.5, 0.0, 0.5],
            [0.7, -0.2, -0.4],
            [0.0, 0.6, -0.6],
        ];

        for (i, &angles) in seed_configs.iter().enumerate() {
            let (mut chain, ee) = three_joint_arm();

            // Set joint values to sample config and compute FK target
            for (j, &a) in angles.iter().enumerate() {
                chain.set_joint_value(chain.joints[j].id, a);
            }
            let fk = chain.compute_transforms();
            let p = &fk[&ee];
            let target_pos = p.translation;

            // Reset joints and run IK
            for joint in chain.joints.iter_mut() {
                joint.value = 0.0;
            }
            let target = PositionTarget {
                body_id: ee,
                target: target_pos,
            };
            let config = SolverConfig {
                max_iter: 150,
                step_size: 0.5,
                ..Default::default()
            };
            let result = solve_ik(&mut chain, &target, &config);

            assert!(
                result.residual < 1e-4,
                "pose {i}: residual {} exceeded 1e-4 (converged={})",
                result.residual,
                result.converged
            );
        }
    }
}
