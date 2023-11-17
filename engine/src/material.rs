use wgpu::BindGroup;

pub trait Material: Sized {
    const BGI: u32 = 0;
    fn bind_group(&self) -> &BindGroup;
    fn set<'r, 's: 'r>(&'s self, render_pass: &mut wgpu::RenderPass<'r>) {
        render_pass.set_bind_group(Self::BGI, self.bind_group(), &[])
    }
}