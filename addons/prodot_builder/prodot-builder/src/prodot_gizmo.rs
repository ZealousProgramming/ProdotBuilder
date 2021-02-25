use gdnative::prelude::*;
use gdnative::api::EditorSpatialGizmoPlugin;


// EditorSpatialGizmoPlugin is automatically managed through reference counting. 
// Inherits Resource
#[derive(NativeClass)]
#[inherit(EditorSpatialGizmoPlugin)]
pub struct ProdotGizmo;

#[methods]
impl ProdotGizmo {
    fn new(_owner: TRef<EditorSpatialGizmoPlugin>) -> Self {
        ProdotGizmo
    }

    #[export]
    fn get_name(&self, _owner: TRef<EditorSpatialGizmoPlugin>) -> GodotString {
        return GodotString::from("Prodot Gizmo");
    }

    #[export]
    fn _enter_tree(&self, _owner: TRef<EditorSpatialGizmoPlugin>) {
        godot_print!("Gizmo is hurrrr!");
    }
}
