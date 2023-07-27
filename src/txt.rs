// A slice of a path (akin to str).
use std::path::{Path, PathBuf};

// Trait to convert a dynamic path segment string to a concrete value.
use rocket::request::FromParam;

// Local stuff.
use crate::rng::generate_id;

/// This is the ID of a `Txt` that will be stored.
#[derive(Clone, UriDisplayPath)]
pub struct TXT(String);

impl TXT {

	/// Returns a new `TXT` with a generated `ID`.
	pub fn new(size: usize) -> TXT {
		// WARNING: This code does not consider that `ID`s might clash..!
		TXT(generate_id(size))
	}

	/// Returns a `PathBuf` to the `Txt` in the `/upload` folder.
	pub fn file_path(&self) -> PathBuf {
		let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
		Path::new(root).join(&self.0)
	}

	/// Returns a reference to the `ID`.
	pub fn id(self) -> String {
		self.0
	}

}

impl FromParam<'_> for TXT {
    type Error = String;

    fn from_param(param: &str) -> Result<Self, Self::Error> {
        
		param

			// Transform into a `[char]`.
			.chars()

			// Check if all are inside the [A-Za-z0-9] range.
			.all(|c| c.is_ascii_alphanumeric())

            // If so, build the `TXT`.
			.then(|| TXT(param.into()))

            // Otherwise, return the `String` itself as an error.
			.ok_or(param.to_string())
			
    }
}