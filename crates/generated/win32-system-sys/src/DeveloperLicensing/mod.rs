#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn AcquireDeveloperLicense(hwndparent: super::super::Foundation::HWND, pexpiration: *mut super::super::Foundation::FILETIME) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckDeveloperLicense(pexpiration: *mut super::super::Foundation::FILETIME) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveDeveloperLicense(hwndparent: super::super::Foundation::HWND) -> ::windows_core_sys::HRESULT;
}
