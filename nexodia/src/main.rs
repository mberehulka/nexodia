#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate utils;

mod initialization;
mod game;
mod builder;

fn main() {
    utils::init_logs();
    game::Game::builder().build()
}