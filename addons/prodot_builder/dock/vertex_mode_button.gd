tool
extends Button
class_name VertexModeButton

signal vertex_mode(mode)

func _enter_tree() -> void:
	pass
	
func _on_Vertex_toggled(button_pressed: bool) -> void:
	if button_pressed:
		emit_signal("vertex_mode", "Vertex")
