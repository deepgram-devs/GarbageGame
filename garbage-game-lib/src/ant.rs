use super::Waste;

use gdnative::api::KinematicBody2D;
use gdnative::prelude::*;

const SPEED: f32 = 100.0;

// See
// https://docs.rs/gdnative/latest/gdnative/api/struct.KinematicBody2D.html#method.move_and_slide
// for these.
const DEFAULT_MAX_SLIDES: i64 = 4;

enum State {
    Idle,
    CollectingWaste(Instance<Waste>),
}

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Ant {
    state: State,
}

impl Ant {
    fn new(_base: &KinematicBody2D) -> Self {
        Ant { state: State::Idle }
    }

    pub(crate) fn collect_waste(&mut self, waste: TInstance<Waste>) {
        waste
            .map_mut(|waste, _| waste.being_collected = true)
            .expect("Failed to mark waste as being collected");

        let waste = waste.claim();
        self.state = State::CollectingWaste(waste);

        godot_print!("Collecting some waste!");
    }

    pub(crate) fn collecting_waste(&self) -> bool {
        matches!(self.state, State::CollectingWaste(_))
    }
}

#[methods]
impl Ant {
    #[method]
    fn _physics_process(&self, #[base] base: &KinematicBody2D, _delta: f32) {
        let velocity = match &self.state {
            State::CollectingWaste(waste) => {
                let waste = unsafe { waste.base().assume_safe() };
                let direction = waste.global_position() - base.global_position();
                let direction = direction.normalized();
                direction * SPEED
            }
            _ => Vector2::ZERO,
        };
        let returned_velocity = base.move_and_slide(
            velocity,
            Vector2::ZERO,
            false,
            DEFAULT_MAX_SLIDES,
            0.0,
            false,
        );

        // This doesn't look totally right :thinking:
        if returned_velocity != Vector2::ZERO {
            base.set_rotation(returned_velocity.angle() as f64);
        }
    }
}
