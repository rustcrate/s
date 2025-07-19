# s

A simple Rust crate to get the number of CPU cores available to the current process.

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
    let cores = s::num_cpu();
    println!("Number of CPU cores: {}", cores);
}
```

### Features

- Gets the number of available CPU cores
- Caches the result for fast subsequent calls
- Defaults to 1 if detection fails
- Zero dependencies

