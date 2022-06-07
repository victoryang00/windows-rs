#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
    pub fn WscGetAntiMalwareUri(ppszuri: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
    pub fn WscGetSecurityProviderHealth(providers: u32, phealth: *mut WSC_SECURITY_PROVIDER_HEALTH) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
    pub fn WscQueryAntiMalwareUri() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`, `\"Win32_System_Threading\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub fn WscRegisterForChanges(reserved: *mut ::core::ffi::c_void, phcallbackregistration: *mut super::super::Foundation::HANDLE, lpcallbackaddress: super::Threading::LPTHREAD_START_ROUTINE, pcontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
    pub fn WscRegisterForUserNotifications() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WscUnRegisterChanges(hregistrationhandle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWSCDefaultProduct {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDefaultProduct: unsafe extern "system" fn(this: *mut *mut Self, etype: SECURITY_PRODUCT_TYPE, pguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDefaultProduct: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IWSCDefaultProduct {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 74897052, data2: 61978, data3: 4581, data4: [156, 233, 94, 85, 23, 80, 124, 102] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWSCProductList {
    pub base__: super::Com::IDispatch,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, provider: WSC_SECURITY_PROVIDER) -> ::windows_sys::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: u32, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IWSCProductList {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1915368332, data2: 28302, data3: 20082, data4: [172, 39, 20, 23, 251, 12, 129, 194] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWscProduct {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub ProductName: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProductName: usize,
    pub ProductState: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut WSC_SECURITY_PRODUCT_STATE) -> ::windows_sys::core::HRESULT,
    pub SignatureStatus: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut WSC_SECURITY_SIGNATURE_STATUS) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RemediationPath: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemediationPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ProductStateTimestamp: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProductStateTimestamp: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ProductGuid: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProductGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ProductIsDefault: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProductIsDefault: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IWscProduct {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2352489262, data2: 14917, data3: 18983, data4: [146, 176, 26, 22, 169, 117, 246, 105] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWscProduct2 {
    pub base__: IWscProduct,
    pub AntivirusScanSubstatus: unsafe extern "system" fn(this: *mut *mut Self, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows_sys::core::HRESULT,
    pub AntivirusSettingsSubstatus: unsafe extern "system" fn(this: *mut *mut Self, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows_sys::core::HRESULT,
    pub AntivirusProtectionUpdateSubstatus: unsafe extern "system" fn(this: *mut *mut Self, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows_sys::core::HRESULT,
    pub FirewallDomainProfileSubstatus: unsafe extern "system" fn(this: *mut *mut Self, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows_sys::core::HRESULT,
    pub FirewallPrivateProfileSubstatus: unsafe extern "system" fn(this: *mut *mut Self, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows_sys::core::HRESULT,
    pub FirewallPublicProfileSubstatus: unsafe extern "system" fn(this: *mut *mut Self, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IWscProduct2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4170631764, data2: 65033, data3: 17411, data4: [134, 212, 35, 203, 72, 141, 129, 216] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWscProduct3 {
    pub base__: IWscProduct2,
    pub AntivirusDaysUntilExpired: unsafe extern "system" fn(this: *mut *mut Self, pdwdays: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IWscProduct3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1431528740, data2: 53713, data3: 18214, data4: [140, 124, 4, 153, 106, 25, 4, 231] };
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub type SECURITY_PRODUCT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const SECURITY_PRODUCT_TYPE_ANTIVIRUS: SECURITY_PRODUCT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const SECURITY_PRODUCT_TYPE_FIREWALL: SECURITY_PRODUCT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const SECURITY_PRODUCT_TYPE_ANTISPYWARE: SECURITY_PRODUCT_TYPE = 2i32;
pub const WSCDefaultProduct: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 696361838, data2: 61997, data3: 4581, data4: [156, 233, 94, 85, 23, 80, 124, 102] };
pub const WSCProductList: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 386346875, data2: 39614, data3: 19060, data4: [162, 97, 30, 183, 107, 85, 16, 122] };
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub type WSC_SECURITY_PRODUCT_STATE = i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_STATE_ON: WSC_SECURITY_PRODUCT_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_STATE_OFF: WSC_SECURITY_PRODUCT_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_STATE_SNOOZED: WSC_SECURITY_PRODUCT_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_STATE_EXPIRED: WSC_SECURITY_PRODUCT_STATE = 3i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub type WSC_SECURITY_PRODUCT_SUBSTATUS = i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_NOT_SET: WSC_SECURITY_PRODUCT_SUBSTATUS = 0i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_NO_ACTION: WSC_SECURITY_PRODUCT_SUBSTATUS = 1i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_ACTION_RECOMMENDED: WSC_SECURITY_PRODUCT_SUBSTATUS = 2i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_ACTION_NEEDED: WSC_SECURITY_PRODUCT_SUBSTATUS = 3i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub type WSC_SECURITY_PROVIDER = i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_FIREWALL: WSC_SECURITY_PROVIDER = 1i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_AUTOUPDATE_SETTINGS: WSC_SECURITY_PROVIDER = 2i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_ANTIVIRUS: WSC_SECURITY_PROVIDER = 4i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_ANTISPYWARE: WSC_SECURITY_PROVIDER = 8i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_INTERNET_SETTINGS: WSC_SECURITY_PROVIDER = 16i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_USER_ACCOUNT_CONTROL: WSC_SECURITY_PROVIDER = 32i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_SERVICE: WSC_SECURITY_PROVIDER = 64i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_NONE: WSC_SECURITY_PROVIDER = 0i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_ALL: WSC_SECURITY_PROVIDER = 127i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub type WSC_SECURITY_PROVIDER_HEALTH = i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_HEALTH_GOOD: WSC_SECURITY_PROVIDER_HEALTH = 0i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_HEALTH_NOTMONITORED: WSC_SECURITY_PROVIDER_HEALTH = 1i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_HEALTH_POOR: WSC_SECURITY_PROVIDER_HEALTH = 2i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_HEALTH_SNOOZE: WSC_SECURITY_PROVIDER_HEALTH = 3i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub type WSC_SECURITY_SIGNATURE_STATUS = i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_OUT_OF_DATE: WSC_SECURITY_SIGNATURE_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_UP_TO_DATE: WSC_SECURITY_SIGNATURE_STATUS = 1i32;
