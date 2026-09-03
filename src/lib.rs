//! A cross-platform dynamic library loader.
//!
//! `dlloader` is built around `dlopen2` and provides a simpler API for loading
//! dynamic libraries from files or bytes.
//!
//! # Example
//!
//! ```no_run
//! use dlloader::{Loader, WrapperApi};
//!
//! #[derive(WrapperApi)]
//! struct PluginApi {
//!     add: extern "C" fn(i32, i32) -> i32,
//! }
//!
//! let plugin = Loader::<PluginApi>::load("plugin.dll")?;
//! let result = plugin.add(2, 3);
//!
//! assert_eq!(result, 5);
//! # Ok::<(), dlloader::LoaderError>(())
//! ```

pub mod errors;

use dlopen2::wrapper::Container;
use std::{fs, ops::Deref, path::PathBuf};

pub use dlopen2::wrapper::WrapperApi;
pub use errors::LoaderError;

/// # Loads a dynamic library and exposes its typed API.
///
/// `Loader<T>` owns the loaded library container. The library stays loaded as
/// long as the loader value is alive.
///
/// ## Safety
///
/// Loading dynamic libraries and binding foreign functions via FFI is inherently `unsafe`.
/// When using `Loader`, the following safety invariants must be upheld by the caller:
///
/// 1. **Binary Trust:** The source file or byte stream must come from a trusted source.
///    Loading arbitrary foreign code executes raw binary instructions in the host process context.
/// 2. **ABI & Signature Matching:** The structure `T` (annotated with `#[derive(WrapperApi)]`)
///    must exactly match the exported C ABI function names, types, argument layouts, and calling
///    conventions (`extern "C"`, `__stdcall`, etc.) of the dynamic library.
/// 3. **Lifetime & Memory Safety:** Any pointers, raw references, or resources returned by
///    the library's functions must not outlive the `Loader` instance. Dropping `Loader`
///    unloads the library (`dlclose` / `FreeLibrary`), making any subsequent calls or memory
///    accesses to the unmapped memory region Undefined Behavior (UB).
pub struct Loader<T: WrapperApi> {
    container: Container<T>,
    _temp_dir: Option<tempfile::TempDir>,
}

impl<T: WrapperApi> Loader<T> {
    /// # Loads a dynamic library from a filesystem path.
    ///
    /// ## Errors
    ///
    /// Returns [`LoaderError::LibraryNotFound`] if the path does not exist.
    /// Returns [`LoaderError::OpenLibrary`] if the library could not be opened.
    pub fn load(path: impl Into<PathBuf>) -> Result<Self, LoaderError> {
        let path = path.into();

        if !path.exists() {
            return Err(LoaderError::LibraryNotFound { path });
        }

        let container = unsafe { Container::load(&path) }
            .map_err(|error| LoaderError::from_open_error(path, error))?;

        Ok(Self {
            container,
            _temp_dir: None,
        })
    }

    /// # Loads a dynamic library from bytes.
    ///
    /// WARNING! THIS FUNCTION CREATES A TEMP FILE!!
    ///
    /// The bytes are written to a temporary file first, because native dynamic
    /// library loaders usually require a filesystem path.
    ///
    /// ## Errors
    ///
    /// Returns [`LoaderError::CreateTempDir`] if a temporary directory could not
    /// be created.
    ///
    /// Returns [`LoaderError::WriteTempLibrary`] if the temporary library file
    /// could not be written.
    ///
    /// Returns [`LoaderError::OpenLibrary`] if the written library could not be
    /// loaded by the operating system.
    pub fn load_from_bytes(bytes: &[u8], temp_file_name: &str) -> Result<Self, LoaderError> {
        let temp_dir =
            tempfile::tempdir().map_err(|source| LoaderError::CreateTempDir { source })?;

        let path = temp_dir.path().join(temp_file_name);

        fs::write(&path, bytes).map_err(|source| LoaderError::WriteTempLibrary {
            path: path.clone(),
            source,
        })?;

        let container = unsafe { Container::<T>::load(&path) }
            .map_err(|error| LoaderError::from_open_error(path, error))?;

        Ok(Self {
            container,
            _temp_dir: Some(temp_dir),
        })
    }

    /// # Returns a reference to the loaded API.
    pub fn api(&self) -> &T {
        &self.container
    }
}

impl<T: WrapperApi> Deref for Loader<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.container
    }
}
