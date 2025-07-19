//! A simple system utilities crate.
//!
//! Provides functions to get:
//! - Number of CPU cores available
//! - Current Unix timestamp
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
use std::time::{SystemTime, UNIX_EPOCH};

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

/// Returns the current Unix timestamp in seconds since epoch.
///
/// This function uses `SystemTime::now()` internally.
///
/// # Examples
///
/// ```
/// let timestamp = s::time();
/// println!("Current timestamp: {}", timestamp);
/// ```
///
/// # Panics
///
/// This function will panic if the system time is set to before Unix epoch (1970-01-01).
pub fn time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
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

    #[test]
    fn test_time() {
        let t1 = time();
        let t2 = time();
        assert!(t2 >= t1, "Time should move forward");
    }
}
