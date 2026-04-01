extends Node2D

const PLAYER = preload("res://scenes/player/player_gd.tscn")
@onready var label: Label = $CanvasLayer/Label

@export var spawn_batch: int = 10

var spawned_count: int = 0
var random_seed: int = 12345
var time_passed: float = 0.0

var spawned_players: Array[Node2D] = []
var start_positions: Array[Vector2] = []

func _ready() -> void:
	pass

func _input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.is_pressed() and event.button_index == MOUSE_BUTTON_LEFT:
			for i in range(spawn_batch):
				var random_position = get_random_position_in_window()
				spawn_player(random_position)

func _process(delta: float) -> void:
	time_passed += delta
	var time: float = time_passed

	for i in range(spawned_players.size()):
		var player: Node2D = spawned_players[i]
		var start_pos: Vector2 = start_positions[i]

		var dummy_load: float = 0.0
		for j in range(50):
			dummy_load += sin(float(j)) * cos(time)

		var wave_speed: float = 3.0
		var wave_height: float = 30.0
		var offset: float = float(i) * 0.1

		var new_y: float = start_pos.y + sin(time * wave_speed + offset) * wave_height + (dummy_load * 0.0001)
		var new_x: float = start_pos.x + cos(time * wave_speed * 0.5 + offset) * (wave_height * 0.5)

		player.position = Vector2(new_x, new_y)
		
	label.text = "GD\nFPS: %d\nSpawn Count: %d" % [Engine.get_frames_per_second(), spawned_count]

func spawn_player(pos: Vector2) -> void:
	var new_player: Node2D = PLAYER.instantiate() as Node2D
	new_player.position = pos

	spawned_players.append(new_player)
	start_positions.append(pos)
	spawned_count += 1

	add_child(new_player)

func get_random_position_in_window() -> Vector2:
	var viewport = get_viewport()
	var window_size = viewport.get_visible_rect().size
	
	random_seed = (random_seed * 1103515245 + 12345) & 0xFFFFFFFF
	var random_x = float(random_seed % 1000000) / 1000000.0

	random_seed = (random_seed * 1103515245 + 12345) & 0xFFFFFFFF
	var random_y = float(random_seed % 1000000) / 1000000.0

	return Vector2(random_x * window_size.x, random_y * window_size.y)

func get_spawned_count() -> int:
	return spawned_count
