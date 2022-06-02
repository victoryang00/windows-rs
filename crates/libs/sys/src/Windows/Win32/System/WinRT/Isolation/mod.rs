#[repr(C)]
pub struct IIsolatedEnvironmentInterop {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHostHwndInterop: unsafe extern "system" fn(this: *mut *mut Self, containerhwnd: super::super::super::Foundation::HWND, hosthwnd: *mut super::super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHostHwndInterop: usize,
}
