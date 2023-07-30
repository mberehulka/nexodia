use winit::event_loop::EventLoop;

pub struct GameBuilder {
    event_loop: EventLoop<()>
}
impl GameBuilder {
    pub fn new() -> Self {
        Self {
            event_loop: EventLoop::new()
        }
    }
    pub fn build(self) -> ! {
        super::Game::new(&self.event_loop)
            .run(self.event_loop)
    }
}