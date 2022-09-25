use super::game::load_audio_stream;
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
    fn on_area_2d_body_entered(&self, #[base] base: &Node2D, body: Ref<PhysicsBody2D>) {
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
                waste.queue_free();

                self.jiggle(base);
            }
        }
    }

    #[method]
    fn on_jiggle_timer_timeout(&self, #[base] base: &Node2D) {
        let animated_sprite = unsafe {
            base.get_node_as::<AnimatedSprite>("MushroomHead")
                .expect("Mushroom should have an AnimatedSprite named MushroomHead.")
        };
        animated_sprite.play("idle", false);
    }

    #[method]
    pub fn jiggle(&self, #[base] base: &Node2D) {
        let animated_sprite = unsafe {
            base.get_node_as::<AnimatedSprite>("MushroomHead")
                .expect("Mushroom should have an AnimatedSprite named MushroomHead.")
        };
        animated_sprite.play("jiggle", false);

        let jiggle_timer = unsafe {
            base.get_node_as::<Timer>("JiggleTimer")
                .expect("Mushroom should have a Timer named JiggleTimer.")
        };
        jiggle_timer.start(0.2);

        // TODO: the mushroom spores should probably be a separate object that we instance
        // because as it is, only one mushroom spore animation can happen at a time
        // even if the mushroom is hit twice in quick succession
        let animated_sprite = unsafe {
            base.get_node_as::<AnimatedSprite>("MushroomSpores")
                .expect("Mushroom should have an AnimatedSprite named MushroomSpores.")
        };
        if !animated_sprite.is_visible() {
            animated_sprite.set_frame(0);
            animated_sprite.set_visible(true);
            animated_sprite.play("excreting", false);
        }

        // Play a sound effect! TODO: figure out how to preload...
        let audio_stream_player = unsafe {
            base.get_node_as::<AudioStreamPlayer2D>("MushroomJiggleStreamPlayer")
                .expect(
                    "Mushroom should have an AudioStreamPlayer2D named MushroomJiggleStreamPlayer.",
                )
        };
        if let Some(audio_stream) = load_audio_stream("res://Assets/Sfx/mushroom_jiggle.wav") {
            let audio_stream = unsafe { audio_stream.assume_shared() };
            if !audio_stream_player.is_playing() {
                audio_stream_player.set_stream(audio_stream);
                audio_stream_player.play(0.0);
            }
        }
    }

    #[method]
    pub fn on_mushroom_spores_animation_finished(&mut self, #[base] base: &Node2D) {
        let animated_sprite = unsafe {
            base.get_node_as::<AnimatedSprite>("MushroomSpores")
                .expect("Mushroom should have an AnimatedSprite named MushroomSpores.")
        };
        animated_sprite.set_visible(false);
    }
}
