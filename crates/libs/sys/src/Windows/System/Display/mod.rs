pub type DisplayRequest = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IDisplayRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestActive: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RequestRelease: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
