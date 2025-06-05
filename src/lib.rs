//!
//! Crate to load and cache themed icons.
//!
//! # Examples
//!
//! * Using a global [`IconLoader`](IconLoader) object to load icons from the systems `hicolor` icon theme:
//! ```no_run
//! use icon_loader::icon_loader_hicolor;
//!
//! if let Some(icon) = icon_loader_hicolor().load_icon("audio-headphones") {
//!     let path = icon.file_for_size(64).path();
//! }
//! ```
//!
//! * Loading icons from the default icon theme set in KDE:
//! ```no_run
//! use icon_loader::IconLoader;
//!
//! let loader = IconLoader::new_kde().unwrap();
//!
//! if let Some(icon) = loader.load_icon("audio-headphones") {
//!     let path = icon.file_for_size(64).path();
//! }
//! ```
//!
//! * Loading icons from a custom theme in a provided folder:
//! ```no_run
//! use icon_loader::{IconLoader, ThemeNameProvider};
//!
//! let mut loader = IconLoader::new_from_provider(ThemeNameProvider::User("my-theme".into()))
//!     .unwrap();
//!
//! if let Some(icon) = loader.load_icon("icon_name") {
//!     let path = icon.file_for_size(32).path();
//! }
//! ```

#![deny(
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

use std::sync::OnceLock;

/// This function returns a reference to a global [`IconLoader`](IconLoader) object with default settings.
pub fn icon_loader_hicolor() -> &'static IconLoader {
    static LOADER: OnceLock<IconLoader> = OnceLock::new();

    LOADER.get_or_init(IconLoader::new_hicolor)
}
