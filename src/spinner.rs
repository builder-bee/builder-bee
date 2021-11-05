//! Spinner format for Builder Bee
pub static SPINNER_FORMAT: &'static [&str] = &["-", "\\", "|", "/"];

/// Return bbee's spinner format.
#[macro_export]
macro_rules! bbee_spinner {
	() => {
		(*crate::spinner::SPINNER_FORMAT).to_vec()
	};
}
