use std::sync::atomic::AtomicU64;

/// Incrementing ID for boba items
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct BobaId {
    _id: u64,
}

impl Default for BobaId {
    fn default() -> Self {
        Self::new()
    }
}

impl BobaId {
    /// Creates a new BobaId.
    ///
    /// It increments a atomic u64 and uses that as its id value, so each Id will be constructed with a unique value.
    /// This will never run out because there are more possible ids than there are atoms in the universe.
    pub fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        Self {
            _id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        }
    }
}
