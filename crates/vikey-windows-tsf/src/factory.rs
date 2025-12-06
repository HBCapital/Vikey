//! Class Factory for Vikey Text Service

use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::Com::*;
use std::sync::atomic::{AtomicU32, Ordering};
use crate::com::{dll_add_ref, dll_release};
use crate::text_service::TextService;

/// Class Factory for creating TextService instances
#[implement(IClassFactory)]
pub struct ClassFactory {
    ref_count: AtomicU32,
}

impl ClassFactory {
    pub fn new() -> Self {
        dll_add_ref();
        Self {
            ref_count: AtomicU32::new(1),
        }
    }
}

impl IClassFactory_Impl for ClassFactory {
    fn CreateInstance(
        &self,
        punkouter: Option<&IUnknown>,
        riid: *const GUID,
        ppvobject: *mut *mut core::ffi::c_void,
    ) -> Result<()> {
        unsafe {
            if ppvobject.is_null() {
                return Err(E_POINTER.into());
            }
            
            *ppvobject = std::ptr::null_mut();
            
            // We don't support aggregation
            if punkouter.is_some() {
                return Err(CLASS_E_NOAGGREGATION.into());
            }
            
            // Create TextService instance
            let text_service: IUnknown = TextService::new().into();
            
            // Query for requested interface
            let hr = text_service.query(riid, ppvobject as *mut _);
            if hr.is_ok() {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }
    
    fn LockServer(&self, flock: BOOL) -> Result<()> {
        if flock.as_bool() {
            dll_add_ref();
        } else {
            dll_release();
        }
        Ok(())
    }
}

impl Drop for ClassFactory {
    fn drop(&mut self) {
        dll_release();
    }
}
