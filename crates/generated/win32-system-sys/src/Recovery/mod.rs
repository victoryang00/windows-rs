#[link(name = "windows")]
extern "system" {
    pub fn ApplicationRecoveryFinished(bsuccess: ::win32_foundation_sys::BOOL);
    pub fn ApplicationRecoveryInProgress(pbcancelled: *mut ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_WindowsProgramming")]
    pub fn GetApplicationRecoveryCallback(hprocess: ::win32_foundation_sys::HANDLE, precoverycallback: *mut super::WindowsProgramming::APPLICATION_RECOVERY_CALLBACK, ppvparameter: *mut *mut ::core::ffi::c_void, pdwpinginterval: *mut u32, pdwflags: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn GetApplicationRestartSettings(hprocess: ::win32_foundation_sys::HANDLE, pwzcommandline: ::windows_core_sys::PWSTR, pcchsize: *mut u32, pdwflags: *mut u32) -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "Win32_System_WindowsProgramming")]
    pub fn RegisterApplicationRecoveryCallback(precoveycallback: super::WindowsProgramming::APPLICATION_RECOVERY_CALLBACK, pvparameter: *const ::core::ffi::c_void, dwpinginterval: u32, dwflags: u32) -> ::windows_core_sys::HRESULT;
    pub fn RegisterApplicationRestart(pwzcommandline: ::windows_core_sys::PCWSTR, dwflags: REGISTER_APPLICATION_RESTART_FLAGS) -> ::windows_core_sys::HRESULT;
    pub fn UnregisterApplicationRecoveryCallback() -> ::windows_core_sys::HRESULT;
    pub fn UnregisterApplicationRestart() -> ::windows_core_sys::HRESULT;
}
pub type REGISTER_APPLICATION_RESTART_FLAGS = u32;
pub const RESTART_NO_CRASH: REGISTER_APPLICATION_RESTART_FLAGS = 1u32;
pub const RESTART_NO_HANG: REGISTER_APPLICATION_RESTART_FLAGS = 2u32;
pub const RESTART_NO_PATCH: REGISTER_APPLICATION_RESTART_FLAGS = 4u32;
pub const RESTART_NO_REBOOT: REGISTER_APPLICATION_RESTART_FLAGS = 8u32;
