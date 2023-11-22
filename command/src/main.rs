cargo_component_bindings::generate!();

use bindings::component::hello::message::hello_world;

fn main() {
    println!("{}", hello_world());
}
