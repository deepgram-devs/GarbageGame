use crate::Waste;

use gdnative::api::{PinJoint2D, RigidBody2D};
use gdnative::prelude::*;

const FACTOR: f32 = 100.0;

enum State {
    CollectingGarbage(Waste),
    GoingToArea {
        area_position: Vector2,
    },
    GoingToMushroom {
        mushroom_position: Vector2,
        pin_joint: PinJoint2D,
        waste: Waste,
    },
    Idle,
}

#[derive(NativeClass)]
#[inherit(RigidBody2D)]
pub(crate) struct Ant {
    /// Whether this is being carried by an Ant or not.
    #[property]
    being_carried: bool,
    #[property]
    being_collected: bool,
    state: State,
}

impl Ant {
    fn new(_base: &RigidBody2D) -> Self {
        Ant {
            being_carried: false,
            being_collected: false,
            state: State::Idle,
        }
    }
}

#[methods]
impl Ant {
    #[method]
    fn _physics_process(&mut self, #[base] base: &RigidBody2D, delta: f32) {
        match &self.state {
            &State::GoingToArea { area_position } => {
                let global_position = base.global_position();
                if global_position.distance_to(area_position) < 10.0 {
                    self.state = State::Idle;
                } else {
                    let direction = (area_position - global_position).normalized();
                    base.apply_central_impulse(direction * FACTOR * delta)
                }
            }
            State::CollectingGarbage(waste) => {
                let direction = (waste.global_position - base.global_position()).normalized();
                base.apply_central_impulse(direction * FACTOR * delta)
            }
            State::GoingToMushroom {
                mushroom_position,
                pin_joint,
                ..
            } => {
                let mushroom_position = *mushroom_position;
                let global_position = base.global_position();
                if global_position.distance_to(mushroom_position) < 10.0 {
                    pin_joint.queue_free();
                    self.state = State::Idle;
                    // Waste is dropped :crossed_fingers:
                } else {
                    let direction = (mushroom_position - global_position).normalized();
                    base.apply_central_impulse(direction * FACTOR * delta)
                }
            }
            _ => todo!(),
        }
    }
}
// func _on_Ant_body_entered(body):
// 	if body.is_in_group("Waste"):
// 		#if !carrying_waste and !body.being_carried:
// 		if body == waste:
// 			carrying_waste = true
// 			body.being_carried = true
// 			body.being_collected = false
//
// 			# this is kind of a hack for the AI, so that ants don't get stuck colliding into waste being carried by other ants
// 			body.set_collision_layer_bit(0, false)
// 			body.set_collision_layer_bit(1, true)
//
// 			state = State.GOING_TO_MUSHROOM
//
// 			pin_joint_2d = PinJoint2D.new()
// 			pin_joint_2d.add_to_group("PinJoint2D")
//
// 			pin_joint_2d.set_node_a(self.get_path())
// 			pin_joint_2d.set_node_b(body.get_path())
//
// 			add_child(pin_joint_2d)
