use std::time::{Duration, Instant};

pub struct Stopwatch {
    _instant: Instant
}

impl Stopwatch {
    pub fn new() -> Self {
        Self {_instant: Instant::now()}
    }

    pub fn current_time(&self) -> Duration {
        self._instant.elapsed()
    }

    pub fn reset(&mut self) -> Duration {
        let output = self._instant.elapsed();
        self._instant = Instant::now();
        output
    }
}