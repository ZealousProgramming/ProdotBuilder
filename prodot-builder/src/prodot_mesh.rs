use gdnative::prelude::*;
use gdnative::api::MeshInstance;

#[derive(NativeClass)]
#[inherit(MeshInstance)]
pub struct ProdotMesh;

#[methods]
impl ProdotMesh {
    pub fn new(_owner: TRef<MeshInstance>) -> Self {
        ProdotMesh
    }

    #[export]
    fn _enter_tree(&self, _owner: TRef<MeshInstance>) {
        godot_print!("Prodot Mesh created!");
    }

    #[export]
    fn _exit_tree(&self, _owner: TRef<MeshInstance>) {

    }
}
