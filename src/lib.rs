//!
//! Crate to load and cache themed icons.
//!
//! # Examples
//!
//! * Loading icons from the default icon theme set in KDE:
//! ```
//! use icon_loader::{IconLoader, ThemeNameProvider};
//!
//! let mut loader = IconLoader::new();
//! loader.set_theme_name_provider(ThemeNameProvider::KDE);
//! loader.update_theme_name();
//!
//! if let Ok(icon) = loader.load_icon("audio-headphones") {
//!     let path = icon.file_for_size(64).path();
//! }
//! ```
//!
//! * Loading icons from a custom theme in a provided folder:
//! ```
//! use icon_loader::IconLoader;
//!
//! let mut loader = IconLoader::new();
//! loader.set_search_paths(&["path_to_your_icon_theme"]);
//! loader.set_theme_name_provider("name_of_your_icon_theme");
//! loader.update_theme_name();
//!
//! if let Ok(icon) = loader.load_icon("icon_name") {
//!     let path = icon.file_for_size(32).path();
//! }
//! ```

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts
)]
#![forbid(unsafe_code, unstable_features)]

mod error;
mod icon;
mod loader;
mod search_paths;
mod theme_name_provider;

pub use error::{Error, ProviderError, Result};
pub use icon::{Icon, IconDir, IconFile, IconFileType, IconSizeType};
pub use loader::*;
pub use search_paths::SearchPaths;
pub use theme_name_provider::ThemeNameProvider;
