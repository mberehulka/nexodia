#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern  crate macros;

mod shaders;
mod objects;
mod scenes;

fn main() {
    let (el, e) = engine::Engine::new();
    e.set_scene::<scenes::main::Scene>(());
    e.start(el)
}