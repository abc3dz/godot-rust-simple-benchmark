use crate::box3d;

use godot::global::{randi_range};
use godot::prelude::*;
use godot::classes::{Node, PackedScene, InputEvent, Timer, Engine, Label};

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct MainScene {
    #[export]
    box_scene: Option<Gd<PackedScene>>,

    #[init(node = "SpawnTimer")]
    spawn_timer: OnReady<Gd<Timer>>,

    #[init(node = "CanvasLayer/Label")]
    ui_label: OnReady<Gd<Label>>,

    spawned_count: u32,

    base: Base<Node>,
}

#[godot_api]
impl INode for MainScene {
    fn ready(&mut self) {
        self.spawned_count = 0;
    }

    // fn unhandled_input(&mut self, event: Gd<InputEvent>) {
    //     if event.is_action_pressed("ui_accept"){
    //         let scene = self.box_scene.as_ref().unwrap();

    //         let mut box3d = scene
    //             .instantiate_as::<box3d::Box3d>();
    //         box3d.set_position(Vector3 { x: randi_range(-5, 5) as f32, y: randi_range(3, 5) as f32, z: randi_range(-5, 5) as f32 });
            
    //         self.base_mut().add_child(&box3d);
    //     }
    // }
}

#[godot_api]
impl MainScene {
    fn spawn_box(&mut self) {
        let width = 10;
        let height = 8;
        let spacing = 1.0;
        

        for y in 0..height {
            for x in -5..width {
                
                let scene = self.box_scene.as_ref().unwrap();

                let mut box3d = scene
                    .instantiate_as::<box3d::Box3d>();

                self.spawned_count += 80;

                let pos = Vector3::new(
                    x as f32 * spacing,
                    y as f32 * spacing,
                    0.0,
                );

                box3d.set_position(pos);

                // เพิ่ม instance ใหม่เข้าไปใน Scene Tree
                self.base_mut().add_child(&box3d); 
        
                
        
            }
        }
        godot_print!("Spawned {} boxes!", &self.spawned_count);

        let fps = Engine::singleton().get_frames_per_second();
        let text = format!("Rust\nFPS: {}\nSpawn Count: {}", fps, self.spawned_count);
        self.ui_label.set_text(&text);
        
    }
    #[func]
    fn _on_spawn_timer_timeout(&mut self) {
        self.spawn_box();
    }
}