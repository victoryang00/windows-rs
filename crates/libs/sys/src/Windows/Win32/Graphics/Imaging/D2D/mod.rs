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
impl ::windows_sys::core::Interface for IWICImageEncoder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 80174072, data2: 15585, data3: 18235, data4: [172, 197, 60, 196, 245, 233, 73, 153] };
}
#[repr(C)]
pub struct IWICImagingFactory2 {
    pub base__: super::IWICImagingFactory,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub CreateImageEncoder: unsafe extern "system" fn(this: *mut *mut Self, pd2ddevice: *mut ::core::ffi::c_void, ppwicimageencoder: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))]
    CreateImageEncoder: usize,
}
impl ::windows_sys::core::Interface for IWICImagingFactory2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2072079173, data2: 6550, data3: 17526, data4: [177, 50, 222, 158, 36, 124, 138, 240] };
}
