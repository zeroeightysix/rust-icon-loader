use std::path::{Path, PathBuf};

/// Struct that holds information about a directory containing a set of icons
/// with a particular size.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct IconDir {
    path: PathBuf,
    pub(crate) dir_type: IconDirType,
    pub(crate) size: u16,
    pub(crate) max_size: Option<u16>,
    pub(crate) min_size: Option<u16>,
    pub(crate) threshold: Option<u16>,
    pub(crate) scale: u16,
}

impl IconDir {
    pub(crate) const fn new(path: PathBuf) -> Self {
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

    /// Returns the path of this icon dir.
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) const fn is_valid(&self) -> bool {
        self.size != 0
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub(crate) enum IconDirType {
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
