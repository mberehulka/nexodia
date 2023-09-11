use wgpu::{RenderPass, Color, CommandEncoder};

use crate::{Engine, DepthTexture, OutputTexture};

pub struct Frame {
    e: &'static Engine,
    depth_texture: Option<DepthTexture>,
    output_texture: OutputTexture,
    encoder: Option<CommandEncoder>
}
impl Frame {
    pub fn new(e: &'static Engine, depth: bool) -> Self {
        Self {
            e,
            depth_texture: if depth { Some(e.new_depth_texture()) } else { None },
            output_texture: e.new_output_texture(),
            encoder: Some(e.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default()))
        }
    }
    pub fn window_resized(&mut self) {
        if let Some(depth_texture) = &mut self.depth_texture {
            *depth_texture = self.e.new_depth_texture()
        }
        self.output_texture = self.e.new_output_texture();
        self.encoder = Some(self.e.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default()))
    }
    pub fn new_render_pass(&mut self, clear: bool) -> RenderPass<'_> {
        self.encoder.as_mut().unwrap().begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.output_texture.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: if clear { wgpu::LoadOp::Clear(Color::BLACK) } else { wgpu::LoadOp::Load },
                    store: true
                }
            })],
            depth_stencil_attachment: if let Some(depth_texture) = &self.depth_texture {
                Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: true
                    }),
                    stencil_ops: None
                })
            } else {
                None
            }
        })
    }
    pub fn render(&mut self) {
        let output_texture = if let Ok(v) = self.e.surface.get_current_texture() { v } else { return };
        assert!(output_texture.texture.size() == self.output_texture.texture.size());
        let mut encoder = if let Some(v) = self.encoder.replace(
            self.e.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default())
        ) { v } else { return };
        encoder.copy_texture_to_texture(
            wgpu::ImageCopyTextureBase {
                texture: &self.output_texture.texture,
                mip_level: 0,
                origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                aspect: wgpu::TextureAspect::All
            },
            wgpu::ImageCopyTextureBase {
                texture: &output_texture.texture,
                mip_level: 0,
                origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                aspect: wgpu::TextureAspect::All
            },
            output_texture.texture.size()
        );
        self.e.queue.submit(std::iter::once(encoder.finish()));
        output_texture.present();
    }
}