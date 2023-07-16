use winit::{window::{Window, WindowBuilder}, event_loop::EventLoop};

pub fn new<T>(event_loop: &EventLoop<T>) -> Window {
    WindowBuilder::new()
        .with_title("Nexodia")
        .build(event_loop).unwrap()
}