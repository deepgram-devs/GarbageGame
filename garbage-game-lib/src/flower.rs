use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct Flower;

impl Flower {
    fn new(_base: &Area2D) -> Self {
        Flower
    }
}

#[methods]
impl Flower {
    #[method]
    fn _ready(&mut self, #[base] base: &Area2D) {
        let animated_sprite = unsafe {
            base.get_node_as::<AnimatedSprite>("AnimatedSprite")
                .expect("Flower should have an AnimatedSprite.")
        };
        animated_sprite.play("default", false);
    }

    #[method]
    fn on_flower_body_entered(&self, #[base] base: &Area2D, body: Ref<PhysicsBody2D>) {
        let body = unsafe { body.assume_safe() };
        if body.is_in_group("Waste") {
            base.queue_free()
        }
    }
}
