use std::sync::{Arc, Mutex, atomic};
use std::thread;
use std::time::Duration;

use crate::emitter::Emitter;

pub struct Counter {
    value: i32,
    count_rate: f64, // count_rate in increments per second
    emitter: Emitter
}

impl Counter {
    pub fn new(count_rate: f64, emitter: Emitter) -> Self {
        return Counter { value: 0, count_rate, emitter };
    }

    pub fn increment(&mut self) {
        self.value += 1;
        self.emitter.emit("counter_updated", self.value.to_string());
    }

    pub fn get_value(&self) -> i32 {
        return self.value;
    }

    pub fn get_count_rate(&self) -> f64 {
        return self.count_rate;
    }
}

// Declare a shared counter struct to use the state of the counter
// Arc Mutex makes it usable in different threads
pub struct CounterRunner {
    pub counter: Arc<Mutex<Counter>>,
    pub running: Arc<atomic::AtomicBool>
}

impl CounterRunner {
    pub fn run(&self) {
        // Early return if the counter is already started
        if self.running.load(atomic::Ordering::SeqCst) {
            println!("Counter is already running!");
            return;
        }

        // Set running flag to true
        self.running.store(true, atomic::Ordering::SeqCst);

        // Clone the counter and running flag to move them to the thread
        let counter = self.counter.clone();
        let running_flag = self.running.clone();

        // Spawn a thread to increment the counter at set intervals
        thread::spawn(move || {
            let mut sleep_time;

            while running_flag.load(atomic::Ordering::SeqCst) {
                {
                    let mut counter = counter.lock().unwrap();
                    counter.increment();
                    sleep_time = 1. / counter.get_count_rate();
                }
                thread::sleep(Duration::from_secs_f64(sleep_time));
            }
        });
    }
}
