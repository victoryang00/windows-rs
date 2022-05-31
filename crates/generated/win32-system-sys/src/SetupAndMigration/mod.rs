#[link(name = "windows")]
extern "system" {
    pub fn OOBEComplete(isoobecomplete: *mut ::win32_foundation_sys::BOOL) -> ::win32_foundation_sys::BOOL;
    pub fn RegisterWaitUntilOOBECompleted(oobecompletedcallback: OOBE_COMPLETED_CALLBACK, callbackcontext: *const ::core::ffi::c_void, waithandle: *mut *mut ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
    pub fn UnregisterWaitUntilOOBECompleted(waithandle: *const ::core::ffi::c_void) -> ::win32_foundation_sys::BOOL;
}
pub type OOBE_COMPLETED_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackcontext: *const ::core::ffi::c_void)>;
