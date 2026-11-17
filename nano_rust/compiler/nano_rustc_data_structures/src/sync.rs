pub use self::mode::set_dyn_thread_safe_mode;

mod mode {
    use std::sync::atomic::{AtomicU8, Ordering};

    const UNINITIALIZED: u8 = 0;
    const DYN_NOT_THREAD_SAFE: u8 = 1;
    const DYN_THREAD_SAFE: u8 = 2;

    static DYN_THREAD_SAFE_MODE: AtomicU8 = AtomicU8::new(UNINITIALIZED);

    pub fn set_dyn_thread_safe_mode(mode: bool) {
        let set: u8 = if mode {
            DYN_THREAD_SAFE
        } else {
            DYN_NOT_THREAD_SAFE
        };
        let previous = DYN_THREAD_SAFE_MODE.compare_exchange(
            UNINITIALIZED,
            set,
            Ordering::Relaxed,
            Ordering::Relaxed,
        );

        // Check that the mode was either uninitialized or was already set to the requested mode
        assert!(previous.is_ok() || previous == Err(set));
    }
}
