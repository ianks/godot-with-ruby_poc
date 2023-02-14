use gdnative::prelude::*;
use magnus::{embed, eval};

#[derive(NativeClass)]
#[inherit(Node)]
struct Ruby;

#[methods]
impl Ruby {
    fn new(_owner: &Node) -> Self {
        Ruby
    }

    #[method]
    fn _ready(&self) {
        let val: f64 = eval!("a + rand", a = 1).unwrap();
        godot_print!("Random Number from Ruby: {}", val);
        godot_print!("Hello World");
    }
}

fn init(handle: InitHandle) {
    let _cleanup = unsafe { embed::setup() };
    handle.add_class::<Ruby>();
}

godot_init!(init);
