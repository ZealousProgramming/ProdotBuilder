use gdnative::api::{ImmediateGeometry, MeshInstance};
use gdnative::prelude::*;

//use crate::prodot_utils::*;

#[derive(NativeClass)]
#[inherit(MeshInstance)]
pub struct ProdotMesh {
    imm_geo: Option<Ref<ImmediateGeometry, Shared>>,
}

#[methods]
impl ProdotMesh {
    pub fn new(_owner: TRef<MeshInstance>) -> Self {
        ProdotMesh {
            imm_geo: None,
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
        /*let geo = unsafe { self.imm_geo.unwrap().assume_safe() };
        geo.clear();
        
        geo.begin(Mesh::PRIMITIVE_TRIANGLES, Null::null());
        let tr_vertex = Vector3::new(1.0, 1.0, 0.0);
        let half_height = 0.1;
        let length = 1.0;
        //geo.add_sphere(20, 20, 0.1, false);
    // Right
        //bl
        geo.add_vertex(tr_vertex + Vector3::new(0.0, -half_height, 0.0));

        //tl
        geo.add_vertex(tr_vertex + Vector3::new(0.0, half_height, 0.0));

        //tr
        geo.add_vertex(tr_vertex + Vector3::new(length, half_height, 0.0));

        //tr
        geo.add_vertex(tr_vertex + Vector3::new(length, half_height, 0.0));

        //br
        geo.add_vertex(tr_vertex + Vector3::new(length, -half_height, 0.0));
        //bl
        geo.add_vertex(tr_vertex + Vector3::new(0.0, -half_height, 0.0));

        geo.end();*/
    }
}
