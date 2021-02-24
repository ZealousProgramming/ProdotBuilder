tool
extends Button
class_name CreateCubeButton

signal create_cube

func _enter_tree() -> void:
	connect("pressed", self, "on_click")

func on_click() -> void:
	emit_signal("create_cube")
