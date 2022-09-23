use gdnative::api::RigidBody2D;
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
}

impl Waste {
    fn new(_base: &RigidBody2D) -> Self {
        Waste {
            being_carried: false,
            being_collected: false,
        }
    }
}

#[methods]
impl Waste {
    #[method]
    fn _physics_process(&self, #[base] base: &RigidBody2D, _delta: f32) {
        if base.position().y > 240.0 {
            base.set_gravity_scale(0.0);
        }
    }
}
