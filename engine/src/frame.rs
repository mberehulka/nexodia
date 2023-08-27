use std::sync::Arc;

use wgpu::{SurfaceTexture, TextureView, CommandEncoder, RenderPass};

use crate::{Engine, DepthTexture};

pub struct Frame {
    pub depth_texture: Arc<DepthTexture>,
    pub output_texture: SurfaceTexture,
    pub view: TextureView,
    pub encoder: CommandEncoder
}
impl Frame {
    pub fn new(e: &Engine) -> Option<Self> {
        let depth_texture = e.depth_texture.lock().unwrap().clone();
        let output_texture = match e.surface.get_current_texture() {
            Ok(v) => v,
            Err(wgpu::SurfaceError::Lost) | Err(wgpu::SurfaceError::Outdated) => return None,
            Err(e) => panic!("Error getting current surface texture: {}", e)
        };
        let view = output_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let encoder = e.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        Some(Self {
            depth_texture,
            output_texture,
            view,
            encoder
        })
    }
    #[inline(always)]
    pub fn new_render_pass(&mut self, depth: bool, clear: bool) -> RenderPass {
        self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: if clear {
                        wgpu::LoadOp::Clear(wgpu::Color::BLACK)
                    } else {
                        wgpu::LoadOp::Load
                    },
                    store: true
                }
            })],
            depth_stencil_attachment: if depth {
                Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: if clear {
                            wgpu::LoadOp::Clear(1.)
                        } else {
                            wgpu::LoadOp::Load
                        },
                        store: true
                    }),
                    stencil_ops: None
                })
            } else {
                None
            }
        })
    }
}