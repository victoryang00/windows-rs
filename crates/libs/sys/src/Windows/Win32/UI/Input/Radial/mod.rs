#[repr(C)]
pub struct IRadialControllerConfigurationInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut *mut Self, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
#[repr(C)]
pub struct IRadialControllerIndependentInputSourceInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateForWindow: unsafe extern "system" fn(this: *mut *mut Self, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateForWindow: usize,
}
#[repr(C)]
pub struct IRadialControllerInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateForWindow: unsafe extern "system" fn(this: *mut *mut Self, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateForWindow: usize,
}
