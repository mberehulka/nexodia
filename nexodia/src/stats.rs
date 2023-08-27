use engine::{Engine, Script, Reader};
use wgpu_text::{BrushBuilder, glyph_brush::{Section, Layout, VerticalAlign, ab_glyph::{FontArc, FontVec}, Text, HorizontalAlign}, TextBrush};
use winit::event::{Event, WindowEvent};

pub struct Stats {
    e: &'static Engine,
    brush: TextBrush
}
impl Script for Stats {
    fn new(e: &'static Engine) -> Self {
        let font = Reader::new("assets/fonts/Roboto/Roboto-Regular.bin");
        let font = FontArc::new(FontVec::try_from_vec(font.get_rest()).unwrap());
        let ws = e.window.inner_size();
        let brush = BrushBuilder::using_font(font)
            .build(
                &e.device,
                ws.width,
                ws.height,
                e.surface_config.lock().unwrap().format
            );
        Self {
            e,
            brush: brush.into()
        }
    }
    fn event(&mut self, event: winit::event::Event<'static, ()>) {
        match event {
            Event::WindowEvent { event: WindowEvent::Resized(new_size), .. } => {
                self.brush.resize_view(new_size.width as f32, new_size.height as f32, &self.e.queue);
            }
            Event::RedrawRequested(_) => {
                let ws = self.e.window.inner_size();
                let section = Section::default()
                    .with_text(vec![
                        Text::new(&format!("{} fps", (1. / self.e.time.delta())as usize))
                            .with_scale(15.)
                            .with_color([1.;4])
                    ])
                    .with_bounds((ws.width as f32, ws.height as f32))
                    .with_layout(Layout::default().v_align(VerticalAlign::Top).h_align(HorizontalAlign::Right))
                    .with_screen_position((ws.width as f32 - 10., 10.))
                    .to_owned();
                self.brush.queue(&self.e.device, &self.e.queue, vec![&section]).unwrap();
            }
            _ => {}
        }
    }
    fn render(&mut self, frame: &mut engine::Frame) {
        let mut render_pass = frame.new_render_pass(false, false);
        self.brush.draw(&mut render_pass)
    }
}