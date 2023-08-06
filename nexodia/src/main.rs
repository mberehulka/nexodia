#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate engine;
use engine::*;

mod scenes;
mod camera;

fn main() {
    let (el, e) = Engine::new();
    e.load_scene::<scenes::main::Scene>();
    e.start(el)
}