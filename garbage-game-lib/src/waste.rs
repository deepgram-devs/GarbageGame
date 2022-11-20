use super::game::load_audio_stream;

use gdnative::api::{AnimatedSprite, AudioStreamPlayer2D, RandomNumberGenerator, RigidBody2D};
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
            state: State::Falling(rng.randf_range(55.0, 340.0) as f32),
        }
    }
}

#[methods]
impl Waste {
    #[method]
    fn _ready(&mut self, #[base] base: &RigidBody2D) {
        // Play a sound effect! TODO: figure out how to preload...
        let audio_stream_player = unsafe {
            base.get_node_as::<AudioStreamPlayer2D>("WasteFallStreamPlayer")
                .expect("Waste should have an AudioStreamPlayer2D named WasteFallStreamPlayer.")
        };
        if let Some(audio_stream) = load_audio_stream("res://Assets/Sfx/waste_fall.wav") {
            let audio_stream = unsafe { audio_stream.assume_shared() };
            if !audio_stream_player.is_playing() {
                audio_stream_player.set_stream(audio_stream);
                audio_stream_player.play(0.0);
            }
        }
    }

    #[method]
    fn _physics_process(&mut self, #[base] base: &RigidBody2D, _delta: f32) {
        if let State::Falling(distance) = self.state {
            let shadow = unsafe {
                base.get_node_as::<Sprite>("Shadow")
                    .expect("Waste should have a Shadow.")
            };

            if base.position().y > distance {
                base.set_gravity_scale(0.0);
                base.set_linear_damp(10.0);
                base.set_collision_mask_bit(0, true); // tilemap
                base.set_collision_mask_bit(1, true); // ants
                base.set_collision_mask_bit(2, true); // waste on the ground

                base.set_collision_layer_bit(2, true); // waste on the ground

                self.state = State::Grounded;
                shadow.set_visible(false);
            } else {
                shadow.set_position(Vector2::new(0.0, distance - base.position().y));
            }
        }
    }

    #[method]
    pub fn explode(&mut self, #[base] base: &RigidBody2D) {
        let animated_sprite = unsafe {
            base.get_node_as::<AnimatedSprite>("AnimatedSprite")
                .expect("Waste should have an AnimatedSprite.")
        };
        animated_sprite.play("explosion", false);

        // Play a sound effect! TODO: figure out how to preload...
        let audio_stream_player = unsafe {
            base.get_node_as::<AudioStreamPlayer2D>("WasteExplodeStreamPlayer")
                .expect("Waste should have an AudioStreamPlayer2D named WasteExplodeStreamPlayer.")
        };
        if let Some(audio_stream) = load_audio_stream("res://Assets/Sfx/waste_explode.wav") {
            let audio_stream = unsafe { audio_stream.assume_shared() };
            if !audio_stream_player.is_playing() {
                audio_stream_player.set_stream(audio_stream);
                audio_stream_player.play(0.0);
            }
        }
    }

    #[method]
    pub fn on_animated_sprite_animation_finished(&mut self, #[base] base: &RigidBody2D) {
        let animated_sprite = unsafe {
            base.get_node_as::<AnimatedSprite>("AnimatedSprite")
                .expect("Waste should have an AnimatedSprite.")
        };
        if animated_sprite.animation() == GodotString::from("explosion") {
            base.queue_free();
        }
    }
}
