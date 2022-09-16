use gdnative::api::RigidBody2D;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(RigidBody2D)]
pub(crate) struct Waste {
    /// Whether this is being carried by an Ant or not.
    #[property]
    being_carried: bool,
    /// Tells if an Ant is moving towards the Waste to collect it
    #[property]
    being_collected: bool,

    // HACK - should maybe store the base? But need to store `Ref` and I don't get that in new?
    pub(crate) global_position: Vector2,
}

impl Waste {
    fn new(base: &RigidBody2D) -> Self {
        Waste {
            being_carried: false,
            being_collected: false,
            global_position: base.global_position(),
        }
    }
}

#[methods]
impl Waste {
    #[method]
    fn _physics_process(&mut self, #[base] base: &RigidBody2D, _delta: f32) {
        if base.position().y > 240.0 {
            base.set_gravity_scale(0.0);
        }
        self.global_position = base.global_position();
    }
}
