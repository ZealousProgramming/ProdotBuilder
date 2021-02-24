tool
extends Button
class_name ObjectModeButton

signal object_mode(mode)

func _enter_tree() -> void:
	pass

func _on_Object_toggled(button_pressed: bool) -> void:
	if button_pressed:
		emit_signal("object_mode", "Object")
