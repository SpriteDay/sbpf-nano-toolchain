mod mode {
    const UNINITIALIZED: u8 = 0;
    const DYN_NOT_THREAD_SAFE: u8 = 1;
    const DYN_THREAD_SAFE: u8 = 2;

    pub fn set_dyn_thread_safe_mode(mode: bool) {
        let set: u8 = if mode {
            DYN_THREAD_SAFE
        } else {
            DYN_NOT_THREAD_SAFE
        };
    }
}
