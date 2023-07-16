use std::{ffi::CStr, borrow::Cow, fmt::Debug};
use ash::{vk::{DebugUtilsMessageSeverityFlagsEXT, PhysicalDeviceType, DebugUtilsMessengerEXT}, extensions::ext::DebugUtils, Entry, Instance};

pub unsafe extern "system" fn vulkan_debug_callback(
    message_severity: DebugUtilsMessageSeverityFlagsEXT,
    _message_type: ash::vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const ash::vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::os::raw::c_void,
) -> ash::vk::Bool32 {
    let message = if (*p_callback_data).p_message.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr((*p_callback_data).p_message).to_string_lossy()
    };
    match message_severity {
        DebugUtilsMessageSeverityFlagsEXT::VERBOSE =>
            trace!("{message}"),
        DebugUtilsMessageSeverityFlagsEXT::INFO =>
            info!("{message}"),
        DebugUtilsMessageSeverityFlagsEXT::WARNING =>
            warn!("{message}"),
        DebugUtilsMessageSeverityFlagsEXT::ERROR =>
            error!("{message}"),
        _ => unimplemented!("unimplemented: {message}")
    }
    ash::vk::FALSE
}

pub struct DebugPhysicalDeviceProps<'s>(pub &'s ash::vk::PhysicalDeviceProperties);
impl<'s> Debug for DebugPhysicalDeviceProps<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("")
            .field("api_version", &self.0.api_version)
            .field("device_id", &self.0.device_id)
            .field("api_version", &unsafe {CStr::from_ptr(self.0.device_name.as_ptr())})
            .field("device_type", &match self.0.device_type {
                PhysicalDeviceType::CPU => "CPU",
                PhysicalDeviceType::DISCRETE_GPU => "DISCRETE_GPU",
                PhysicalDeviceType::INTEGRATED_GPU => "INTEGRATED_GPU",
                PhysicalDeviceType::VIRTUAL_GPU => "VIRTUAL_GPU",
                PhysicalDeviceType::OTHER => "OTHER",
                _ => "UNKNOW"
            }.to_string())
            .field("driver_version", &self.0.driver_version)
            .field("vendor_id", &self.0.vendor_id)
            .finish()
    }
}

pub fn init(entry: &Entry, instance: &Instance) -> (DebugUtils, DebugUtilsMessengerEXT) {
    let debug_info = ash::vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(
            ash::vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE |
            ash::vk::DebugUtilsMessageSeverityFlagsEXT::INFO |
            ash::vk::DebugUtilsMessageSeverityFlagsEXT::WARNING |
            ash::vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
        )
        .message_type(
            ash::vk::DebugUtilsMessageTypeFlagsEXT::DEVICE_ADDRESS_BINDING |
            ash::vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE |
            ash::vk::DebugUtilsMessageTypeFlagsEXT::GENERAL |
            ash::vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
        )
        .pfn_user_callback(Some(vulkan_debug_callback));
    let debug_utils = DebugUtils::new(&entry, &instance);
    let debug_call_back = unsafe{debug_utils.create_debug_utils_messenger(&debug_info, None)}.unwrap();
    (debug_utils, debug_call_back)
}