#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D11on12\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Direct3D11\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Direct3D11"))]
    pub fn D3D11On12CreateDevice(pdevice: *mut *mut ::windows_sys::core::IUnknown, flags: u32, pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, ppcommandqueues: *const *mut *mut ::windows_sys::core::IUnknown, numqueues: u32, nodemask: u32, ppdevice: *mut *mut *mut super::Direct3D11::ID3D11Device, ppimmediatecontext: *mut *mut *mut super::Direct3D11::ID3D11DeviceContext, pchosenfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D11on12\"`*"]
pub struct D3D11_RESOURCE_FLAGS {
    pub BindFlags: u32,
    pub MiscFlags: u32,
    pub CPUAccessFlags: u32,
    pub StructureByteStride: u32,
}
impl ::core::marker::Copy for D3D11_RESOURCE_FLAGS {}
impl ::core::clone::Clone for D3D11_RESOURCE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ID3D11On12Device {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CreateWrappedResource: unsafe extern "system" fn(this: *mut *mut Self, presource12: *mut ::core::ffi::c_void, pflags11: *const D3D11_RESOURCE_FLAGS, instate: super::Direct3D12::D3D12_RESOURCE_STATES, outstate: super::Direct3D12::D3D12_RESOURCE_STATES, riid: *const ::windows_sys::core::GUID, ppresource11: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CreateWrappedResource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub ReleaseWrappedResources: unsafe extern "system" fn(this: *mut *mut Self, ppresources: *const *mut ::core::ffi::c_void, numresources: u32),
    #[cfg(not(feature = "Win32_Graphics_Direct3D11"))]
    ReleaseWrappedResources: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D11")]
    pub AcquireWrappedResources: unsafe extern "system" fn(this: *mut *mut Self, ppresources: *const *mut ::core::ffi::c_void, numresources: u32),
    #[cfg(not(feature = "Win32_Graphics_Direct3D11"))]
    AcquireWrappedResources: usize,
}
#[repr(C)]
pub struct ID3D11On12Device1 {
    pub base__: ID3D11On12Device,
    pub GetD3D12Device: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ID3D11On12Device2 {
    pub base__: ID3D11On12Device1,
    #[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
    pub UnwrapUnderlyingResource: unsafe extern "system" fn(this: *mut *mut Self, presource11: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12")))]
    UnwrapUnderlyingResource: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12"))]
    pub ReturnUnderlyingResource: unsafe extern "system" fn(this: *mut *mut Self, presource11: *mut ::core::ffi::c_void, numsync: u32, psignalvalues: *const u64, ppfences: *const *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D11", feature = "Win32_Graphics_Direct3D12")))]
    ReturnUnderlyingResource: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D11on12\"`, `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Direct3D11\"`*"]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Direct3D11"))]
pub type PFN_D3D11ON12_CREATE_DEVICE = ::core::option::Option<unsafe extern "system" fn(param0: *mut *mut ::windows_sys::core::IUnknown, param1: u32, param2: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, param4: *const *mut *mut ::windows_sys::core::IUnknown, numqueues: u32, param6: u32, param7: *mut *mut *mut super::Direct3D11::ID3D11Device, param8: *mut *mut *mut super::Direct3D11::ID3D11DeviceContext, param9: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> ::windows_sys::core::HRESULT>;
