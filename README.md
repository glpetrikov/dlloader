# dlloader
A cross-platform dynamic library loader

free of panic, expect, or unwrap.

`dlloader` is built around `dlopen2` and provides a simpler API for loading
dynamic libraries from files or bytes.

## Example
```rust
use dlloader::{Loader, WrapperApi};

#[derive(WrapperApi)]
struct PluginApi {
    add: extern "C" fn(a: i32, b: i32) -> i32,
}

let plugin = Loader::<PluginApi>::load("plugin.dll")?;
let result = plugin.add(2, 3);

assert_eq!(result, 5);
```

## Example with loading from bytes
```rust
use dlloader::{Loader, WrapperApi};

#[derive(WrapperApi)]
struct PluginApi {
    add: extern "C" fn(a: i32, b: i32) -> i32,
}

let lib_bytes = include_bytes!("../target/debug/cool_library.so");
let lib = Loader::<PluginApi>::load_from_bytes(lib_bytes, "cool_library")?;
assert_eq!(5, lib.add(2, 3));
```

## How to run examples
```bash
# builds the workspace first - load_and_call and load_from_bytes_and_call need
# dlloader-fixture's .so/.dll/.dylib to already exist at compile time
cargo build --workspace
cargo run --example load_and_call
cargo run --example load_from_bytes_and_call
```

## License
dlloader is licensed under the MIT License OR Apache License 2.0 - see the [LICENSE.MIT](LICENSE.MIT) OR [LICENSE.APACHE](LICENSE.APACHE) file for details.
