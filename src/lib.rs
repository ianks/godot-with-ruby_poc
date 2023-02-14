use gdnative::prelude::*;
use magnus::{embed, eval};

#[derive(NativeClass)]
#[inherit(Node)]
struct Ruby;

#[methods]
impl Ruby {
    fn new(_owner: &Node) -> Self {
        Self
    }

    #[method]
    fn _ready(&self) {
        godot_print!("Loading ruby");
        godot_print!("Ruby loaded");
        let _cleanup = unsafe { embed::setup() };
        let val: f64 = eval!("a + rand", a = 1).unwrap();
        godot_print!("Random Number from Ruby: {}", val);
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<Ruby>();
}

godot_init!(init);
