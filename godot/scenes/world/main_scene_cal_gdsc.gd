extends Node

@onready var ui_label = $CanvasLayer/Label

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

	var end = Time.get_ticks_msec()
	var duration = end - start

	ui_label.text = "Language: GDScript\nTime: %d ms\nResult: %f" % [duration, result]
