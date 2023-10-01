use std::time::Instant;
use engine::{Engine, Reader, Script};
use wgpu_text::{BrushBuilder, glyph_brush::{Section, Layout, VerticalAlign, ab_glyph::{FontArc, FontVec}, Text, HorizontalAlign}, TextBrush};

pub struct Stats {
    pub e: &'static Engine,
    pub brush: TextBrush,
    pub last_time: Instant
}
impl Stats {
    pub fn new(e: &'static Engine) -> Self {
        let font = Reader::new("assets/fonts/Roboto-Regular.bin");
        let font = FontArc::new(FontVec::try_from_vec(font.get_rest()).unwrap());
        let ws = e.window.inner_size();
        Self {
            e,
            brush: BrushBuilder::using_font(font).build(
                &e.device,
                ws.width,
                ws.height,
                e.surface_config.lock().unwrap().format
            ),
            last_time: Instant::now()
        }
    }
}
impl Script for Stats {
    fn update(&mut self) {
        let now = Instant::now();
        if (now - self.last_time).as_secs_f32() <= 1. / 10. { return }
        self.last_time = now;

        let ws = self.e.window.inner_size();
        let section = Section::default()
            .with_text(vec![
                Text::new(&format!("{} fps", (1. / self.e.time.delta())as usize ))
                    .with_scale(15.)
                    .with_color([1.;4])
            ])
            .with_bounds((ws.width as f32, ws.height as f32))
            .with_layout(Layout::default().v_align(VerticalAlign::Top).h_align(HorizontalAlign::Left))
            .with_screen_position((10., 10.))
            .to_owned();
        self.brush.queue(&self.e.device, &self.e.queue, vec![&section]).unwrap()
    }
    fn window_resized(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.brush.resize_view(new_size.width as f32, new_size.height as f32, &self.e.queue)
    }
    fn render(&mut self, frame: &mut engine::Frame) {
        let mut render_pass = frame.new_render_pass(false);
        self.brush.draw(&mut render_pass)
    }
}