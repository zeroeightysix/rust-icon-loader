use crate::{
    error::Result,
    icon::{Icon, IconThemeChain},
    search_paths::SearchPaths,
    theme_name_provider::ThemeNameProvider,
};
use dashmap::DashMap;
use std::collections::VecDeque;
use std::sync::Arc;
use std::{borrow::Cow, path::PathBuf};

/// The central icon loader struct.
///
/// It lets you load named theme icons from system themes as well as custom themes.
#[derive(Debug)]
pub struct IconLoader {
    theme_name: String,
    fallback_theme_name: String,
    theme_cache: Arc<ThemeCache>,
}

#[derive(Debug, Default)]
pub struct ThemeCache {
    cache: DashMap<String, Arc<IconThemeChain>>,
    search_paths: SearchPaths,
}

impl ThemeCache {
    /// Returns the paths that are searched for icon themes.
    pub fn search_paths(&self) -> Cow<[PathBuf]> {
        self.search_paths.paths()
    }

    pub fn theme<'a>(self: &'a Arc<Self>, theme_name: &'a str) -> Arc<IconThemeChain> {
        if !self.cache.contains_key(theme_name) {
            let new_themes = IconThemeChain::find(self.clone(), theme_name, &self.search_paths());

            self.cache.insert(theme_name.into(), Arc::new(new_themes));
        }

        // Unwrapping is ok, since we just added a value
        self.cache.get(theme_name).unwrap().clone()
    }
}

impl IconLoader {
    pub fn new(theme_name: impl Into<String>, fallback_theme_name: impl Into<String>) -> Self {
        IconLoader {
            theme_name: theme_name.into(),
            fallback_theme_name: fallback_theme_name.into(),
            theme_cache: Default::default(),
        }
    }

    /// Creates a new `IconLoader` with default settings.
    pub fn new_hicolor() -> Self {
        Self::new("hicolor", "hicolor")
    }

    pub fn new_from_provider(theme_name_provider: ThemeNameProvider) -> Result<Self> {
        let theme_name = theme_name_provider.theme_name()?;

        Ok(Self::new(&theme_name, &theme_name))
    }

    /// Creates a new KDE `IconLoader`.
    /// This is a convenience function.
    #[cfg(feature = "kde")]
    pub fn new_kde() -> Result<Self> {
        Self::new_from_provider(ThemeNameProvider::KDE)
    }

    /// Creates a new GTK `IconLoader`.
    /// This is a convenience function.
    #[cfg(feature = "gtk")]
    pub fn new_gtk() -> Result<Self> {
        Self::new_from_provider(ThemeNameProvider::GTK)
    }

    /// Loads the icon with the name `icon_name` from the current icon theme.
    /// If the icon cannot be found, it will be looked for in the fallback icon theme.
    /// If it cannot be found in the fallback theme, `None` is returned.
    pub fn load_icon(&self, icon_name: impl AsRef<str>) -> Option<Icon> {
        self.find_icon(self.theme_name(), icon_name.as_ref())
    }

    /// Returns the currently used theme name.
    ///
    /// See also [`IconLoader::update_theme_name()`].
    pub fn theme_name(&self) -> &str {
        &self.theme_name
    }

    /// Returns the currently used fallback theme name.
    ///
    /// See also [`IconLoader::set_fallback_theme_name()`].
    pub fn fallback_theme_name(&self) -> &str {
        &self.fallback_theme_name
    }

    /// Returns the paths that are searched for icon themes.
    pub fn search_paths(&self) -> Cow<[PathBuf]> {
        self.theme_cache.search_paths()
    }

    /// Sets a new fallback theme name. If an icon cannot be found in the set theme,
    /// it will be looked for in the fallback theme.
    /// The default fallback theme name is 'hicolor'.
    pub fn set_fallback_theme_name(&mut self, fallback_theme_name: impl Into<String>) {
        let fallback_theme_name = fallback_theme_name.into();

        if self.fallback_theme_name == fallback_theme_name {
            return;
        }

        self.fallback_theme_name = fallback_theme_name;
    }

    /// Returns whether a theme with the name `theme_name` exists in the current search paths.
    pub fn theme_exists(&self, theme_name: impl AsRef<str>) -> bool {
        let theme_name = theme_name.as_ref();

        if theme_name.is_empty() {
            return false;
        }

        !self.theme_cache.theme(theme_name).is_empty()
    }

    fn find_icon(&self, theme_name: &str, icon_name: &str) -> Option<Icon> {
        if theme_name.is_empty() || icon_name.is_empty() {
            return None;
        }

        let mut searched_themes = vec![];

        let themes = self.theme_cache.theme(theme_name);
        let fallback_themes = self.theme_cache.theme(&self.fallback_theme_name);
        let mut themes = VecDeque::from([themes, fallback_themes]);

        while let Some(theme) = themes.pop_front() {
            let theme_name = theme.name().to_string();
            if searched_themes.contains(&theme_name) {
                continue;
            }

            theme.parents().for_each(|theme| themes.push_front(theme));

            if let Some(icon) = theme.find_icon(icon_name) {
                return Some(icon);
            }

            searched_themes.push(theme_name);
        }

        None
    }
}

impl Default for IconLoader {
    fn default() -> Self {
        IconLoader::new_hicolor()
    }
}
