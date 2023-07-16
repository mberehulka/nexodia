use ash::{Device, extensions::khr::{Surface, Swapchain}, vk::{SurfaceKHR, PhysicalDevice, SwapchainKHR}, Instance};
use winit::window::Window;

pub fn new(
    window: &Window,
    instance: &Instance,
    pdevice: PhysicalDevice,
    device: &Device,
    surface: &Surface,
    surface_khr: SurfaceKHR
) -> (Swapchain, SwapchainKHR) {
    let surface_format = unsafe{surface.get_physical_device_surface_formats(pdevice, surface_khr)}.unwrap()[0];

    let surface_capabilities = unsafe{surface.get_physical_device_surface_capabilities(pdevice, surface_khr)}.unwrap();
    let mut desired_image_count = surface_capabilities.min_image_count + 1;
    if surface_capabilities.max_image_count > 0 && desired_image_count > surface_capabilities.max_image_count {
        desired_image_count = surface_capabilities.max_image_count
    }
    let ws = window.inner_size();
    let surface_resolution = match surface_capabilities.current_extent.width {
        std::u32::MAX => ash::vk::Extent2D {
            width: ws.width,
            height: ws.height
        },
        _ => surface_capabilities.current_extent
    };
    let pre_transform = if surface_capabilities.supported_transforms.contains(ash::vk::SurfaceTransformFlagsKHR::IDENTITY) {
        ash::vk::SurfaceTransformFlagsKHR::IDENTITY
    } else {
        surface_capabilities.current_transform
    };
    let present_modes = unsafe{surface.get_physical_device_surface_present_modes(pdevice, surface_khr)}.unwrap();
    let present_mode = present_modes.iter().cloned()
        .find(|&mode| mode == ash::vk::PresentModeKHR::MAILBOX)
        .unwrap_or(ash::vk::PresentModeKHR::FIFO);
    let swapchain = Swapchain::new(&instance, &device);

    let swapchain_create_info = ash::vk::SwapchainCreateInfoKHR::builder()
        .surface(surface_khr)
        .min_image_count(desired_image_count)
        .image_color_space(surface_format.color_space)
        .image_format(surface_format.format)
        .image_extent(surface_resolution)
        .image_usage(ash::vk::ImageUsageFlags::COLOR_ATTACHMENT)
        .image_sharing_mode(ash::vk::SharingMode::EXCLUSIVE)
        .pre_transform(pre_transform)
        .composite_alpha(ash::vk::CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(present_mode)
        .clipped(true)
        .image_array_layers(1);

    let swapchain_khr = unsafe{swapchain.create_swapchain(&swapchain_create_info, None)}.unwrap();

    (swapchain, swapchain_khr)
}