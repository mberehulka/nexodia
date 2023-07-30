use winit::{window::{Window, WindowBuilder}, dpi::{PhysicalSize, PhysicalPosition}, event_loop::EventLoop};

pub fn new(event_loop: &EventLoop<()>) -> Window {
    let w = WindowBuilder::new()
        .with_title("Nexodia")
        .with_inner_size(PhysicalSize {
            width: 900,
            height: 600
        })
        .with_min_inner_size(PhysicalSize {
            width: 100,
            height: 100
        })
        .build(event_loop).unwrap();
    // center window
    if let Some(monitor) = w.current_monitor() {
        let ms = monitor.size();
        let ws = w.inner_size();
        w.set_outer_position(PhysicalPosition {
            x: ((ms.width - ws.width) / 2),
            y: (ms.height - ws.height) / 2
        })
    }
    w
}