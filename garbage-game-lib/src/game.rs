use gdnative::api::{Area2D, GlobalConstants, RandomNumberGenerator, RigidBody2D};
use gdnative::prelude::*;

use super::ant::{Ant, State as AntState};
use super::waste::{State as WasteState, Waste};

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Game {
    rng: Ref<RandomNumberGenerator, Unique>,
}

impl Game {
    fn new(_base: &Node2D) -> Self {
        let rng = RandomNumberGenerator::new();
        rng.randomize();
        Game { rng }
    }
}

#[methods]
impl Game {
    #[method]
    fn _input(&self, #[base] base: &Node2D, event: Ref<InputEvent>) {
        let event = unsafe { event.assume_safe() };
        if let Some(key) = event.cast::<InputEventKey>() {
            if key.is_pressed() {
                let area_button_path = if key.scancode() == GlobalConstants::KEY_N {
                    Some("CanvasLayer/MarginContainer/HBoxContainer/VBoxContainer2/ButtonN")
                } else if key.scancode() == GlobalConstants::KEY_S {
                    Some("CanvasLayer/MarginContainer/HBoxContainer/VBoxContainer2/ButtonS")
                } else if key.scancode() == GlobalConstants::KEY_E {
                    Some("CanvasLayer/MarginContainer/HBoxContainer/VBoxContainer3/ButtonE")
                } else if key.scancode() == GlobalConstants::KEY_W {
                    Some("CanvasLayer/MarginContainer/HBoxContainer/VBoxContainer1/ButtonW")
                } else {
                    None
                };

                if let Some(area_button_path) = area_button_path {
                    let button = unsafe {
                        base.get_node(area_button_path)
                            .expect(&format!(
                                "The following button node path couldn't be found: {:?}.",
                                area_button_path
                            ))
                            .assume_safe()
                    };
                    button.emit_signal("pressed", &[]);
                }
            }
        }
    }

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
            let can_pickup_garbage = ant_instance
                .map(|ant, _| matches!(ant.state, AntState::Idle | AntState::GoingToArea(_)))
                .expect("Should always be able to ask if ant is idle");

            if can_pickup_garbage {
                for waste in wastes.into_iter() {
                    let waste = unsafe {
                        waste
                            .to_object::<RigidBody2D>()
                            .expect("Waste wasn't RigidBody2D")
                            .assume_safe()
                    };
                    let waste_instance = waste.cast_instance::<Waste>().unwrap();
                    let waste_grounded = waste_instance
                        .map(|waste, _| matches!(waste.state, WasteState::Grounded))
                        .expect("Waste is missing collected property");
                    if waste_grounded
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
        let scene = base.get_tree().expect("Game tree should always be there");
        let scene = unsafe { scene.assume_safe() };
        let wastes = scene.get_nodes_in_group("Waste");

        if wastes.len() >= 20 {
            return;
        }

        let packed_scene = load_scene("res://Scenes/Waste.tscn").expect("Waste scene should exist");
        let new_waste = packed_scene
            .instance(0)
            .expect("Failed to instantiate Waste scene");
        let new_waste_node = unsafe { new_waste.assume_safe() };

        let waste_node = new_waste_node
            .cast::<Node2D>()
            .expect("All wastes are RigidBody2D which are Node2D");
        let x_pos = self.rng.randf_range(100.0, 500.0);
        waste_node.set_position(Vector2::new(x_pos as f32, -100.0));

        base.add_child(new_waste, false);
    }

    #[method]
    fn on_ant_spawn_timer_timeout(&self, #[base] base: &Node2D) {
        let scene = base.get_tree().expect("Game tree should always be there");
        let scene = unsafe { scene.assume_safe() };
        let ants = scene.get_nodes_in_group("Ant");

        if ants.len() >= 5 {
            return;
        }

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

    #[method]
    fn on_deepgram_instance_message_received(&self, #[base] base: &Node2D, message: String) {
        godot_print!("In game.rs and received Deepgram message: {:?}", message);

        let area_button_path = if message.contains("north") {
            Some("CanvasLayer/MarginContainer/HBoxContainer/VBoxContainer2/ButtonN")
        } else if message.contains("south") {
            Some("CanvasLayer/MarginContainer/HBoxContainer/VBoxContainer2/ButtonS")
        } else if message.contains("east") {
            Some("CanvasLayer/MarginContainer/HBoxContainer/VBoxContainer3/ButtonE")
        } else if message.contains("west") {
            Some("CanvasLayer/MarginContainer/HBoxContainer/VBoxContainer1/ButtonW")
        } else {
            None
        };

        if let Some(area_button_path) = area_button_path {
            let button = unsafe {
                base.get_node(area_button_path)
                    .expect(&format!(
                        "The following button node path couldn't be found: {:?}.",
                        area_button_path
                    ))
                    .assume_safe()
            };
            button.emit_signal("pressed", &[]);
        }
    }

    #[method]
    fn on_north_button_pressed(&self, #[base] base: &Node2D) {
        self.handle_move_to_area_command(base, "AreaN")
    }

    #[method]
    fn on_south_button_pressed(&self, #[base] base: &Node2D) {
        self.handle_move_to_area_command(base, "AreaS")
    }

    #[method]
    fn on_east_button_pressed(&self, #[base] base: &Node2D) {
        self.handle_move_to_area_command(base, "AreaE")
    }

    #[method]
    fn on_west_button_pressed(&self, #[base] base: &Node2D) {
        self.handle_move_to_area_command(base, "AreaW")
    }

    fn handle_move_to_area_command(&self, base: &Node2D, area_path: &str) {
        let scene = base.get_tree().expect("Game tree should always be there");
        let scene = unsafe { scene.assume_safe() };
        let ants = scene.get_nodes_in_group("Ant");

        for ant in ants.into_iter() {
            let ant = unsafe {
                ant.to_object::<KinematicBody2D>()
                    .expect("Ant wasn't RigidBody2D")
                    .assume_safe()
            };
            let ant_instance = ant.cast_instance::<Ant>().unwrap();
            let should_go_to_area = ant_instance
                .map(|ant, _| matches!(ant.state, AntState::GoingToArea(_) | AntState::Idle))
                .expect("Should always be able to ask if ant is idle");
            if should_go_to_area {
                ant_instance
                    .map_mut(|ant, _| {
                        let area = unsafe {
                            base.get_node(area_path)
                                .expect(&format!(
                                    "Could not get the Area with path: {:?}",
                                    area_path
                                ))
                                .assume_safe()
                        };
                        let area = area
                            .cast::<Area2D>()
                            .expect("Unable to get Are2D in handle_move_to_area_command.");
                        let destination = area.global_position()
                            + Vector2::new(
                                self.rng.randf_range(-32.0, 32.0) as f32,
                                self.rng.randf_range(-32.0, 32.0) as f32,
                            );
                        ant.state = AntState::GoingToArea(destination)
                    })
                    .expect("Failed to send ant to area");
            }
        }
    }
}

pub fn load_scene(path: &str) -> Option<Ref<PackedScene, ThreadLocal>> {
    let scene = ResourceLoader::godot_singleton().load(path, "", false)?;
    let scene = unsafe { scene.assume_thread_local() };
    scene.cast::<PackedScene>()
}
