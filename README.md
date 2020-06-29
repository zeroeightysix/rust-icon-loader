[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/onur/cargo-license/master/LICENSE)

# Rust Icon Loader

A crate that loads and caches themed icons in 100% safe rust.

## Usage

Just add it to your `cargo.toml` file like this:
```
[dependencies]
icon-loader = "0.2"
```

## Cargo-Features

* `kde`: Standard feature that lets you read the default system theme name from '~/.config/kdeglobals'.
* `gtk`: Standard feature that lets you read the default system theme name from '~/.config/gtk-3.0/settings.ini'.

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

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

