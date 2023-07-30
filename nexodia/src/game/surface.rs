pub fn configure(
    window_size: &winit::dpi::PhysicalSize<u32>,
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