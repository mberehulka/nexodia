use ash::{vk::PhysicalDevice, Instance, Device, extensions::khr::Swapchain};

pub fn new(
    instance: &Instance,
    pdevice: PhysicalDevice,
    queue_family_index: u32
) -> Device {
    
    let device_extension_names_raw = [
        Swapchain::name().as_ptr(),
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        KhrPortabilitySubsetFn::NAME.as_ptr(),
    ];
    let features = ash::vk::PhysicalDeviceFeatures {
        shader_clip_distance: 1,
        ..Default::default()
    };
    let priorities = [1.0];

    let queue_info = ash::vk::DeviceQueueCreateInfo::builder()
        .queue_family_index(queue_family_index)
        .queue_priorities(&priorities);

    let device_create_info = ash::vk::DeviceCreateInfo::builder()
        .queue_create_infos(std::slice::from_ref(&queue_info))
        .enabled_extension_names(&device_extension_names_raw)
        .enabled_features(&features);

    unsafe{instance.create_device(pdevice, &device_create_info, None)}.unwrap()
}