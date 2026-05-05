extends Node

@export var box_scene: PackedScene

@onready var spawn_timer: Timer = $SpawnTimer
@onready var ui_label: Label = $CanvasLayer/Label

var spawned_count: int = 0

func _ready():
	spawned_count = 0
	spawn_box()


func spawn_box():
	var spawn_amount = 5000

	if box_scene == null:
		push_error("Box scene not assigned!")
		return

	for i in range(spawn_amount):
		var box_instance = box_scene.instantiate()

		var pos = Vector3(
			randi_range(-20, 20),
			randi_range(10, 30),
			randi_range(-20, 20)
		)

		box_instance.position = pos

		add_child(box_instance)

		spawned_count += 1

	# อัปเดต UI
	var fps = Engine.get_frames_per_second()
	var text = "Language: GDScript\nFPS: %d\nObjects: %d" % [fps, spawned_count]
	ui_label.text = text


func _on_spawn_timer_timeout():
	spawn_box()
