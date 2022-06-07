#[repr(C)]
pub struct ICoreFrameworkInputViewInterop {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut *mut Self, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, coreframeworkinputview: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
impl ::windows_sys::core::Interface for ICoreFrameworkInputViewInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 238920514, data2: 45340, data3: 18507, data4: [156, 28, 190, 13, 97, 194, 246, 197] };
}
