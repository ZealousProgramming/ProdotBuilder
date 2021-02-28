use gdnative::api::{ArrayMesh, Mesh, MeshDataTool, ImmediateGeometry, MeshInstance};
use gdnative::prelude::*;

//use crate::prodot_utils::*;

#[derive(NativeClass)]
#[inherit(MeshInstance)]
pub struct ProdotMesh {
    imm_geo: Option<Ref<ImmediateGeometry, Shared>>,
    vertices: TypedArray::<Vector3>,
    _uvs: TypedArray::<Vector2>,
    _normals: TypedArray::<Vector3>,
    _indices: TypedArray::<i32>,
    // Holds the indices of the 3 vertices that make up a face
    faces: TypedArray::<Vector3>,
    normal_color: Color,
    hover_color: Color,
    selected_color: Color,
    handle_x_color: Color,
    handle_x_color_hover: Color,
    handle_y_color: Color,
    handle_y_color_hover: Color,
    handle_z_color: Color,
    handle_z_color_hover: Color,
    handle_dist: f32,
}

#[methods]
impl ProdotMesh {
    pub fn new(_owner: TRef<MeshInstance>) -> Self {
        ProdotMesh {
            imm_geo: None,
            vertices: TypedArray::<Vector3>::new(),
            _uvs: TypedArray::<Vector2>::new(),
            _normals: TypedArray::<Vector3>::new(),
            _indices: TypedArray::<i32>::new(),
            faces: TypedArray::<Vector3>::new(),
            normal_color: Color::rgba(0.4, 0.4, 0.4, 0.8),
            hover_color: Color::rgba(0.7, 0.7, 0.7, 1.0),
            selected_color: Color::rgba(0.2, 0.2, 0.2, 1.0),
            //handle_x_color: Color::rgba(0.98039, 0.60784, 0.60784, 0.7),
            handle_x_color: Color::rgba(0.96078, 0.2, 0.31765, 0.7),
            //handle_x_color_hover: Color::rgba(0.98039, 0.60784, 0.60784, 1.0),
            handle_x_color_hover: Color::rgba(0.96078, 0.2, 0.31765, 1.0),
            //handle_y_color: Color::rgba(0.64706, 0.93725, 0.67451, 0.7),
            handle_y_color: Color::rgba(0.52941, 0.83922, 0.00784, 0.7),
            //handle_y_color_hover: Color::rgba(0.64706, 0.93725, 0.67451, 1.0),
            handle_y_color_hover: Color::rgba(0.52941, 0.83922, 0.00784, 1.0),
            //handle_z_color: Color::rgba(0.57255, 0.63529, 0.84314, 0.7),
            handle_z_color: Color::rgba(0.16078, 0.54902, 0.96078, 0.7),
            //handle_z_color_hover: Color::rgba(0.57255, 0.63529, 0.84314, 1.0),
            handle_z_color_hover: Color::rgba(0.16078, 0.54902, 0.96078, 1.0),
            handle_dist: 0.15,
        }
    }

    #[export]
    fn _enter_tree(&mut self, owner: TRef<MeshInstance>) {
        godot_print!("Prodot Mesh created!");
        
        self.imm_geo = unsafe {
            Some( 
                owner.
                    get_node("./ProdotIG")
                    .unwrap()
                    .assume_safe()
                    .cast::<ImmediateGeometry>()
                    .unwrap()
                    .claim()
            )
        };



    }

    #[export]
    fn _exit_tree(&self, _owner: TRef<MeshInstance>) {}
    
    #[export]
    pub fn clear(&mut self, _owner: TRef<MeshInstance>) {
        if let Some(geo_ref) = self.imm_geo {
            let geo = unsafe { geo_ref.assume_safe() };
            geo.clear();
        }
    }

    #[export]
    pub fn draw_vertices(&mut self, _owner: TRef<MeshInstance>, selected_index: i32, hover_index: i32, hovering_gizmo_axis: Vector3) {
        if self.vertices.len() > 0 {
            let geo = unsafe { self.imm_geo.unwrap().assume_safe() };
            geo.clear();
            
            geo.begin(Mesh::PRIMITIVE_TRIANGLES, Null::null());
            let half_height = 0.05;
            let half_depth = 0.05;
            let half_length = 0.05;

            // Standard non-selected non-hovered
            geo.set_color(self.normal_color);

            for i in 0..self.vertices.len() {
                let vertex = self.vertices.get(i);
                if selected_index == i {
                    geo.set_color(self.selected_color);
                }else if hover_index == i {
                    geo.set_color(self.hover_color);
                }else {
                    geo.set_color(self.normal_color);
                }

                // Front Face 
                //bl
                geo.add_vertex(vertex + Vector3::new(-half_length, -half_height, half_depth));
                //tl
                geo.add_vertex(vertex + Vector3::new(-half_length, half_height, half_depth));

                //tr
                geo.add_vertex(vertex + Vector3::new(half_length, half_height, half_depth));

                //tr
                geo.add_vertex(vertex + Vector3::new(half_length, half_height, half_depth));

                //br
                geo.add_vertex(vertex + Vector3::new(half_length, -half_height, half_depth));
                //bl
                geo.add_vertex(vertex + Vector3::new(-half_length, -half_height, half_depth));
                
                // Back Face 
                //bl
                geo.add_vertex(vertex + Vector3::new(-half_length, -half_height, -half_depth));

                //tl
                geo.add_vertex(vertex + Vector3::new(-half_length, half_height, -half_depth));

                //tr
                geo.add_vertex(vertex + Vector3::new(half_length, half_height, -half_depth));

                //tr
                geo.add_vertex(vertex + Vector3::new(half_length, half_height, -half_depth));

                //br
                geo.add_vertex(vertex + Vector3::new(half_length, -half_height, -half_depth));
                //bl
                geo.add_vertex(vertex + Vector3::new(-half_length, -half_height, -half_depth));

                // Left Side
                //bl
                geo.add_vertex(vertex + Vector3::new( -half_length, -half_height, -half_depth));
                //tl
                geo.add_vertex(vertex + Vector3::new( -half_length, half_height, -half_depth));
                //tr
                geo.add_vertex(vertex + Vector3::new( -half_length, half_height, half_depth));
                //tr
                geo.add_vertex(vertex + Vector3::new( -half_length, half_height, half_depth));
                //br
                geo.add_vertex(vertex + Vector3::new( -half_length, -half_height, half_depth));
                //bl
                geo.add_vertex(vertex + Vector3::new( -half_length, -half_height, -half_depth));
   
                // Right Side
                //bl
                geo.add_vertex(vertex + Vector3::new( half_length, -half_height, -half_depth));
                //tl
                geo.add_vertex(vertex + Vector3::new( half_length, half_height, -half_depth));
                //tr
                geo.add_vertex(vertex + Vector3::new( half_length, half_height, half_depth));
                //tr
                geo.add_vertex(vertex + Vector3::new( half_length, half_height, half_depth));
                //br
                geo.add_vertex(vertex + Vector3::new( half_length, -half_height, half_depth));
                //bl
                geo.add_vertex(vertex + Vector3::new( half_length, -half_height, -half_depth));

                // Top Side
                //bl
                geo.add_vertex(vertex + Vector3::new( -half_length, half_height, half_depth));
                //tl
                geo.add_vertex(vertex + Vector3::new( -half_length, half_height, -half_depth));
                //tr
                geo.add_vertex(vertex + Vector3::new( half_length, half_height, -half_depth));
                //tr
                geo.add_vertex(vertex + Vector3::new( half_length, half_height, -half_depth));
                //br
                geo.add_vertex(vertex + Vector3::new( half_length, half_height, half_depth));
                //bl
                geo.add_vertex(vertex + Vector3::new( -half_length, half_height, half_depth));
                
                // Bottom Side
                //bl
                geo.add_vertex(vertex + Vector3::new( -half_length, -half_height, half_depth));
                //tl
                geo.add_vertex(vertex + Vector3::new( -half_length, -half_height, -half_depth));
                //tr
                geo.add_vertex(vertex + Vector3::new( half_length, -half_height, -half_depth));
                //tr
                geo.add_vertex(vertex + Vector3::new( half_length, -half_height, -half_depth));
                //br
                geo.add_vertex(vertex + Vector3::new( half_length, -half_height, half_depth));
                //bl
                geo.add_vertex(vertex + Vector3::new( -half_length, -half_height, half_depth));
            }

            if selected_index != -1 {
                // Draw Gizmo

                // X plane
                let mut gizmo_dist: Vector3 = Vector3::new(self.handle_dist, 0.0, 0.0);
                let vertex: Vector3 = self.vertices.get(selected_index);
                if hovering_gizmo_axis == Vector3::new(1.0, 0.0, 0.0) {
                    geo.set_color(self.handle_x_color_hover);
                } else {
                    geo.set_color(self.handle_x_color);
                }
                //bl
                geo.add_vertex(vertex + gizmo_dist + Vector3::new(-half_length, -half_height, 0.0));
                //tl
                geo.add_vertex(vertex + gizmo_dist + Vector3::new(-half_length, half_height, 0.0));

                //tr
                geo.add_vertex(vertex + gizmo_dist + Vector3::new(half_length, half_height, 0.0));

                //tr
                geo.add_vertex(vertex + gizmo_dist + Vector3::new(half_length, half_height, 0.0));

                //br
                geo.add_vertex(vertex + gizmo_dist + Vector3::new(half_length, -half_height, 0.0));
                //bl
                geo.add_vertex(vertex + gizmo_dist + Vector3::new(-half_length, -half_height, 0.0));

                // Y plane
                if hovering_gizmo_axis == Vector3::new(0.0, 1.0, 0.0) {
                    geo.set_color(self.handle_y_color_hover);
                } else {
                    geo.set_color(self.handle_y_color);
                }
                gizmo_dist = Vector3::new(0.0, self.handle_dist, 0.0);

                //bl
                geo.add_vertex(vertex + gizmo_dist + Vector3::new(-half_length, -half_height, 0.0));
                //tl
                geo.add_vertex(vertex + gizmo_dist + Vector3::new(-half_length, half_height, 0.0));

                //tr
                geo.add_vertex(vertex + gizmo_dist + Vector3::new(half_length, half_height, 0.0));

                //tr
                geo.add_vertex(vertex + gizmo_dist + Vector3::new(half_length, half_height, 0.0));

                //br
                geo.add_vertex(vertex + gizmo_dist + Vector3::new(half_length, -half_height, 0.0));
                //bl
                geo.add_vertex(vertex + gizmo_dist + Vector3::new(-half_length, -half_height, 0.0));
                
                // Z plane
                gizmo_dist = Vector3::new(0.0, 0.0, self.handle_dist);
                if hovering_gizmo_axis == Vector3::new(0.0, 0.0, 1.0) {
                    geo.set_color(self.handle_z_color_hover);
                } else {
                    geo.set_color(self.handle_z_color);
                }

                //bl
                geo.add_vertex(vertex + gizmo_dist + Vector3::new(0.0, -half_height, half_depth));
                //tl
                geo.add_vertex(vertex + gizmo_dist + Vector3::new(0.0, half_height, half_depth)); 

                //tr
                geo.add_vertex(vertex + gizmo_dist + Vector3::new(0.0, half_height, -half_depth));

                //tr
                geo.add_vertex(vertex + gizmo_dist + Vector3::new(0.0, half_height, -half_depth));

                //br
                geo.add_vertex(vertex + gizmo_dist + Vector3::new(0.0, -half_height, -half_depth));
                //bl
                geo.add_vertex(vertex + gizmo_dist + Vector3::new(0.0, -half_height, half_depth));
            }
            geo.end();
        }

    }

    #[export]
    pub fn get_vertices(&mut self, _owner: TRef<MeshInstance>) -> TypedArray<Vector3> {
        self.vertices.clone()
    }

    #[export]
    pub fn set_vertices(&mut self, _owner: TRef<MeshInstance>, vertices: TypedArray<Vector3>) {
        self.vertices = vertices;
    }

    #[export]
    pub fn set_vertex(&mut self, owner: TRef<MeshInstance>, index: i32, position: Vector3) {
        self.vertices.set(index, position);
        self.update_mesh_vertex(owner, index, position);
    }

    #[export]
    fn update_mesh_vertex(&mut self, owner: TRef<MeshInstance>, index: i32, position: Vector3) {
        let mesh_pos = owner.global_transform().origin;
        let mesh_ref = owner.mesh().unwrap();
        let mesh = unsafe { mesh_ref.assume_safe() };
        let mesh_array =  mesh.cast::<ArrayMesh>().unwrap();
        let mesh_tool = MeshDataTool::new();
        mesh_tool.create_from_surface(mesh_array, 0).expect("[Prodot Mesh]: Failed to create mesh from surface!");
        
        mesh_tool.set_vertex(index as i64, position);
        mesh_array.surface_remove(0);
        mesh_tool.commit_to_surface(mesh_array).expect("[Prodot Mesh]: Failed to commit mesh array to surface!");
    }


}
