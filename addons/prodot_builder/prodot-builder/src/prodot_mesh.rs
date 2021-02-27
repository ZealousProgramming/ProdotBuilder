use gdnative::api::{Mesh, ImmediateGeometry, MeshInstance};
use gdnative::prelude::*;

//use crate::prodot_utils::*;

#[derive(NativeClass)]
#[inherit(MeshInstance)]
pub struct ProdotMesh {
    imm_geo: Option<Ref<ImmediateGeometry, Shared>>,
    vertices: TypedArray::<Vector3>,
    uvs: TypedArray::<Vector2>,
    normals: TypedArray::<Vector3>,
    indices: TypedArray::<i32>,
    normal_color: Color,
    hover_color: Color,
    selected_color: Color,
    handle_x_color: Color,
    handle_y_color: Color,
    handle_z_color: Color,
    handle_dist: f32,
}

#[methods]
impl ProdotMesh {
    pub fn new(_owner: TRef<MeshInstance>) -> Self {
        ProdotMesh {
            imm_geo: None,
            vertices: TypedArray::<Vector3>::new(),
            uvs: TypedArray::<Vector2>::new(),
            normals: TypedArray::<Vector3>::new(),
            indices: TypedArray::<i32>::new(),
            normal_color: Color::rgba(0.2, 0.2, 0.2, 0.9),
            hover_color: Color::rgba(0.5, 0.5, 0.5, 0.9),
            selected_color: Color::rgba(0.05, 0.05, 0.05, 0.9),
            handle_x_color: Color::rgba(0.98039, 0.60784, 0.60784, 1.0),
            handle_y_color: Color::rgba(0.64706, 0.93725, 0.67451, 1.0),
            handle_z_color: Color::rgba(0.57255, 0.63529, 0.84314, 1.0),
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
    pub fn draw_vertices(&mut self, _owner: TRef<MeshInstance>, selected_index: i32, hover_index: i32) {
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
                let depth = half_depth * 2.0;
                geo.set_color(self.handle_x_color);
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
                gizmo_dist = Vector3::new(0.0, self.handle_dist, 0.0);
                geo.set_color(self.handle_y_color);

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
                geo.set_color(self.handle_z_color);

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

}
