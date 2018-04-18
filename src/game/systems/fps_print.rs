use specs::{Fetch, System};

use game::resources::DeltaTime;

pub struct FPSPrint {
    _history: usize,
    _counter: usize,
    _past_times: Vec<f32>
}

impl FPSPrint {
    pub fn new(history: usize) -> Self {
        Self {
            _history: history,
            _counter: 0,
            _past_times: Vec::with_capacity(history)
        }
    }
}

impl<'a> System<'a> for FPSPrint {
    type SystemData = Fetch<'a, DeltaTime>;

    fn run(&mut self, dt: Self::SystemData) {
        if self._past_times.len() < self._history {
            self._past_times.push(dt.clone());
        }
        else {
            self._past_times[self._counter] = dt.clone();
            self._counter += 1;
            if self._counter == self._history {
                self._counter = 0;
                let sum_dt = self._past_times.iter().fold(0., |sum, val| sum + val);
                let avg_dt = sum_dt / (self._history as f32);
                let avg_fps = 1. / avg_dt;
                println!("FPS: {:.2}", avg_fps);
            }
        }
    }
}