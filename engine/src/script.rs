use wgpu::RenderPass;
use winit::event::Event;

use crate::Engine;

#[allow(unused_variables)]
pub trait Script: Send + Sync {
    fn setup(e: &'static Engine) -> Self where Self: Sized;
    fn update(&self, e: &'static Engine) {}
    fn event(&self, e: &'static Engine, event: Event<()>) {}
    fn render<'r, 's: 'r>(&'s self, e: &'static Engine, render_pass: RenderPass<'r>) {}
}

pub struct EmptyScript {}
impl Script for EmptyScript {
    fn setup(_e: &'static Engine) -> Self where Self: Sized { Self {} }
}