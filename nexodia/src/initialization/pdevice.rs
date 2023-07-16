use std::ffi::CStr;

use ash::{Instance, extensions::khr::Surface, vk::{SurfaceKHR, PhysicalDevice}};

pub fn new(
    instance: &Instance,
    surface: &Surface,
    surface_khr: SurfaceKHR
) -> (PhysicalDevice, u32) {
    unsafe{instance.enumerate_physical_devices()}.unwrap()
        .into_iter()
        .map(|pdevice| {
            let props = unsafe{instance.get_physical_device_properties(pdevice)};
            info!("Found device: {}", unsafe{CStr::from_ptr(props.device_name.as_ptr())}.to_string_lossy());
            unsafe{instance.get_physical_device_queue_family_properties(pdevice)}
                .into_iter().enumerate()
                .find_map(|(index, info)| {
                    if
                        info.queue_flags.contains(ash::vk::QueueFlags::GRAPHICS) &&
                        unsafe{surface.get_physical_device_surface_support(pdevice, index as u32, surface_khr)}.unwrap()
                    {
                        Some((pdevice, index as u32))
                    } else {
                        None
                    }
                })
        })
        .filter_map(|v|v).next()
        .expect("No suitable device found")
}