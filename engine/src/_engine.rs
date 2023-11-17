use std::sync::{Mutex, atomic::AtomicBool};
use math::MVec2;
use wgpu::{Instance, Surface, Adapter, Device, Queue, SurfaceConfiguration, TextureUsages, CommandEncoder};
use winit::{
    event::{Event, WindowEvent, ElementState, KeyboardInput},
    event_loop::{ControlFlow, EventLoop}, window::Window, dpi::PhysicalSize
};

use crate::{
    utils::{initialization::*, pressed_keys::PressedKeys},
    CameraBuffer, Logger, Time, DepthTexture, OutputTexture, Script, ScriptEvent, ScriptInstance
};

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
    pub depth_texture: Mutex<DepthTexture>,
    pub output_texture: Mutex<OutputTexture>,
    pub cursor_movement: MVec2,
    current_scene: Mutex<Option<ScriptInstance<()>>>
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
        let depth_texture = DepthTexture::new(&device, surface_config.width, surface_config.height, TextureUsages::RENDER_ATTACHMENT).into();
        let output_texture = OutputTexture::new(&device, surface_config.width, surface_config.height, surface_config.format).into();
        let s = Self {
            window,
            instance: instance,
            device,
            queue,
            surface,
            surface_config: surface_config.into(),
            adapter: adapter,
            exit: Default::default(),
            pressed_keys: Default::default(),
            camera_buffer,
            time: Time::new(),
            depth_texture,
            output_texture,
            cursor_movement: Default::default(),
            current_scene: Default::default()
        };
        (event_loop, Box::leak(Box::new(s)))
    }
    pub fn start(&'static self, event_loop: EventLoop<()>) -> ! {
        self.window.set_visible(true);
        let mut window_resized = None;
        let mut window_focus = None;
        event_loop.run(move |event, _, control_flow| {
            if self.exit.load(std::sync::atomic::Ordering::Relaxed) {
                self.close_threads();
                return *control_flow = ControlFlow::Exit
            }
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
                Event::WindowEvent { event: WindowEvent::Resized(new_size), .. } => window_resized = Some(new_size),
                Event::WindowEvent { event: WindowEvent::CursorMoved { position, .. }, .. } => {
                    let ws = self.window_size();
                    self.cursor_movement.add_x(position.x as f32 - (ws.x / 2.));
                    self.cursor_movement.add_y(position.y as f32 - (ws.y / 2.));
                },
                Event::WindowEvent { event: WindowEvent::Focused(focus), .. } => window_focus = Some(focus),
                Event::MainEventsCleared => {
                    self.time.update();

                    if let Some(new_size) = window_resized.take() {
                        self.emit_event(ScriptEvent::Nothing);
                        self.resize(new_size);
                        self.emit_events(vec![
                            ScriptEvent::WindowResized,
                            ScriptEvent::Nothing
                        ]);
                    }

                    let mut update_events = vec![ ScriptEvent::Update ];
                    if let Some(focus) = window_focus {
                        if focus { update_events.push(ScriptEvent::WindowFocus) }
                        else { update_events.push(ScriptEvent::WindowBlur) }
                    }
                    self.emit_events(update_events);
                    
                    self.emit_events(vec![
                        ScriptEvent::Render,
                        ScriptEvent::Nothing
                    ]);

                    self.window.request_redraw();
                    self.cursor_movement.set(0.);
                },
                Event::RedrawRequested(_) => {}
                _ => {}
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
        *self.depth_texture.lock().unwrap() = DepthTexture::new(
            &self.device,
            surface_config.width,
            surface_config.height,
            TextureUsages::RENDER_ATTACHMENT
        );
        *self.output_texture.lock().unwrap() = OutputTexture::new(
            &self.device,
            surface_config.width,
            surface_config.height,
            surface_config.format
        );
    }
    pub fn exit(&self) {
        self.exit.store(true, std::sync::atomic::Ordering::Relaxed)
    }
    pub fn encoder(&self) -> CommandEncoder {
        self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default())
    }
    pub fn present_output_texture(&self, mut encoder: CommandEncoder) {
        let output_texture = self.output_texture.lock().unwrap();
        let surface_texture = match self.surface.get_current_texture() {
            Ok(v) => v,
            Err(e) => return error!("{e}")
        };
        assert!(surface_texture.texture.size() == output_texture.texture.size());
        encoder.copy_texture_to_texture(
            wgpu::ImageCopyTextureBase {
                texture: &output_texture.texture,
                mip_level: 0,
                origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                aspect: wgpu::TextureAspect::All
            },
            wgpu::ImageCopyTextureBase {
                texture: &surface_texture.texture,
                mip_level: 0,
                origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                aspect: wgpu::TextureAspect::All
            },
            surface_texture.texture.size()
        );
        self.queue.submit(std::iter::once(encoder.finish()));
        surface_texture.present()
    }
    pub fn render(&self, f: impl FnOnce(&mut CommandEncoder)) {
        let mut encoder = self.encoder();
        f(&mut encoder);
        self.present_output_texture(encoder);
    }
    pub fn set_scene<S: Script<'static, Return = ()> + 'static>(&'static self, params: S::Params) {
        std::thread::spawn(move || {
            if let Some(thread) = self.current_scene.lock().unwrap().take() {
                self.close_thread_sync(thread)
            }
            let script = self.new_script::<S>(params);
            *self.current_scene.lock().unwrap() = Some(script);
        });
    }
}