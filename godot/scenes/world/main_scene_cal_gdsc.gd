extends Node

@onready var ui_label = $CanvasLayer/Label
var result_count = 0

func _ready():
	run_benchmark()

func run_benchmark():
	var iterations = 5000000
	var start = Time.get_ticks_msec()

	var result = 0.0

	for i in range(iterations):
		var x = float(i)
		var v = Vector3(sin(x), cos(x), tan(x))
		var n = v.normalized()
		result += n.length()
		result_count += result

	var end = Time.get_ticks_msec()
	var duration = end - start
	
	var fps = Engine.get_frames_per_second()

	ui_label.text = "Language: GDScript\nTime: %d ms\nResult: %f\nfps: %d" % [duration, result_count,fps]


func _on_event_time_timeout() -> void:
	run_benchmark()
