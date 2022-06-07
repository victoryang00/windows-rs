#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_WinRT_Pdf\"`, `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn PdfCreateRenderer(pdevice: *mut *mut super::super::super::Graphics::Dxgi::IDXGIDevice, pprenderer: *mut *mut *mut IPdfRendererNative) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
pub struct IPdfRendererNative {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi"))]
    pub RenderPageToSurface: unsafe extern "system" fn(this: *mut *mut Self, pdfpage: *mut ::core::ffi::c_void, psurface: *mut ::core::ffi::c_void, offset: super::super::super::Foundation::POINT, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi")))]
    RenderPageToSurface: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub RenderPageToDeviceContext: unsafe extern "system" fn(this: *mut *mut Self, pdfpage: *mut ::core::ffi::c_void, pd2ddevicecontext: *mut ::core::ffi::c_void, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common")))]
    RenderPageToDeviceContext: usize,
}
impl ::windows_sys::core::Interface for IPdfRendererNative {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2107493777, data2: 53879, data3: 18759, data4: [133, 39, 7, 160, 218, 237, 169, 74] };
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WinRT_Pdf\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub struct PDF_RENDER_PARAMS {
    pub SourceRect: super::super::super::Graphics::Direct2D::Common::D2D_RECT_F,
    pub DestinationWidth: u32,
    pub DestinationHeight: u32,
    pub BackgroundColor: super::super::super::Graphics::Direct2D::Common::D2D_COLOR_F,
    pub IgnoreHighContrast: super::super::super::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::marker::Copy for PDF_RENDER_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::clone::Clone for PDF_RENDER_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Pdf\"`, `\"Win32_Graphics_Dxgi\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub type PFN_PDF_CREATE_RENDERER = ::core::option::Option<unsafe extern "system" fn(param0: *mut *mut super::super::super::Graphics::Dxgi::IDXGIDevice, param1: *mut *mut *mut IPdfRendererNative) -> ::windows_sys::core::HRESULT>;
