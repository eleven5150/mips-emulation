use std::time::SystemTime;

pub fn get_curr_time_as_millis() -> u128 {
    SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

#[derive(Default)]
pub struct SystemClock {
    pub freq: u64,
    pub total_cycles: f64,
    pub total_ticks: u64,
    pub start_time: u128,
    pub stop_time: u128,
    pub steps: u128
}

impl SystemClock {
    pub fn new() -> Self {
        Self {
            freq: 15_000_000,
            total_cycles: 0f64,
            total_ticks: 0,
            start_time: 0,
            stop_time: 0,
            steps: 0
        }
    }

    pub fn update(&mut self, cycles: u64) {
        self.total_cycles = self.total_cycles + (cycles / self.freq) as f64;
        self.total_ticks += 1
    }
}

