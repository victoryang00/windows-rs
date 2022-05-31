#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    pub fn Direct3DCreate9On12(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32) -> super::Direct3D9::IDirect3D9;
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    pub fn Direct3DCreate9On12Ex(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32, ppoutputinterface: *mut super::Direct3D9::IDirect3D9Ex) -> ::windows_core_sys::HRESULT;
}
#[repr(C)]
pub struct D3D9ON12_ARGS {
    pub Enable9On12: ::win32_foundation_sys::BOOL,
    pub pD3D12Device: ::windows_core_sys::IUnknown,
    pub ppD3D12Queues: [::windows_core_sys::IUnknown; 2],
    pub NumQueues: u32,
    pub NodeMask: u32,
}
impl ::core::marker::Copy for D3D9ON12_ARGS {}
impl ::core::clone::Clone for D3D9ON12_ARGS {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IDirect3DDevice9On12 = *mut ::core::ffi::c_void;
pub const MAX_D3D9ON12_QUEUES: u32 = 2u32;
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub type PFN_Direct3DCreate9On12 = ::core::option::Option<unsafe extern "system" fn(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32) -> super::Direct3D9::IDirect3D9>;
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub type PFN_Direct3DCreate9On12Ex = ::core::option::Option<unsafe extern "system" fn(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32, ppoutputinterface: *mut super::Direct3D9::IDirect3D9Ex) -> ::windows_core_sys::HRESULT>;
