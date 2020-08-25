use std::{time, thread};
use std::time::Instant;

pub struct FramerateLimiter{
    previous_tick: time::Instant,
    pub tick_rate: f32,
}

impl Default for FramerateLimiter{
    fn default() -> Self {
        FramerateLimiter{
            previous_tick: Instant::now(),
            tick_rate: 1.0,
        }
    }
}

impl FramerateLimiter{
    pub fn new(tick_rate: f32) -> FramerateLimiter{
        FramerateLimiter{
            previous_tick: Instant::now(),
            tick_rate: tick_rate,
        }
    }

    pub fn tick(&mut self) {
        /**
            framerate: given in fps, the approximate rate that a loop will repeat at if tick is called consistently within it
        */
        let elapsed_time = self.previous_tick.elapsed();

        let seconds_per_frame = 1.0/self.tick_rate;

        let time_to_wait = (seconds_per_frame - elapsed_time.as_secs_f32()).max(0.0);

        thread::sleep(time::Duration::from_secs_f32(time_to_wait));

        self.previous_tick = Instant::now();
    }
}