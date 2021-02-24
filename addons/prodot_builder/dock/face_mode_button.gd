tool
extends Button
class_name FaceModeButton

signal face_mode(mode)

func _enter_tree() -> void:
	pass

func _on_Face_toggled(button_pressed: bool) -> void:
	if button_pressed:
		emit_signal("face_mode", "Face")
