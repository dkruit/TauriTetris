use std::sync::{Arc, Mutex, atomic};

use crate::emitter::Emitter;

pub struct Counter {
    value: i32,
    sleep_time: f64, // sleep_time in seconds
    emitter: Emitter
}

impl Counter {
    pub fn new(sleep_time: f64, emitter: Emitter) -> Self {
        Counter { value: 0, sleep_time, emitter }
    }

    pub fn increment(&mut self) {
        self.value += 1;
        self.emitter.emit("counter_updated", self.value.to_string());
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn get_sleep_time(&self) -> f64 {
        self.sleep_time
    }
}

// Declare a shared counter struct to use the state of the counter
// Arc Mutex makes it usable in different threads
pub struct CounterRunner {
    pub counter: Arc<Mutex<Counter>>,
    pub running: Arc<atomic::AtomicBool>
}
