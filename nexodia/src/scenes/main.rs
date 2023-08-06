use engine::{Object, Engine, Script, shader};
use wgpu::RenderPass;

pub struct Scene {
    cube: Object
}
impl Script for Scene {
    fn setup(e: &'static Engine) -> Self where Self: Sized {
        info!("Setup");
        e.set_camera::<crate::camera::Camera>();
        Self {
            cube: e.new_object(
                e.load_mesh("assets/cube.bin"),
                Box::new(shader::basic_f_p::Material {
                    color: [1.;4]
                })
            )
        }
    }
    fn event(&self, e: &'static Engine, event: Event<()>) {
        match event {
            _ => {}
        }
    }
    fn render<'r, 's: 'r>(&'s self, e: &'static Engine, mut render_pass: RenderPass<'r>) {
        e.render_object(&mut render_pass, &self.cube)
    }
}