use ash::{Device, vk::{CommandBuffer, CommandPool}};

pub fn new(
    device: &Device,
    queue_family_index: u32
) -> (CommandPool, CommandBuffer, CommandBuffer) {
    let pool_create_info = ash::vk::CommandPoolCreateInfo::builder()
        .flags(ash::vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
        .queue_family_index(queue_family_index);
    let pool = unsafe{device.create_command_pool(&pool_create_info, None)}.unwrap();
    let command_buffer_allocate_info = ash::vk::CommandBufferAllocateInfo::builder()
        .command_buffer_count(2)
        .command_pool(pool)
        .level(ash::vk::CommandBufferLevel::PRIMARY);
    let command_buffers = unsafe{device.allocate_command_buffers(&command_buffer_allocate_info)}.unwrap();
    (pool, command_buffers[0], command_buffers[1])
}