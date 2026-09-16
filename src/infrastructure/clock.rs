use std::time::Instant;

use crate::application::interface::clock::AppClock;

pub struct RealClock;

impl AppClock for RealClock {
    fn now(&self) -> Instant {
        Instant::now()
    }
}