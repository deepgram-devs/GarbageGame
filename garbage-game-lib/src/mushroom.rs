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
        if body.is_in_group("Flower") {
            body.queue_free();
        } else if body.is_in_group("Waste") {
            let waste = body
                .cast::<RigidBody2D>()
                .expect("Waste wasn't RigidBody2D");

            let waste_instance = waste.cast_instance::<Waste>().unwrap();

            let waste_can_be_destroyed = waste_instance
                .map(|waste, _| match waste.state {
                    WasteState::Grounded => true,
                    WasteState::Falling(distance) => {
                        // what we are saying here is that if the waste is set to fall below 250.0
                        // (i.e. well enough below the mushroom itself), then don't destroy it here,
                        // let it fall down through to the lower portion of the map - this way it
                        // will even out the distribution of areas around the map where the waste can fall
                        if distance < 250.0 {
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                })
                .expect("Could not check the state of Waste.");

            if waste_can_be_destroyed {
                waste.queue_free()
            }
        }
    }
}
