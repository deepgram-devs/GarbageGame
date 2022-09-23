use super::waste::{State as WasteState, Waste};
use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Mushroom;

impl Mushroom {
    fn new(_base: &Node2D) -> Self {
        Mushroom
    }
}

#[methods]
impl Mushroom {
    #[method]
    fn on_area_2d_body_entered(&self, body: Ref<PhysicsBody2D>) {
        let body = unsafe { body.assume_safe() };
        if body.is_in_group("Waste") {
            let waste = body
                .cast::<RigidBody2D>()
                .expect("Waste wasn't RigidBody2D");

            let waste_instance = waste.cast_instance::<Waste>().unwrap();

            let waste_falling = waste_instance
                .map(|waste, _| {
                    matches!(waste.state, WasteState::Grounded | WasteState::Falling(_))
                })
                .expect("Waste is missing collected property");

            if waste_falling {
                waste.queue_free()
            }
        }
    }
}
