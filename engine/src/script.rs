use std::{sync::{Mutex, Arc, mpsc::{channel, Receiver}, atomic::AtomicU64}, thread::Thread, collections::HashMap};
use winit::event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode};

use crate::{Engine, Frame};

#[allow(unused_variables)]
pub trait Script: Send + Sync {
    fn new(e: &'static Engine) -> Self where Self: Sized;
    fn event(&mut self, event: Event<'static, ()>) {}
    fn update(&mut self) {}
    fn render(&mut self, frame: &mut Frame) {}
    fn on_key_press(&mut self, key: VirtualKeyCode) {}
    fn on_key_up(&mut self, key: VirtualKeyCode) {}
    fn dropped(&mut self){}
}

pub enum ThreadEvent {
    Event(Event<'static, ()>),
    Ready,
    Close
}

#[derive(Clone)]
pub struct ScriptThread {
    pub id: u64,
    thread: Thread,
    events: Arc<Mutex<Vec<ThreadEvent>>>,
    rx: Arc<Mutex<Receiver<()>>>,
    pub script: Arc<Mutex<dyn Script>>
}
impl ScriptThread {
    pub fn new<S: Script + 'static>(e: &'static Engine, id: u64) -> ScriptThread {
        let events = Arc::new(Mutex::new(Vec::<ThreadEvent>::new()));
        let (tx, rx) = channel();
        let script = Arc::new(Mutex::new(S::new(e)));
        ScriptThread {
            id,
            script: script.clone(),
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
                                }, .. }, .. } => script.lock().unwrap().on_key_press(key),
                                Event::WindowEvent { event: WindowEvent::KeyboardInput { input: KeyboardInput {
                                    state: ElementState::Released, virtual_keycode: Some(key), ..
                                }, .. }, .. } => script.lock().unwrap().on_key_up(key),
                                Event::RedrawRequested(_) => script.lock().unwrap().update(),
                                _ => script.lock().unwrap().event(event)
                            },
                            ThreadEvent::Ready => tx.send(()).unwrap(),
                            ThreadEvent::Close => break
                        }
                    } else {
                        std::thread::park()
                    }
                }
                script.lock().unwrap().dropped()
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
        self.rx.lock().unwrap().recv().unwrap()
    }
}

#[derive(Default)]
pub struct Scripts {
    pub(crate) threads: Mutex<HashMap<u64, ScriptThread>>,
    id: AtomicU64
}
impl Engine {
    pub fn add_script<S: Script + 'static>(&'static self) -> ScriptThread {
        let id = self.scripts.id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let thread = ScriptThread::new::<S>(self, id);
        self.scripts.threads.lock().unwrap().insert(id, thread.clone());
        thread
    }
    pub fn remove_script(&self, st: ScriptThread) {
        st.send(crate::ThreadEvent::Close);
        self.scripts.threads.lock().unwrap().remove(&st.id).expect("Thread already removed");
    }
}