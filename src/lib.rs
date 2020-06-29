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
mod loader;
mod search_paths;
mod theme_name_provider;

pub use error::{Error, ProviderError, Result};
pub use loader::*;
pub use search_paths::SearchPaths;
pub use theme_name_provider::ThemeNameProvider;

use std::{
    borrow::Borrow,
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct IconThemes {
    name: String,
    themes: Vec<IconTheme>,
    parents: Vec<String>,
}

impl IconThemes {
    fn find(theme_name: &str, search_paths: &[PathBuf]) -> IconThemes {
        let mut themes = IconThemes {
            name: theme_name.to_string(),
            themes: Vec::new(),
            parents: Vec::new(),
        };

        for search_path in search_paths {
            let content_dir = search_path.join(theme_name);
            if !content_dir.is_dir() {
                continue;
            }

            let theme_index_path = content_dir.join("index.theme");
            if !theme_index_path.is_file() {
                continue;
            }

            let mut theme = IconTheme::new(content_dir);

            if let Ok(ini) = ini::Ini::load_from_file(theme_index_path) {
                for (dir_key, properties) in ini.iter() {
                    if let Some(dir_key) = dir_key {
                        match dir_key {
                            "Icon Theme" => {
                                for (key, value) in properties.iter() {
                                    if key == "Inherits" {
                                        for parent in
                                            value.split(',').map(str::trim).map(String::from)
                                        {
                                            if !themes.parents.contains(&parent) {
                                                themes.parents.push(parent);
                                            }
                                        }

                                        let hicolor = String::from("hicolor");

                                        if !themes.parents.contains(&hicolor) {
                                            themes.parents.push(hicolor);
                                        }
                                    }
                                }
                            }
                            _ => {
                                let mut dir_info = IconDir::new(dir_key.into());

                                for (key, value) in properties.iter() {
                                    match key {
                                        "Size" => {
                                            if let Ok(size) = value.parse() {
                                                dir_info.size = size;
                                            }
                                        }
                                        "Type" => dir_info.dir_type = value.into(),
                                        "Threshold" => {
                                            if let Ok(threshold) = value.parse() {
                                                dir_info.threshold = Some(threshold);
                                            }
                                        }
                                        "MinSize" => {
                                            if let Ok(min_size) = value.parse() {
                                                dir_info.min_size = Some(min_size);
                                            }
                                        }
                                        "MaxSize" => {
                                            if let Ok(max_size) = value.parse() {
                                                dir_info.max_size = Some(max_size);
                                            }
                                        }
                                        "Scale" => {
                                            if let Ok(scale) = value.parse() {
                                                dir_info.scale = scale;
                                            }
                                        }
                                        _ => {}
                                    }
                                }

                                if dir_info.is_valid() {
                                    theme.key_list.push(Arc::new(dir_info));
                                }
                            }
                        }
                    }
                }

                if !theme.key_list.is_empty() {
                    themes.themes.push(theme);
                }
            }
        }

        themes
    }

    fn find_icon(&self, icon_name: &str) -> Option<Icon> {
        if self.is_empty() {
            return None;
        }

        let entries: Vec<IconFile> = self
            .themes
            .iter()
            .map(|theme| theme.entries(icon_name))
            .flatten()
            .collect();

        if entries.is_empty() {
            return None;
        }

        Icon::new(icon_name.into(), self.name.clone(), entries)
    }

    fn is_empty(&self) -> bool {
        self.themes.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct IconTheme {
    content_dir: PathBuf,
    key_list: Vec<Arc<IconDir>>,
}

impl IconTheme {
    const fn new(content_dir: PathBuf) -> Self {
        Self {
            content_dir,
            key_list: Vec::new(),
        }
    }

    fn entries(&self, icon_name: &str) -> Vec<IconFile> {
        if icon_name.is_empty() {
            return Vec::new();
        }

        let mut entries = Vec::new();

        for icon_dir_info in &self.key_list {
            for icon_type in IconType::types() {
                let icon_path = self
                    .content_dir
                    .join(&icon_dir_info.path)
                    .join(&icon_name)
                    .with_extension(icon_type.as_ref());

                if icon_path.exists() {
                    entries.push(IconFile {
                        dir_info: icon_dir_info.clone(),
                        path: icon_path,
                        icon_type: *icon_type,
                    });
                }
            }
        }

        entries
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
enum IconDirType {
    Fixed,
    Scalable,
    Threshold,
}

impl<S: AsRef<str>> From<S> for IconDirType {
    fn from(s: S) -> Self {
        match s.as_ref() {
            "Fixed" => IconDirType::Fixed,
            "Scalable" => IconDirType::Scalable,
            _ => IconDirType::Threshold,
        }
    }
}

/// Struct that holds information about a directory containing a set of icons
/// with a particular size.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct IconDir {
    dir_type: IconDirType,
    path: PathBuf,
    size: u16,
    max_size: Option<u16>,
    min_size: Option<u16>,
    threshold: Option<u16>,
    scale: u16,
}

impl IconDir {
    const fn new(path: PathBuf) -> Self {
        Self {
            dir_type: IconDirType::Threshold,
            path,
            size: 0,
            max_size: None,
            min_size: None,
            threshold: None,
            scale: 1,
        }
    }

    /// Returns the size of the icons contained.
    pub const fn size(&self) -> u16 {
        self.size
    }

    const fn is_valid(&self) -> bool {
        self.size != 0
    }
}

/// Struct containing information about a themed icon.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Icon {
    icon_name: String,
    theme_name: String,
    files: Vec<IconFile>,
}

impl Icon {
    /// Returns the associated icon's name.
    pub fn icon_name(&self) -> &str {
        &self.icon_name
    }

    /// Returns the associated icon's theme name.
    pub fn theme_name(&self) -> &str {
        &self.theme_name
    }

    /// Returns the icon files found for the associated icon.
    pub fn files(&self) -> &[IconFile] {
        &self.files
    }

    /// Returns the file of the associated icon that fits the given size best.
    /// If there is no exact fit available, the next bigger one is chosen.
    /// If there is no bigger one, the next smaller is returned.
    ///
    /// # Arguments
    ///
    /// * `size` - The ideal size of the returned icon file.
    ///
    ///# Example
    ///
    /// ```
    /// use icon_loader::{IconLoader, IconType};
    ///
    /// let loader = IconLoader::new();
    /// if let Ok(icon) = loader.load_icon("minimum") {
    ///     let icon_file = icon.file_for_size(32);
    /// }
    /// ```
    pub fn file_for_size(&self, size: u16) -> &IconFile {
        // If we don't filter, then there is always at least one file on disk.
        self.file_for_size_filtered(size, |_| true).unwrap()
    }

    /// Returns the file of the associated icon that fits the given size best and matches the provided filter.
    /// Use this, if you want only files of type PNG or anything like that.
    ///
    /// # Arguments
    ///
    /// * `size` - The ideal size of the returned icon file.
    /// * `filter` - A function that takes a reference to an [`IconFile`] and returns true, if it passes the test and false otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use icon_loader::{IconLoader, IconType};
    ///
    /// let loader = IconLoader::new();
    /// if let Ok(icon) = loader.load_icon("minimum") {
    ///     let icon_file = icon.file_for_size_filtered(32, |file| file.icon_type() == IconType::PNG);
    /// }
    /// ```
    ///
    /// See also [`file_for_size`].
    ///
    /// [`IconFile`]: struct.IconFile.html
    /// [`file_for_size`]: #method.file_for_size
    pub fn file_for_size_filtered<F>(&self, size: u16, filter: F) -> Option<&IconFile>
    where
        F: Fn(&IconFile) -> bool,
    {
        let files: Vec<&IconFile> = self.files.iter().filter(|file| filter(file)).collect();

        // Try to return an exact fit.
        if let Some(icon_file) = files.iter().find(|file| size == file.dir_info().size()) {
            return Some(icon_file);
        }

        // Try to return a slightly bigger fit.
        if let Some(icon_file) = files
            .iter()
            .filter(|file| file.dir_info().size() > size)
            .min_by_key(|file| file.dir_info().size())
        {
            return Some(icon_file);
        }

        // Return the biggest available.
        files.into_iter().max_by_key(|file| file.dir_info().size())
    }

    fn new(icon_name: String, theme_name: String, files: Vec<IconFile>) -> Option<Self> {
        if icon_name.is_empty() || theme_name.is_empty() || files.is_empty() {
            None
        } else {
            Some(Self {
                files,
                icon_name,
                theme_name,
            })
        }
    }
}

/// Enum representing the different file types an icon can be.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum IconType {
    /// PNG file type
    PNG,

    /// SVG file type
    SVG,

    /// XPM file type
    XPM,
}

impl IconType {
    fn types() -> &'static [IconType] {
        const TYPES: [IconType; 3] = [IconType::PNG, IconType::SVG, IconType::XPM];

        &TYPES
    }
}

impl AsRef<str> for IconType {
    fn as_ref(&self) -> &'static str {
        match self {
            IconType::PNG => "png",
            IconType::SVG => "svg",
            IconType::XPM => "xpm",
        }
    }
}

/// Struct containing information about a single icon file on disk.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct IconFile {
    dir_info: Arc<IconDir>,
    path: PathBuf,
    icon_type: IconType,
}

impl IconFile {
    /// Returns information about the directory the icon file lives in.
    pub fn dir_info(&self) -> &IconDir {
        &self.dir_info
    }

    /// Returns this icon's path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns this icon's type.
    pub const fn icon_type(&self) -> IconType {
        self.icon_type
    }
}

impl AsRef<Path> for IconFile {
    fn as_ref(&self) -> &Path {
        self.path()
    }
}

impl Borrow<Path> for IconFile {
    fn borrow(&self) -> &Path {
        self.path()
    }
}
