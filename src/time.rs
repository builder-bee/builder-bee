use std::time::Instant;

/// Gets the time elapsed from an instant passed to now in a readable format.
/// 
/// For example, an instant 45.03 seconds ago will output 45.03 instead of 4503
#[must_use]
pub fn readable_time_elapsed(instant: &Instant) -> String {
	(instant.elapsed().as_millis() as f64 / 1000.0).to_string()
}
