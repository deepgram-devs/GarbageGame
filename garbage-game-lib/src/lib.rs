use gdnative::prelude::*;

pub(crate) mod ant;
pub(crate) mod game;
pub(crate) mod waste;

pub(crate) use ant::Ant;
pub(crate) use game::Game;
pub(crate) use waste::Waste;

godot_init!(init);

fn init(handle: InitHandle) {
    handle.add_class::<Ant>();
    handle.add_class::<Game>();
    handle.add_class::<Waste>();
}
