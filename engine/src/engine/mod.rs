use std::sync::{Mutex, atomic::AtomicBool};
use wgpu::{Instance, Surface, Adapter, Device, Queue, SurfaceConfiguration};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop}, window::Window
};

use crate::{Texture, Shaders, Script, Camera, DefaultCamera, EmptyScript};

mod window;
mod adapter;
mod device;
mod surface;

pub struct Engine {
    pub window: Window,
    pub instance: Instance,
    pub surface: Surface,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub exit: AtomicBool,
    pub surface_config: Mutex<SurfaceConfiguration>,
    pub depth_texture: Mutex<Texture>,
    pub shaders: Shaders,
    pub scene: Mutex<Box<dyn Script>>,
    pub camera: Mutex<Box<dyn Camera>>
}
impl Engine {
    pub fn new() -> (EventLoop<()>, &'static Self) {
        utils::Logger::new();
        let event_loop = EventLoop::new();
        let window = window::new(&event_loop);
        let instance = Instance::new(wgpu::InstanceDescriptor::default());
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = adapter::new(&instance, &surface);
        let (device, queue) = device::new(&adapter);
        let surface_config = surface::configure(&window.inner_size(), &device, &adapter, &surface);
        let depth_texture = Texture::depth(&device, surface_config.width, surface_config.height);
        let shaders = Shaders::new(&device, surface_config.format);
        let camera = DefaultCamera::new(&device);
        let s = Self {
            window,
            instance: instance,
            surface,
            adapter: adapter,
            device,
            queue,
            exit: Default::default(),
            surface_config: surface_config.into(),
            depth_texture: depth_texture.into(),
            shaders,
            scene: Mutex::new(Box::new(EmptyScript {})),
            camera: Mutex::new(Box::new(camera))
        };
        (event_loop, Box::leak(Box::new(s)))
    }
    pub fn start(&'static self, event_loop: EventLoop<()>) -> ! {
        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(_) => self.window_resized(),
                    WindowEvent::ScaleFactorChanged { .. } => self.window_resized(),
                    _ => {}
                }
                Event::MainEventsCleared => self.window.request_redraw(),
                Event::RedrawRequested(_) => {
                    let scene = self.scene.lock().unwrap();
                    let camera = self.camera.lock().unwrap();

                    {
                        let depth_texture = self.depth_texture.lock().unwrap();
                        let output_texture = match self.surface.get_current_texture() {
                            Ok(v) => v,
                            Err(wgpu::SurfaceError::Lost) | Err(wgpu::SurfaceError::Outdated) => return,
                            Err(e) => panic!("Error getting current surface texture: {}", e)
                        };
                        let view = output_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
                        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

                        {
                            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                label: None,
                                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                    view: &view,
                                    resolve_target: None,
                                    ops: wgpu::Operations {
                                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                        store: true
                                    }
                                })],
                                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                                    view: &depth_texture.view,
                                    depth_ops: Some(wgpu::Operations {
                                        load: wgpu::LoadOp::Clear(1.0),
                                        store: true
                                    }),
                                    stencil_ops: None
                                })
                            });
                            render_pass.set_bind_group(0, camera.bind_group(), &[]);
                            scene.render(self, render_pass)
                        }

                        self.queue.submit(std::iter::once(encoder.finish()));
                        output_texture.present();
                    }
                }
                _ => {}
            }
            if self.exit.load(std::sync::atomic::Ordering::Relaxed) {
                *control_flow = ControlFlow::Exit
            }
        })
    }
    pub fn window_resized(&self) {
        let size = self.window.inner_size();
        let mut surface_config = self.surface_config.lock().unwrap();
        surface_config.width = size.width;
        surface_config.height = size.height;
        self.surface.configure(&self.device, &surface_config);
        *self.depth_texture.lock().unwrap() = Texture::depth(&self.device, surface_config.width, surface_config.height);
    }
    pub fn load_scene<S: Script + 'static>(&'static self) {
        let v = Box::new(S::setup(self));
        *self.scene.lock().unwrap() = v
    }
    pub fn set_camera<C: Camera + 'static>(&'static self) {
        let v = Box::new(C::setup(self));
        *self.camera.lock().unwrap() = v
    }
    pub fn exit(&self) {
        self.exit.store(true, std::sync::atomic::Ordering::Relaxed)
    }
}

trait EnsureSync: Send + Sync {}
impl EnsureSync for Engine {}