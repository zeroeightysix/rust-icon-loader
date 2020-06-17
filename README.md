# Rust Icon Loader

A crate that loads and caches themed icons in 100% safe rust.

## Usage

Just add it to your `cargo.toml` file like this:
```
[dependencies]
icon-loader = "0.1"
```

## Examples

* Loading icons from the default icon theme set in KDE:
```rust
use icon_loader::{IconLoader, ThemeNameProvider};

let mut loader = IconLoader::new();
loader.set_theme_name_provider(ThemeNameProvider::KDE);
loader.update_theme_name().unwrap();

let icon = loader.load_icon("audio-headphones").unwrap();
let path = icon.file_for_size(64).path();
```

* Loading icons from a custom theme in a provided folder:
```rust
use icon_loader::IconLoader;

let mut loader = IconLoader::new();
loader.set_search_paths(&["path_to_your_icon_theme"]);
loader.set_theme_name_provider("name_of_your_icon_theme");
loader.update_theme_name().unwrap();

let icon = loader.load_icon("icon_name").unwrap();
let path = icon.file_for_size(32).path();
```

## Features

Default features are "kde" and "gtk", which enable reading the default system icon theme name from the kde and gtk config files, respectively.
Additionally you can activate the "sync" feature, which will make all provided structs and enums in this crate `Send` and `Sync`.

## License

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/onur/cargo-license/master/LICENSE)

