use godot::prelude::*;

pub mod game_manager;
pub mod player;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

mod main_scene;
mod box3d;