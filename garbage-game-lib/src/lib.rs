use gdnative::prelude::*;

godot_init!(init);

mod ant;
mod flower;
mod game;
mod mushroom;
mod waste;

fn init(handle: InitHandle) {
    handle.add_class::<ant::Ant>();
    handle.add_class::<game::Game>();
    handle.add_class::<waste::Waste>();
    handle.add_class::<mushroom::Mushroom>();
    handle.add_class::<flower::Flower>();
}
