use gdnative::api::Button;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Button)]
#[register_with(Self::register_signals)]
pub struct CreateCubeButton;

#[methods]
impl CreateCubeButton {
    fn new(_owner: TRef<Button>) -> Self {
        CreateCubeButton
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "create_cube",
            args: &[],
        });
    }

    #[export]
    fn _enter_tree(&self, owner: TRef<Button>) {
        owner
            .connect("pressed", owner, "on_click", VariantArray::new_shared(), 0)
            .unwrap();
    }

    #[export]
    fn on_click(&self, owner: TRef<Button>) {
        owner.emit_signal("create_cube", &[]);
    }

    #[export]
    fn _exit_tree(&self, _owner: TRef<Button>) {}
}
