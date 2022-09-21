use gdnative::prelude::*;

godot_init!(init);

mod game;
mod waste;

fn init(handle: InitHandle) {
    handle.add_class::<game::Game>();
    handle.add_class::<waste::Waste>();
}
