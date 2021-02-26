use gdnative::api::Button;
use gdnative::prelude::*;
use crate::prodot_builder::*;

#[derive(NativeClass)]
#[inherit(Button)]
#[register_with(Self::register_signals)]
pub struct ObjectModeButton; 

#[methods]
impl ObjectModeButton {
    fn new(_owner: TRef<Button>) -> Self {
        ObjectModeButton
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "object_mode",
            args: &[SignalArgument {
                name: "mode",
                default: Variant::from_i64(BuildMode::Object.value()),
                export_info: ExportInfo::new(VariantType::I64),
                usage: PropertyUsage::DEFAULT,
            }],
        });
    }

    #[export]
    fn _enter_tree(&self, owner: TRef<Button>) {
        owner
            .connect("toggled", owner, "on_toggled", VariantArray::new_shared(), 0)
            .unwrap();
    }

    #[export]
    fn on_toggled(&self, owner: TRef<Button>, _button_pressed: bool) {
        owner.emit_signal("object_mode", &[ Variant::from_i64(BuildMode::Object.value()) ] );
    }
}
