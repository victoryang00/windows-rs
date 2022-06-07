#[repr(C)]
pub struct IDummyHICONIncluder {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub Dummy: unsafe extern "system" fn(this: *mut *mut Self, h1: super::super::super::UI::WindowsAndMessaging::HICON, h2: super::super::super::Graphics::Gdi::HDC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))]
    Dummy: usize,
}
impl ::windows_sys::core::Interface for IDummyHICONIncluder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2490994910, data2: 52264, data3: 4562, data4: [160, 247, 0, 128, 95, 133, 143, 177] };
}
#[repr(C)]
pub struct IThumbnailExtractor {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub ExtractThumbnail: unsafe extern "system" fn(this: *mut *mut Self, pstg: *mut ::core::ffi::c_void, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut super::super::super::Graphics::Gdi::HBITMAP) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    ExtractThumbnail: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub OnFileUpdated: unsafe extern "system" fn(this: *mut *mut Self, pstg: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    OnFileUpdated: usize,
}
impl ::windows_sys::core::Interface for IThumbnailExtractor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2526922504, data2: 23670, data3: 4561, data4: [141, 134, 0, 0, 248, 4, 176, 87] };
}
