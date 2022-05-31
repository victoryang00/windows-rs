#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WSL_DISTRIBUTION_FLAGS(pub u32);
pub const WSL_DISTRIBUTION_FLAGS_NONE: WSL_DISTRIBUTION_FLAGS = WSL_DISTRIBUTION_FLAGS(0u32);
pub const WSL_DISTRIBUTION_FLAGS_ENABLE_INTEROP: WSL_DISTRIBUTION_FLAGS = WSL_DISTRIBUTION_FLAGS(1u32);
pub const WSL_DISTRIBUTION_FLAGS_APPEND_NT_PATH: WSL_DISTRIBUTION_FLAGS = WSL_DISTRIBUTION_FLAGS(2u32);
pub const WSL_DISTRIBUTION_FLAGS_ENABLE_DRIVE_MOUNTING: WSL_DISTRIBUTION_FLAGS = WSL_DISTRIBUTION_FLAGS(4u32);
impl ::core::marker::Copy for WSL_DISTRIBUTION_FLAGS {}
impl ::core::clone::Clone for WSL_DISTRIBUTION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSL_DISTRIBUTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WSL_DISTRIBUTION_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WSL_DISTRIBUTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSL_DISTRIBUTION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WSL_DISTRIBUTION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WSL_DISTRIBUTION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WSL_DISTRIBUTION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WSL_DISTRIBUTION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WSL_DISTRIBUTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[inline]
pub unsafe fn WslConfigureDistribution<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(distributionname: Param0, defaultuid: u32, wsldistributionflags: WSL_DISTRIBUTION_FLAGS) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WslConfigureDistribution(distributionname: ::windows_core::PCWSTR, defaultuid: u32, wsldistributionflags: WSL_DISTRIBUTION_FLAGS) -> ::windows_core::HRESULT;
        }
        WslConfigureDistribution(distributionname.into_param().abi(), ::core::mem::transmute(defaultuid), ::core::mem::transmute(wsldistributionflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WslGetDistributionConfiguration<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(distributionname: Param0, distributionversion: *mut u32, defaultuid: *mut u32, wsldistributionflags: *mut WSL_DISTRIBUTION_FLAGS, defaultenvironmentvariables: *mut *mut ::windows_core::PSTR, defaultenvironmentvariablecount: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WslGetDistributionConfiguration(distributionname: ::windows_core::PCWSTR, distributionversion: *mut u32, defaultuid: *mut u32, wsldistributionflags: *mut WSL_DISTRIBUTION_FLAGS, defaultenvironmentvariables: *mut *mut ::windows_core::PSTR, defaultenvironmentvariablecount: *mut u32) -> ::windows_core::HRESULT;
        }
        WslGetDistributionConfiguration(distributionname.into_param().abi(), ::core::mem::transmute(distributionversion), ::core::mem::transmute(defaultuid), ::core::mem::transmute(wsldistributionflags), ::core::mem::transmute(defaultenvironmentvariables), ::core::mem::transmute(defaultenvironmentvariablecount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WslIsDistributionRegistered<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(distributionname: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WslIsDistributionRegistered(distributionname: ::windows_core::PCWSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WslIsDistributionRegistered(distributionname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WslLaunch<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(distributionname: Param0, command: Param1, usecurrentworkingdirectory: Param2, stdin: Param3, stdout: Param4, stderr: Param5) -> ::windows_core::Result<::win32_foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WslLaunch(distributionname: ::windows_core::PCWSTR, command: ::windows_core::PCWSTR, usecurrentworkingdirectory: ::win32_foundation::BOOL, stdin: ::win32_foundation::HANDLE, stdout: ::win32_foundation::HANDLE, stderr: ::win32_foundation::HANDLE, process: *mut ::win32_foundation::HANDLE) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::HANDLE>::zeroed();
        WslLaunch(distributionname.into_param().abi(), command.into_param().abi(), usecurrentworkingdirectory.into_param().abi(), stdin.into_param().abi(), stdout.into_param().abi(), stderr.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WslLaunchInteractive<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(distributionname: Param0, command: Param1, usecurrentworkingdirectory: Param2) -> ::windows_core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WslLaunchInteractive(distributionname: ::windows_core::PCWSTR, command: ::windows_core::PCWSTR, usecurrentworkingdirectory: ::win32_foundation::BOOL, exitcode: *mut u32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        WslLaunchInteractive(distributionname.into_param().abi(), command.into_param().abi(), usecurrentworkingdirectory.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WslRegisterDistribution<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(distributionname: Param0, targzfilename: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WslRegisterDistribution(distributionname: ::windows_core::PCWSTR, targzfilename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        WslRegisterDistribution(distributionname.into_param().abi(), targzfilename.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WslUnregisterDistribution<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(distributionname: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WslUnregisterDistribution(distributionname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        WslUnregisterDistribution(distributionname.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
