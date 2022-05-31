#[inline]
pub unsafe fn CreateAppContainerProfile<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pszappcontainername: Param0, pszdisplayname: Param1, pszdescription: Param2, pcapabilities: &[super::SID_AND_ATTRIBUTES]) -> ::windows_core::Result<::win32_foundation::PSID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateAppContainerProfile(pszappcontainername: ::windows_core::PCWSTR, pszdisplayname: ::windows_core::PCWSTR, pszdescription: ::windows_core::PCWSTR, pcapabilities: *const super::SID_AND_ATTRIBUTES, dwcapabilitycount: u32, ppsidappcontainersid: *mut ::win32_foundation::PSID) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::PSID>::zeroed();
        CreateAppContainerProfile(pszappcontainername.into_param().abi(), pszdisplayname.into_param().abi(), pszdescription.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(pcapabilities)), pcapabilities.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::PSID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DeleteAppContainerProfile<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pszappcontainername: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteAppContainerProfile(pszappcontainername: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        DeleteAppContainerProfile(pszappcontainername.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DeriveAppContainerSidFromAppContainerName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pszappcontainername: Param0) -> ::windows_core::Result<::win32_foundation::PSID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeriveAppContainerSidFromAppContainerName(pszappcontainername: ::windows_core::PCWSTR, ppsidappcontainersid: *mut ::win32_foundation::PSID) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::PSID>::zeroed();
        DeriveAppContainerSidFromAppContainerName(pszappcontainername.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::PSID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(psidappcontainersid: Param0, pszrestrictedappcontainername: Param1) -> ::windows_core::Result<::win32_foundation::PSID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(psidappcontainersid: ::win32_foundation::PSID, pszrestrictedappcontainername: ::windows_core::PCWSTR, ppsidrestrictedappcontainersid: *mut ::win32_foundation::PSID) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::PSID>::zeroed();
        DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(psidappcontainersid.into_param().abi(), pszrestrictedappcontainername.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::PSID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetAppContainerFolderPath<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pszappcontainersid: Param0) -> ::windows_core::Result<::windows_core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAppContainerFolderPath(pszappcontainersid: ::windows_core::PCWSTR, ppszpath: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        GetAppContainerFolderPath(pszappcontainersid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetAppContainerNamedObjectPath<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>>(token: Param0, appcontainersid: Param1, objectpath: &mut [u16], returnlength: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAppContainerNamedObjectPath(token: ::win32_foundation::HANDLE, appcontainersid: ::win32_foundation::PSID, objectpathlength: u32, objectpath: ::windows_core::PWSTR, returnlength: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetAppContainerNamedObjectPath(token.into_param().abi(), appcontainersid.into_param().abi(), objectpath.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(objectpath)), ::core::mem::transmute(returnlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-system")]
#[inline]
pub unsafe fn GetAppContainerRegistryLocation(desiredaccess: u32) -> ::windows_core::Result<::win32_system::Registry::HKEY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAppContainerRegistryLocation(desiredaccess: u32, phappcontainerkey: *mut ::win32_system::Registry::HKEY) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_system::Registry::HKEY>::zeroed();
        GetAppContainerRegistryLocation(::core::mem::transmute(desiredaccess), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Registry::HKEY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
pub struct IIsolatedAppLauncher(::windows_core::IUnknown);
impl IIsolatedAppLauncher {
    pub unsafe fn Launch<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, appusermodelid: Param0, arguments: Param1, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Launch)(::windows_core::Interface::as_raw(self), appusermodelid.into_param().abi(), arguments.into_param().abi(), ::core::mem::transmute(telemetryparameters)).ok()
    }
}
impl ::core::convert::From<IIsolatedAppLauncher> for ::windows_core::IUnknown {
    fn from(value: IIsolatedAppLauncher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IIsolatedAppLauncher> for ::windows_core::IUnknown {
    fn from(value: &IIsolatedAppLauncher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IIsolatedAppLauncher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IIsolatedAppLauncher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IIsolatedAppLauncher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IIsolatedAppLauncher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IIsolatedAppLauncher {}
impl ::core::fmt::Debug for IIsolatedAppLauncher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIsolatedAppLauncher").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IIsolatedAppLauncher {
    type Vtable = IIsolatedAppLauncher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf686878f_7b42_4cc4_96fb_f4f3b6e3d24d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedAppLauncher_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Launch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appusermodelid: ::windows_core::PCWSTR, arguments: ::windows_core::PCWSTR, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> ::windows_core::HRESULT,
}
#[inline]
pub unsafe fn IsProcessInIsolatedContainer() -> ::windows_core::Result<::win32_foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsProcessInIsolatedContainer(isprocessinisolatedcontainer: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        IsProcessInIsolatedContainer(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn IsProcessInIsolatedWindowsEnvironment() -> ::windows_core::Result<::win32_foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsProcessInIsolatedWindowsEnvironment(isprocessinisolatedwindowsenvironment: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        IsProcessInIsolatedWindowsEnvironment(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn IsProcessInWDAGContainer(reserved: *const ::core::ffi::c_void) -> ::windows_core::Result<::win32_foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsProcessInWDAGContainer(reserved: *const ::core::ffi::c_void, isprocessinwdagcontainer: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        IsProcessInWDAGContainer(::core::mem::transmute(reserved), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const IsolatedAppLauncher: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc812430_e75e_4fd1_9641_1f9f1e2d9a1f);
#[repr(C)]
pub struct IsolatedAppLauncherTelemetryParameters {
    pub EnableForLaunch: ::win32_foundation::BOOL,
    pub CorrelationGUID: ::windows_core::GUID,
}
impl ::core::marker::Copy for IsolatedAppLauncherTelemetryParameters {}
impl ::core::clone::Clone for IsolatedAppLauncherTelemetryParameters {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IsolatedAppLauncherTelemetryParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IsolatedAppLauncherTelemetryParameters").field("EnableForLaunch", &self.EnableForLaunch).field("CorrelationGUID", &self.CorrelationGUID).finish()
    }
}
unsafe impl ::windows_core::Abi for IsolatedAppLauncherTelemetryParameters {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IsolatedAppLauncherTelemetryParameters {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IsolatedAppLauncherTelemetryParameters>()) == 0 }
    }
}
impl ::core::cmp::Eq for IsolatedAppLauncherTelemetryParameters {}
impl ::core::default::Default for IsolatedAppLauncherTelemetryParameters {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
