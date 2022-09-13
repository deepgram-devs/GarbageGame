use gdnative::api::RigidBody2D;
use gdnative::prelude::*;

godot_init!(init);

#[derive(NativeClass)]
#[inherit(RigidBody2D)]
pub struct Waste {
    /// Whether this is being carried by an Ant or not.
    #[property]
    being_carried: bool,
}

impl Waste {
    fn new(_base: &RigidBody2D) -> Self {
        Waste {
            being_carried: false,
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

fn init(handle: InitHandle) {
    handle.add_class::<Waste>();
}