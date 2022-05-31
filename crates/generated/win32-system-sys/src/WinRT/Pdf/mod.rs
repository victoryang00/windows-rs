#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn PdfCreateRenderer(pdevice: ::win32_graphics_sys::Dxgi::IDXGIDevice, pprenderer: *mut IPdfRendererNative) -> ::windows_core_sys::HRESULT;
}
pub type IPdfRendererNative = *mut ::core::ffi::c_void;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub struct PDF_RENDER_PARAMS {
    pub SourceRect: ::win32_graphics_sys::Direct2D::Common::D2D_RECT_F,
    pub DestinationWidth: u32,
    pub DestinationHeight: u32,
    pub BackgroundColor: ::win32_graphics_sys::Direct2D::Common::D2D_COLOR_F,
    pub IgnoreHighContrast: ::win32_foundation_sys::BOOLEAN,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::marker::Copy for PDF_RENDER_PARAMS {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::clone::Clone for PDF_RENDER_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub type PFN_PDF_CREATE_RENDERER = ::core::option::Option<unsafe extern "system" fn(param0: ::win32_graphics_sys::Dxgi::IDXGIDevice, param1: *mut IPdfRendererNative) -> ::windows_core_sys::HRESULT>;
