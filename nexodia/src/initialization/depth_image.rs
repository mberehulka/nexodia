use ash::{vk::{Queue, Image, Extent2D, PhysicalDevice, MemoryPropertyFlags, ImageView, DeviceMemory, CommandBuffer, Fence}, Instance, Device};

pub struct DepthImage {
    pub image: Image,
    pub view: ImageView,
    pub memory: DeviceMemory,
    pub draw_commands_reuse_fence: Fence,
    pub setup_commands_reuse_fence: Fence
}
impl DepthImage {
    pub fn new(
        instance: &Instance,
        pdevice: PhysicalDevice,
        device: &Device,
        surface_resolution: Extent2D,
        setup_command_buffer: CommandBuffer,
        present_queue: Queue
    ) -> Self {
        let create_info = ash::vk::ImageCreateInfo::builder()
            .image_type(ash::vk::ImageType::TYPE_2D)
            .format(ash::vk::Format::D16_UNORM)
            .extent(surface_resolution.into())
            .mip_levels(1)
            .array_layers(1)
            .samples(ash::vk::SampleCountFlags::TYPE_1)
            .tiling(ash::vk::ImageTiling::OPTIMAL)
            .usage(ash::vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
            .sharing_mode(ash::vk::SharingMode::EXCLUSIVE);

        let image = unsafe{device.create_image(&create_info, None)}.unwrap();
        let memory_req = unsafe{device.get_image_memory_requirements(image)};
        let properties = unsafe{instance.get_physical_device_memory_properties(pdevice)};
        let flags = MemoryPropertyFlags::DEVICE_LOCAL;
        let depth_image_memory_index = properties.memory_types[..properties.memory_type_count as usize]
            .iter().enumerate()
            .find(|(index, memory_type)|
                (1 << index) & memory_req.memory_type_bits != 0 &&
                memory_type.property_flags & flags == flags
            ).map(|(index, _memory_type)| index as u32).unwrap();

        let depth_image_allocate_info = ash::vk::MemoryAllocateInfo::builder()
            .allocation_size(memory_req.size)
            .memory_type_index(depth_image_memory_index);

        let memory = unsafe{device.allocate_memory(&depth_image_allocate_info, None)}.unwrap();

        unsafe{device.bind_image_memory(image, memory, 0)}.unwrap();

        let fence_create_info = ash::vk::FenceCreateInfo::builder()
            .flags(ash::vk::FenceCreateFlags::SIGNALED);

        let draw_commands_reuse_fence = unsafe{device.create_fence(&fence_create_info, None)}.unwrap();
        let setup_commands_reuse_fence = unsafe{device.create_fence(&fence_create_info, None)}.unwrap();

        unsafe {crate::initialization::record_submit_commandbuffer(
            &device,
            setup_command_buffer,
            setup_commands_reuse_fence,
            present_queue,
            &[],
            &[],
            &[],
            |device, setup_command_buffer| {
                device.cmd_pipeline_barrier(
                    setup_command_buffer,
                    ash::vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                    ash::vk::PipelineStageFlags::LATE_FRAGMENT_TESTS,
                    ash::vk::DependencyFlags::empty(),
                    &[],
                    &[],
                    &[
                        ash::vk::ImageMemoryBarrier::builder()
                            .image(image)
                            .dst_access_mask(
                                ash::vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ |
                                ash::vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE
                            )
                            .new_layout(ash::vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
                            .old_layout(ash::vk::ImageLayout::UNDEFINED)
                            .subresource_range(
                                ash::vk::ImageSubresourceRange::builder()
                                    .aspect_mask(ash::vk::ImageAspectFlags::DEPTH)
                                    .layer_count(1)
                                    .level_count(1)
                                    .build()
                            )
                            .build()
                    ]
                );
            }
        )};

        let view_info = ash::vk::ImageViewCreateInfo::builder()
            .subresource_range(
                ash::vk::ImageSubresourceRange::builder()
                    .aspect_mask(ash::vk::ImageAspectFlags::DEPTH)
                    .level_count(1)
                    .layer_count(1)
                    .build()
            )
            .image(image)
            .format(create_info.format)
            .view_type(ash::vk::ImageViewType::TYPE_2D);

        let view = unsafe{device.create_image_view(&view_info, None)}.unwrap();
        
        Self {
            image,
            view,
            memory,
            draw_commands_reuse_fence,
            setup_commands_reuse_fence
        }
    }
}