#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "win32-security-sys")]
    pub fn CreateMailslotA(lpname: ::windows_core_sys::PCSTR, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES) -> ::win32_foundation_sys::HANDLE;
    #[cfg(feature = "win32-security-sys")]
    pub fn CreateMailslotW(lpname: ::windows_core_sys::PCWSTR, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: *const ::win32_security_sys::SECURITY_ATTRIBUTES) -> ::win32_foundation_sys::HANDLE;
    pub fn GetMailslotInfo(hmailslot: ::win32_foundation_sys::HANDLE, lpmaxmessagesize: *mut u32, lpnextsize: *mut u32, lpmessagecount: *mut u32, lpreadtimeout: *mut u32) -> ::win32_foundation_sys::BOOL;
    pub fn SetMailslotInfo(hmailslot: ::win32_foundation_sys::HANDLE, lreadtimeout: u32) -> ::win32_foundation_sys::BOOL;
}
