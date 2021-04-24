use crate::ffi;
use crate::from_raw::FromRaw;
use std::ffi::c_void;

#[derive(Debug, Clone)]
pub struct Unit {
    raw: *const ffi::UnitInterface,
}

impl FromRaw for Unit {
    unsafe fn from_raw(raw: *const c_void) -> Self {
        assert!(!raw.is_null());
        // Self::from_raw(raw as *const ffi::UnitInterface)
        Self { raw: raw as *const ffi::UnitInterface }
    }
}

impl Unit {
    // pub unsafe fn from_raw(raw: *const ffi::UnitInterface) -> Self {
    //     assert!(!raw.is_null());
    //     Self { raw }
    // }

    pub fn id(&self) -> i32 {
        unsafe { ffi::Unit_getId(self.raw) }
    }
}
