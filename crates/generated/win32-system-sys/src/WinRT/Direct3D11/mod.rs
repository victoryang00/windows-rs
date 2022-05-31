#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn CreateDirect3D11DeviceFromDXGIDevice(dxgidevice: super::super::super::Graphics::Dxgi::IDXGIDevice, graphicsdevice: *mut ::windows_core_sys::IInspectable) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn CreateDirect3D11SurfaceFromDXGISurface(dgxisurface: super::super::super::Graphics::Dxgi::IDXGISurface, graphicssurface: *mut ::windows_core_sys::IInspectable) -> ::windows_core_sys::HRESULT;
}
pub type IDirect3DDxgiInterfaceAccess = *mut ::core::ffi::c_void;
