use gdnative::api::{PackedScene, Resource, ResourceLoader};
use gdnative::prelude::*;
use gdnative::GodotObject;

#[allow(dead_code)]
pub unsafe fn load_resource<T>(path: &str, hint: &str) -> Option<Ref<T, Shared>>
where
    T: GodotObject<RefKind = RefCounted> + SubClass<Resource>,
{
    let resource = ResourceLoader::godot_singleton().load(path, hint, false)?;
    let resource = resource.assume_safe().claim();
    resource.cast::<T>()
}

pub unsafe fn load_scene(path: &str) -> Option<Ref<PackedScene>> {
    Some(
        ResourceLoader::godot_singleton()
            .load(path, "", false)
            .unwrap()
            .assume_safe()
            .cast::<PackedScene>()
            .unwrap()
            .claim(),
    )
}
