use std::time::Instant;

pub fn readableTimeElapsed(instant: &Instant) -> String {
	(instant.elapsed().as_millis() as f64 / 1000.0).to_string()
}