use gdnative::prelude::*;
use gdnative::api::{
    Position3D,
    StaticBody,
    EditorSpatialGizmo,
};


// EditorSpatialGizmoPlugin is automatically managed through reference counting. 
// Inherits Resource
#[derive(NativeClass)]
#[inherit(Position3D)]
pub struct ProdotGizmo {
    x_handle: Option<Ref<StaticBody, Shared>>,
    y_handle: Option<Ref<StaticBody, Shared>>,
    z_handle: Option<Ref<StaticBody, Shared>>,
    
}

#[methods]
impl ProdotGizmo {
    fn new(_owner: TRef<Position3D>) -> Self {
        ProdotGizmo {
            x_handle: None,
            y_handle: None,
            z_handle: None,
        }
    }

    #[export]
    fn _enter_tree(&mut self, owner: TRef<Position3D>) {
        godot_print!("Gizmo is hurrrr!");
        
        self.x_handle = unsafe {
            Some(
                owner
                    .get_node("./X")
                    .unwrap()
                    .assume_safe()
                    .cast::<StaticBody>()
                    .unwrap()
                    .claim()
            )
        };
        
        self.y_handle = unsafe {
            Some(
                owner
                    .get_node("./Y")
                    .unwrap()
                    .assume_safe()
                    .cast::<StaticBody>()
                    .unwrap()
                    .claim()
            )
        };

        self.z_handle = unsafe {
            Some(
                owner
                    .get_node("./Z")
                    .unwrap()
                    .assume_safe()
                    .cast::<StaticBody>()
                    .unwrap()
                    .claim()
            )
        };

    }

    #[export]
    fn to_string(&self, _owner: TRef<Position3D>) -> GodotString {
        GodotString::from("ProdotGizmo")
    }

    #[export]
    pub fn hide_gizmo(&mut self, owner: TRef<Position3D>) {
        unsafe {
            owner
                .gizmo()
                .unwrap()
                .assume_safe()
                .cast::<EditorSpatialGizmo>()
                .unwrap()
                .set_hidden(true)
        };
    }
}
