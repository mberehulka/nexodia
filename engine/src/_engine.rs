use std::sync::{Mutex, atomic::AtomicBool};
use wgpu::{Instance, Surface, Adapter, Device, Queue, SurfaceConfiguration};
use winit::{
    event::{Event, WindowEvent, ElementState, KeyboardInput},
    event_loop::{ControlFlow, EventLoop}, window::Window, dpi::PhysicalSize
};

use crate::{CameraBuffer, Logger, Time, utils::{initialization::*, pressed_keys::PressedKeys}, Scripts};

pub struct Engine {
    pub window: Window,
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface,
    pub surface_config: Mutex<SurfaceConfiguration>,
    pub exit: AtomicBool,
    pub pressed_keys: PressedKeys,
    pub camera_buffer: CameraBuffer,
    pub time: Time,
    pub scripts: Scripts
}
impl Engine {
    pub fn new() -> (EventLoop<()>, &'static Self) {
        Logger::new();
        let event_loop = EventLoop::new();
        let window = new_window(&event_loop);
        let instance = new_instance();
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = new_adapter(&instance, &surface);
        let (device, queue) = new_device(&adapter);
        let surface_config = configure_surface(window.inner_size(), &device, &adapter, &surface);
        let camera_buffer = CameraBuffer::new(&device);
        let s = Self {
            window,
            instance: instance,
            surface,
            device,
            queue,
            surface_config: surface_config.into(),
            adapter: adapter,
            exit: Default::default(),
            pressed_keys: Default::default(),
            camera_buffer,
            time: Time::new(),
            scripts: Default::default()
        };
        (event_loop, Box::leak(Box::new(s)))
    }
    pub fn start(&'static self, event_loop: EventLoop<()>) -> ! {
        self.window.set_visible(true);
        event_loop.run(move |event, _, control_flow| {
            if self.exit.load(std::sync::atomic::Ordering::Relaxed) { return *control_flow = ControlFlow::Exit }
            let event = if let Some(v) = event.to_static() { v } else { return };
            let threads = self.scripts.threads.lock().unwrap();
            match event {
                Event::WindowEvent { event: WindowEvent::KeyboardInput { input: KeyboardInput {
                    state: ElementState::Pressed, virtual_keycode: Some(code), ..
                }, .. }, .. } =>
                    self.pressed_keys.set(code, true),
                Event::WindowEvent { event: WindowEvent::KeyboardInput { input: KeyboardInput {
                    state: ElementState::Released, virtual_keycode: Some(code), ..
                }, .. }, .. } =>
                    self.pressed_keys.set(code, false),
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => self.exit(),
                Event::WindowEvent { event: WindowEvent::Resized(new_size), .. } => self.resize(new_size),
                Event::MainEventsCleared => {
                    self.time.update();
                    self.window.request_redraw()
                },
                Event::RedrawRequested(_) => for thread in threads.values() { thread.wait() }
                _ => {}
            }
            for thread in threads.values() {
                thread.send(crate::ThreadEvent::Event(event.clone()))
            }
        })
    }
    pub fn resize(&self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 { return }
        self.instance.poll_all(true);
        let mut surface_config = self.surface_config.lock().unwrap();
        surface_config.width = new_size.width;
        surface_config.height = new_size.height;
        self.surface.configure(&self.device, &surface_config);
    }
    pub fn exit(&self) {
        self.exit.store(true, std::sync::atomic::Ordering::Relaxed)
    }
}