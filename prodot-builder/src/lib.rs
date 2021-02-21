use gdnative::prelude::*;

mod prodot_utils;
mod prodot_builder;
mod prodot_mesh;
#[path = "dock/dock.rs"]
mod dock;


fn init(handle: InitHandle) {
    handle.add_tool_class::<prodot_builder::ProdotBuilderNode>();
    handle.add_tool_class::<prodot_mesh::ProdotMesh>();
    handle.add_tool_class::<dock::create_cube_button::CreateCubeButton>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
