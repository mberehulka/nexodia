use std::ffi::CString;
use ash::{extensions::ext::DebugUtils, Device};
use raw_window_handle::HasRawDisplayHandle;
use winit::window::Window;

pub mod window;
pub mod entry;
pub mod instance;
pub mod debug;
pub mod surface;
pub mod pdevice;
pub mod device;
pub mod swapchain;
pub mod cmd_buffer;
pub mod present_images;
mod depth_image;  pub use depth_image::*;

const REQUIRED_LAYERS: [&'static str; 1] = ["VK_LAYER_KHRONOS_validation"];

fn required_extension_names(window: &Window) -> Vec<*const i8> {
    let mut extension_names = ash_window::enumerate_required_extensions(window.raw_display_handle()).unwrap().to_vec();
    extension_names.push(DebugUtils::name().as_ptr());
    #[cfg(any(target_os = "macos", target_os = "ios"))] {
        extension_names.push(ash::vk::KhrPortabilityEnumerationFn::NAME.as_ptr());
        extension_names.push(ash::vk::KhrGetPhysicalDeviceProperties2Fn::NAME.as_ptr());
    }
    extension_names
}

fn get_layer_names_and_pointers() -> (Vec<CString>, Vec<*const i8>) {
    let layer_names = REQUIRED_LAYERS.iter().map(|name| CString::new(*name).unwrap()).collect::<Vec<_>>();
    let layer_names_ptrs = layer_names.iter().map(|name| name.as_ptr()).collect::<Vec<_>>();
    (layer_names, layer_names_ptrs)
}

pub unsafe fn record_submit_commandbuffer<F: FnOnce(&Device, ash::vk::CommandBuffer)>(
    device: &Device,
    command_buffer: ash::vk::CommandBuffer,
    command_buffer_reuse_fence: ash::vk::Fence,
    submit_queue: ash::vk::Queue,
    wait_mask: &[ash::vk::PipelineStageFlags],
    wait_semaphores: &[ash::vk::Semaphore],
    signal_semaphores: &[ash::vk::Semaphore],
    f: F
) {
    device.wait_for_fences(&[command_buffer_reuse_fence], true, std::u64::MAX).unwrap();

    device.reset_fences(&[command_buffer_reuse_fence]).unwrap();

    device.reset_command_buffer(
            command_buffer,
            ash::vk::CommandBufferResetFlags::RELEASE_RESOURCES,
        ).unwrap();

    let command_buffer_begin_info = ash::vk::CommandBufferBeginInfo::builder()
        .flags(ash::vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

    device.begin_command_buffer(command_buffer, &command_buffer_begin_info).unwrap();
    f(device, command_buffer);
    device.end_command_buffer(command_buffer).unwrap();

    let command_buffers = vec![command_buffer];

    let submit_info = ash::vk::SubmitInfo::builder()
        .wait_semaphores(wait_semaphores)
        .wait_dst_stage_mask(wait_mask)
        .command_buffers(&command_buffers)
        .signal_semaphores(signal_semaphores)
        .build();

    device.queue_submit(submit_queue, &[submit_info], command_buffer_reuse_fence).unwrap();
}