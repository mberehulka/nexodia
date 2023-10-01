use wgpu::BindGroup;

pub trait Material: Sized {
    fn bind_group(&self) -> &BindGroup;
    fn set<'r, 's: 'r>(&'s self, render_pass: &mut wgpu::RenderPass<'r>, index: u32) {
        render_pass.set_bind_group(index, self.bind_group(), &[])
    }
}

impl Material for () {
    fn bind_group(&self) -> &BindGroup { unreachable!() }
    fn set<'r, 's: 'r>(&'s self, _render_pass: &mut wgpu::RenderPass<'r>, _index: u32) {}
}