use std::ffi::NulError;
use std::{io, path::PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum LoaderError {
    #[error("library not found: {path}")]
    LibraryNotFound { path: PathBuf },

    #[error("failed to open library `{path}`: {source}")]
    OpenLibrary {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("failed to create temporary directory: {source}")]
    CreateTempDir {
        #[source]
        source: io::Error,
    },

    #[error("failed to write temporary library file `{path}`: {source}")]
    WriteTempLibrary {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("failed to load symbol: {source}")]
    SymbolGetting {
        #[source]
        source: io::Error,
    },

    #[error("symbol name contains a null byte")]
    NullCharacter {
        #[from]
        source: NulError,
    },

    #[error("symbol pointer was null")]
    NullSymbol,

    #[error("address could not be matched to a dynamic library: {source}")]
    AddrNotMatchingDll {
        #[source]
        source: io::Error,
    },

    #[error("dynamic library error: {source}")]
    Dlopen {
        #[source]
        source: dlopen2::Error,
    },
}

impl LoaderError {
    pub fn from_open_error(path: PathBuf, error: dlopen2::Error) -> Self {
        match error {
            dlopen2::Error::OpeningLibraryError(source) => Self::OpenLibrary { path, source },
            source => Self::Dlopen { source },
        }
    }
}

impl From<dlopen2::Error> for LoaderError {
    fn from(error: dlopen2::Error) -> Self {
        match error {
            dlopen2::Error::NullCharacter(source) => Self::NullCharacter { source },

            dlopen2::Error::OpeningLibraryError(source) => Self::OpenLibrary {
                path: PathBuf::new(),
                source,
            },

            dlopen2::Error::SymbolGettingError(source) => Self::SymbolGetting { source },

            dlopen2::Error::NullSymbol => Self::NullSymbol,

            dlopen2::Error::AddrNotMatchingDll(source) => Self::AddrNotMatchingDll { source },
        }
    }
}
