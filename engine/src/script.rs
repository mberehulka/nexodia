use std::{sync::{Mutex, Arc, mpsc::{channel, Receiver}}, thread::Thread, collections::HashMap};
use winit::{event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode}, dpi::PhysicalSize};

use crate::utils::id::{Id, IdT};

#[allow(unused_variables)]
pub trait Script: Send + Sync {
    fn event(&mut self, event: Event<'static, ()>) {}
    fn update(&mut self) {}
    fn render(&mut self) {}
    fn on_key_press(&mut self, key: VirtualKeyCode) {}
    fn on_key_up(&mut self, key: VirtualKeyCode) {}
    fn window_resized(&mut self, new_size: PhysicalSize<u32>) {}
    fn dropped(&mut self){}
}

pub enum ThreadEvent {
    Event(Event<'static, ()>),
    Ready,
    Close
}

#[derive(Clone)]
pub struct ScriptHandler {
    id: IdT,
    scripts: &'static Scripts,
    thread: Thread,
    events: Arc<Mutex<Vec<ThreadEvent>>>,
    rx: Arc<Mutex<Receiver<()>>>
}
impl ScriptHandler {
    pub fn new<S: Script + 'static>(scripts: &'static Scripts, mut script: S, id: IdT) -> Self {
        let events = Arc::new(Mutex::new(Vec::<ThreadEvent>::new()));
        let (tx, rx) = channel();
        Self {
            id,
            scripts,
            events: events.clone(),
            thread: std::thread::spawn(move || {
                loop {
                    let mut events = events.lock().unwrap();
                    let recv = if events.len() > 0 { Some(events.remove(0)) } else { None };
                    drop(events);
                    if let Some(event) = recv {
                        match event {
                            ThreadEvent::Event(event) => match event {
                                Event::WindowEvent { event: WindowEvent::KeyboardInput { input: KeyboardInput {
                                    state: ElementState::Pressed, virtual_keycode: Some(key), ..
                                }, .. }, .. } =>
                                    script.on_key_press(key),
                                Event::WindowEvent { event: WindowEvent::KeyboardInput { input: KeyboardInput {
                                    state: ElementState::Released, virtual_keycode: Some(key), ..
                                }, .. }, .. } =>
                                    script.on_key_up(key),
                                Event::WindowEvent { event: WindowEvent::Resized(new_size), .. } =>
                                    script.window_resized(new_size),
                                Event::MainEventsCleared => script.update(),
                                Event::RedrawRequested(_) => script.render(),
                                _ => script.event(event)
                            },
                            ThreadEvent::Ready => tx.send(()).unwrap(),
                            ThreadEvent::Close => break
                        }
                    } else {
                        std::thread::park()
                    }
                }
                script.dropped();
                info!("Thread {id} dropped")
            }).thread().clone().into(),
            rx: Arc::new(Mutex::new(rx))
        }
    }
    pub fn send(&self, e: ThreadEvent) {
        self.events.lock().unwrap().push(e);
        self.thread.unpark()
    }
    pub fn wait(&self) {
        self.send(ThreadEvent::Ready);
        self.rx.lock().unwrap().recv().ok();
    }
}

impl Drop for ScriptHandler {
    fn drop(&mut self) {
        self.scripts.remove(self)
    }
}

static ID: Id = Id::default();

#[derive(Default)]
pub struct Scripts {
    pub(crate) threads: Mutex<HashMap<IdT, ScriptHandler>>
}
impl Scripts {
    pub fn add<S: Script + 'static>(&'static self, script: S) -> ScriptHandler {
        let id = ID.next();
        let handler = ScriptHandler::new(&self, script, id);
        self.threads.lock().unwrap().insert(id, handler.clone());
        handler
    }
    pub fn remove(&self, st: &ScriptHandler) {
        st.send(crate::ThreadEvent::Close);
        self.threads.lock().unwrap().remove(&st.id).expect("Thread already removed");
    }
}