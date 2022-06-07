#[repr(C)]
pub struct IIsolatedEnvironmentInterop {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHostHwndInterop: unsafe extern "system" fn(this: *mut *mut Self, containerhwnd: super::super::super::Foundation::HWND, hosthwnd: *mut super::super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHostHwndInterop: usize,
}
impl ::windows_sys::core::Interface for IIsolatedEnvironmentInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2238790702, data2: 36450, data3: 18117, data4: [141, 226, 198, 71, 225, 213, 70, 54] };
}
