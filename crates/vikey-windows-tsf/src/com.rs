//! COM helper utilities

use windows::core::*;
use windows::Win32::System::Com::*;
use std::sync::atomic::{AtomicU32, Ordering};

/// Global DLL reference count
static DLL_REF_COUNT: AtomicU32 = AtomicU32::new(0);

/// Increment DLL reference count
pub fn dll_add_ref() {
    DLL_REF_COUNT.fetch_add(1, Ordering::SeqCst);
}

/// Decrement DLL reference count
pub fn dll_release() {
    DLL_REF_COUNT.fetch_sub(1, Ordering::SeqCst);
}

/// Get current DLL reference count
pub fn dll_get_ref_count() -> u32 {
    DLL_REF_COUNT.load(Ordering::SeqCst)
}

/// Check if DLL can be unloaded
pub fn dll_can_unload() -> bool {
    dll_get_ref_count() == 0
}
