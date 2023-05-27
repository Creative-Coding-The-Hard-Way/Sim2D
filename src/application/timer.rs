use std::time::{Duration, Instant};

/// A simple timer for measuring the time between two locations in code.
pub struct Timer {
    report_rate: Duration,
    next_report_time: Instant,

    frame_time: TickTock,
    simulation_time: TickTock,
    render_time: TickTock,

    avg_frame_time: RollingAverage,
    avg_simulation_time: RollingAverage,
    avg_render_time: RollingAverage,
}

impl Timer {
    pub fn new() -> Self {
        let report_rate = Duration::from_secs(1);
        Self {
            report_rate,
            next_report_time: Instant::now() + report_rate,

            frame_time: TickTock::new(),
            simulation_time: TickTock::new(),
            render_time: TickTock::new(),

            avg_frame_time: RollingAverage::new(),
            avg_simulation_time: RollingAverage::new(),
            avg_render_time: RollingAverage::new(),
        }
    }

    pub fn reset_frame_time(&mut self) {
        self.frame_time.tick();
    }

    pub fn frame_tick_tock(&mut self) -> Duration {
        let duration = self.frame_time.tock();
        self.avg_frame_time.record(duration);

        if Instant::now() >= self.next_report_time {
            self.next_report_time = Instant::now() + self.report_rate;

            let round_ms = |val: f32| (val * 1000.0 * 100.0).floor() / 100.0;

            log::info!(
                indoc::indoc!(
                    "
                    -  Frame Time: {}ms
                    - Update Time: {}ms
                    - Render Time: {}ms
                    "
                ),
                round_ms(self.avg_frame_time.report().as_secs_f32()),
                round_ms(self.avg_simulation_time.report().as_secs_f32()),
                round_ms(self.avg_render_time.report().as_secs_f32()),
            );
        }

        self.frame_time.tick();
        duration
    }

    pub fn simulation_tick(&mut self) {
        self.simulation_time.tick();
    }

    pub fn simulation_tock(&mut self) {
        let duration = self.simulation_time.tock();
        self.avg_simulation_time.record(duration);
    }

    pub fn render_tick(&mut self) {
        self.render_time.tick();
    }

    pub fn render_tock(&mut self) {
        let duration = self.render_time.tock();
        self.avg_render_time.record(duration);
    }
}

/// Keep track of a rolling average duration.
#[derive(Default)]
struct RollingAverage {
    average: Duration,
    samples: u32,
}

impl RollingAverage {
    /// Create a new rolling average.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a duration.
    pub fn record(&mut self, d: Duration) {
        self.average = ((self.average * self.samples) + d) / (self.samples + 1);
        self.samples += 1;
    }

    /// Get the average recorded duration since the last report.
    pub fn report(&mut self) -> Duration {
        let result = self.average;
        self.average = Duration::ZERO;
        self.samples = 0;
        result
    }
}

struct TickTock {
    last_tick: Instant,
}

impl TickTock {
    pub fn new() -> Self {
        Self {
            last_tick: Instant::now(),
        }
    }

    pub fn tick(&mut self) {
        self.last_tick = Instant::now();
    }

    pub fn tock(&mut self) -> Duration {
        Instant::now() - self.last_tick
    }
}
