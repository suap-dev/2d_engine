use std::time::{Duration, Instant};

// TODO: Generalise
pub struct Bench {
    iteration_nr: u32,
    report_every_iterations: u32,
    loop_started: Instant,
    last_update: Instant,
    sum_events_clear: Duration,
    sum_positions_update: Duration,
    sum_collisions_solve: Duration,
    sum_vb_update: Duration,
    sum_rendering: Duration,
    sum_loop_time: Duration,
}
impl Bench {
    pub fn init(report_every_iterations: u32) -> Self {
        let now = Instant::now();
        Self {
            iteration_nr: 0,
            report_every_iterations,
            last_update: now,
            loop_started: now,

            sum_events_clear: Duration::ZERO,
            sum_positions_update: Duration::ZERO,
            sum_collisions_solve: Duration::ZERO,
            sum_vb_update: Duration::ZERO,
            sum_rendering: Duration::ZERO,
            sum_loop_time: Duration::ZERO,
        }
    }

    pub fn reset(&mut self) {
        self.iteration_nr = 0;
        self.sum_events_clear = Duration::ZERO;
        self.sum_positions_update = Duration::ZERO;
        self.sum_collisions_solve = Duration::ZERO;
        self.sum_vb_update = Duration::ZERO;
        self.sum_rendering = Duration::ZERO;
        self.sum_loop_time = Duration::ZERO;
    }

    pub fn loop_started(&mut self) {
        self.iteration_nr += 1;
        let now = Instant::now();
        self.loop_started = now;
        self.last_update = now;
    }

    pub fn loop_ended(&mut self) {
        self.sum_loop_time += self.loop_started.elapsed();
    }

    pub fn events_cleared(&mut self) {
        self.sum_events_clear += self.last_update.elapsed();
        self.last_update = Instant::now();
    }

    pub fn positions_updated(&mut self) {
        self.sum_positions_update += self.last_update.elapsed();
        self.last_update = Instant::now();
    }

    pub fn collisions_solved(&mut self) {
        self.sum_collisions_solve += self.last_update.elapsed();
        self.last_update = Instant::now();
    }

    pub fn vb_updated(&mut self) {
        self.sum_vb_update += self.last_update.elapsed();
        self.last_update = Instant::now();
    }

    pub fn rendering_finished(&mut self) {
        self.sum_rendering += self.last_update.elapsed();
        self.last_update = Instant::now();
    }

    fn log_duration(name: &str, duration: Duration, loop_duration: Duration) {
        println!(
            "{0:<23}{1:<11?}{2:.2?}%",
            name,
            duration,
            100.0 * duration.as_secs_f64() / loop_duration.as_secs_f64()
        );
    }

    pub fn report(&mut self) -> bool {
        #[allow(clippy::uninlined_format_args)]
        if self.iteration_nr % self.report_every_iterations == 0 {
            let avg_loop_time = self.sum_loop_time / self.iteration_nr;
            let avg_events_clear = self.sum_events_clear / self.iteration_nr;
            let avg_positions_update = self.sum_positions_update / self.iteration_nr;
            let avg_collisions_solve = self.sum_collisions_solve / self.iteration_nr;
            let avg_vb_update = self.sum_vb_update / self.iteration_nr;
            let avg_rendering = self.sum_rendering / self.iteration_nr;

            println!("---------------------------------------------");
            println!("Averaging over {} loop iterations", self.iteration_nr);
            Self::log_duration(stringify!(avg_loop_time), avg_loop_time, avg_loop_time);
            Self::log_duration(
                stringify!(avg_events_clear),
                avg_events_clear,
                avg_loop_time,
            );
            Self::log_duration(
                stringify!(avg_positions_update),
                avg_positions_update,
                avg_loop_time,
            );
            Self::log_duration(
                stringify!(avg_collisions_solve),
                avg_collisions_solve,
                avg_loop_time,
            );
            Self::log_duration(stringify!(avg_vb_update), avg_vb_update, avg_loop_time);
            Self::log_duration(stringify!(avg_rendering), avg_rendering, avg_loop_time);
            println!("---------------------------------------------");
            true
        } else {
            false
        }
    }
}
