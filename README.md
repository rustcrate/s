# s

A simple Rust system utilities crate with functions to:
- Get the number of CPU cores available
- Get the current Unix timestamp

[Available on crates.io as `s`](https://crates.io/crates/s)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
s = "0.1"
```

Then in your code:

```rust
fn main() {
    let cores = s::ncpu();
    println!("Number of CPU cores: {}", cores);
    println!("Current timestamp: {}", s::time());
}
```

### Features

- Gets the number of available CPU cores
- Caches the result for fast subsequent calls
- Defaults to 1 if detection fails
- Zero dependencies

