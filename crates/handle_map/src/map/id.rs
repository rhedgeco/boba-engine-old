use std::sync::atomic::{AtomicU16, Ordering};

/// A simple ZST for generating unique `u16` values for use in handle maps.
pub struct HandleMapId {
    // use private `()` to prevent struct from being created
    _private: (),
}

impl HandleMapId {
    /// Returns a new unique u16 value
    pub fn generate() -> u16 {
        static ID_GEN: AtomicU16 = AtomicU16::new(0);
        ID_GEN.fetch_add(1, Ordering::Relaxed)
    }
}
