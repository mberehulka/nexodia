use wgpu::{
    Extent3d, TextureDimension, TextureFormat, TextureUsages, TextureViewDescriptor, SamplerDescriptor,
    AddressMode, FilterMode, CompareFunction, TextureView, Sampler, Device, TextureDescriptor
};

pub const DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: TextureView,
    pub sampler: Sampler
}
impl Texture {
    pub fn depth(device: &Device, width: u32, height: u32) -> Self {
        let texture = device.create_texture(&TextureDescriptor {
            label: None,
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[]
        });
        let view = texture.create_view(&TextureViewDescriptor::default());
        let sampler = device.create_sampler(
            &SamplerDescriptor {
                address_mode_u: AddressMode::ClampToEdge,
                address_mode_v: AddressMode::ClampToEdge,
                address_mode_w: AddressMode::ClampToEdge,
                mag_filter: FilterMode::Linear,
                min_filter: FilterMode::Linear,
                mipmap_filter: FilterMode::Nearest,
                compare: Some(CompareFunction::LessEqual),
                lod_min_clamp: 0.0,
                lod_max_clamp: 100.0,
                ..Default::default()
            }
        );
        Self { texture, view, sampler }
    }
}