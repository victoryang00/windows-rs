#[repr(C)]
pub struct IDummyHICONIncluder {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub Dummy: unsafe extern "system" fn(this: *mut *mut Self, h1: super::super::super::UI::WindowsAndMessaging::HICON, h2: super::super::super::Graphics::Gdi::HDC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))]
    Dummy: usize,
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
