extends Node

@export var box_scene: PackedScene

@onready var spawn_timer: Timer = $SpawnTimer
@onready var ui_label: Label = $CanvasLayer/Label

var spawned_count: int = 0


func _ready():
	spawned_count = 0


# ถ้าจะใช้ input แบบเดิม
# func _unhandled_input(event):
#     if event.is_action_pressed("ui_accept"):
#         var box3d = box_scene.instantiate()
#         box3d.position = Vector3(
#             randi_range(-5, 5),
#             randi_range(3, 5),
#             randi_range(-5, 5)
#         )
#         add_child(box3d)


func spawn_box():
	var width = 10
	var height = 8
	var spacing = 1.0

	for y in range(height):
		for x in range(-5, width):
			
			var box3d = box_scene.instantiate()

			spawned_count += 80

			var pos = Vector3(
				x * spacing,
				y * spacing,
				0.0
			)

			box3d.position = pos

			add_child(box3d)

	print("Spawned %s boxes!" % spawned_count)

	var fps = Engine.get_frames_per_second()
	var text = "Gdscript\nFPS: %s\nSpawn Count: %s" % [fps, spawned_count]
	ui_label.text = text


func _on_spawn_timer_timeout():
	spawn_box()
