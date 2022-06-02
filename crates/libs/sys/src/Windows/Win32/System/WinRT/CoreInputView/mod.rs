#[repr(C)]
pub struct ICoreFrameworkInputViewInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut *mut Self, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, coreframeworkinputview: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
