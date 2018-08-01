use specs::{ReadExpect, System};

use game::resources::DeltaTime;

pub struct FPSPrint {
    _time_accumulator: f32,
    _history: usize,
    _counter: usize,
    _past_times: Vec<f32>
}

impl FPSPrint {
    pub fn new(history: usize) -> Self {
        Self {
            _time_accumulator: 0.,
            _history: history,
            _counter: 0,
            _past_times: Vec::with_capacity(history)
        }
    }
}

impl<'a> System<'a> for FPSPrint {
    type SystemData = ReadExpect<'a, DeltaTime>;

    fn run(&mut self, data: Self::SystemData) {
        self._time_accumulator += data.dt;
        if self._past_times.len() < self._history {
            self._past_times.push(data.dt);
        }
        else {
            self._past_times[self._counter] = data.dt;
            self._counter += 1;
            self._counter %= self._history;
        }

        if self._time_accumulator >= 1. {
            self._time_accumulator -= 1.;
            let sum_dt = self._past_times.iter().fold(0., |sum, val| sum + val);
            let avg_dt = sum_dt / (self._history as f32);
            let avg_fps = 1. / avg_dt;
            println!("FPS: {:.2}", avg_fps);
        }
    }
}