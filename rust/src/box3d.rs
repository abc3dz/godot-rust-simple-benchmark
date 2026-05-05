use godot::prelude::*;
use godot::classes::{StaticBody3D, IStaticBody3D};

#[derive(GodotClass)]
#[class(init, base=StaticBody3D)]
pub struct Box3d {
    base: Base<StaticBody3D>,
}

#[godot_api]
impl IStaticBody3D for Box3d {
    fn ready(&mut self) {
        // Initialization code for the box
    }
}

#[godot_api]
impl Box3d {
}