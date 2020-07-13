use std::path::{Path, PathBuf};

/// Struct that holds information about a directory containing a set of icons
/// with a particular size.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct IconDir {
    path: PathBuf,
    pub(crate) size: u16,
    pub(crate) scale: u16,
    pub(crate) dir_type: IconDirType,
    pub(crate) max_size: Option<u16>,
    pub(crate) min_size: Option<u16>,
    pub(crate) threshold: Option<u16>,
}

impl IconDir {
    pub(crate) fn new(path: PathBuf, properties: &ini::ini::Properties) -> Self {
        let mut dir_info = Self {
            dir_type: IconDirType::Threshold,
            path,
            size: 0,
            max_size: None,
            min_size: None,
            threshold: None,
            scale: 1,
        };

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

        dir_info
    }

    /// Returns the path of this icon dir.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the size of the icons contained.
    pub const fn size(&self) -> u16 {
        self.size
    }

    /// Returns the unscaled size of the icons contained.
    pub const fn scale(&self) -> u16 {
        self.scale
    }

    /// Returns the type of icon sizes contained.
    pub const fn dir_type(&self) -> IconDirType {
        self.dir_type
    }

    /// Returns the max size of icons contained.
    /// Returns `None`, if icons in this dir are not scalable.
    pub const fn max_size(&self) -> Option<u16> {
        self.max_size
    }

    /// Returns the min size of icons contained.
    /// Returns `None`, if icons in this dir are not scalable.
    pub const fn min_size(&self) -> Option<u16> {
        self.min_size
    }

    /// Returns the threshold of icons contained.
    /// Returns `None`, if icons in this dir are not of type threshold.
    pub const fn threshold(&self) -> Option<u16> {
        self.threshold
    }

    pub(crate) const fn is_valid(&self) -> bool {
        self.size != 0
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
/// The size type of icons contained in an [`IconDir`].
///
/// [`IconDir`]: struct.IconDir.html
pub enum IconDirType {
    /// Icons with a fixed size.
    Fixed,

    /// Scalable icons.
    Scalable,

    /// Icons with a fixed size and a threshold.
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
