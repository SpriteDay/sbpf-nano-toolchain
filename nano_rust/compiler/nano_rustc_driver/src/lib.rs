// This crate is intentionally empty and a re-export of `nano_rustc_driver_impl` to allow the code
// in `nano_rsut_driver_impl` to be compiled in parallel with other crates
// UPD: I actually removed dylib thing to make build simpler, so this wrapper exists just to mirror original Rust reference structure

pub use nano_rustc_driver_impl::*;
