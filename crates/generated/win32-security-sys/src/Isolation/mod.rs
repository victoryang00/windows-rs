#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateAppContainerProfile(pszappcontainername: ::windows_core_sys::PCWSTR, pszdisplayname: ::windows_core_sys::PCWSTR, pszdescription: ::windows_core_sys::PCWSTR, pcapabilities: *const super::SID_AND_ATTRIBUTES, dwcapabilitycount: u32, ppsidappcontainersid: *mut super::super::Foundation::PSID) -> ::windows_core_sys::HRESULT;
    pub fn DeleteAppContainerProfile(pszappcontainername: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeriveAppContainerSidFromAppContainerName(pszappcontainername: ::windows_core_sys::PCWSTR, ppsidappcontainersid: *mut super::super::Foundation::PSID) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(psidappcontainersid: super::super::Foundation::PSID, pszrestrictedappcontainername: ::windows_core_sys::PCWSTR, ppsidrestrictedappcontainersid: *mut super::super::Foundation::PSID) -> ::windows_core_sys::HRESULT;
    pub fn GetAppContainerFolderPath(pszappcontainersid: ::windows_core_sys::PCWSTR, ppszpath: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAppContainerNamedObjectPath(token: super::super::Foundation::HANDLE, appcontainersid: super::super::Foundation::PSID, objectpathlength: u32, objectpath: ::windows_core_sys::PWSTR, returnlength: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_System_Registry")]
    pub fn GetAppContainerRegistryLocation(desiredaccess: u32, phappcontainerkey: *mut super::super::System::Registry::HKEY) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsProcessInIsolatedContainer(isprocessinisolatedcontainer: *mut super::super::Foundation::BOOL) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsProcessInIsolatedWindowsEnvironment(isprocessinisolatedwindowsenvironment: *mut super::super::Foundation::BOOL) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsProcessInWDAGContainer(reserved: *const ::core::ffi::c_void, isprocessinwdagcontainer: *mut super::super::Foundation::BOOL) -> ::windows_core_sys::HRESULT;
}
pub type IIsolatedAppLauncher = *mut ::core::ffi::c_void;
pub const IsolatedAppLauncher: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 3162580016, data2: 59230, data3: 20433, data4: [150, 65, 31, 159, 30, 45, 154, 31] };
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IsolatedAppLauncherTelemetryParameters {
    pub EnableForLaunch: super::super::Foundation::BOOL,
    pub CorrelationGUID: ::windows_core_sys::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IsolatedAppLauncherTelemetryParameters {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IsolatedAppLauncherTelemetryParameters {
    fn clone(&self) -> Self {
        *self
    }
}
