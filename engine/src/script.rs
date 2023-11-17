use std::{sync::{Mutex, Arc}, collections::HashMap, borrow::BorrowMut, thread::JoinHandle};
use crossbeam_channel::{Sender, TrySendError};

use crate::{utils::{IdHandler, Id}, Engine};

lazy_static::lazy_static! {
    static ref THREADS: Mutex<HashMap<Id, ScriptThread>> = Default::default();
}
static ID: IdHandler = IdHandler::default();

#[derive(Debug, Clone, Copy)]
pub enum ScriptEvent {
    WindowResized,
    WindowFocus,
    WindowBlur,
    Update,
    Render,
    Nothing,
    Close
}

#[allow(unused_variables)]
pub trait Script<'s>: Send + Sized + 'static {
    type Params: Send;
    type Return: Send;
    const NAME: &'static str;
    fn new(e: &'static Engine, id: Id, params: Self::Params) -> (Self, Self::Return);
    fn window_focus(&mut self) {}
    fn window_blur(&mut self) {}
    fn update(&mut self) {}
    fn render(&mut self) {}
    fn window_resized(&mut self) {}
    fn dropped(self) {}
}

/// Close script's thread when dropped
pub struct ScriptInstance<T>(pub T, pub Id);
impl<T> Drop for ScriptInstance<T> {
    fn drop(&mut self) {
        let mut threads = THREADS.lock().unwrap();
        let thread = threads.remove(&self.1);
        drop(threads);
        if let Some(thread) = thread {
            loop {
                match thread.tx.try_send(ScriptEvent::Close) {
                    Ok(_) => break,
                    Err(e) => match e {
                        TrySendError::Disconnected(_) => break,
                        TrySendError::Full(_) => continue
                    }
                }
            }
        }
    }
}

pub(crate) struct ScriptThread {
    tx: Sender<ScriptEvent>,
    jh: JoinHandle<()>
}

impl Engine {
    pub fn new_script<'s, S: Script<'s>>(&'static self, params: S::Params) -> ScriptInstance<S::Return> {
        let id = ID.next();
        let (tx, rx) = crossbeam_channel::bounded(0);
        let (script, r) = S::new(self, id, params);
        let script = Arc::new(Mutex::new(Some(script)));
        let jh = std::thread::spawn(move || {
            let mut script = script.lock().unwrap().borrow_mut().take().unwrap();
            loop {
                match rx.recv().unwrap() {
                    ScriptEvent::WindowFocus => script.window_focus(),
                    ScriptEvent::WindowBlur => script.window_blur(),
                    ScriptEvent::Update => script.update(),
                    ScriptEvent::Render => script.render(),
                    ScriptEvent::WindowResized => script.window_resized(),
                    ScriptEvent::Nothing => {}
                    ScriptEvent::Close => {
                        script.dropped();
                        break
                    }
                }
            }
            info!("Script {}, thread {}, dropped", S::NAME, id);
        });
        THREADS.lock().unwrap().insert(id, ScriptThread { tx, jh });
        ScriptInstance(r, id)
    }
    pub(crate) fn emit_event(&self, event: ScriptEvent) {
        let threads = THREADS.lock().unwrap();
        let mut threads = threads.values().collect::<Vec<_>>();
        while threads.len() > 0 {
            threads.retain(|thread| {
                match thread.tx.try_send(event) {
                    Ok(_) => false,
                    Err(e) => match e {
                        TrySendError::Disconnected(_) => unreachable!(),
                        TrySendError::Full(_) => true
                    }
                }
            })
        }
    }
    pub(crate) fn emit_events(&self, events: Vec<ScriptEvent>) {
        let threads = THREADS.lock().unwrap();
        let mut threads = threads.values()
            .into_iter()
            .map(|thread|(thread, events.clone()))
            .collect::<Vec<_>>();
        while threads.len() > 0 {
            let mut script_id = 0;
            while script_id < threads.len() {
                if let Some(event) = threads[script_id].1.last() {
                    match threads[script_id].0.tx.try_send(*event) {
                        Ok(_) => {
                            threads[script_id].1.pop();
                        },
                        Err(e) => match e {
                            TrySendError::Disconnected(_) => unreachable!(),
                            TrySendError::Full(_) => {}
                        }
                    }
                } else {
                    threads.remove(script_id);
                }
                script_id += 1
            }
        }
    }
}

impl Engine {
    pub fn close_thread_sync<T>(&self, script: ScriptInstance<T>) {
        let st = THREADS.lock().unwrap().remove(&script.1).unwrap();
        st.tx.send(ScriptEvent::Close).unwrap();
        st.jh.join().unwrap()
    }
    pub(crate) fn close_threads(&self) {
        self.emit_event(ScriptEvent::Close);
        let mut threads = THREADS.lock().unwrap();
        let ts = threads.drain().collect::<Vec<_>>();
        drop(threads);
        for (_id, st) in ts.into_iter() {
            st.jh.join().unwrap()
        }
    }
}