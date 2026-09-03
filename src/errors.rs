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

    #[error("invalid path: {path}")]
    InvalidPath { path: PathBuf },

    #[error("failed to write temporary library file `{path}`: {source}")]
    WriteTempLibrary {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error(
        "an extension was detected in the path when it should have been detected automatically: {path}"
    )]
    ExtensionDetected { path: PathBuf },

    // catch-all for dlopen2 errors with no extra context to add beyond what
    // dlopen2::Error's own Display already provides (symbol lookup, null
    // symbol, addr-to-library lookup — none of these had a path or other
    // detail attached even in the old per-variant versions)
    #[error("dynamic library error: {source}")]
    Dlopen {
        #[source]
        #[from]
        source: dlopen2::Error,
    },
}

impl LoaderError {
    /// Attaches the path being opened to a dlopen2 error — used by
    /// `load`/`load_from_bytes`, which know the path but `dlopen2::Error`
    /// itself doesn't carry it. Every other dlopen2::Error kind falls back
    /// to the generic `Dlopen` variant, where there's no path to attach.
    pub fn from_open_error(path: PathBuf, error: dlopen2::Error) -> Self {
        match error {
            dlopen2::Error::OpeningLibraryError(source) => Self::OpenLibrary { path, source },
            source => Self::Dlopen { source },
        }
    }
}
