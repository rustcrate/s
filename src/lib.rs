//! A simple crate to get the number of CPU cores available.
//!
//! # Examples
//!
//! ```
//! let cores = s::num_cpu();
//! println!("This machine has {} CPU cores available", cores);
//! ```
//!
//! The function caches the result internally, so subsequent calls are very fast.

use std::sync::OnceLock;

/// Returns the number of CPU cores available to the current process.
///
/// This function uses `std::thread::available_parallelism()` internally
/// and caches the result for subsequent calls.
///
/// # Examples
///
/// ```
/// let cores = s::num_cpu();
/// println!("This machine has {} CPU cores available", cores);
/// ```
///
/// # Panics
///
/// This function will not panic under normal circumstances.
/// If `available_parallelism()` returns an error, it will default to 1.
pub fn num_cpu() -> u16 {
    static CPU_CORES: OnceLock<u16> = OnceLock::new();
    *CPU_CORES.get_or_init(|| {
        std::thread::available_parallelism()
            .map(|p| p.get() as u16)
            .unwrap_or(1)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_cpu() {
        let cores = num_cpu();
        assert!(cores >= 1, "Should return at least 1 core");
    }

    #[test]
    fn test_num_cpu_is_cached() {
        let cores1 = num_cpu();
        let cores2 = num_cpu();
        assert_eq!(cores1, cores2, "Subsequent calls should return the same value");
    }
}
