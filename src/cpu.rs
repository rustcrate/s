use std::sync::OnceLock;

pub fn ncpu() -> u16 {
    static CPU_CORES: OnceLock<u16> = OnceLock::new();
    *CPU_CORES.get_or_init(|| {
        std::thread::available_parallelism()
            .map(|p| p.get() as u16)
            .unwrap_or(1)
    })
}
