use super::IconDir;

use std::{
    borrow::Borrow,
    path::{Path, PathBuf},
    sync::Arc,
};

/// Enum representing the different file types an icon can be.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum IconFileType {
    /// PNG file type
    PNG,

    /// SVG file type
    SVG,

    /// XPM file type
    XPM,
}

impl IconFileType {
    pub(crate) const fn types() -> &'static [IconFileType; 3] {
        const TYPES: [IconFileType; 3] = [IconFileType::PNG, IconFileType::SVG, IconFileType::XPM];

        &TYPES
    }
}

impl AsRef<str> for IconFileType {
    fn as_ref(&self) -> &'static str {
        match self {
            IconFileType::PNG => "png",
            IconFileType::SVG => "svg",
            IconFileType::XPM => "xpm",
        }
    }
}

/// Struct containing information about a single icon file on disk.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct IconFile {
    pub(crate) dir_info: Arc<IconDir>,
    pub(crate) path: PathBuf,
    pub(crate) icon_type: IconFileType,
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
    pub const fn icon_type(&self) -> IconFileType {
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
