use gdnative::prelude::*;

enum SelectedFaction {
    Ant,
    Beetle,
    NoneSelected, // Option<>?
}

#[derive(NativeClass)]
#[inherit(Node2D)]
pub(crate) struct Game {
    selected_faction: SelectedFaction,
}

impl Game {
    fn new(base: &Node2D) -> Self {
        Game {
            selected_faction: SelectedFaction::NoneSelected,
        }
    }
}

#[methods]
impl Game {
    #[method]
    fn _ready(&mut self, #[base] base: &Node2D, _delta: f32) {}
    fn _process(&mut self, #[base] base: &Node2D, _delta: f32) {}
}
