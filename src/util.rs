use std::{time, thread};
use std::time::Instant;
use std::cmp;
use std::borrow::BorrowMut;

pub struct FramerateLimiter{
    previous_tick: time::Instant,
}

trait FramerateLimiterMethods{
    fn new() -> FramerateLimiter;
    fn tick(&mut self, framerate: f32);
}

impl FramerateLimiterMethods for FramerateLimiter{
    fn new() -> FramerateLimiter{
        FramerateLimiter{
            previous_tick: time::Instant::now()
        }
    }

    fn tick(&mut self, framerate: f32) {
        let elapsed_time = self.previous_tick.elapsed();

        let seconds_per_frame = 1.0/framerate;

        let time_to_wait = (seconds_per_frame - elapsed_time.as_secs_f32()).max(0.0);

        thread::sleep(time::Duration::from_secs_f32(time_to_wait));

        self.previous_tick = Instant::now();
    }
}