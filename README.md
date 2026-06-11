# dlloader
A cross-platform dynamic library loader

`dlloader` is built around `dlopen2` and provides a simpler API for loading
dynamic libraries from files or bytes.

# Example
```rust
use dlloader::{Loader, WrapperApi};

#[derive(WrapperApi)]
struct PluginApi {
    add: extern "C" fn(i32, i32) -> i32,
}

let plugin = Loader::<PluginApi>::load("plugin.dll")?;
let result = plugin.add(2, 3);

assert_eq!(result, 5);
```
