#[link(name = "windows")]
extern "system" {
    pub fn CreateAppContainerProfile(pszappcontainername: ::windows_core_sys::PCWSTR, pszdisplayname: ::windows_core_sys::PCWSTR, pszdescription: ::windows_core_sys::PCWSTR, pcapabilities: *const super::SID_AND_ATTRIBUTES, dwcapabilitycount: u32, ppsidappcontainersid: *mut ::win32_foundation_sys::PSID) -> ::windows_core_sys::HRESULT;
    pub fn DeleteAppContainerProfile(pszappcontainername: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn DeriveAppContainerSidFromAppContainerName(pszappcontainername: ::windows_core_sys::PCWSTR, ppsidappcontainersid: *mut ::win32_foundation_sys::PSID) -> ::windows_core_sys::HRESULT;
    pub fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(psidappcontainersid: ::win32_foundation_sys::PSID, pszrestrictedappcontainername: ::windows_core_sys::PCWSTR, ppsidrestrictedappcontainersid: *mut ::win32_foundation_sys::PSID) -> ::windows_core_sys::HRESULT;
    pub fn GetAppContainerFolderPath(pszappcontainersid: ::windows_core_sys::PCWSTR, ppszpath: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn GetAppContainerNamedObjectPath(token: ::win32_foundation_sys::HANDLE, appcontainersid: ::win32_foundation_sys::PSID, objectpathlength: u32, objectpath: ::windows_core_sys::PWSTR, returnlength: *mut u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-system-sys")]
    pub fn GetAppContainerRegistryLocation(desiredaccess: u32, phappcontainerkey: *mut ::win32_system_sys::Registry::HKEY) -> ::windows_core_sys::HRESULT;
    pub fn IsProcessInIsolatedContainer(isprocessinisolatedcontainer: *mut ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    pub fn IsProcessInIsolatedWindowsEnvironment(isprocessinisolatedwindowsenvironment: *mut ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    pub fn IsProcessInWDAGContainer(reserved: *const ::core::ffi::c_void, isprocessinwdagcontainer: *mut ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
}
pub type IIsolatedAppLauncher = *mut ::core::ffi::c_void;
pub const IsolatedAppLauncher: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 3162580016, data2: 59230, data3: 20433, data4: [150, 65, 31, 159, 30, 45, 154, 31] };
#[repr(C)]
pub struct IsolatedAppLauncherTelemetryParameters {
    pub EnableForLaunch: ::win32_foundation_sys::BOOL,
    pub CorrelationGUID: ::windows_core_sys::GUID,
}
impl ::core::marker::Copy for IsolatedAppLauncherTelemetryParameters {}
impl ::core::clone::Clone for IsolatedAppLauncherTelemetryParameters {
    fn clone(&self) -> Self {
        *self
    }
}
