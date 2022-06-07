#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_WinRT_Direct3D11\"`, `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn CreateDirect3D11DeviceFromDXGIDevice(dxgidevice: *mut *mut super::super::super::Graphics::Dxgi::IDXGIDevice, graphicsdevice: *mut *mut *mut ::windows_sys::core::IInspectable) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_WinRT_Direct3D11\"`, `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn CreateDirect3D11SurfaceFromDXGISurface(dgxisurface: *mut *mut super::super::super::Graphics::Dxgi::IDXGISurface, graphicssurface: *mut *mut *mut ::windows_sys::core::IInspectable) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
pub struct IDirect3DDxgiInterfaceAccess {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetInterface: unsafe extern "system" fn(this: *mut *mut Self, iid: *const ::windows_sys::core::GUID, p: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDirect3DDxgiInterfaceAccess {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2847133714, data2: 15858, data3: 20195, data4: [184, 209, 134, 149, 244, 87, 211, 193] };
}
