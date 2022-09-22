use gdnative::prelude::*;

godot_init!(init);

mod ant;
mod game;
mod waste;

use ant::Ant;
use game::Game;
use waste::Waste;

fn init(handle: InitHandle) {
    handle.add_class::<Ant>();
    handle.add_class::<Game>();
    handle.add_class::<Waste>();
}
