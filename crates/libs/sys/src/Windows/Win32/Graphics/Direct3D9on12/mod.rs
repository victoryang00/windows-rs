#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9on12\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
    pub fn Direct3DCreate9On12(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32) -> *mut *mut super::Direct3D9::IDirect3D9;
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D9on12\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
    pub fn Direct3DCreate9On12Ex(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32, ppoutputinterface: *mut *mut *mut super::Direct3D9::IDirect3D9Ex) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9on12\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D9ON12_ARGS {
    pub Enable9On12: super::super::Foundation::BOOL,
    pub pD3D12Device: *mut *mut *mut *mut ::windows_sys::core::IUnknown,
    pub ppD3D12Queues: [*mut *mut ::windows_sys::core::IUnknown; 2],
    pub NumQueues: u32,
    pub NodeMask: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D9ON12_ARGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D9ON12_ARGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IDirect3DDevice9On12 {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetD3D12Device: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppvdevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
    pub UnwrapUnderlyingResource: unsafe extern "system" fn(this: *mut *mut Self, presource: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, ppvresource12: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9")))]
    UnwrapUnderlyingResource: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9"))]
    pub ReturnUnderlyingResource: unsafe extern "system" fn(this: *mut *mut Self, presource: *mut ::core::ffi::c_void, numsync: u32, psignalvalues: *mut u64, ppfences: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Direct3D9")))]
    ReturnUnderlyingResource: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9on12\"`*"]
pub const MAX_D3D9ON12_QUEUES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9on12\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub type PFN_Direct3DCreate9On12 = ::core::option::Option<unsafe extern "system" fn(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32) -> *mut *mut super::Direct3D9::IDirect3D9>;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D9on12\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
pub type PFN_Direct3DCreate9On12Ex = ::core::option::Option<unsafe extern "system" fn(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32, ppoutputinterface: *mut *mut *mut super::Direct3D9::IDirect3D9Ex) -> ::windows_sys::core::HRESULT>;
