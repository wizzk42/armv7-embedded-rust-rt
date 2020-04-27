//! primask module
//! provides:


/// All exceptions with configurable priority are ...
#[allow(clippy::missing_inline_in_public_items)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Primask {
    /// Active
    Active,
    /// Inactive
    Inactive,
}

impl Primask {
    /// All exceptions with configurable priority are active
    #[inline]
    pub fn is_active(self) -> bool {
        self == Primask::Active
    }

    /// All exceptions with configurable priority are inactive
    #[inline]
    pub fn is_inactive(self) -> bool {
        self == Primask::Inactive
    }
}

/// Reads the CPU register
#[inline]
pub fn read() -> Primask {
    extern "C" {
        fn __primask() -> u32;
    }

    let r = unsafe { __primask() };

    if r & 1 == 1 {
        Primask::Inactive
    } else {
        Primask::Active
    }
}
