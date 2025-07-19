use std::time::{SystemTime, UNIX_EPOCH};

/// This function will panic if the system time is set to before Unix epoch (1970-01-01).
pub fn time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
