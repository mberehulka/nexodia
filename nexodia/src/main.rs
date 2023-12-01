#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(trait_alias)]

#[macro_use]
extern  crate macros;

mod utils;
mod shaders;
mod objects;
mod scenes;

fn main() {
    let (el, e) = engine::Engine::new();
    e.set_scene::<scenes::main::Scene>(());
    e.start(el)
}