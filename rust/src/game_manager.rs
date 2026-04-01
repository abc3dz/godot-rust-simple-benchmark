use godot::classes::{Engine, INode2D, InputEvent, InputEventMouseButton, Label, Node, Node2D};
use godot::global::MouseButton;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct GameManager {
    base: Base<Node2D>,
    player_scene: Gd<PackedScene>,

    #[export]
    spawn_batch: u32,
    spawned_count: u32,
    random_seed: u64,

    time_passed: f64,

    spawned_players: Vec<Gd<Node2D>>,
    start_positions: Vec<Vector2>,
    ui_label: Option<Gd<Label>>,
}

#[godot_api]
impl INode2D for GameManager {
    fn init(base: Base<Node2D>) -> Self {
        let player_scene = load::<PackedScene>("res://scenes/player/player_rust.tscn");

        GameManager {
            base,
            player_scene,
            spawned_count: 0,
            spawn_batch: 10,
            random_seed: 12345,
            time_passed: 0.0,
            spawned_players: Vec::new(),
            start_positions: Vec::new(),
            ui_label: None,
        }
    }

    fn ready(&mut self) {
        let label = self.base().get_node_as::<Label>("CanvasLayer/Label");
        self.ui_label = Some(label);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Ok(mouse_event) = event.try_cast::<InputEventMouseButton>() {
            if mouse_event.is_pressed() && mouse_event.get_button_index() == MouseButton::LEFT {
                for _ in 0..self.spawn_batch {
                    let random_position = self.get_random_position_in_window();
                    self.spawn_player(random_position);
                }
            }
        }
    }

    fn process(&mut self, delta: f64) {
        self.time_passed += delta;
        let time = self.time_passed as f32;

        for i in 0..self.spawned_players.len() {
            let mut player = self.spawned_players[i].clone();
            let start_pos = self.start_positions[i];

            let mut dummy_load = 0.0;
            for j in 0..50 {
                dummy_load += f32::sin(j as f32) * f32::cos(time);
            }

            let wave_speed = 3.0;
            let wave_height = 30.0;
            let offset = i as f32 * 0.1;

            let new_y = start_pos.y
                + f32::sin(time * wave_speed + offset) * wave_height
                + (dummy_load * 0.0001);
            let new_x =
                start_pos.x + f32::cos(time * wave_speed * 0.5 + offset) * (wave_height * 0.5);

            player.set_position(Vector2::new(new_x, new_y));
        }

        if let Some(mut label) = self.ui_label.clone() {
            let fps = Engine::singleton().get_frames_per_second();

            let text = format!("Rust\nFPS: {}\nSpawn Count: {}", fps, self.spawned_count);
            label.set_text(&text);
        }
    }
}

#[godot_api]
impl GameManager {
    #[func]
    fn spawn_player(&mut self, position: Vector2) {
        let mut new_player = self.player_scene.instantiate_as::<Node2D>();
        new_player.set_position(position);

        self.spawned_players.push(new_player.clone());
        self.start_positions.push(position);
        self.spawned_count += 1;

        self.base_mut().add_child(&new_player.upcast::<Node>());
    }

    fn get_random_position_in_window(&mut self) -> Vector2 {
        let viewport = self.base().get_viewport().unwrap();
        let window_size = viewport.get_visible_rect().size;

        self.random_seed = self
            .random_seed
            .wrapping_mul(1103515245)
            .wrapping_add(12345);
        let random_x = (self.random_seed % 1000000) as f32 / 1000000.0;

        self.random_seed = self
            .random_seed
            .wrapping_mul(1103515245)
            .wrapping_add(12345);
        let random_y = (self.random_seed % 1000000) as f32 / 1000000.0;

        Vector2::new(random_x * window_size.x, random_y * window_size.y)
    }

    #[func]
    pub fn get_spawned_count(&self) -> u32 {
        self.spawned_count
    }
}
