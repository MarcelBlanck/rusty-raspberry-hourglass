use std::sync::{Arc, RwLock};

pub const MAXIMUM_DURATION_MS: u128 = 52 * 60 * 1000;

#[derive(Debug)]
pub struct HourglassState {
    pub ticking: bool,
    pub finalize: bool,
    pub target_time_ms: u128,
    pub duration_ms: u128,
}

impl HourglassState {
    pub fn new() -> Self {
        HourglassState {
            ticking: false,
            finalize: false,
            target_time_ms: 0,
            duration_ms: 0,
        }
    }
}

pub type ThreadSafeHourglassState = Arc<RwLock<HourglassState>>;
