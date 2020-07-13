use ini::ini::Error as IniError;

use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NotDirectory,
    IndexThemeNotFound,
    KeyListEmpty,
    Ini(IniError),
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Ini(e) => Some(e),
            _ => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Error::NotDirectory => write!(f, "Provided path is not a directory."),
            Error::IndexThemeNotFound => write!(f, "File 'index.theme' could not be found."),
            Error::KeyListEmpty => write!(f, "Icon theme has no valid key entries."),
            Error::Ini(e) => write!(f, "Error reading 'theme.index' file: {}", e),
        }
    }
}

impl From<IniError> for Error {
    fn from(source: IniError) -> Self {
        Error::Ini(source)
    }
}
