#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate utils;

mod game;
mod assets;
mod shader;
mod object;

fn main() {
    utils::Logger::new();
    game::Game::builder()
        .build()
}