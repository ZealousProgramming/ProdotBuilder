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
    fn _process(&mut self, _owner: TRef<MeshInstance>, _delta: f64) {
        if self.vertices.len() > 0 {
            let geo = unsafe { self.imm_geo.unwrap().assume_safe() };
            geo.clear();
            
            geo.begin(Mesh::PRIMITIVE_TRIANGLES, Null::null());
            let half_height = 0.05;
            let half_depth = 0.05;
            let half_length = 0.05;
            
            for i in 0..self.vertices.len() {
                let vertex = self.vertices.get(i);
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
