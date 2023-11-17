use math::Vec2;
use wgpu::{Buffer, util::DeviceExt, BufferUsages, RenderPass, CommandEncoder, TextureView};
use winit::dpi::PhysicalPosition;

use crate::Engine;

pub mod initialization;
pub mod pressed_keys;
mod color;  pub use color::*;
mod id;     pub use id::*;

impl Engine {
    pub fn window_size(&self) -> Vec2 {
        let ws = self.window.inner_size();
        Vec2::new(ws.width as f32, ws.height as f32)
    }
    pub fn new_buffer(&self, contents: &[u8], usage: BufferUsages) -> Buffer {
        self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents,
                usage
            }
        )
    }
    pub fn center_cursor(&self) {
        let s = self.window.inner_size();
        self.window.set_cursor_position(PhysicalPosition {
            x: s.width/2,
            y: s.height/2
        }).ok();
    }
}

pub fn new_render_pass<'s>(
    encoder: &'s mut CommandEncoder,
    color: Option<&'s TextureView>,
    depth: Option<&'s TextureView>
) -> RenderPass<'s> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: &if let Some(view) = color {
            vec![
                Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true
                    }
                })
            ]
        } else {
            vec![]
        },
        depth_stencil_attachment: if let Some(view) = depth {
            Some(wgpu::RenderPassDepthStencilAttachment {
                view,
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