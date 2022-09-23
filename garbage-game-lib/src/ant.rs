use super::waste::{State as WasteState, Waste};

use gdnative::api::{KinematicBody2D, PinJoint2D};
use gdnative::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

const SPEED: f32 = 100.0;

// See
// https://docs.rs/gdnative/latest/gdnative/api/struct.KinematicBody2D.html#method.move_and_slide
// for these.
const DEFAULT_MAX_SLIDES: i64 = 4;

pub(crate) enum State {
    Idle,
    GoingToArea(Vector2),
    CollectingWaste(Rc<RefCell<Instance<Waste>>>),
    GoingToMushroom(Rc<RefCell<Instance<Waste>>>),
}

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Ant {
    pub(crate) state: State,
}

impl Ant {
    fn new(_base: &KinematicBody2D) -> Self {
        // Maybe fuzzy/random speed
        Ant { state: State::Idle }
    }

    pub(crate) fn collect_waste(&mut self, waste: TInstance<Waste>) {
        waste
            .map_mut(|waste, _| waste.state = WasteState::Sought)
            .expect("Failed to mark waste as being collected");

        let waste = waste.claim();
        self.state = State::CollectingWaste(Rc::new(RefCell::new(waste)));
    }
}

#[methods]
impl Ant {
    #[method]
    fn _physics_process(&mut self, #[base] base: &KinematicBody2D, _delta: f32) {
        let velocity = match &mut self.state {
            State::CollectingWaste(waste) => {
                let waste = unsafe { waste.borrow().base().assume_safe() };
                let direction = waste.global_position() - base.global_position();
                let direction = direction.normalized();
                direction * SPEED
            }
            State::GoingToArea(destination) => {
                if destination.distance_to(base.global_position()) < 10.0 {
                    self.state = State::Idle;
                    Vector2::ZERO
                } else {
                    let direction = *destination - base.global_position();
                    let direction = direction.normalized();
                    direction * SPEED
                }
            }
            State::GoingToMushroom(waste) => {
                // TODO store mushroom position somewhere
                let mushroom_position = Vector2::new(320.0, 204.0);
                let waste = unsafe { waste.borrow().base().assume_safe() };

                // TODO: this is a kind of hacky way of asking "is the waste close to the mushroom"
                if waste.global_position().distance_to(mushroom_position) < 50.0 {
                    waste.queue_free();
                    self.state = State::Idle;
                    for child in base.get_children().into_iter() {
                        if let Some(joint) = child.to_object::<PinJoint2D>() {
                            unsafe { joint.assume_safe() }.queue_free();
                        }
                    }
                    // Don't just stay at mushroom - go back somewhere

                    Vector2::ZERO
                } else {
                    let direction = mushroom_position - base.global_position();
                    let direction = direction.normalized();
                    direction * SPEED
                }
            }
            State::Idle => Vector2::ZERO,
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
            let inverted_returned_velocity =
                Vector2::new(returned_velocity.x, -returned_velocity.y);

            base.set_rotation(inverted_returned_velocity.angle() as f64);
        }

        let mut reached_waste = false;

        // TODO `any()`
        if let State::CollectingWaste(waste) = &mut self.state {
            for collision_idx in 0..base.get_slide_count() {
                if let Some(collision) = base.get_slide_collision(collision_idx) {
                    let collision = unsafe { collision.assume_safe() };
                    if let Some(collider) = collision.collider() {
                        let collider = unsafe { collider.assume_safe() };
                        let waste_id =
                            unsafe { waste.borrow().base().assume_safe().get_instance_id() };
                        if collider.get_instance_id() == waste_id {
                            reached_waste = true;
                        }
                    }
                }
            }
        }

        if reached_waste {
            if let State::CollectingWaste(waste) = &mut self.state {
                let waste_ref = unsafe { waste.borrow_mut().assume_safe() };
                waste_ref
                    .map_mut(|waste, _| {
                        waste.state = WasteState::Carried;
                    })
                    .expect("Faild to mark waste as being carried");

                let joint = PinJoint2D::new();
                joint.add_to_group("PinJoint2D", false);
                joint.set_node_a(base.get_path());
                joint.set_node_b(waste_ref.base().get_path());

                // TODO maybe store joint in state?
                base.add_child(joint, false);

                self.state = State::GoingToMushroom(Rc::clone(waste));
            }
        }
    }
}
