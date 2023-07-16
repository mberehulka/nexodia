use winit::{
    event_loop::{EventLoop, ControlFlow},
    event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode}
};

use crate::game::Game;

pub struct GameBuilder {
    pub game: &'static Game,
    pub event_loop: EventLoop<()>
}
impl GameBuilder {
    pub fn build(self) {
        let game = self.game;
        self.event_loop.run(move |e, _window_target, control_flow| {
            *control_flow = ControlFlow::Wait;
            match e {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput { state, virtual_keycode, .. }, ..
                    } => if let Some(virtual_keycode) = virtual_keycode {
                        match state {
                            ElementState::Pressed => match virtual_keycode {
                                VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
                                _ => {}
                            }
                            ElementState::Released => match virtual_keycode {
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
                Event::MainEventsCleared => game.window.request_redraw(),
                Event::RedrawRequested(_) => {
                }
                _ => {}
            }
        })
    }
}