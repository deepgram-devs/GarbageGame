use gdnative::api::{RandomNumberGenerator, RigidBody2D};
use gdnative::prelude::*;

use super::Ant;
use super::Waste;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Game {
    rng: Ref<RandomNumberGenerator, Unique>,
}

impl Game {
    fn new(_base: &Node2D) -> Self {
        Game {
            rng: RandomNumberGenerator::new(),
        }
    }
}

#[methods]
impl Game {
    #[method]
    fn _process(&self, #[base] base: &Node2D, _delta: f32) {
        let scene = base.get_tree().expect("Game tree should always be there");
        let scene = unsafe { scene.assume_safe() };
        let ants = scene.get_nodes_in_group("Ant");
        let wastes = scene.get_nodes_in_group("Waste");

        for ant in ants.into_iter() {
            let ant = unsafe {
                ant.to_object::<KinematicBody2D>()
                    .expect("Ant wasn't RigidBody2D")
                    .assume_safe()
            };
            let ant_instance = ant.cast_instance::<Ant>().unwrap();
            let is_idle = ant_instance
                .map(|ant, _| ant.is_idle())
                .expect("Should always be able to ask if ant is idle");

            if is_idle {
                for waste in wastes.into_iter() {
                    let waste = unsafe {
                        waste
                            .to_object::<RigidBody2D>()
                            .expect("Waste wasn't RigidBody2D")
                            .assume_safe()
                    };
                    let waste_instance = waste.cast_instance::<Waste>().unwrap();
                    let being_collected = waste_instance
                        .map(|waste, _| waste.being_collected)
                        .expect("Waste is missing collected property");
                    if !being_collected
                        && ant.global_position().distance_to(waste.global_position()) < 100.0
                    {
                        ant_instance
                            .map_mut(|ant, _| {
                                ant.collect_waste(waste_instance);
                            })
                            .expect("Couldn't mutate the ant or something");
                    }
                }
            }
        }
    }

    #[method]
    fn on_waste_timer_timeout(&self, #[base] base: &Node2D) {
        let packed_scene = load_scene("res://Scenes/Waste.tscn").expect("Waste scene should exist");
        let new_waste = packed_scene
            .instance(0)
            .expect("Failed to instantiate Waste scene");
        let new_waste_node = unsafe { new_waste.assume_safe() };

        let waste_node = new_waste_node
            .cast::<Node2D>()
            .expect("All wastes are RigidBody2D which are Node2D");
        let x_pos = self.rng.randf_range(100.0, 500.0);
        waste_node.set_position(Vector2::new(x_pos as f32, 100.0));

        base.add_child(new_waste, false);
    }

    #[method]
    fn on_ant_spawn_timer_timeout(&self, #[base] base: &Node2D) {
        let packed_scene = load_scene("res://Scenes/Ant.tscn").expect("Ant scene should exist");
        let new_ant = packed_scene
            .instance(0)
            .expect("Failed to instantiate Ant scene");
        let new_ant_node = unsafe { new_ant.assume_safe() };

        let ant_node = new_ant_node
            .cast::<Node2D>()
            .expect("All ants are KinematicBody2D which are Node2D");
        let x_pos = self.rng.randf_range(300.0, 400.0);
        ant_node.set_position(Vector2::new(x_pos as f32, 300.0));

        base.add_child(new_ant, false);
    }
}

pub fn load_scene(path: &str) -> Option<Ref<PackedScene, ThreadLocal>> {
    let scene = ResourceLoader::godot_singleton().load(path, "", false)?;
    let scene = unsafe { scene.assume_thread_local() };
    scene.cast::<PackedScene>()
}
