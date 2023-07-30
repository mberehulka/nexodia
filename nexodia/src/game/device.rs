use wgpu::{Device, Adapter, Queue};

pub fn new(adapter: &Adapter) -> (Device, Queue) {
    pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            label: None
        },
        None
    )).unwrap()
}