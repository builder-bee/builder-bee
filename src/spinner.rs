//! Spinner format for Builder Bee
pub static SPINNER_FORMAT: &[&str] = &["-", "\\", "|", "/"];

/// Return bbee's spinner format.
/// 
/// This macro turns the static array of spinner strings into a simple `bbee_spinner!()` command.
#[macro_export]
macro_rules! bbee_spinner {
	() => {
		(*crate::spinner::SPINNER_FORMAT).to_vec()
	};
}
