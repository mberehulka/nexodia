use std::sync::Mutex;
use wgpu::{Instance, Surface, Adapter, Device, Queue, SurfaceConfiguration};
use winit::{
    event_loop::{EventLoop, ControlFlow},
    event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode},
    window::Window
};

use crate::assets::Texture;

mod builder;
mod window;
mod adapter;
mod device;
mod surface;

pub struct Game {
    window: Window,
    _instance: Instance,
    surface: Surface,
    _adapter: Adapter,
    device: Device,
    queue: Queue,
    surface_config: Mutex<SurfaceConfiguration>,
    depth_texture: Mutex<Texture>
}
impl Game {
    pub fn builder() -> builder::GameBuilder {
        builder::GameBuilder::new()
    }
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window = window::new(event_loop);
        let instance = Instance::new(wgpu::InstanceDescriptor::default());
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = adapter::new(&instance, &surface);
        let (device, queue) = device::new(&adapter);
        let surface_config = surface::configure(&window.inner_size(), &device, &adapter, &surface);
        let depth_texture = Texture::depth(&device, surface_config.width, surface_config.height);
        Self {
            window,
            _instance: instance,
            surface,
            _adapter: adapter,
            device,
            queue,
            surface_config: surface_config.into(),
            depth_texture: depth_texture.into()
        }
    }
    pub fn window_resized(&self) {
        let size = self.window.inner_size();
        let mut surface_config = self.surface_config.lock().unwrap();
        surface_config.width = size.width;
        surface_config.height = size.height;
        self.surface.configure(&self.device, &surface_config);
        *self.depth_texture.lock().unwrap() = Texture::depth(&self.device, surface_config.width, surface_config.height);
    }
    pub fn run(self, event_loop: EventLoop<()>) -> ! {
        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput { virtual_keycode, state, .. }, ..
                    } => if let Some(vkc) = virtual_keycode {
                        match state {
                            ElementState::Pressed => match vkc {
                                _ => {}
                            }
                            ElementState::Released => match vkc {
                                VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
                                _ => {}
                            }
                        }
                    }
                    WindowEvent::Resized(_) => self.window_resized(),
                    WindowEvent::ScaleFactorChanged { .. } => self.window_resized(),
                    _ => {}
                }
                Event::MainEventsCleared => self.window.request_redraw(),
                Event::RedrawRequested(_) => {
                    let depth_texture = self.depth_texture.lock().unwrap();
                    let output_texture = match self.surface.get_current_texture() {
                        Ok(v) => v,
                        Err(wgpu::SurfaceError::Lost) | Err(wgpu::SurfaceError::Outdated) => return,
                        Err(e) => panic!("Error getting current surface texture: {}", e)
                    };
                    let view = output_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
                    let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
                    {
                        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
                    }
                    self.queue.submit(std::iter::once(encoder.finish()));
                    output_texture.present();
                }
                _ => {}
            }
        })
    }
}

trait EnsureSync: Send + Sync {}
impl EnsureSync for Game {}