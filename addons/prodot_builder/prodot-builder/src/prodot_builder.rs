use gdnative::api::{
    Button,
    Camera,
    Control,
    EditorPlugin,
    InputEvent,
    InputEventMouseButton,
    InputEventMouseMotion,
    InputEventWithModifiers,
    Geometry,
    GlobalConstants,
    Mesh,
    ArrayMesh,
    MeshInstance,
    //MeshDataTool,
    Object,
    //Texture,
    //Script,
    //Spatial,
    PackedScene,
    //Viewport,
    //World,
};
use gdnative::prelude::*;
use crate::prodot_mesh::*;
use crate::prodot_utils::*;

use std::collections::HashMap;

//use std::borrow::Borrow;

#[derive(Copy, Clone, Debug, ToVariant, FromVariant)]
pub enum BuildMode {
    Object = 0,
    Vertex,
    Face,
    Edge
}

impl BuildMode {
    pub fn value(&self) -> i64 {
        match *self {
            BuildMode::Object => 0,
            BuildMode::Vertex => 1,
            BuildMode::Face => 2,
            BuildMode::Edge => 3,
        }
    }

    pub fn set(&self, value: i64) -> BuildMode {
        match value {
            0 => BuildMode::Object,
            1 => BuildMode::Vertex,
            2 => BuildMode::Face,
            3 => BuildMode::Edge,
            _ => {
                godot_print!("[Prodot Builder]: Cannot set BuildMode to an invalid value!");
                BuildMode::Vertex
            },
        }
    }
}

#[derive(NativeClass)]
#[inherit(EditorPlugin)]
pub struct ProdotBuilderPlugin {
    dock: Option<Ref<Control, Shared>>,
    mesh_scene: Option<Ref<PackedScene, Shared>>,
    selected_node: Option<Ref<MeshInstance, Shared>>,
    selected_indices: Vec::<i32>,
    vertices_drag_state: HashMap::<i32, Vector3>,
    hover_index: i32,
    hovering_gizmo_axis: Vector3,
    build_mode: BuildMode,
    object_mode_button: Option<Ref<Button, Shared>>,
    vertex_mode_button: Option<Ref<Button, Shared>>,
    face_mode_button: Option<Ref<Button, Shared>>,
    edge_mode_button: Option<Ref<Button, Shared>>,

    // flags
    is_dragging: bool,
}

#[methods]
impl ProdotBuilderPlugin {
    fn new(_owner: TRef<EditorPlugin>) -> Self {
        ProdotBuilderPlugin {
            dock: None,
            mesh_scene: None,
            selected_node: None,
            selected_indices: Vec::<i32>::new(),
            vertices_drag_state: HashMap::<i32, Vector3>::new(),
            hover_index: -1,
            hovering_gizmo_axis: Vector3::zero(),
            build_mode: BuildMode::Vertex,
            object_mode_button: None,
            vertex_mode_button: None,
            face_mode_button: None,
            edge_mode_button: None,

            is_dragging: false,
        }
    }

    #[export]
    fn _enter_tree(&mut self, owner: TRef<EditorPlugin>) {
        // Initialization of the plugin
        owner.set_force_draw_over_forwarding_enabled();
        owner.set_input_event_forwarding_always_enabled();
        
        godot_print!("[Prodot Builder]: Enabled");
        
    
        self.mesh_scene = unsafe {
            Some(
                load_scene("addons/prodot_builder/prodot_mesh.tscn")
                .unwrap()
            )
        };

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
                    .claim(),
            )
        };

        owner.add_control_to_dock(EditorPlugin::DOCK_SLOT_RIGHT_BL, self.dock.unwrap());
        
        /*
        let script = unsafe {
            load_resource::<Script>("res://addons/prodot_builder/prodot_mesh.gdns", "Script")
                .unwrap()
        };
        let texture = unsafe { load_resource::<Texture>("res://addons/prodot_builder/textures/mesh_icon_v5.png", "Texture").unwrap() };
        */
        //owner.add_custom_type("ProdotMesh", "MeshInstance", script, texture);

        let create_cube_button = unsafe {
            self.dock
                .unwrap()
                .assume_safe()
                .get_node("./DockVC/CreateCube")
                .unwrap()
                .assume_safe()
                .cast::<Button>()
                .unwrap()
        };

        let object_mode_button = unsafe {
            self.dock
                .unwrap()
                .assume_safe()
                .get_node("./DockVC/ModeVC/RowOne/Object")
                .unwrap()
                .assume_safe()
                .cast::<Button>()
                .unwrap()
        };

        object_mode_button.set_pressed(false);

        let vertex_mode_button = unsafe {
            self.dock
                .unwrap()
                .assume_safe()
                .get_node("./DockVC/ModeVC/RowOne/Vertex")
                .unwrap()
                .assume_safe()
                .cast::<Button>()
                .unwrap()
        };

        vertex_mode_button.set_pressed(true);

        let face_mode_button = unsafe {
            self.dock
                .unwrap()
                .assume_safe()
                .get_node("./DockVC/ModeVC/RowTwo/Face")
                .unwrap()
                .assume_safe()
                .cast::<Button>()
                .unwrap()
        };

        face_mode_button.set_pressed(false);

        let edge_mode_button = unsafe {
            self.dock
                .unwrap()
                .assume_safe()
                .get_node("./DockVC/ModeVC/RowTwo/Edge")
                .unwrap()
                .assume_safe()
                .cast::<Button>()
                .unwrap()
        };

        edge_mode_button.set_pressed(false);


        // Signals
        create_cube_button.connect(
            "create_cube",
            owner,
            "create_cube",
            VariantArray::new_shared(),
            0,
        ).expect("[Prodot Builder]: Error when connecting the create cube button!");

        object_mode_button.connect(
            "object_mode",
            owner,
            "change_build_mode",
            VariantArray::new_shared(),
            0,
        ).expect("[Prodot Builder]: Error when connecting the object mode button!");

        vertex_mode_button.connect(
            "vertex_mode",
            owner,
            "change_build_mode",
            VariantArray::new_shared(),
            0,
        ).expect("[Prodot Builder]: Error when connecting the vertex mode button!");

        face_mode_button.connect(
            "face_mode",
            owner,
            "change_build_mode",
            VariantArray::new_shared(),
            0,
        ).expect("[Prodot Builder]: Error when connecting the face mode button!");

        edge_mode_button.connect(
            "edge_mode",
            owner,
            "change_build_mode",
            VariantArray::new_shared(),
            0,
        ).expect("[Prodot Builder]: Error when connecting the edge mode button!");


        // Cache
        self.object_mode_button = Some ( object_mode_button.claim() );
        self.vertex_mode_button = Some ( vertex_mode_button.claim() );
        self.face_mode_button = Some ( face_mode_button.claim() );
        self.edge_mode_button = Some ( edge_mode_button.claim() );
        
    }

    #[export]
    fn _exit_tree(&mut self, owner: TRef<EditorPlugin>) {
        
        //Remove it from the engine
        //owner.remove_custom_type("ProdotMesh");

        // Remove the dock
        owner.remove_control_from_docks(self.dock.unwrap());
        
        // Free the stored instanciated nodes
        unsafe { self.dock.unwrap().assume_safe().queue_free() };
        //unsafe { self.object_mode_button.unwrap().assume_safe().queue_free() };
        //unsafe { self.vertex_mode_button.unwrap().assume_safe().queue_free() };
        //unsafe { self.face_mode_button.unwrap().assume_safe().queue_free() };
        //unsafe { self.edge_mode_button.unwrap().assume_safe().queue_free() };

        // Clean up
        self.selected_node = None;
        self.mesh_scene = None;
        self.object_mode_button = None;
        self.vertex_mode_button = None;
        self.face_mode_button = None;
        self.edge_mode_button = None;

    }

    #[export]
    fn _process(&mut self, owner: TRef<EditorPlugin>, _delta: f64) {
        match self.selected_node {
            Some(mesh_ref) => {
                let editor_instance = unsafe { EditorPlugin::get_editor_interface(&owner).unwrap().assume_safe() };
                let selection = unsafe { editor_instance.get_selection().unwrap().assume_safe() };
                if selection.get_selected_nodes().is_empty() {
                    self.reset(owner);
                    self.selected_node = None;
                } else {
                    match self.build_mode {
                        BuildMode::Object => {},
                        BuildMode::Vertex => {
                            let mesh = unsafe { mesh_ref.assume_safe() };
                            let mesh_script = mesh.cast_instance::<ProdotMesh>().unwrap();
                            mesh_script
                                .map_mut(|mesh, owner: TRef<MeshInstance>| {
                                    mesh.draw_vertices(owner, self.selected_indices.clone(), self.hover_index, self.hovering_gizmo_axis);
                                })
                                .ok()
                                .unwrap();
                        },
                        BuildMode::Face => {
                            let mesh = unsafe { mesh_ref.assume_safe() };
                            let mesh_script = mesh.cast_instance::<ProdotMesh>().unwrap();
                            mesh_script
                                .map_mut(|mesh, owner: TRef<MeshInstance>| {
                                    mesh.draw_faces(owner, self.selected_indices.clone(), self.hover_index, self.hovering_gizmo_axis);
                                })
                                .ok()
                                .unwrap();
                        },
                        BuildMode::Edge => {},
                    }
                }
            },
            None => (),
        }
    }

    /// Requests the editor to edit the given object
    /// Sends the plugin the object that is being edited.
    ///
    #[export]
    fn edit(&mut self, _owner: TRef<EditorPlugin>, object: Ref<Object>) {
        match unsafe { object.assume_safe().cast::<MeshInstance>() } {
            Some(node) => {
                self.selected_node = Some(node.claim());
            }
            None => self.selected_node = None,
        }
    }

    #[export]
    fn handles(&mut self, owner: TRef<EditorPlugin>, object: Ref<Object>) -> bool {
        match unsafe { object.assume_safe().cast::<MeshInstance>() } {
            Some(_node) => {
                return true;
            }
            None =>{
                self.reset(owner);
                self.selected_node = None
            },
        }

        false
    }

    #[export]
    fn forward_spatial_force_draw_over_viewport(
        &mut self, _owner: TRef<EditorPlugin>, _overlay: Ref<Control>,
    ) {
        let overlay = unsafe { _overlay.assume_safe() };
        let peach_color = Color::rgba(0.98431, 0.39216, 0.47451, 1.0);
        //let white_color = Color::rgba(1.0, 1.0, 1.0, alpha);
        //let plum_color = Color::rgba(0.16078, 0.19608, 0.32157, alpha);
        if self.is_dragging {
            let font = overlay.get_font("font", "").unwrap();
            let _viewport_size = overlay.size();
            overlay.draw_string(font, Vector2::new(10.0, -20.0), "Event: Dragging", peach_color, -1);
        }
    }

    #[export]
    fn forward_spatial_gui_input(
        &mut self, owner: TRef<EditorPlugin>, camera: Ref<Camera>, event: Ref<InputEvent>,
    ) -> bool {

        let mut consume_input = false;
        
        if let Some(node) = self.selected_node {
            

            let input = unsafe { event.assume_safe() };
            let mut control_down = false;

            let mesh = unsafe { node.assume_safe() };
            let mesh_pos = mesh.global_transform().origin;
            let mesh_script = mesh.cast_instance::<ProdotMesh>().unwrap();

    // --------------- Input Event With Modifers ------------------------ //
    //
            
            if let Some(modifer) = input.cast::<InputEventWithModifiers>() {
                control_down = modifer.control();
            }

    // --------------- Mouse Motion Input ------------------------ //
    //
            if let Some(motion) = input.cast::<InputEventMouseMotion>() {
                // This is in viewport coords
                let mouse = motion.position();
                // Cache if the mouse if hovering over a gizmo
                let mut hover_index_found = false;
                
                let cam = unsafe { camera.assume_safe() };

                let mut plane: Plane = Plane::new(Vector3::new(0.0, 0.0, 1.0), 0.0);
                let origin: Vector3 = cam.project_ray_origin(mouse);
                let normal: Vector3 = cam.project_ray_normal(mouse);

                match self.build_mode {
                    BuildMode::Object => {

                    },
                    BuildMode::Vertex => {
                        let vertices = 
                            mesh_script
                                .map_mut(|mesh, owner: TRef<MeshInstance>| {
                                    mesh.get_vertices(owner)
                                })
                                .ok()
                                .unwrap();

                        let box_size: f32 = 0.1;
                        
                        // Check to see if the user is still dragging, and if so update the position
                        if self.is_dragging && !self.selected_indices.is_empty() && self.hovering_gizmo_axis != Vector3::zero() {
                            //let vertex_pos = vertices.get(self.active_vertex_index);
                            let mut moved: bool = false;
                            let mut new_positions: HashMap<i32, Vector3> = HashMap::<i32, Vector3>::new();
                            
                            for index in self.selected_indices.iter() {
                                let vertex_pos: Vector3 = vertices.get(*index);
                                let mut g_pos: Vector3; 
                                if self.hovering_gizmo_axis == Vector3::new(1.0, 0.0, 0.0) {
                                    plane.d = vertex_pos.z;
                                    if let Some(proj_pos) = plane.intersects_ray(origin, normal) {
                                        let added_x_pos = Vector3::new(proj_pos.x, vertex_pos.y, vertex_pos.z);
                                        g_pos = added_x_pos - mesh_pos - Vector3::new(1.0, 0.0, 0.0) * 0.15;
                                        g_pos.y = 0.0;
                                        g_pos.z = 0.0;
                                        moved = true;
                                        let old_pos = *self.vertices_drag_state.get(&(*index)).unwrap();
                                        godot_print!("g: {:?}", g_pos);
                                        new_positions.insert(*index, Vector3::new( g_pos.x, old_pos.y, old_pos.z));

                                        //godot_print!("{:?}", *index);
                                    }
                                }else if self.hovering_gizmo_axis == Vector3::new(0.0, 1.0, 0.0) {
                                    plane.d = vertex_pos.z;
                                    if let Some(proj_pos) = plane.intersects_ray(origin, normal) {
                                        let added_y_pos = Vector3::new(vertex_pos.x, proj_pos.y, vertex_pos.z);
                                        g_pos = added_y_pos - mesh_pos - Vector3::new(0.0, 1.0, 0.0) * 0.15;
                                        moved = true;
                                        new_positions.insert(*index, g_pos);
                                    }
                                // Z Plane is calculated on a different plane
                                }else if self.hovering_gizmo_axis == Vector3::new(0.0, 0.0, 1.0) {
                                    plane = Plane::new(Vector3::new(1.0, 0.0, 0.0), vertex_pos.x);
                                    if let Some(proj_pos) = plane.intersects_ray(origin, normal) {
                                        let added_z_pos = Vector3::new(vertex_pos.x, vertex_pos.y, proj_pos.z);
                                        g_pos = added_z_pos - mesh_pos - Vector3::new(0.0, 0.0, 1.0) * 0.15;
                                        moved = true;
                                        new_positions.insert(*index, g_pos);
                                    }
                                }
                            }
                                
                            // Check to see if there were changes
                            if moved {
                                mesh_script 
                                    .map_mut(|mesh, owner: TRef<MeshInstance>| {
                                        mesh.set_vertex(owner, new_positions);
                                    })
                                    .ok()
                                    .unwrap();
                            }
                                                
                        } else {
                            for i in 0..vertices.len() {
                                let base_pos: Vector3 = vertices.get(i) + mesh_pos;

                                let vertex_pos = base_pos;
                                plane.d = vertex_pos.z;
                                
                                if let Some(proj_pos) = plane.intersects_ray(origin, normal) {
                                    if proj_pos.x > vertex_pos.x - box_size &&
                                        proj_pos.x < vertex_pos.x + box_size &&
                                        proj_pos.y > vertex_pos.y - box_size &&
                                        proj_pos.y < vertex_pos.y + box_size &&
                                        proj_pos.z > vertex_pos.z - box_size &&
                                        proj_pos.z < vertex_pos.z + box_size { 
                                        self.hover_index = i as i32;
                                        hover_index_found = true;
                                    }
                                }
                            }
                        }
                        
                        // Check to see if the mouse if hovering over a gizmo on the selected vertex
                        if !self.selected_indices.is_empty() && !self.is_dragging {
                            let vertices_center: Vector3 = 
                                mesh_script
                                    .map_mut(|mesh, owner: TRef<MeshInstance>| {
                                        mesh.get_vertices_center(owner, self.selected_indices.clone())
                                    })
                                    .ok()
                                    .unwrap();
                            self.detect_gizmo(owner, vertices_center + mesh_pos, origin, normal);
                        }
                    },
                    BuildMode::Face => {
                        let geom = Geometry::godot_singleton();
                        let vertices = 
                            mesh_script
                                .map_mut(|mesh, owner: TRef<MeshInstance>| {
                                    mesh.get_vertices(owner)
                                })
                                .ok()
                                .unwrap();
                        // Check faces
                        let faces: Vec::<Face> = 
                            mesh_script
                                .map_mut(|mesh, owner: TRef<MeshInstance>| {
                                    mesh.get_faces(owner)
                                })
                                .ok()
                                .unwrap();
                        
                        let mut closest_index = -1;
                        let mut closest_dist = 10000000.0;

                        // Check to see if the user is still dragging, and if so update the position
                        /*if self.is_dragging && !self.selected_indices.is_empty() && self.hovering_gizmo_axis != Vector3::zero() {
                            let face_pos: Vector3 =  
                                mesh_script
                                    .map_mut(|mesh, owner: TRef<MeshInstance>| {
                                        mesh.get_face_center(owner, self.selected_indices)
                                    })
                                    .ok()
                                    .unwrap();
                            let mut delta_pos: Vector3 = face_pos; 
                            let mut moved: bool = false;
                            let mut updated_faces: HashMap<i32, Vector3> = HashMap::<i32, Vector3>::new();

                            for index in self.selected_indices {
                                if self.hovering_gizmo_axis == Vector3::new(1.0, 0.0, 0.0) {
                                    plane.d = face_pos.z;
                                    if let Some(proj_pos) = plane.intersects_ray(origin, normal) {
                                        let added_x_pos = Vector3::new(proj_pos.x, face_pos.y, face_pos.z);
                                        delta_pos = added_x_pos - mesh_pos - Vector3::new(1.0, 0.0, 0.0) * 0.15;
                                        moved = true;
                                    }
                                }else if self.hovering_gizmo_axis == Vector3::new(0.0, 1.0, 0.0) {
                                    plane.d = face_pos.z;
                                    if let Some(proj_pos) = plane.intersects_ray(origin, normal) {
                                        let added_y_pos = Vector3::new(face_pos.x, proj_pos.y, face_pos.z);
                                        delta_pos = added_y_pos - mesh_pos - Vector3::new(0.0, 1.0, 0.0) * 0.15;
                                        moved = true;
                                    }
                                // Z Plane is calculated on a different plane
                                }else if self.hovering_gizmo_axis == Vector3::new(0.0, 0.0, 1.0) {
                                    plane = Plane::new(Vector3::new(1.0, 0.0, 0.0), face_pos.x);
                                    if let Some(proj_pos) = plane.intersects_ray(origin, normal) {
                                        let added_z_pos = Vector3::new(face_pos.x, face_pos.y, proj_pos.z);
                                        delta_pos = added_z_pos - mesh_pos - Vector3::new(0.0, 0.0, 1.0) * 0.15;
                                        moved = true;
                                    }
                                }
                            }
                                
                            // Check to see if there were changes
                            if moved {
                                mesh_script 
                                    .map_mut(|mesh, owner: TRef<MeshInstance>| {
                                        mesh.set_face(owner, updated_faces);
                                    })
                                    .ok()
                                    .unwrap();
                            }
                        } else {*/
                            for i in 0..faces.len() {
                                let face = faces[i];
                                let vertex_one = mesh_pos + vertices.get(face.tris_one.x as i32);
                                let vertex_two = mesh_pos + vertices.get(face.tris_one.y as i32);
                                let vertex_three = mesh_pos + vertices.get(face.tris_one.z as i32);
                                

                                let vertex_four = mesh_pos + vertices.get(face.tris_two.x as i32);
                                let vertex_five = mesh_pos + vertices.get(face.tris_two.y as i32);
                                let vertex_six =  mesh_pos + vertices.get(face.tris_two.z as i32);

                                let result_one = geom.ray_intersects_triangle(
                                    origin,
                                    normal,
                                    vertex_one,
                                    vertex_two,
                                    vertex_three
                                );

                                let result_two = geom.ray_intersects_triangle(
                                    origin,
                                    normal,
                                    vertex_four,
                                    vertex_five,
                                    vertex_six
                                );

                                let mut dist_one = 1000000.0; 
                                let mut intersects = false; 
                                if !result_one.is_nil() {
                                    intersects = true;
                                    dist_one = origin.distance_to(result_one.to_vector3()).abs();
                                }
                                
                                let mut dist_two = 1000000.0;
                                if !result_two.is_nil() {
                                    intersects = true;
                                    dist_two = origin.distance_to(result_two.to_vector3()).abs();
                                }

                                if intersects && (dist_one < closest_dist || dist_two < closest_dist) {
                                    closest_index = i as i32;
                                    if dist_one <= dist_two {
                                        closest_dist = dist_one;
                                    } else {
                                        closest_dist = dist_two;
                                    }
                                }
                            }

                            if closest_index != -1 {
                                hover_index_found = true;
                                self.hover_index = closest_index as i32;
                            }

                            // Check to see if the mouse if hovering over a gizmo on the selected vertex
                            if !self.selected_indices.is_empty() && !self.is_dragging {
                                let face_center = 
                                    mesh_script
                                        .map_mut(|mesh, owner: TRef<MeshInstance>| {
                                            mesh.get_face_center(owner, self.selected_indices.clone())
                                        })
                                        .ok()
                                        .unwrap();
                                self.detect_gizmo(owner, face_center + mesh_pos, origin, normal);
                            }
                        //}

                    },
                    BuildMode::Edge => {

                    },
                }

                
                if !hover_index_found {
                    self.hover_index = -1;
                }
                //}
            }
    
    // --------------- Mouse Button Input ------------------------ //
    //
            if let Some(button) = input.cast::<InputEventMouseButton>() {
                match button.button_index() { 
                    GlobalConstants::BUTTON_LEFT => {
                        let zero_vector = Vector3::zero();
                        // If we're hovering over a handle
                        if self.hover_index != -1 || self.hovering_gizmo_axis != zero_vector{
                            // Was it a press or release event?
                            if button.is_pressed() { 
                                // Is this a gizmo hover_index
                                if self.hovering_gizmo_axis != zero_vector {

                                } else if !self.selected_indices.contains(&self.hover_index) {
                                    if !self.selected_indices.is_empty() && !control_down {
                                        self.selected_indices.clear();
                                        self.vertices_drag_state.clear();
                                    } 
                                    self.selected_indices.push(self.hover_index);
                                    
                                    // Get the starting vertex position
                                    let vertex_pos = 
                                        mesh_script
                                            .map_mut(|mesh, owner: TRef<MeshInstance>| {
                                                mesh.get_vertex(owner, self.hover_index)
                                            })
                                            .ok()
                                            .unwrap();
                                    self.vertices_drag_state.insert(self.hover_index, vertex_pos);
                                } else if control_down{
                                    let mut index: i32 = -1;
                                    for i in 0..self.selected_indices.len() {
                                        if self.selected_indices[i] == self.hover_index {
                                            index = i as i32;
                                            break;
                                        }
                                    }
                                    if index != -1 {
                                        self.vertices_drag_state.remove(&self.selected_indices[index as usize]);
                                        self.selected_indices.remove(index as usize);
                                    }
                                }
                            
                                self.is_dragging = true;
                                consume_input = true;
                                owner.update_overlays();
                            } else {
                                self.is_dragging = false;
                                for (index, _position) in self.vertices_drag_state.clone() {
                                // Get the starting vertex position
                                let vertex_pos = 
                                    mesh_script
                                        .map_mut(|mesh, owner: TRef<MeshInstance>| {
                                            mesh.get_vertex(owner, index)
                                        })
                                        .ok()
                                        .unwrap();
                                self.vertices_drag_state.insert(index, vertex_pos);
                            }
                                //self.vertices_drag_state.clear();
                            }
                        } else if self.is_dragging && !button.is_pressed() {
                            self.is_dragging = false;
                            //self.vertices_drag_state.clear();
                            
                            for (index, _position) in self.vertices_drag_state.clone() {
                                godot_print!("Update them");
                                // Get the starting vertex position
                                let vertex_pos = 
                                    mesh_script
                                        .map_mut(|mesh, owner: TRef<MeshInstance>| {
                                            mesh.get_vertex(owner, index)
                                        })
                                        .ok()
                                        .unwrap();
                                self.vertices_drag_state.insert(index, vertex_pos);
                            }
                            owner.update_overlays();
                        }
                    },
                    _ => (),
                }
            }
    
    // --------------- Keyboard Input ------------------------ //
    //
            if input.is_action_pressed("ui_cancel", false) {
                self.reset(owner);
                consume_input = true;
            }
            
            owner.update_overlays();
        }

        consume_input
    }

    /// Creates a default 1x1x1 cube and attaches it to the current
    /// scenes root node
    ///
    #[export]
    pub fn create_cube(&mut self, owner: TRef<EditorPlugin>) {
        godot_print!("Created cube!");

        let editor = EditorPlugin::get_editor_interface(owner.as_ref()).unwrap();
        let root_node = unsafe { editor.assume_safe().get_edited_scene_root().unwrap().assume_safe() };

        
        let arrays = VariantArray::new();
        let mut normal_array = TypedArray::<Vector3>::new();
        let mut uv_array = TypedArray::<Vector2>::new();
        let mut vertex_array = TypedArray::<Vector3>::new();
        let mut index_array = TypedArray::<i32>::new();
        let mut face_array = Vec::<Face>::new();

        arrays.resize(Mesh::ARRAY_MAX as i32);
        
        // Front face
        // 

        //bl
        normal_array.push(Vector3::new(0.0, 0.0, 1.0));
        uv_array.push(Vector2::new(0.0, 0.0));
        vertex_array.push(Vector3::new(0.0, 0.0, 1.0));

        // tl
        normal_array.push(Vector3::new(0.0, 0.0, 1.0));
        uv_array.push(Vector2::new(0.0, 1.0));
        vertex_array.push(Vector3::new(0.0, 1.0, 1.0));

        // tr 
        normal_array.push(Vector3::new(0.0, 0.0, 1.0));
        uv_array.push(Vector2::new(1.0, 1.0));
        vertex_array.push(Vector3::new(1.0, 1.0, 1.0));
        
        // br 
        normal_array.push(Vector3::new(0.0, 0.0, 1.0));
        uv_array.push(Vector2::new(1.0, 0.0));
        vertex_array.push(Vector3::new(1.0, 0.0, 1.0));
        
        index_array.push(0);
        index_array.push(1);
        index_array.push(2);

        index_array.push(2);
        index_array.push(3);
        index_array.push(0);
        

        face_array.push(Face {
            tris_one: Vector3::new(0.0, 1.0, 2.0),
            tris_two: Vector3::new(2.0, 3.0, 0.0),
        });

        // Right face
        //
        // tr 
        normal_array.push(Vector3::new(1.0, 0.0, 0.0));
        uv_array.push(Vector2::new(1.0, 1.0));
        vertex_array.push(Vector3::new(1.0, 1.0, 0.0));
        
        // br 
        normal_array.push(Vector3::new(1.0, 0.0, 0.0));
        uv_array.push(Vector2::new(1.0, 0.0));
        vertex_array.push(Vector3::new(1.0, 0.0, 0.0));
        
        index_array.push(3);
        index_array.push(2);
        index_array.push(4);

        index_array.push(4);
        index_array.push(5);
        index_array.push(3);

        face_array.push(Face {
            tris_one: Vector3::new(3.0, 2.0, 4.0),
            tris_two: Vector3::new(4.0, 5.0, 3.0),
        });

        // Left face
        //bl
        normal_array.push(Vector3::new(-1.0, 0.0, 0.0));
        uv_array.push(Vector2::new(0.0, 0.0));
        vertex_array.push(Vector3::new(0.0, 0.0, 0.0));

        // tl
        normal_array.push(Vector3::new(-1.0, 0.0, 0.0));
        uv_array.push(Vector2::new(0.0, 1.0));
        vertex_array.push(Vector3::new(0.0, 1.0, 0.0));
        
        index_array.push(6);
        index_array.push(7);
        index_array.push(1);

        index_array.push(1);
        index_array.push(0);
        index_array.push(6);

        face_array.push(Face {
            tris_one: Vector3::new(6.0, 7.0, 1.0),
            tris_two: Vector3::new(1.0, 0.0, 6.0),
        });

        // Back Face
        //
        index_array.push(5);
        index_array.push(4);
        index_array.push(7);

        index_array.push(7);
        index_array.push(6);
        index_array.push(5);
        
        face_array.push(Face {
            tris_one: Vector3::new(5.0, 4.0, 7.0),
            tris_two: Vector3::new(7.0, 6.0, 5.0),
        });

        // Top Face
        // 
        index_array.push(1);
        index_array.push(7);
        index_array.push(4);

        index_array.push(4);
        index_array.push(2);
        index_array.push(1);


        face_array.push(Face {
            tris_one: Vector3::new(1.0, 7.0, 4.0),
            tris_two: Vector3::new(4.0, 2.0, 1.0),
        });

        // Bottom Face
        //
        index_array.push(6);
        index_array.push(0);
        index_array.push(3);

        index_array.push(3);
        index_array.push(5);
        index_array.push(6);
        
        face_array.push(Face {
            tris_one: Vector3::new(6.0, 0.0, 3.0),
            tris_two: Vector3::new(3.0, 5.0, 6.0),
        });

        arrays.set(Mesh::ARRAY_VERTEX as i32, vertex_array.clone());
        arrays.set(Mesh::ARRAY_NORMAL as i32, normal_array);
        arrays.set(Mesh::ARRAY_TEX_UV as i32, uv_array);
        arrays.set(Mesh::ARRAY_INDEX as i32, index_array);

        let blend_shapes = VariantArray::new_shared();
        //blend_shapes.resize(Mesh::ARRAY_MAX as i32);
        let end_mesh = ArrayMesh::new();
        end_mesh.add_surface_from_arrays(Mesh::PRIMITIVE_TRIANGLES, arrays.into_shared(), blend_shapes, 97280);
        
        let mesh_instance = unsafe {
            self
                .mesh_scene
                .as_ref()
                .unwrap()
                .assume_safe()
                .instance(PackedScene::GEN_EDIT_STATE_INSTANCE)
                .unwrap()
                .assume_safe()
                .cast::<MeshInstance>()
                .unwrap()
        };
        
        root_node.add_child(mesh_instance, false);
        mesh_instance.set_owner(root_node);
        mesh_instance.set_mesh(end_mesh);

        let mesh_script = 
            mesh_instance
                .cast_instance::<ProdotMesh>()
                .unwrap();
        mesh_script
            .map_mut(|mesh, owner: TRef<MeshInstance>| {
                mesh.set_vertices(owner, vertex_array);
                mesh.set_faces(owner, face_array);
            })
            .ok()
            .unwrap();

    }
    /// Sets the build mode of the plugin
    ///
    /// BuildMode::Vertex
    /// BuildMode::Face
    /// BuildMode::Edge
    ///
    #[export]
    pub fn change_build_mode(&mut self, owner: TRef<EditorPlugin>, mode: i64) {
        self.build_mode = self.build_mode.set(mode);
        godot_print!("[Prodot Builder]: Switched to {:?} mode", self.build_mode);
        match self.build_mode {
            BuildMode::Object => {
                // Toggle the correct button, and untoggle the other buttons
                
            },
            BuildMode::Vertex => {
                // Toggle the correct button, and untoggle the other buttons
                
            },
            BuildMode::Face => {
                // Toggle the correct button, and untoggle the other buttons
            },
            BuildMode::Edge => {
                // Toggle the correct button, and untoggle the other buttons
               
            }
        }

        self.reset(owner);

    }
    
    /// Resets the indices of the active index and hover index
    ///
    ///
    #[export]
    fn reset(&mut self, _owner: TRef<EditorPlugin>) {
        self.selected_indices.clear();
        self.hover_index = -1;
        self.hovering_gizmo_axis = Vector3::zero();

        if let Some(mesh_ref) = self.selected_node {
            let mesh = unsafe { mesh_ref.assume_safe() };
            let mesh_script = mesh.cast_instance::<ProdotMesh>().unwrap();
            mesh_script
                .map_mut(|mesh, owner: TRef<MeshInstance>| {
                    mesh.clear(owner);
                })
                .ok()
                .unwrap();
        }

    }
    
    #[export]
    fn detect_gizmo(&mut self, _owner: TRef<EditorPlugin>, center: Vector3, origin: Vector3, normal: Vector3) {
        //let vertex_pos = vertices.get(self.active_vertex_index) + mesh_pos;
        let mut plane = Plane::new(Vector3::new(0.0, 0.0, 1.0), center.z);
        let gizmo_dist: f32 = 0.15;
        let box_size = 0.05;

        let mut hovering_gizmo = false;
        if let Some(proj_pos) = plane.intersects_ray(origin, normal) {
            // X_Plane
            if proj_pos.x > center.x - box_size + gizmo_dist &&
                proj_pos.x < center.x + box_size + gizmo_dist &&
                proj_pos.y > center.y - box_size &&
                proj_pos.y < center.y + box_size {
                self.hovering_gizmo_axis = Vector3::new(1.0, 0.0, 0.0);
                hovering_gizmo = true;
            }
            
            // Y_Plane
            if proj_pos.x > center.x - box_size &&
                proj_pos.x < center.x + box_size &&
                proj_pos.y > center.y - box_size + gizmo_dist &&
                proj_pos.y < center.y + box_size + gizmo_dist {
                self.hovering_gizmo_axis = Vector3::new(0.0, 1.0, 0.0);
                hovering_gizmo = true;
            }
                                

        }
        
        plane = Plane::new(Vector3::new(1.0, 0.0, 0.0), center.x);
        if let Some(proj_pos) = plane.intersects_ray(origin, normal) {
            // Z_Plane
            if proj_pos.y > center.y - box_size &&
                proj_pos.y < center.y + box_size &&
                proj_pos.z > center.z - box_size + gizmo_dist &&
                proj_pos.z < center.z + box_size + gizmo_dist {
                self.hovering_gizmo_axis = Vector3::new(0.0, 0.0, 1.0);
                hovering_gizmo = true;
            }
        }

        if !hovering_gizmo {
            self.hovering_gizmo_axis = Vector3::zero();
        }

    }

    
    
}
