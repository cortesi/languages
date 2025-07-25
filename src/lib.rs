//! A Rust library for GitHub's programming language data.
//!
//! This library provides functions to look up language information
//! based on name, file extension, or CodeMirror mode.
//! The data is sourced from GitHub's Linguist repository and embedded
//! at compile time.
//!
//! # Examples
//!
//! ```
//! // Look up a language by its name
//! let rust = languages::from_name("Rust").unwrap();
//! assert_eq!(rust.language_type, "programming");
//! assert_eq!(rust.color, Some("#dea584"));
//!
//! // Look up by extension
//! let js = languages::from_extension("js").unwrap();
//! assert_eq!(js.name, "JavaScript");
//! ```


// Include the code generated by the build.rs script.
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

/// Retrieves a Language by its official name or one of its aliases.
///
/// The lookup is case-insensitive.
pub fn from_name(name: &str) -> Option<&'static Language> {
    NAME_MAP.get(name.to_lowercase().as_str()).copied()
}

/// Retrieves a Language by a file extension.
///
/// The extension should be provided without the leading dot (e.g., "rs", not ".rs").
/// The lookup is case-insensitive.
pub fn from_extension(extension: &str) -> Option<&'static Language> {
    EXTENSION_MAP.get(extension.to_lowercase().as_str()).copied()
}

/// Retrieves a Language by its CodeMirror mode.
///
/// The lookup is case-insensitive.
pub fn from_codemirror_mode(mode: &str) -> Option<&'static Language> {
    CODEMIRROR_MODE_MAP.get(mode.to_lowercase().as_str()).copied()
}

