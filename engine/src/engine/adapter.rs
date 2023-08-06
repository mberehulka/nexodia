use wgpu::{Adapter, Instance, Surface};

pub fn new(instance: &Instance, surface: &Surface) -> Adapter {
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