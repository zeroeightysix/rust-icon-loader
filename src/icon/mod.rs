mod icon_dir;
mod icon_file;
mod icon_theme;

pub use icon_dir::IconDir;
pub use icon_file::{IconFile, IconFileType};

pub(crate) use icon_theme::IconThemes;

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
    /// use icon_loader::IconLoader;
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
    /// use icon_loader::{IconLoader, IconFileType};
    ///
    /// let loader = IconLoader::new();
    /// if let Ok(icon) = loader.load_icon("minimum") {
    ///     let icon_file = icon.file_for_size_filtered(32, |file| file.icon_type() == IconFileType::PNG);
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

    pub(crate) fn new(icon_name: String, theme_name: String, files: Vec<IconFile>) -> Option<Self> {
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
