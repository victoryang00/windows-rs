pub type DesktopWindowTarget = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IDesktopWindowTarget {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTopmost: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
