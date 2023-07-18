use std::ffi::CString;
use ash::{Instance, vk::make_api_version, Entry};
use winit::window::Window;

pub fn new(window: &Window, entry: &Entry) -> Instance {
    let app_info = ash::vk::ApplicationInfo::builder()
        .application_name(CString::new("Nexodia").unwrap().as_c_str())
        .engine_name(CString::new("No Engine").unwrap().as_c_str())
        .build();

    let extension_names = super::required_extension_names(window);

    let (_layer_names, layer_names_ptrs) = super::get_layer_names_and_pointers();

    let instance_create_info = ash::vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_extension_names(&extension_names)
        .flags(if cfg!(any(target_os = "macos", target_os = "ios")) {
            ash::vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
        } else {
            ash::vk::InstanceCreateFlags::default()
        })
        .enabled_layer_names(&layer_names_ptrs);

    unsafe { entry.create_instance(&instance_create_info, None).unwrap() }
}