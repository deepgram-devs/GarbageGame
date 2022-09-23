use gdnative::api::{RandomNumberGenerator, RigidBody2D};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(RigidBody2D)]
pub struct Waste {
    /// Whether this is being carried by an Ant or not.
    #[property]
    pub(crate) being_carried: bool,
    /// Tells if an Ant is moving towards the Waste to collect it
    #[property]
    pub(crate) being_collected: bool,

    /// Where this waste will stop falling
    finish_position_y: f32,
}

impl Waste {
    fn new(_base: &RigidBody2D) -> Self {
        let rng = RandomNumberGenerator::new();
        rng.randomize();

        let random_finish_position = rng.randf_range(150.0, 330.0);
        Waste {
            being_carried: false,
            being_collected: false,
            finish_position_y: random_finish_position as f32,
        }
    }
}

#[methods]
impl Waste {
    #[method]
    fn _physics_process(&self, #[base] base: &RigidBody2D, _delta: f32) {
        if base.position().y > self.finish_position_y {
            base.set_gravity_scale(0.0);
            base.set_linear_damp(10.0);
        }
    }
}
