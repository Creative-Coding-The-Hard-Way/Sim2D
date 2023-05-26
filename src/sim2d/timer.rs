use std::time::{Duration, Instant};

/// A simple timer for measuring the time between two locations in code.
pub struct Timer {
    last_tick: Instant,
    accumulated_time: Duration,
    next_report_time: Instant,
    samples_since_last_report: u32,
    average_time: Duration,
    report_rate: Duration,
}

impl Timer {
    pub fn new_with_report_rate(report_rate: Duration) -> Self {
        Self {
            last_tick: Instant::now(),
            accumulated_time: Duration::from_secs(0),
            next_report_time: Instant::now() + report_rate,
            samples_since_last_report: 0,
            average_time: Duration::from_secs(0),
            report_rate,
        }
    }

    pub fn tick(&mut self) {
        self.last_tick = Instant::now();
    }

    pub fn tock(&mut self) -> f32 {
        let delta = Instant::now() - self.last_tick;
        self.accumulated_time += delta;

        self.average_time =
            ((self.average_time * self.samples_since_last_report) + delta)
                / (self.samples_since_last_report + 1);
        self.samples_since_last_report += 1;

        delta.as_secs_f32()
    }

    pub fn report_average(&mut self) -> Option<Duration> {
        if Instant::now() >= self.next_report_time {
            self.next_report_time = Instant::now() + self.report_rate;
            self.samples_since_last_report = 0;
            let time = self.average_time;
            self.average_time = Duration::from_secs(0);
            Some(time)
        } else {
            None
        }
    }
}
