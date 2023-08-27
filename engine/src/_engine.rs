use std::{sync::{Mutex, atomic::AtomicBool, Arc}, collections::HashSet};
use wgpu::{Instance, Surface, Adapter, Device, Queue, SurfaceConfiguration};
use winit::{
    event::{Event, WindowEvent, VirtualKeyCode, ElementState, KeyboardInput},
    event_loop::{ControlFlow, EventLoop}, window::Window, dpi::PhysicalSize
};

use crate::{Camera, Logger, Time, DepthTexture, Frame, utils::initialization::*, Scripts};

pub struct Engine {
    pub window: Window,
    pub instance: Instance,
    pub surface: Surface,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub exit: AtomicBool,
    pub surface_config: Mutex<SurfaceConfiguration>,
    pub depth_texture: Mutex<Arc<DepthTexture>>,
    pub pressed: Mutex<HashSet<VirtualKeyCode>>,
    pub camera: Camera,
    pub time: Time,
    pub scripts: Scripts
}
impl Engine {
    pub fn new() -> (EventLoop<()>, &'static Self) {
        Logger::new();
        let event_loop = EventLoop::new();
        let window = new_window(&event_loop);
        let instance = Instance::new(wgpu::InstanceDescriptor::default());
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = new_adapter(&instance, &surface);
        let (device, queue) = new_device(&adapter);
        let surface_config = configure_surface(window.inner_size(), &device, &adapter, &surface);
        let depth_texture = DepthTexture::new(&device, surface_config.width, surface_config.height);
        let camera = Camera::new(&device);
        let s = Self {
            window,
            instance: instance,
            surface,
            adapter: adapter,
            device,
            queue,
            exit: Default::default(),
            surface_config: surface_config.into(),
            depth_texture: Mutex::new(Arc::new(depth_texture)),
            pressed: Default::default(),
            camera,
            time: Time::new(),
            scripts: Default::default()
        };
        (event_loop, Box::leak(Box::new(s)))
    }
    pub fn start(&'static self, event_loop: EventLoop<()>) -> ! {
        event_loop.run(move |event, _, control_flow| {
            let event = if let Some(v) = event.to_static() { v } else { return };
            let threads = self.scripts.threads.lock().unwrap();
            for thread in threads.values() {
                thread.send(crate::ThreadEvent::Event(event.clone()))
            }
            match event {
                Event::WindowEvent { event: WindowEvent::KeyboardInput { input: KeyboardInput {
                    state: ElementState::Pressed, virtual_keycode: Some(code), ..
                }, .. }, .. } => {
                    self.pressed.lock().unwrap().insert(code);
                },
                Event::WindowEvent { event: WindowEvent::KeyboardInput { input: KeyboardInput {
                    state: ElementState::Released, virtual_keycode: Some(code), ..
                }, .. }, .. } => {
                    self.pressed.lock().unwrap().remove(&code);
                },
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => self.exit(),
                Event::WindowEvent { event: WindowEvent::Resized(new_size), .. } => self.resize(new_size),
                Event::MainEventsCleared => self.window.request_redraw(),
                Event::RedrawRequested(_) => {
                    self.time.update();
                    if let Some(mut frame) = Frame::new(self) {
                        for thread in threads.values() {
                            thread.wait()
                        }
                        for thread in threads.values() {
                            thread.script.lock().unwrap().render(&mut frame)
                        }
                        self.queue.submit(std::iter::once(frame.encoder.finish()));
                        frame.output_texture.present()
                    }
                }
                _ => {}
            }
            if self.exit.load(std::sync::atomic::Ordering::Relaxed) {
                *control_flow = ControlFlow::Exit
            }
        })
    }
    pub fn resize(&self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 { return }
        let mut surface_config = self.surface_config.lock().unwrap();
        surface_config.width = new_size.width;
        surface_config.height = new_size.height;
        self.surface.configure(&self.device, &surface_config);
        *self.depth_texture.lock().unwrap() = Arc::new(DepthTexture::new(&self.device, surface_config.width, surface_config.height));
    }
    pub fn exit(&self) {
        self.exit.store(true, std::sync::atomic::Ordering::Relaxed)
    }
}