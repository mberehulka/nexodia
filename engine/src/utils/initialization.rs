use wgpu::{Instance, Adapter, Device, Queue, Surface, Features, Dx12Compiler};
use winit::{event_loop::EventLoop, window::{Window, WindowBuilder}, dpi::{PhysicalSize, PhysicalPosition}};

use crate::Engine;

pub fn new_instance() -> wgpu::Instance {
    wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all() - wgpu::Backends::GL,
        dx12_shader_compiler: Dx12Compiler::Fxc
    })
}

pub fn new_device(adapter: &Adapter) -> (Device, Queue) {
    let mut features = adapter.features();
    features.remove(Features::MAPPABLE_PRIMARY_BUFFERS);
    features.insert(Features::PARTIALLY_BOUND_BINDING_ARRAY);
    features.insert(Features::TEXTURE_BINDING_ARRAY);
    features.insert(Features::BUFFER_BINDING_ARRAY);
    features.insert(Features::STORAGE_RESOURCE_BINDING_ARRAY);
    println!("Using device features: {features:#?}");
    pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            features,
            limits: wgpu::Limits::default(),
            label: None
        },
        None
    )).unwrap()
}

pub fn new_adapter(instance: &Instance, surface: &Surface) -> Adapter {
    pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false
    })).unwrap();
    let adapters = instance.enumerate_adapters(wgpu::Backends::VULKAN)
        .filter(|adapter| {
            let info = adapter.get_info();
            let supported = adapter.is_surface_supported(surface);
            info!("{:?}: {} ({}, {}), {:?} ({}), {supported}",
                info.device_type,
                info.name, info.vendor, info.device,
                info.backend, info.driver_info
            );
            supported
        })
        .collect::<Vec<_>>();
    adapters.into_iter()
        .next()
        .unwrap()
}

pub fn configure_surface(
    window_size: winit::dpi::PhysicalSize<u32>,
    device: &wgpu::Device,
    adapter: &wgpu::Adapter,
    surface: &wgpu::Surface
) -> wgpu::SurfaceConfiguration {
    let caps = surface.get_capabilities(adapter);
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_DST,
        format: caps.formats.into_iter().find(|v| v.is_srgb()).unwrap(),
        width: window_size.width,
        height: window_size.height,
        present_mode: wgpu::PresentMode::AutoNoVsync,
        alpha_mode: caps.alpha_modes.into_iter().next().unwrap(),
        view_formats: vec![]
    };
    surface.configure(device, &config);
    config
}

pub fn new_window(event_loop: &EventLoop<()>) -> Window {
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
        .with_visible(false)
        .build(event_loop).unwrap();
    w.focus_window();
    w
}

impl Engine {
    pub fn center_window(&self) {
        let ms = self.window.current_monitor().unwrap().size();
        let ws = self.window.inner_size();
        self.window.set_outer_position(PhysicalPosition {
            x: (ms.width - ws.width) / 2,
            y: (ms.height - ws.height) / 2
        })
    }
}