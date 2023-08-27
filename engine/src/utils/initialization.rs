use wgpu::{Instance, Adapter, Device, Queue, Surface};
use winit::{event_loop::EventLoop, window::{Window, WindowBuilder}, dpi::{PhysicalSize, PhysicalPosition}};

pub fn new_device(adapter: &Adapter) -> (Device, Queue) {
    pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
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
    let adapters = instance.enumerate_adapters(wgpu::Backends::all())
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
    let format = caps.formats.into_iter()
        .filter(|format| {
            info!("Format: {format:?} {}", format.is_srgb());
            format.is_srgb()
        })
        .collect::<Vec<_>>()
        .into_iter().next().unwrap();
    for pm in caps.present_modes {
        info!("PM: {pm:?}")
    }
    let alpha_mode = caps.alpha_modes.into_iter()
        .map(|am| {
            info!("AM: {am:?}");
            am
        })
        .collect::<Vec<_>>()
        .into_iter().next().unwrap();
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: window_size.width,
        height: window_size.height,
        present_mode: wgpu::PresentMode::AutoVsync,
        alpha_mode,
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