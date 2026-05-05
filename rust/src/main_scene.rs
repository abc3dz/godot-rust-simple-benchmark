use crate::box3d;

use godot::global::{randi_range};
use godot::prelude::*;
use godot::classes::{Node, PackedScene, Timer, Engine, Label};

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
        self.spawn_box();
    }
}

#[godot_api]
impl MainScene {
    fn spawn_box(&mut self) {
        let spawn_amount = 5000; 

        let scene = match self.box_scene.as_ref() {
            Some(s) => s.clone(),
            None => {
                godot_error!("Box scene not assigned!");
                return;
            }
        };

        for _ in 0..spawn_amount {
            let mut box_instance = scene.instantiate_as::<box3d::Box3d>();

            let pos = Vector3::new(
                randi_range(-20, 20) as f32,
                randi_range(10, 30) as f32,
                randi_range(-20, 20) as f32,
            );
            box_instance.set_position(pos);

            self.base_mut().add_child(&box_instance);
            
            self.spawned_count += 1;
        }

        // อัปเดต UI
        let fps = Engine::singleton().get_frames_per_second();
        let text = format!("Language: Rust\nFPS: {:.0}\nObjects: {}", fps, self.spawned_count);
        self.ui_label.set_text(&text);
    }

    #[func]
    fn _on_spawn_timer_timeout(&mut self) {
        self.spawn_box();
    }
}