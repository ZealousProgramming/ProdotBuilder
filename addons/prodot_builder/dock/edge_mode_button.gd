tool
extends Button
class_name EdgeModeButton

signal edge_mode(mode)

func _enter_tree() -> void:
	pass

func _on_Edge_toggled(button_pressed: bool) -> void:
	if button_pressed:
		emit_signal("edge_mode", "Edge")
