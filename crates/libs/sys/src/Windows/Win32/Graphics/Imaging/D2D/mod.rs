#[repr(C)]
pub struct IWICImageEncoder {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub WriteFrame: unsafe extern "system" fn(this: *mut *mut Self, pimage: *mut ::core::ffi::c_void, pframeencode: *mut ::core::ffi::c_void, pimageparameters: *const super::WICImageParameters) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    WriteFrame: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub WriteFrameThumbnail: unsafe extern "system" fn(this: *mut *mut Self, pimage: *mut ::core::ffi::c_void, pframeencode: *mut ::core::ffi::c_void, pimageparameters: *const super::WICImageParameters) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    WriteFrameThumbnail: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub WriteThumbnail: unsafe extern "system" fn(this: *mut *mut Self, pimage: *mut ::core::ffi::c_void, pencoder: *mut ::core::ffi::c_void, pimageparameters: *const super::WICImageParameters) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    WriteThumbnail: usize,
}
#[repr(C)]
pub struct IWICImagingFactory2 {
    pub base__: super::IWICImagingFactory,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub CreateImageEncoder: unsafe extern "system" fn(this: *mut *mut Self, pd2ddevice: *mut ::core::ffi::c_void, ppwicimageencoder: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))]
    CreateImageEncoder: usize,
}
