use super::{Icon, IconDir, IconFile, IconFileType};

use std::{path::PathBuf, sync::Arc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct IconTheme {
    content_dir: PathBuf,
    key_list: Vec<Arc<IconDir>>,
}

impl IconTheme {
    fn from_dir(content_dir: PathBuf) -> Option<(Self, Vec<String>)> {
        if !content_dir.is_dir() {
            return None;
        }

        let theme_index_path = content_dir.join("index.theme");
        if !theme_index_path.is_file() {
            return None;
        }

        let mut theme = Self {
            content_dir,
            key_list: Vec::new(),
        };
        let mut parents = Vec::new();

        if let Ok(ini) = ini::Ini::load_from_file(theme_index_path) {
            for (dir_key, properties) in ini.iter() {
                if let Some(dir_key) = dir_key {
                    match dir_key {
                        "Icon Theme" => {
                            for (key, value) in properties.iter() {
                                if key == "Inherits" {
                                    for parent in value.split(',').map(str::trim).map(String::from)
                                    {
                                        if !parents.contains(&parent) {
                                            parents.push(parent);
                                        }
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
        }

        if theme.key_list.is_empty() {
            None
        } else {
            Some((theme, parents))
        }
    }

    fn entries(&self, icon_name: &str) -> Vec<IconFile> {
        if icon_name.is_empty() {
            return Vec::new();
        }

        let mut entries = Vec::new();

        for icon_dir_info in &self.key_list {
            for icon_type in IconFileType::types() {
                let icon_path = self
                    .content_dir
                    .join(&icon_dir_info.path())
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct IconThemes {
    name: String,
    themes: Vec<IconTheme>,
    parents: Vec<String>,
}

impl IconThemes {
    pub(crate) fn find(theme_name: &str, search_paths: &[PathBuf]) -> IconThemes {
        let mut themes = IconThemes {
            name: theme_name.to_string(),
            themes: Vec::new(),
            parents: Vec::new(),
        };

        for search_path in search_paths {
            let content_dir = search_path.join(theme_name);

            if let Some((theme, parents)) = IconTheme::from_dir(content_dir) {
                for parent in parents {
                    if !themes.parents.contains(&parent) {
                        themes.parents.push(parent);
                    }
                }

                themes.themes.push(theme);
            }
        }

        let hicolor = String::from("hicolor");

        if !themes.parents.contains(&hicolor) {
            themes.parents.push(hicolor);
        }

        themes
    }

    pub(crate) fn find_icon(&self, icon_name: &str) -> Option<Icon> {
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

    pub(crate) fn is_empty(&self) -> bool {
        self.themes.is_empty()
    }

    pub(crate) fn parents(&self) -> &[String] {
        &self.parents
    }
}
