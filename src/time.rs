use std::time::Instant;

pub fn readable_time_elapsed(instant: &Instant) -> String {
	(instant.elapsed().as_millis() as f64 / 1000.0).to_string()
}
