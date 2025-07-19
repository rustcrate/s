use std::sync::OnceLock;

/// Returns the number of available CPU cores.
///
/// The result is cached for fast subsequent calls.
/// If detection fails, defaults to 1.
pub fn num_cpu() -> u16 {
    static CPU_CORES: OnceLock<u16> = OnceLock::new();
    *CPU_CORES.get_or_init(|| {
        std::thread::available_parallelism()
            .map(|p| p.get() as u16)
            .unwrap_or(1)
    })
}
