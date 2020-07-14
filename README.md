[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/onur/cargo-license/master/LICENSE)

# Rust Icon Loader

A crate that loads and caches themed icons in 100% safe rust.

## Usage

Just add it to your `cargo.toml` file like this:
```
[dependencies]
icon-loader = "0.3"
```

## Cargo-Features

### Standard Features

* `kde`: Feature that lets you read the default system theme name from '~/.config/kdeglobals'.
* `gtk`: Feature that lets you read the default system theme name from '~/.config/gtk-3.0/settings.ini'.

### Additional Features

* `theme_error_log`: Feature that uses the [`log`](https://crates.io/crates/log) crate to log errors that occur while parsing icon themes. 

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

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

