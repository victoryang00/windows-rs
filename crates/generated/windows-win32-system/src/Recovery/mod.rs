#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplicationRecoveryFinished<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(bsuccess: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ApplicationRecoveryFinished(bsuccess: super::super::Foundation::BOOL);
        }
        ApplicationRecoveryFinished(bsuccess.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplicationRecoveryInProgress() -> ::windows_core::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ApplicationRecoveryInProgress(pbcancelled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        ApplicationRecoveryInProgress(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn GetApplicationRecoveryCallback<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, precoverycallback: *mut super::WindowsProgramming::APPLICATION_RECOVERY_CALLBACK, ppvparameter: *mut *mut ::core::ffi::c_void, pdwpinginterval: *mut u32, pdwflags: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetApplicationRecoveryCallback(hprocess: super::super::Foundation::HANDLE, precoverycallback: *mut ::windows_core::RawPtr, ppvparameter: *mut *mut ::core::ffi::c_void, pdwpinginterval: *mut u32, pdwflags: *mut u32) -> ::windows_core::HRESULT;
        }
        GetApplicationRecoveryCallback(hprocess.into_param().abi(), ::core::mem::transmute(precoverycallback), ::core::mem::transmute(ppvparameter), ::core::mem::transmute(pdwpinginterval), ::core::mem::transmute(pdwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetApplicationRestartSettings<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, pwzcommandline: ::windows_core::PWSTR, pcchsize: *mut u32, pdwflags: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetApplicationRestartSettings(hprocess: super::super::Foundation::HANDLE, pwzcommandline: ::windows_core::PWSTR, pcchsize: *mut u32, pdwflags: *mut u32) -> ::windows_core::HRESULT;
        }
        GetApplicationRestartSettings(hprocess.into_param().abi(), ::core::mem::transmute(pwzcommandline), ::core::mem::transmute(pcchsize), ::core::mem::transmute(pdwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REGISTER_APPLICATION_RESTART_FLAGS(pub u32);
pub const RESTART_NO_CRASH: REGISTER_APPLICATION_RESTART_FLAGS = REGISTER_APPLICATION_RESTART_FLAGS(1u32);
pub const RESTART_NO_HANG: REGISTER_APPLICATION_RESTART_FLAGS = REGISTER_APPLICATION_RESTART_FLAGS(2u32);
pub const RESTART_NO_PATCH: REGISTER_APPLICATION_RESTART_FLAGS = REGISTER_APPLICATION_RESTART_FLAGS(4u32);
pub const RESTART_NO_REBOOT: REGISTER_APPLICATION_RESTART_FLAGS = REGISTER_APPLICATION_RESTART_FLAGS(8u32);
impl ::core::marker::Copy for REGISTER_APPLICATION_RESTART_FLAGS {}
impl ::core::clone::Clone for REGISTER_APPLICATION_RESTART_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REGISTER_APPLICATION_RESTART_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for REGISTER_APPLICATION_RESTART_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for REGISTER_APPLICATION_RESTART_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REGISTER_APPLICATION_RESTART_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for REGISTER_APPLICATION_RESTART_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for REGISTER_APPLICATION_RESTART_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for REGISTER_APPLICATION_RESTART_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for REGISTER_APPLICATION_RESTART_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for REGISTER_APPLICATION_RESTART_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[inline]
pub unsafe fn RegisterApplicationRecoveryCallback(precoveycallback: super::WindowsProgramming::APPLICATION_RECOVERY_CALLBACK, pvparameter: *const ::core::ffi::c_void, dwpinginterval: u32, dwflags: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterApplicationRecoveryCallback(precoveycallback: ::windows_core::RawPtr, pvparameter: *const ::core::ffi::c_void, dwpinginterval: u32, dwflags: u32) -> ::windows_core::HRESULT;
        }
        RegisterApplicationRecoveryCallback(::core::mem::transmute(precoveycallback), ::core::mem::transmute(pvparameter), ::core::mem::transmute(dwpinginterval), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RegisterApplicationRestart<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pwzcommandline: Param0, dwflags: REGISTER_APPLICATION_RESTART_FLAGS) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterApplicationRestart(pwzcommandline: ::windows_core::PCWSTR, dwflags: REGISTER_APPLICATION_RESTART_FLAGS) -> ::windows_core::HRESULT;
        }
        RegisterApplicationRestart(pwzcommandline.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UnregisterApplicationRecoveryCallback() -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterApplicationRecoveryCallback() -> ::windows_core::HRESULT;
        }
        UnregisterApplicationRecoveryCallback().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UnregisterApplicationRestart() -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterApplicationRestart() -> ::windows_core::HRESULT;
        }
        UnregisterApplicationRestart().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
