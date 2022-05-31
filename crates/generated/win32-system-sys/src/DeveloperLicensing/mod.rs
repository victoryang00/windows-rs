#[link(name = "windows")]
extern "system" {
    pub fn AcquireDeveloperLicense(hwndparent: ::win32_foundation_sys::HWND, pexpiration: *mut ::win32_foundation_sys::FILETIME) -> ::windows_core_sys::HRESULT;
    pub fn CheckDeveloperLicense(pexpiration: *mut ::win32_foundation_sys::FILETIME) -> ::windows_core_sys::HRESULT;
    pub fn RemoveDeveloperLicense(hwndparent: ::win32_foundation_sys::HWND) -> ::windows_core_sys::HRESULT;
}
