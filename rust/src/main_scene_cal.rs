use godot::prelude::*;
use godot::classes::{Node, Label, Time};

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct MainSceneCal {
    #[init(node = "CanvasLayer/Label")]
    ui_label: OnReady<Gd<Label>>,

    base: Base<Node>,
}

#[godot_api]
impl INode for MainSceneCal {
    fn ready(&mut self) {
        self.run_benchmark();
    }
}

#[godot_api]
impl MainSceneCal {
    fn run_benchmark(&mut self) {
        let iterations = 5_000_000;

        let start = Time::singleton().get_ticks_msec();

        let mut result = 0.0;

        // for i in 0..iterations {
        //     let x = i as f32;
        //     let v = Vector3::new(x.sin(), x.cos(), x.tan());
        //     let n = v.normalized();
        //     result += n.length();
        // }
        //Pure Rust Math Implementation without using Godot's Vector3 or math functions
        for i in 0..iterations {
            let x = i as f32;
            
            let vx = x.sin();
            let vy = x.cos();
            let vz = x.tan();
            
            let length = (vx*vx + vy*vy + vz*vz).sqrt();
            
            if length > 0.0 {
                let nx = vx / length;
                let ny = vy / length;
                let nz = vz / length;
                result += (nx*nx + ny*ny + nz*nz).sqrt();
            }
        }

        let end = Time::singleton().get_ticks_msec();
        let duration = end - start;

        let text = format!(
            "Language: Rust\nTime: {} ms\nResult: {}",
            duration, result
        );

        self.ui_label.set_text(&text);
    }
}