use ash::{
    extensions::{ext::DebugUtils, khr::{Surface, Swapchain}},
    vk::{DebugUtilsMessengerEXT, SurfaceKHR, PhysicalDevice, Queue, SwapchainKHR, CommandPool, CommandBuffer,
            Image, ImageView, Format, Extent2D, Semaphore},
    Entry, Instance, Device
};
use winit::{event_loop::EventLoop, window::Window};

use crate::{initialization::*, builder::GameBuilder};


#[allow(dead_code)]
pub struct Game {
    pub window: Window,
    pub entry: Entry,
    pub instance: Instance,
    pub debug_utils: DebugUtils,
    pub debug_call_back: DebugUtilsMessengerEXT,
    pub surface: Surface,
    pub surface_khr: SurfaceKHR,
    pub surface_format: Format,
    pub surface_resolution: Extent2D,
    pub pdevice: PhysicalDevice,
    pub device: Device,
    pub present_queue: Queue,
    pub swapchain: Swapchain,
    pub swapchain_khr: SwapchainKHR,
    pub pool: CommandPool,
    pub setup_command_buffer: CommandBuffer,
    pub draw_command_buffer: CommandBuffer,
    pub present_images: Vec<Image>,
    pub present_images_views: Vec<ImageView>,
    pub depth_image: DepthImage,
    pub present_complete_semaphore: Semaphore,
    pub rendering_complete_semaphore: Semaphore
}
impl Game {
    pub fn new(event_loop: &EventLoop<()>) -> &'static Self {
        let window = window::new(event_loop);
        let entry = entry::new();
        let instance = instance::new(&window, &entry);
        let (debug_utils, debug_call_back) = debug::init(&entry, &instance);
        let (surface, surface_khr) = surface::new(&window, &entry, &instance);
        let (pdevice, queue_family_index) = pdevice::new(&instance, &surface, surface_khr);
        let surface_format = unsafe{surface.get_physical_device_surface_formats(pdevice, surface_khr)}.unwrap()[0].format;
        let surface_resolution = surface::resolution(&window, pdevice, &surface, surface_khr);
        let device = device::new(&instance, pdevice, queue_family_index);
        let present_queue = unsafe{device.get_device_queue(queue_family_index, 0)};
        let (swapchain, swapchain_khr) = swapchain::new(&window, &instance, pdevice, &device, &surface, surface_khr);
        let (pool, setup_command_buffer, draw_command_buffer) = cmd_buffer::new(&device, queue_family_index);
        let (present_images, present_images_views) = present_images::new(&device, &swapchain, swapchain_khr, surface_format);
        let depth_image = DepthImage::new(&instance, pdevice, &device, surface_resolution, setup_command_buffer, present_queue);
        let semaphore_create_info = ash::vk::SemaphoreCreateInfo::default();
        let present_complete_semaphore = unsafe{device.create_semaphore(&semaphore_create_info, None)}.unwrap();
        let rendering_complete_semaphore = unsafe{device.create_semaphore(&semaphore_create_info, None)}.unwrap();
        Box::leak(Box::new(Self {
            window: window.into(),
            entry,
            instance,
            debug_utils,
            debug_call_back,
            surface,
            surface_khr,
            surface_format,
            surface_resolution,
            pdevice,
            device,
            present_queue,
            swapchain,
            swapchain_khr,
            pool,
            setup_command_buffer,
            draw_command_buffer,
            present_images,
            present_images_views,
            depth_image: depth_image.into(),
            present_complete_semaphore,
            rendering_complete_semaphore
        }))
    }
    pub fn builder() -> GameBuilder {
        let event_loop = EventLoop::new();
        GameBuilder {
            game: Self::new(&event_loop),
            event_loop
        }
    }
}
impl Drop for Game {
    fn drop(&mut self) {
        unsafe {
            self.device.device_wait_idle().unwrap();
            self.device.destroy_semaphore(self.present_complete_semaphore, None);
            self.device.destroy_semaphore(self.rendering_complete_semaphore, None);
            self.device.destroy_fence(self.depth_image.draw_commands_reuse_fence, None);
            self.device.destroy_fence(self.depth_image.setup_commands_reuse_fence, None);
            self.device.free_memory(self.depth_image.memory, None);
            self.device.destroy_image_view(self.depth_image.view, None);
            self.device.destroy_image(self.depth_image.image, None);
            for &image_view in self.present_images_views.iter() {
                self.device.destroy_image_view(image_view, None)
            }
            self.device.destroy_command_pool(self.pool, None);
            self.swapchain.destroy_swapchain(self.swapchain_khr, None);
            self.device.destroy_device(None);
            self.surface.destroy_surface(self.surface_khr, None);
            self.debug_utils.destroy_debug_utils_messenger(self.debug_call_back, None);
            self.instance.destroy_instance(None);
        }
    }
}