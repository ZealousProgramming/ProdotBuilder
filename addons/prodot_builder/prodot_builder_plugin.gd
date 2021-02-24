tool
extends EditorPlugin
class_name ProdotBuilderPlugin

# -----------------------------------
# 		- Signals -
# -----------------------------------

# -----------------------------------
#		- Variables
# -----------------------------------

# Cache the ProdotMesh Resource
const prodot_mesh_resource = preload("res://addons/prodot_builder/prodot_mesh.tscn")

# Store the main Prodot Builder dock
var dock: Control

# Currently selected ProdotMesh node
var selected_mesh: ProdotMesh;

# Store the dock's buttons
var object_button: Button
var vertex_button: Button
var face_button: Button
var edge_button: Button
var create_cube_button: Button

enum BuildMode { BMObject, BMVertex, BMFace, BMEdge }

### NOTE (devon): Enums cannot be used as types here
# Stores the current build mode the plugin is in 
var build_mode

## NOTE (devon): Move to a state 
# Stores the indicators location of the vertices
var vertex_indicators = []
# -----------------------------------
# 		- Virtual Overrides	-
# -----------------------------------
func _enter_tree() -> void:
	set_force_draw_over_forwarding_enabled()
	set_input_event_forwarding_always_enabled()
	build_mode = BuildMode.BMObject
	create_dock()

func _exit_tree() -> void:
	free_dock()

func enable_plugin() -> void:
	print("[Prodot Builder]: Enabled")
	reset_button_states()

func disable_plugin() -> void:
	print("[Prodot Builder]: Disabled")

# Allows us to edit a specific type of resource/node in the editor
## NOTE (devon): Required to use forward_canvas_draw_over_viewport!
func handles(object: Object) -> bool:
	if object is ProdotMesh:
		return true
	# If it is not a ProdotMesh, we need to clear our screen of indicators
	if selected_mesh != null:
		selected_mesh = null
	update_overlays()
	return false

# Used to edit a specific object
func edit(object: Object) -> void:
	print("[Prodot Builder]: Selected Mesh[%s]" % object.name)
	selected_mesh = object

# Allows for drawing on the viewport
func forward_spatial_draw_over_viewport(overlay: Control) -> void:
	if selected_mesh != null:
		for vertex in vertex_indicators:
			overlay.draw_circle(vertex, 6, Color.red)

# Gets the input events inside the editor viewport
func forward_spatial_gui_input(camera: Camera, event: InputEvent) -> bool:
	if selected_mesh != null:
		if event is InputEventMouseMotion:
			var mesh_pos = selected_mesh.global_transform.origin
			if vertex_indicators.size() == 0:
				vertex_indicators.resize(selected_mesh.vertex_data.vertices.size())
			for i in range(0, selected_mesh.vertex_data.vertices.size()):
				var vertex_pos = selected_mesh.vertex_data.vertices[i];
				vertex_indicators[i] = camera.unproject_position(mesh_pos + vertex_pos)
			update_overlays()
			return true
	return false

func _process(delta) -> void:
	
	if selected_mesh != null:
		if get_editor_interface().get_selection().get_selected_nodes().size() == 0:
			print("[Prodot Builder]: Clear Selection")
			selected_mesh = null
			# Reset hover index
			update_overlays()

# ------------------------------------
# 		- Functionality -
# ------------------------------------

# Creates a 1x1x1 ProdotMesh
func create_cube() -> void:
	print("[Prodot Builder]: Created cube!")
	
	var mesh_arrays = []
	var vertex_data = VertexData.new()
	
	var normals: PoolVector3Array = []
	var uvs: PoolVector2Array = []
	var vertices: PoolVector3Array = []
	var indices: PoolIntArray = []
	
	
	mesh_arrays.resize(Mesh.ARRAY_MAX)
	normals.resize(4)
	uvs.resize(4)
	vertices.resize(4)
	indices.resize(6)

	normals.set(0, Vector3(0, 0, 1))
	uvs.set(0, Vector2(0, 0))
	vertices.set(0, Vector3(-1, -1, 0))
	
	normals.set(1, Vector3(0, 0, 1))
	uvs.set(1, Vector2(0,1))
	vertices.set(1, Vector3(-1, 1, 0))

	normals.set(2, Vector3(0, 0, 1))
	uvs.set(2, Vector2(1, 1))
	vertices.set(2, Vector3(1, 1, 0))

	normals.set(3, Vector3(0, 0, 1))
	uvs.set(3, Vector2(1, 0))
	vertices.set(3, Vector3(1, -1, 0))
	
	indices.set(0, 0)
	indices.set(1, 1)
	indices.set(2, 2)
	
	indices.set(3, 2)
	indices.set(4, 3)
	indices.set(5, 0)
	
	mesh_arrays[Mesh.ARRAY_VERTEX] = vertices
	mesh_arrays[Mesh.ARRAY_NORMAL] = normals
	mesh_arrays[Mesh.ARRAY_TEX_UV] = uvs
	mesh_arrays[Mesh.ARRAY_INDEX] = indices
	
	vertex_data.set_data(normals, uvs, vertices, indices)
	
	var mesh = ArrayMesh.new()
	
	mesh.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, mesh_arrays)
	
	var prodot_mesh = prodot_mesh_resource.instance()
	prodot_mesh.set_mesh(mesh)
	prodot_mesh.set_vertex_data(vertex_data)
	
	var scene_root_node = get_editor_interface().get_edited_scene_root()
	
	scene_root_node.add_child(prodot_mesh, false)
	prodot_mesh.set_owner(scene_root_node)
	
	vertex_indicators.resize(vertices.size())

### NOTE (devon): Enums cannot be used as types here
# Changes the plugins build mode to the passed mode
func change_build_mode(mode: String) -> void:
	print("[Prodot Builder]: " + mode)
	match mode:
		"Object":
			build_mode = BuildMode.BMObject
		"Vertex":
			build_mode = BuildMode.BMVertex
		"Face":
			build_mode = BuildMode.BMFace
		"Edge":
			build_mode = BuildMode.BMEdge

# ------------------------------------
#		- Helpers -
# ------------------------------------

# Creates and attaches the dock to the editor
func create_dock() -> void:
	if dock != null:
		return
	dock = preload("res://addons/prodot_builder/prodot_dock.tscn").instance()
	add_control_to_dock(DOCK_SLOT_RIGHT_BL, dock)

	# Cache the buttons
	object_button = dock.get_node("./DockVC/ModeVC/RowOne/Object")
	vertex_button = dock.get_node("./DockVC/ModeVC/RowOne/Vertex")
	face_button = dock.get_node("./DockVC/ModeVC/RowTwo/Face")
	edge_button = dock.get_node("./DockVC/ModeVC/RowTwo/Edge")
	create_cube_button = dock.get_node("./DockVC/CreateCube")

	# Setup signals
	object_button.connect("object_mode", self, "change_build_mode")
	vertex_button.connect("vertex_mode", self, "change_build_mode")
	face_button.connect("face_mode", self, "change_build_mode")
	edge_button.connect("edge_mode", self, "change_build_mode")
	create_cube_button.connect("create_cube", self, "create_cube")

# Adds the custom mesh type so that it is visible to the engine
func create_mesh_type() -> void:
	add_custom_type("ProdotMesh", "MeshInstance", preload("res://addons/prodot_builder/prodot_mesh.gd"), preload("res://textures/mesh_icon_v5.png"))

# Required clean up for the dock control	
func free_dock() -> void:
	if dock != null:
		remove_control_from_docks(dock)
		dock.free()		

# Required clean up for the custom mesh type
func free_mesh_type() -> void:	
	remove_custom_type("ProdotMesh")

# Resets the toggle states of the dock's buttons	
func reset_button_states() -> void:
	if object_button != null:
		object_button.set_pressed(false)
		
	if vertex_button != null:
		vertex_button.set_pressed(false)
		
	if face_button != null:
		face_button.set_pressed(false)
		
	if edge_button != null:
		edge_button.set_pressed(false)

