cargo_component_bindings::generate!();

use crate::bindings::exports::component::hello::message::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        "Hello, World!".to_string()
    }
}
