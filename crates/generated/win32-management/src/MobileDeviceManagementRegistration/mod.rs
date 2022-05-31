#[inline]
pub unsafe fn ApplyLocalManagementSyncML<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(syncmlrequest: Param0) -> ::windows_core::Result<::windows_core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ApplyLocalManagementSyncML(syncmlrequest: ::windows_core::PCWSTR, syncmlresult: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        ApplyLocalManagementSyncML(syncmlrequest.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const DEVICEREGISTRATIONTYPE_MAM: u32 = 5u32;
pub const DEVICEREGISTRATIONTYPE_MDM_DEVICEWIDE_WITH_AAD: u32 = 6u32;
pub const DEVICEREGISTRATIONTYPE_MDM_ONLY: u32 = 0u32;
pub const DEVICEREGISTRATIONTYPE_MDM_USERSPECIFIC_WITH_AAD: u32 = 13u32;
pub const DEVICE_ENROLLER_FACILITY_CODE: u32 = 24u32;
#[inline]
pub unsafe fn DiscoverManagementService<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pszupn: Param0) -> ::windows_core::Result<*mut MANAGEMENT_SERVICE_INFO> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DiscoverManagementService(pszupn: ::windows_core::PCWSTR, ppmgmtinfo: *mut *mut MANAGEMENT_SERVICE_INFO) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<*mut MANAGEMENT_SERVICE_INFO>::zeroed();
        DiscoverManagementService(pszupn.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut MANAGEMENT_SERVICE_INFO>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DiscoverManagementServiceEx<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pszupn: Param0, pszdiscoveryservicecandidate: Param1) -> ::windows_core::Result<*mut MANAGEMENT_SERVICE_INFO> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DiscoverManagementServiceEx(pszupn: ::windows_core::PCWSTR, pszdiscoveryservicecandidate: ::windows_core::PCWSTR, ppmgmtinfo: *mut *mut MANAGEMENT_SERVICE_INFO) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<*mut MANAGEMENT_SERVICE_INFO>::zeroed();
        DiscoverManagementServiceEx(pszupn.into_param().abi(), pszdiscoveryservicecandidate.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut MANAGEMENT_SERVICE_INFO>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetDeviceManagementConfigInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(providerid: Param0, configstringbufferlength: *mut u32, configstring: ::windows_core::PWSTR) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDeviceManagementConfigInfo(providerid: ::windows_core::PCWSTR, configstringbufferlength: *mut u32, configstring: ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        GetDeviceManagementConfigInfo(providerid.into_param().abi(), ::core::mem::transmute(configstringbufferlength), ::core::mem::transmute(configstring)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetDeviceRegistrationInfo(deviceinformationclass: REGISTRATION_INFORMATION_CLASS, ppdeviceregistrationinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDeviceRegistrationInfo(deviceinformationclass: REGISTRATION_INFORMATION_CLASS, ppdeviceregistrationinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        GetDeviceRegistrationInfo(::core::mem::transmute(deviceinformationclass), ::core::mem::transmute(ppdeviceregistrationinfo)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetManagementAppHyperlink(pszhyperlink: &mut [u16]) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetManagementAppHyperlink(cchhyperlink: u32, pszhyperlink: ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        GetManagementAppHyperlink(pszhyperlink.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pszhyperlink))).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn IsDeviceRegisteredWithManagement(pfisdeviceregisteredwithmanagement: *mut ::win32_foundation::BOOL, pszupn: &mut [u16]) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsDeviceRegisteredWithManagement(pfisdeviceregisteredwithmanagement: *mut ::win32_foundation::BOOL, cchupn: u32, pszupn: ::windows_core::PWSTR) -> ::windows_core::HRESULT;
        }
        IsDeviceRegisteredWithManagement(::core::mem::transmute(pfisdeviceregisteredwithmanagement), pszupn.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pszupn))).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn IsManagementRegistrationAllowed() -> ::windows_core::Result<::win32_foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsManagementRegistrationAllowed(pfismanagementregistrationallowed: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        IsManagementRegistrationAllowed(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn IsMdmUxWithoutAadAllowed() -> ::windows_core::Result<::win32_foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsMdmUxWithoutAadAllowed(isenrollmentallowed: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        IsMdmUxWithoutAadAllowed(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct MANAGEMENT_REGISTRATION_INFO {
    pub fDeviceRegisteredWithManagement: ::win32_foundation::BOOL,
    pub dwDeviceRegistionKind: u32,
    pub pszUPN: ::windows_core::PWSTR,
    pub pszMDMServiceUri: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for MANAGEMENT_REGISTRATION_INFO {}
impl ::core::clone::Clone for MANAGEMENT_REGISTRATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MANAGEMENT_REGISTRATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MANAGEMENT_REGISTRATION_INFO").field("fDeviceRegisteredWithManagement", &self.fDeviceRegisteredWithManagement).field("dwDeviceRegistionKind", &self.dwDeviceRegistionKind).field("pszUPN", &self.pszUPN).field("pszMDMServiceUri", &self.pszMDMServiceUri).finish()
    }
}
unsafe impl ::windows_core::Abi for MANAGEMENT_REGISTRATION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MANAGEMENT_REGISTRATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MANAGEMENT_REGISTRATION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for MANAGEMENT_REGISTRATION_INFO {}
impl ::core::default::Default for MANAGEMENT_REGISTRATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MANAGEMENT_SERVICE_INFO {
    pub pszMDMServiceUri: ::windows_core::PWSTR,
    pub pszAuthenticationUri: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for MANAGEMENT_SERVICE_INFO {}
impl ::core::clone::Clone for MANAGEMENT_SERVICE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MANAGEMENT_SERVICE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MANAGEMENT_SERVICE_INFO").field("pszMDMServiceUri", &self.pszMDMServiceUri).field("pszAuthenticationUri", &self.pszAuthenticationUri).finish()
    }
}
unsafe impl ::windows_core::Abi for MANAGEMENT_SERVICE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MANAGEMENT_SERVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MANAGEMENT_SERVICE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for MANAGEMENT_SERVICE_INFO {}
impl ::core::default::Default for MANAGEMENT_SERVICE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const MDM_REGISTRATION_FACILITY_CODE: u32 = 25u32;
pub const MENROLL_E_CERTAUTH_FAILED_TO_FIND_CERT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910744i32);
pub const MENROLL_E_CERTPOLICY_PRIVATEKEYCREATION_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910745i32);
pub const MENROLL_E_CONNECTIVITY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910768i32);
pub const MENROLL_E_DEVICEAPREACHED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910765i32);
pub const MENROLL_E_DEVICECAPREACHED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910765i32);
pub const MENROLL_E_DEVICENOTSUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910764i32);
pub const MENROLL_E_DEVICE_ALREADY_ENROLLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910774i32);
pub const MENROLL_E_DEVICE_AUTHENTICATION_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910782i32);
pub const MENROLL_E_DEVICE_AUTHORIZATION_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910781i32);
pub const MENROLL_E_DEVICE_CERTIFCATEREQUEST_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910780i32);
pub const MENROLL_E_DEVICE_CERTIFICATEREQUEST_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910780i32);
pub const MENROLL_E_DEVICE_CONFIGMGRSERVER_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910779i32);
pub const MENROLL_E_DEVICE_INTERNALSERVICE_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910778i32);
pub const MENROLL_E_DEVICE_INVALIDSECURITY_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910777i32);
pub const MENROLL_E_DEVICE_MANAGEMENT_BLOCKED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910746i32);
pub const MENROLL_E_DEVICE_MESSAGE_FORMAT_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910783i32);
pub const MENROLL_E_DEVICE_NOT_ENROLLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910773i32);
pub const MENROLL_E_DEVICE_UNKNOWN_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910776i32);
pub const MENROLL_E_DISCOVERY_SEC_CERT_DATE_INVALID: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910771i32);
pub const MENROLL_E_EMPTY_MESSAGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910743i32);
pub const MENROLL_E_ENROLLMENTDATAINVALID: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910759i32);
pub const MENROLL_E_ENROLLMENT_IN_PROGRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910775i32);
pub const MENROLL_E_INMAINTENANCE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910761i32);
pub const MENROLL_E_INSECUREREDIRECT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910758i32);
pub const MENROLL_E_INVALIDSSLCERT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910766i32);
pub const MENROLL_E_MDM_NOT_CONFIGURED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910735i32);
pub const MENROLL_E_NOTELIGIBLETORENEW: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910762i32);
pub const MENROLL_E_NOTSUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910763i32);
pub const MENROLL_E_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910763i32);
pub const MENROLL_E_PASSWORD_NEEDED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910770i32);
pub const MENROLL_E_PLATFORM_LICENSE_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910756i32);
pub const MENROLL_E_PLATFORM_UNKNOWN_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910755i32);
pub const MENROLL_E_PLATFORM_WRONG_STATE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910757i32);
pub const MENROLL_E_PROV_CSP_APPMGMT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910747i32);
pub const MENROLL_E_PROV_CSP_CERTSTORE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910754i32);
pub const MENROLL_E_PROV_CSP_DMCLIENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910752i32);
pub const MENROLL_E_PROV_CSP_MISC: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910750i32);
pub const MENROLL_E_PROV_CSP_PFW: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910751i32);
pub const MENROLL_E_PROV_CSP_W7: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910753i32);
pub const MENROLL_E_PROV_SSLCERTNOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910748i32);
pub const MENROLL_E_PROV_UNKNOWN: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910749i32);
pub const MENROLL_E_USERLICENSE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910760i32);
pub const MENROLL_E_USER_CANCELED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910742i32);
pub const MENROLL_E_USER_CANCELLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910736i32);
pub const MENROLL_E_USER_LICENSE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910760i32);
pub const MENROLL_E_WAB_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145910769i32);
pub const MREGISTER_E_DEVICE_ALREADY_REGISTERED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145845238i32);
pub const MREGISTER_E_DEVICE_AUTHENTICATION_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145845246i32);
pub const MREGISTER_E_DEVICE_AUTHORIZATION_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145845245i32);
pub const MREGISTER_E_DEVICE_CERTIFCATEREQUEST_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145845244i32);
pub const MREGISTER_E_DEVICE_CONFIGMGRSERVER_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145845243i32);
pub const MREGISTER_E_DEVICE_INTERNALSERVICE_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145845242i32);
pub const MREGISTER_E_DEVICE_INVALIDSECURITY_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145845241i32);
pub const MREGISTER_E_DEVICE_MESSAGE_FORMAT_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145845247i32);
pub const MREGISTER_E_DEVICE_NOT_AD_REGISTERED_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145845235i32);
pub const MREGISTER_E_DEVICE_NOT_REGISTERED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145845237i32);
pub const MREGISTER_E_DEVICE_UNKNOWN_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145845240i32);
pub const MREGISTER_E_DISCOVERY_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145845234i32);
pub const MREGISTER_E_DISCOVERY_REDIRECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145845236i32);
pub const MREGISTER_E_REGISTRATION_IN_PROGRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145845239i32);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REGISTRATION_INFORMATION_CLASS(pub i32);
pub const DeviceRegistrationBasicInfo: REGISTRATION_INFORMATION_CLASS = REGISTRATION_INFORMATION_CLASS(1i32);
pub const MaxDeviceInfoClass: REGISTRATION_INFORMATION_CLASS = REGISTRATION_INFORMATION_CLASS(2i32);
impl ::core::marker::Copy for REGISTRATION_INFORMATION_CLASS {}
impl ::core::clone::Clone for REGISTRATION_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REGISTRATION_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for REGISTRATION_INFORMATION_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for REGISTRATION_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REGISTRATION_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn RegisterDeviceWithLocalManagement() -> ::windows_core::Result<::win32_foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterDeviceWithLocalManagement(alreadyregistered: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        RegisterDeviceWithLocalManagement(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RegisterDeviceWithManagement<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pszupn: Param0, ppszmdmserviceuri: Param1, ppzsaccesstoken: Param2) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterDeviceWithManagement(pszupn: ::windows_core::PCWSTR, ppszmdmserviceuri: ::windows_core::PCWSTR, ppzsaccesstoken: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        RegisterDeviceWithManagement(pszupn.into_param().abi(), ppszmdmserviceuri.into_param().abi(), ppzsaccesstoken.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RegisterDeviceWithManagementUsingAADCredentials<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(usertoken: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterDeviceWithManagementUsingAADCredentials(usertoken: ::win32_foundation::HANDLE) -> ::windows_core::HRESULT;
        }
        RegisterDeviceWithManagementUsingAADCredentials(usertoken.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RegisterDeviceWithManagementUsingAADDeviceCredentials() -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterDeviceWithManagementUsingAADDeviceCredentials() -> ::windows_core::HRESULT;
        }
        RegisterDeviceWithManagementUsingAADDeviceCredentials().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RegisterDeviceWithManagementUsingAADDeviceCredentials2<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(mdmapplicationid: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterDeviceWithManagementUsingAADDeviceCredentials2(mdmapplicationid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        RegisterDeviceWithManagementUsingAADDeviceCredentials2(mdmapplicationid.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetDeviceManagementConfigInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(providerid: Param0, configstring: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDeviceManagementConfigInfo(providerid: ::windows_core::PCWSTR, configstring: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        SetDeviceManagementConfigInfo(providerid.into_param().abi(), configstring.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetManagedExternally<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(ismanagedexternally: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetManagedExternally(ismanagedexternally: ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        SetManagedExternally(ismanagedexternally.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UnregisterDeviceWithLocalManagement() -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterDeviceWithLocalManagement() -> ::windows_core::HRESULT;
        }
        UnregisterDeviceWithLocalManagement().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UnregisterDeviceWithManagement<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(enrollmentid: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterDeviceWithManagement(enrollmentid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        UnregisterDeviceWithManagement(enrollmentid.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
