use gdnative::prelude::*;
use gdnative::api::{
    Object,
    EditorPlugin, 
    Script, 
    Texture, 
    Control, 
    Button, 
    Mesh,
    //ArrayMesh, 
    MeshInstance,
    SurfaceTool,
    Camera,
    InputEvent,
    InputEventMouseMotion,
    InputEventMouseButton,
};

use std::borrow::BorrowMut;

use crate::prodot_utils::*;
//use crate::prodot_mesh::ProdotMesh;
//use crate::dock::create_cube_button::CreateCubeButton;

#[derive(NativeClass)]
#[inherit(EditorPlugin)]
pub struct ProdotBuilderNode {
    dock: Option<Ref<Control, Shared>>,
    selected_node: Option<Ref<MeshInstance, Shared>>,
}

#[methods]
impl ProdotBuilderNode {
    fn new(_owner: TRef<EditorPlugin>) -> Self {
        ProdotBuilderNode {
            dock: None,
            selected_node: None,
        }
    }

    #[export]
    fn _enter_tree(&mut self, owner: TRef<EditorPlugin>) {
        // Initialization of the plugin
        owner.set_force_draw_over_forwarding_enabled();
        owner.set_input_event_forwarding_always_enabled();

        godot_print!("[Prodot Builder]: Enabled");

        self.dock = unsafe { 
            Some(
                load_scene("addons/prodot_builder/prodot_dock.tscn")
                .unwrap()
                .assume_safe()
                .instance(PackedScene::GEN_EDIT_STATE_INSTANCE)
                .unwrap()
                .assume_safe()
                .cast::<Control>()
                .unwrap()
                .claim()
            )
        };
        
        owner.add_control_to_dock(EditorPlugin::DOCK_SLOT_RIGHT_BL, self.dock.unwrap());

        let script = unsafe { load_resource::<Script>("res://addons/prodot_builder/prodot_mesh.gdns", "Script").unwrap() };
        let texture = unsafe { load_resource::<Texture>("res://icon.png", "Texture").unwrap() };
        owner.add_custom_type("ProdotMesh", "MeshInstance", script, texture);

        let create_cube_button = unsafe {
            self.
                dock
                .unwrap()
                .assume_safe()
                .get_node("./CreateCube")
                .unwrap()
                .assume_safe()
                .cast::<Button>()
                .unwrap()
                
        };
        
        // Signals
        let create_cube_result = 
            create_cube_button.connect (
                "create_cube",
                owner,
                "create_cube",
                VariantArray::new_shared(),
                0
            );
        match create_cube_result {
            Ok(_) => (),
            Err(_) => godot_print!("[Prodot Builder]: Error when connecting the create cube button!"),
        }

    }

    #[export]
    fn _exit_tree(&mut self, owner: TRef<EditorPlugin>) {
        
        // Clean up
        unsafe {
            match self.selected_node {
                Some(node) => {
                    node
                        .assume_safe()
                        .queue_free();
                },
                _ => (),
            }
        };

        //Remove it from the engine
        owner.remove_custom_type("ProdotMesh");
        
        // Remove the dock
        owner.remove_control_from_docks(self.dock.unwrap());

        // Free the control
        unsafe {
            self
                .dock
                .unwrap()
                .assume_safe()
                .queue_free()
        };
    }
    
    /// Requests the editor to edit the given object
    /// Sends the plugin the object that is being edited.
    ///
    #[export]
    fn edit(&mut self, _owner: TRef<EditorPlugin>, object: Ref<Object>) {
        godot_print!("WHA");
        /*match unsafe {object.assume_safe().cast::<MeshInstance>() } {
            Some(mesh) => {
                self.selected_node =  Some( mesh.claim() ) ;
                godot_print!("Selected Mesh: {:?}", self.selected_node.borrow_mut())
            },
            None => self.selected_node = None,
        }*/
    }

    #[export]
    fn handles(&self, _owner: TRef<EditorPlugin>, object: Ref<Object>) {
        godot_print!("Handle");
    }


    #[export]
    fn forward_spatial_force_draw_over_viewport(&self, _owner: TRef<EditorPlugin>, _overlay: Ref<Control>) {
        //let overlay_ref = unsafe{ overlay.assume_safe() };
        //overlay_ref.draw_circle(overlay_ref.get_local_mouse_position(), 32.0, Color::rgb(1.0, 0.0, 0.0));
        //overlay_ref.draw_circle(Vector2::new(50.0, 50.0), 128.0, Color::rgb(1.0, 0.0, 0.0));
        

        // get selected node
        /*let editor = EditorPlugin::get_editor_interface(owner.as_ref()).unwrap();
        let selection = unsafe { editor.assume_safe().get_selection().unwrap().assume_safe()};

        let selection_array: VariantArray = selection.get_transformable_selected_nodes();
        if selection_array.len() > 0 {
            //godot_print!("Selected: {:?}", selection_array);
            let variant = unsafe{ selection_array.get(0) };
            let obj =  unsafe{ variant.try_to_object::<Object>().unwrap().assume_safe() };
            let node_inst = obj.cast::<Node>().expect("Not a node");
            let mesh = node_inst.cast::<MeshInstance>().expect("Not a mesh");
            //mesh.translate(Vector3::new(1.0, 0.0, 0.0));
            let pos = Vector2::new(mesh.global_transform().origin.x, mesh.global_transform().origin.y);
            overlay_ref.draw_circle(pos, 32.0, Color::rgb(1.0, 0.0, 0.0));
        }*/
    }
    

    #[export]
    fn forward_spatial_gui_input(&self, _owner: TRef<EditorPlugin>, _camera: Ref<Camera>, _event: Ref<InputEvent>) -> bool {
        
        return false;
/*
        let event_two = event.clone();

        match event.cast::<InputEventMouseMotion>() {
            Some(_e) => {
                //return true
            },
            _ => (),
        }

        
        match event_two.cast::<InputEventMouseButton>() {
            Some(e) => {
                unsafe {
                    if e.assume_safe().button_index() == 1 {
                        owner.update_overlays();
                    }
                    
                    //godot_print!("Clicked: {:?}", e.assume_safe().button_index());
                }
            },
            _ => return false,
        }
*/
    }

    #[export]
    pub fn create_cube(&mut self, owner: TRef<EditorPlugin>) {
        godot_print!("Created cube!");

        let editor = EditorPlugin::get_editor_interface(owner.as_ref()).unwrap();
        let root_node = unsafe {
            editor
                .assume_safe()
                .get_edited_scene_root()
                .unwrap()
        };

        //let root_node = unsafe {godot_print!("Scene Root Name: {:?}", scene.assume_safe().name())};
        //let mut arrays: [i32; j];
        /*
        let arrays = VariantArray::new();
        let normal_array = VariantArray::new();
        let uv_array = VariantArray::new();
        let vertex_array = VariantArray::new();
        let index_array = VariantArray::new();

        unsafe {
            arrays.resize(Mesh::ARRAY_MAX as i32);
            normal_array.resize(3);
            uv_array.resize(3);
            vertex_array.resize(3);
            index_array.resize(3);
            
            // Point one
            normal_array.push(Vector3::new(0.0, 0.0, 1.0));
            uv_array.push(Vector2::new(0.0, 0.0));
            vertex_array.push(Vector3::new(-1.0, -1.0, 0.0));

            // Point two
            normal_array.push(Vector3::new(0.0, 0.0, 1.0));
            uv_array.push(Vector2::new(0.0, 1.0));
            vertex_array.push(Vector3::new(-1.0, 1.0, 0.0));

            // Point three
            normal_array.push(Vector3::new(0.0, 0.0, 1.0));
            uv_array.push(Vector2::new(1.0, 1.0));
            vertex_array.push(Vector3::new(1.0, 1.0, 0.0));

            index_array.push(0);
            index_array.push(1);
            index_array.push(2);

            arrays.insert(Mesh::ARRAY_VERTEX as i32, vertex_array);
            arrays.insert(Mesh::ARRAY_NORMAL as i32, normal_array);
            arrays.insert(Mesh::ARRAY_TEX_UV as i32, uv_array);
            arrays.insert(Mesh::ARRAY_INDEX as i32, index_array);

            let mesh_instance = MeshInstance::new().into_shared().assume_safe();
            root_node.assume_safe().add_child(mesh_instance, false);
            mesh_instance.set_owner(root_node.assume_safe());

            //let _mesh = ProdotMesh::new(mesh_instance);
                
            let end_mesh = ArrayMesh::new();
            end_mesh.add_surface_from_arrays(Mesh::PRIMITIVE_TRIANGLES, arrays.into_shared(), VariantArray::new_shared(), 97280);
            mesh_instance.set_mesh(end_mesh);
        }

        */

        let surface_tool = SurfaceTool::new();

        surface_tool.begin(Mesh::PRIMITIVE_TRIANGLES);
        
        // BL v0
        surface_tool.add_normal(Vector3::new(0.0, 0.0, 1.0));
        surface_tool.add_uv(Vector2::new(0.0, 0.0));
        surface_tool.add_vertex(Vector3::new(-1.0, -1.0, 0.0));
        
        // TL v1
        surface_tool.add_normal(Vector3::new(0.0, 0.0, 1.0));
        surface_tool.add_uv(Vector2::new(0.0, 1.0));
        surface_tool.add_vertex(Vector3::new(-1.0, 1.0, 0.0));
        
        // TR v2
        surface_tool.add_normal(Vector3::new(0.0, 0.0, 1.0));
        surface_tool.add_uv(Vector2::new(1.0, 1.0));
        surface_tool.add_vertex(Vector3::new(1.0, 1.0, 0.0));
        
        // TR v3
        surface_tool.add_normal(Vector3::new(0.0, 0.0, 1.0));
        surface_tool.add_uv(Vector2::new(1.0, 1.0));
        surface_tool.add_vertex(Vector3::new(1.0, 1.0, 0.0));

        // BR v4
        surface_tool.add_normal(Vector3::new(0.0, 0.0, 1.0));
        surface_tool.add_uv(Vector2::new(1.0, 0.0));
        surface_tool.add_vertex(Vector3::new(1.0, -1.0, 0.0));

        // BL v5
        surface_tool.add_normal(Vector3::new(0.0, 0.0, 1.0));
        surface_tool.add_uv(Vector2::new(0.0, 0.0));
        surface_tool.add_vertex(Vector3::new(-1.0, -1.0, 0.0));
        
        // Creates indices, indices are optional.
        surface_tool.index();

        unsafe {
            let mesh_instance = MeshInstance::new().into_shared().assume_safe();
            root_node.assume_safe().add_child(mesh_instance, false);
            mesh_instance.set_owner(root_node.assume_safe());

            //let _mesh = ProdotMesh::new(mesh_instance);
                
            mesh_instance.set_mesh(surface_tool.commit(GodotObject::null(), Mesh::ARRAY_COMPRESS_DEFAULT).unwrap());
        }
        
    }
}


