#[macro_use]
extern crate yew;

use yew::html::*;

struct Model {
    input: String,
    todos: Vec<String>,
}

enum Msg {
    Add,
    Update(String),
    Remove(usize),
    RemoveAll,
    Nothing,
}
fn main() {
    println!("Hello, world!");
}
