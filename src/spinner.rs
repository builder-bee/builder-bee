//! Spinner format for Builder Bee

use lazy_static::lazy_static;

lazy_static! {
	pub static ref SPINNER_FORMAT: [&'static str; 4] = ["-", "\\", "|", "/"];
}
