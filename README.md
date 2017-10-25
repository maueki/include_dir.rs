
# include_dir_bytes

Read the directory recursively, and creates a `HashMap` with these contents at compile time with the file path as the key.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
include_dir_bytes = "0.2"
```

and use it like this:

```rust
#![feature(plugin)]
#![plugin(include_dir_bytes)]

// ...

let file_map = include_dir!("path/to/dir");
```
