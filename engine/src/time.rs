use std::{time::{Instant, Duration}, sync::Mutex};

use spin_sleep::{SpinSleeper, SubsecondNanoseconds};

pub struct Time {
    spin_sleep: SpinSleeper,
    last_time: Mutex<Instant>,
    delta: Mutex<Duration>,
    max_fps: Mutex<Duration>
}
impl Time {
    pub fn new() -> Self {
        Self {
            spin_sleep: SpinSleeper::new(SubsecondNanoseconds::MAX).with_spin_strategy(spin_sleep::SpinStrategy::SpinLoopHint),
            last_time: Mutex::new(Instant::now()),
            delta: Default::default(),
            max_fps: Default::default()
        }
    }
    pub fn update(&self) {
        let mut last_time = self.last_time.lock().unwrap();
        self.spin_sleep.sleep(self.max_fps.lock().unwrap().saturating_sub(last_time.elapsed()));
        *self.delta.lock().unwrap() = last_time.elapsed();
        *last_time = Instant::now();
    }
    #[inline(always)]
    pub fn delta(&self) -> f32 {
        self.delta.lock().unwrap().as_secs_f32()
    }
    pub fn set_max_fps(&self, max: f64) {
        *self.max_fps.lock().unwrap() = if max > 0. {
            Duration::from_secs_f64(1. / max)
        } else {
            Default::default()
        }
    }
}