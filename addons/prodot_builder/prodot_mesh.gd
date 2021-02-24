tool
extends MeshInstance
class_name ProdotMesh, "res://textures/mesh_icon_v5.png"

var vertex_data: VertexData;

func _enter_tree() -> void:
	print("[Prodot Mesh]: Created")
	


func set_vertex_data(data: VertexData) -> void:
	vertex_data = data
