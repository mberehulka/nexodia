use std::{path::Path, sync::Arc};

use wgpu::{
    Extent3d, TextureDimension, TextureFormat, TextureViewDescriptor, SamplerDescriptor,
    AddressMode, FilterMode, CompareFunction, TextureView, Sampler, Device, TextureDescriptor, BindGroup, SurfaceConfiguration
};

use crate::{Engine, decode};

pub const DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float;

#[derive(Clone)]
pub struct Texture {
    pub texture: Arc<wgpu::Texture>,
    pub view: Arc<TextureView>,
    pub sampler: Arc<Sampler>,
    pub bind_group: Arc<BindGroup>
}
impl Texture {
    pub fn bind_group(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true }
                    },
                    count: None
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None
                }
            ]
        })
    }
}
impl Engine {
    pub fn load_texture(&self, path: impl AsRef<Path>) -> Texture {
        let image: compiler::Image = decode(&path);
        let width = image.width;
        let height = image.height;
        
        let texture_size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1
        };
        let texture = self.device.create_texture(
            &wgpu::TextureDescriptor {
                label: None,
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[]
            }
        );
        self.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All
            },
            &image.get_pixels_rgba(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height)
            },
            texture_size
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let bind_group = self.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: None,
                layout: &Texture::bind_group(&self.device),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&view)
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler)
                    }
                ]
            }
        );

        Texture {
            bind_group: bind_group.into(),
            texture: texture.into(),
            view: view.into(),
            sampler: sampler.into()
        }
    }
}

pub struct DepthTexture {
    pub texture: wgpu::Texture,
    pub view: TextureView,
    pub sampler: Sampler
}
impl DepthTexture {
    pub fn new(device: &Device, surface_config: &SurfaceConfiguration) -> Self {
        let texture = device.create_texture(&TextureDescriptor {
            label: None,
            size: Extent3d {
                width: surface_config.width,
                height: surface_config.height,
                depth_or_array_layers: 1
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
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
        Self {
            texture,
            view,
            sampler
        }
    }
}
impl Engine {
    pub fn new_depth_texture(&self) -> DepthTexture {
        DepthTexture::new(&self.device, &self.surface_config.lock().unwrap())
    }
}

pub struct OutputTexture {
    pub texture: wgpu::Texture,
    pub view: TextureView
}
impl OutputTexture {
    pub fn new(device: &Device, surface_config: &SurfaceConfiguration) -> Self {
        let texture = device.create_texture(&TextureDescriptor {
            label: None,
            size: Extent3d {
                width: surface_config.width,
                height: surface_config.height,
                depth_or_array_layers: 1
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: surface_config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[]
        });
        let view = texture.create_view(&TextureViewDescriptor::default());
        Self {
            texture,
            view
        }
    }
}
impl Engine {
    pub fn new_output_texture(&self) -> OutputTexture {
        OutputTexture::new(&self.device, &self.surface_config.lock().unwrap())
    }
}