use gdnative::prelude::*;

#[path = "dock/dock.rs"]
mod dock;
mod prodot_builder;
mod prodot_mesh;
mod prodot_utils;
mod prodot_gizmo;

fn init(handle: InitHandle) {
    handle.add_tool_class::<prodot_builder::ProdotBuilderPlugin>();
    handle.add_tool_class::<prodot_mesh::ProdotMesh>();
    handle.add_tool_class::<prodot_gizmo::ProdotGizmo>();
    handle.add_tool_class::<dock::create_cube_button::CreateCubeButton>();
    handle.add_tool_class::<dock::object_mode_button::ObjectModeButton>();
    handle.add_tool_class::<dock::vertex_mode_button::VertexModeButton>();
    handle.add_tool_class::<dock::face_mode_button::FaceModeButton>();
    handle.add_tool_class::<dock::edge_mode_button::EdgeModeButton>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
