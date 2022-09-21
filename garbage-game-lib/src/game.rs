use gdnative::{api::RigidBody2D, prelude::*};

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Game {}

impl Game {
    fn new(_base: &Node2D) -> Self {
        Game {}
    }
}

#[methods]
impl Game {
    #[method]
    fn _process(&self, #[base] base: &Node2D, _delta: f32) {
        let scene = base.get_tree().expect("Game tree should always be there");
        let scene = unsafe { scene.assume_safe() };
        let wastes = scene.get_nodes_in_group("Waste");
        for waste in wastes.into_iter() {
            let waste = waste
                .to_object::<RigidBody2D>()
                .expect("Waste wasn't RigidBody2D");
            godot_print!("There is a waste!");
        }
    }

    #[method]
    fn on_waste_timer_timeout(&self, #[base] base: &Node2D) {
        let packed_scene = load_scene("res://Scenes/Waste.tscn").expect("Waste scene should exist");
        let new_waste = packed_scene
            .instance(0)
            .expect("Failed to instantiate Waste scene");
        base.add_child(new_waste, false);
    }
}

pub fn load_scene(path: &str) -> Option<Ref<PackedScene, ThreadLocal>> {
    let scene = ResourceLoader::godot_singleton().load(path, "", false)?;
    let scene = unsafe { scene.assume_thread_local() };
    scene.cast::<PackedScene>()
}
