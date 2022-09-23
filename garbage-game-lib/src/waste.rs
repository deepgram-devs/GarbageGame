use gdnative::api::{RandomNumberGenerator, RigidBody2D};
use gdnative::prelude::*;

pub(crate) enum State {
    Falling(f32),
    Grounded,
    Sought,
    Carried,
}

#[derive(NativeClass)]
#[inherit(RigidBody2D)]
pub struct Waste {
    pub(crate) state: State,
}

impl Waste {
    fn new(_base: &RigidBody2D) -> Self {
        let rng = RandomNumberGenerator::new();
        rng.randomize();

        Waste {
            state: State::Falling(rng.randf_range(50.0, 335.0) as f32),
        }
    }
}

#[methods]
impl Waste {
    #[method]
    fn _physics_process(&mut self, #[base] base: &RigidBody2D, _delta: f32) {
        if let State::Falling(distance) = self.state {
            if base.position().y > distance {
                base.set_gravity_scale(0.0);
                base.set_linear_damp(10.0);
                base.set_collision_mask_bit(0, true); // tilemap
                base.set_collision_mask_bit(1, true); // ants
                base.set_collision_mask_bit(2, true); // waste on the ground

                base.set_collision_layer_bit(2, true); // waste on the ground

                self.state = State::Grounded;
            }
        }
    }
}
