#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn UalInstrument(data: *const UAL_DATA_BLOB) -> ::windows_core_sys::HRESULT;
    pub fn UalRegisterProduct(wszproductname: ::windows_core_sys::PCWSTR, wszrolename: ::windows_core_sys::PCWSTR, wszguid: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn UalStart(data: *const UAL_DATA_BLOB) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub fn UalStop(data: *const UAL_DATA_BLOB) -> ::windows_core_sys::HRESULT;
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct UAL_DATA_BLOB {
    pub Size: u32,
    pub RoleGuid: ::windows_core_sys::GUID,
    pub TenantId: ::windows_core_sys::GUID,
    pub Address: ::win32_networking_sys::WinSock::SOCKADDR_STORAGE,
    pub UserName: [u16; 260],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for UAL_DATA_BLOB {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for UAL_DATA_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
