use ash::{extensions::khr::Swapchain, vk::{SwapchainKHR, ComponentSwizzle, ImageAspectFlags, Format, Image}, Device};

pub fn new(
    device: &Device,
    swapchain: &Swapchain,
    swapchain_khr: SwapchainKHR,
    surface_format: Format
) -> (Vec<Image>, Vec<ash::vk::ImageView>) {
    let images = unsafe{swapchain.get_swapchain_images(swapchain_khr)}.unwrap();
    let views = images.iter().map(|&image| {
            let create_view_info = ash::vk::ImageViewCreateInfo::builder()
                .view_type(ash::vk::ImageViewType::TYPE_2D)
                .format(surface_format)
                .components(ash::vk::ComponentMapping {
                    r: ComponentSwizzle::R,
                    g: ComponentSwizzle::G,
                    b: ComponentSwizzle::B,
                    a: ComponentSwizzle::A
                })
                .subresource_range(ash::vk::ImageSubresourceRange {
                    aspect_mask: ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                })
                .image(image);
            unsafe{device.create_image_view(&create_view_info, None)}.unwrap()
        }).collect();
    (images, views)
}