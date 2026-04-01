use godot::classes::{IRigidBody2D, RigidBody2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct PlayerRust {
    base: Base<RigidBody2D>,
}

#[godot_api]
impl IRigidBody2D for PlayerRust {
    fn init(base: Base<RigidBody2D>) -> Self {
        PlayerRust { base }
    }
}
