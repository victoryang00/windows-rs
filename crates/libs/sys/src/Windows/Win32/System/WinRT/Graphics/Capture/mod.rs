#[repr(C)]
pub struct IGraphicsCaptureItemInterop {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateForWindow: unsafe extern "system" fn(this: *mut *mut Self, window: super::super::super::super::Foundation::HWND, riid: *const ::windows_sys::core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateForWindow: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreateForMonitor: unsafe extern "system" fn(this: *mut *mut Self, monitor: super::super::super::super::Graphics::Gdi::HMONITOR, riid: *const ::windows_sys::core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreateForMonitor: usize,
}
impl ::windows_sys::core::Interface for IGraphicsCaptureItemInterop {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 908650523, data2: 15532, data3: 19552, data4: [183, 244, 35, 206, 14, 12, 51, 86] };
}
