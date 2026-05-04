use godot::prelude::*;
use godot::classes::{Node3D};

#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct Box3d {
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for Box3d {
    fn ready(&mut self) {
        // Initialization code for the box
    }
}

#[godot_api]
impl Box3d {

    // // #[func]
    // // fn set_random_color(&mut self) {
    // //     let mut rng = random_number_generator;

    // //     let color = Color::from_rgb(
    // //         rng.gen::<f32>(),
    // //         rng.gen::<f32>(),
    // //         rng.gen::<f32>(),
    // //     );

    // //     // หา MeshInstance3D
    // //     let mesh = self
    // //         .base()
    // //         .get_node_as::<MeshInstance3D>("MeshInstance3D")
    // //         .unwrap();

    // //     // สร้าง material ใหม่
    // //     let mut material = StandardMaterial3D::new_gd();
    // //     material.set_albedo(color);

    // //     // apply
    // //     mesh.set_surface_override_material(0, material.upcast());
    // // }
}