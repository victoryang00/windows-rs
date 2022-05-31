#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`*"]
    pub fn WslConfigureDistribution(distributionname: ::windows_core_sys::PCWSTR, defaultuid: u32, wsldistributionflags: WSL_DISTRIBUTION_FLAGS) -> ::windows_core_sys::HRESULT;
    #[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`*"]
    pub fn WslGetDistributionConfiguration(distributionname: ::windows_core_sys::PCWSTR, distributionversion: *mut u32, defaultuid: *mut u32, wsldistributionflags: *mut WSL_DISTRIBUTION_FLAGS, defaultenvironmentvariables: *mut *mut ::windows_core_sys::PSTR, defaultenvironmentvariablecount: *mut u32) -> ::windows_core_sys::HRESULT;
    #[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WslIsDistributionRegistered(distributionname: ::windows_core_sys::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WslLaunch(distributionname: ::windows_core_sys::PCWSTR, command: ::windows_core_sys::PCWSTR, usecurrentworkingdirectory: super::super::Foundation::BOOL, stdin: super::super::Foundation::HANDLE, stdout: super::super::Foundation::HANDLE, stderr: super::super::Foundation::HANDLE, process: *mut super::super::Foundation::HANDLE) -> ::windows_core_sys::HRESULT;
    #[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WslLaunchInteractive(distributionname: ::windows_core_sys::PCWSTR, command: ::windows_core_sys::PCWSTR, usecurrentworkingdirectory: super::super::Foundation::BOOL, exitcode: *mut u32) -> ::windows_core_sys::HRESULT;
    #[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`*"]
    pub fn WslRegisterDistribution(distributionname: ::windows_core_sys::PCWSTR, targzfilename: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    #[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`*"]
    pub fn WslUnregisterDistribution(distributionname: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
}
#[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`*"]
pub type WSL_DISTRIBUTION_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`*"]
pub const WSL_DISTRIBUTION_FLAGS_NONE: WSL_DISTRIBUTION_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`*"]
pub const WSL_DISTRIBUTION_FLAGS_ENABLE_INTEROP: WSL_DISTRIBUTION_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`*"]
pub const WSL_DISTRIBUTION_FLAGS_APPEND_NT_PATH: WSL_DISTRIBUTION_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`*"]
pub const WSL_DISTRIBUTION_FLAGS_ENABLE_DRIVE_MOUNTING: WSL_DISTRIBUTION_FLAGS = 4u32;
