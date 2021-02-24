extends Object
class_name VertexData

var vertices: PoolVector3Array = []
var normals: PoolVector3Array = []
var uvs: PoolVector2Array = []
var indices: PoolIntArray = []

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass
	#vertices = []
	#normals = []
	#uvs = []
	#indices = []

func set_data(normals: PoolVector3Array, uvs: PoolVector2Array, vertices: PoolVector3Array, indices: PoolIntArray) -> void:
	self.normals = normals
	self.uvs = uvs
	self.vertices = vertices
	self.indices = indices
	
