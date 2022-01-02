use rapier2d::prelude::*;

pub fn create_engine() {
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    let gravity = vector![0.0, -9.81];
    let mut integration_parameters = IntegrationParameters::default();
    integration_parameters.dt = 1.0 / 120.0;
    integration_parameters.max_ccd_substeps = 1;

    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut joint_set = JointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();
}