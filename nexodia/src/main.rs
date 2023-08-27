#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Instant;

#[macro_use]
extern crate engine;

mod main_scene;
mod camera;
mod shaders;
mod stats;

fn main() {
    let (el, e) = engine::Engine::new();

    let start = Instant::now();
    
    e.add_script::<main_scene::Scene>();
    e.add_script::<camera::OrbitalCamera>();
    e.add_script::<stats::Stats>();

    info!("Game initialized in {}ms", (Instant::now()-start).as_millis());
    
    e.start(el)
}