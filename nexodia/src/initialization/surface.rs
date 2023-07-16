use ash::{Entry, Instance, extensions::khr::Surface, vk::{SurfaceKHR, PhysicalDevice, Extent2D}};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use winit::window::Window;

pub fn new(
    window: &Window,
    entry: &Entry,
    instance: &Instance
) -> (Surface, SurfaceKHR) {
    let surface = Surface::new(entry, instance);
    let surface_khr = unsafe{ash_window::create_surface(
        &entry,
        &instance,
        window.raw_display_handle(),
        window.raw_window_handle(),
        None,
    )}.unwrap();
    (surface, surface_khr)
}
pub fn resolution(
    window: &Window,
    pdevice: PhysicalDevice,
    surface: &Surface,
    surface_khr: SurfaceKHR
) -> Extent2D {
    let surface_capabilities = unsafe{surface.get_physical_device_surface_capabilities(pdevice, surface_khr)}.unwrap();
    let window_size = window.inner_size();
    match surface_capabilities.current_extent.width {
        std::u32::MAX => ash::vk::Extent2D {
            width: window_size.width,
            height: window_size.height
        },
        _ => surface_capabilities.current_extent
    }
}