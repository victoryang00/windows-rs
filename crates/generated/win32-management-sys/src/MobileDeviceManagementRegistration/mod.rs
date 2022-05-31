#[link(name = "windows")]
extern "system" {
    pub fn ApplyLocalManagementSyncML(syncmlrequest: ::windows_core_sys::PCWSTR, syncmlresult: *mut ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn DiscoverManagementService(pszupn: ::windows_core_sys::PCWSTR, ppmgmtinfo: *mut *mut MANAGEMENT_SERVICE_INFO) -> ::windows_core_sys::HRESULT;
    pub fn DiscoverManagementServiceEx(pszupn: ::windows_core_sys::PCWSTR, pszdiscoveryservicecandidate: ::windows_core_sys::PCWSTR, ppmgmtinfo: *mut *mut MANAGEMENT_SERVICE_INFO) -> ::windows_core_sys::HRESULT;
    pub fn GetDeviceManagementConfigInfo(providerid: ::windows_core_sys::PCWSTR, configstringbufferlength: *mut u32, configstring: ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn GetDeviceRegistrationInfo(deviceinformationclass: REGISTRATION_INFORMATION_CLASS, ppdeviceregistrationinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core_sys::HRESULT;
    pub fn GetManagementAppHyperlink(cchhyperlink: u32, pszhyperlink: ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn IsDeviceRegisteredWithManagement(pfisdeviceregisteredwithmanagement: *mut ::win32_foundation_sys::BOOL, cchupn: u32, pszupn: ::windows_core_sys::PWSTR) -> ::windows_core_sys::HRESULT;
    pub fn IsManagementRegistrationAllowed(pfismanagementregistrationallowed: *mut ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    pub fn IsMdmUxWithoutAadAllowed(isenrollmentallowed: *mut ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    pub fn RegisterDeviceWithLocalManagement(alreadyregistered: *mut ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    pub fn RegisterDeviceWithManagement(pszupn: ::windows_core_sys::PCWSTR, ppszmdmserviceuri: ::windows_core_sys::PCWSTR, ppzsaccesstoken: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn RegisterDeviceWithManagementUsingAADCredentials(usertoken: ::win32_foundation_sys::HANDLE) -> ::windows_core_sys::HRESULT;
    pub fn RegisterDeviceWithManagementUsingAADDeviceCredentials() -> ::windows_core_sys::HRESULT;
    pub fn RegisterDeviceWithManagementUsingAADDeviceCredentials2(mdmapplicationid: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn SetDeviceManagementConfigInfo(providerid: ::windows_core_sys::PCWSTR, configstring: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn SetManagedExternally(ismanagedexternally: ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    pub fn UnregisterDeviceWithLocalManagement() -> ::windows_core_sys::HRESULT;
    pub fn UnregisterDeviceWithManagement(enrollmentid: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
}
pub const DEVICEREGISTRATIONTYPE_MAM: u32 = 5u32;
pub const DEVICEREGISTRATIONTYPE_MDM_DEVICEWIDE_WITH_AAD: u32 = 6u32;
pub const DEVICEREGISTRATIONTYPE_MDM_ONLY: u32 = 0u32;
pub const DEVICEREGISTRATIONTYPE_MDM_USERSPECIFIC_WITH_AAD: u32 = 13u32;
pub const DEVICE_ENROLLER_FACILITY_CODE: u32 = 24u32;
#[repr(C)]
pub struct MANAGEMENT_REGISTRATION_INFO {
    pub fDeviceRegisteredWithManagement: ::win32_foundation_sys::BOOL,
    pub dwDeviceRegistionKind: u32,
    pub pszUPN: ::windows_core_sys::PWSTR,
    pub pszMDMServiceUri: ::windows_core_sys::PWSTR,
}
impl ::core::marker::Copy for MANAGEMENT_REGISTRATION_INFO {}
impl ::core::clone::Clone for MANAGEMENT_REGISTRATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MANAGEMENT_SERVICE_INFO {
    pub pszMDMServiceUri: ::windows_core_sys::PWSTR,
    pub pszAuthenticationUri: ::windows_core_sys::PWSTR,
}
impl ::core::marker::Copy for MANAGEMENT_SERVICE_INFO {}
impl ::core::clone::Clone for MANAGEMENT_SERVICE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MDM_REGISTRATION_FACILITY_CODE: u32 = 25u32;
pub const MENROLL_E_CERTAUTH_FAILED_TO_FIND_CERT: ::windows_core_sys::HRESULT = -2145910744i32;
pub const MENROLL_E_CERTPOLICY_PRIVATEKEYCREATION_FAILED: ::windows_core_sys::HRESULT = -2145910745i32;
pub const MENROLL_E_CONNECTIVITY: ::windows_core_sys::HRESULT = -2145910768i32;
pub const MENROLL_E_DEVICEAPREACHED: ::windows_core_sys::HRESULT = -2145910765i32;
pub const MENROLL_E_DEVICECAPREACHED: ::windows_core_sys::HRESULT = -2145910765i32;
pub const MENROLL_E_DEVICENOTSUPPORTED: ::windows_core_sys::HRESULT = -2145910764i32;
pub const MENROLL_E_DEVICE_ALREADY_ENROLLED: ::windows_core_sys::HRESULT = -2145910774i32;
pub const MENROLL_E_DEVICE_AUTHENTICATION_ERROR: ::windows_core_sys::HRESULT = -2145910782i32;
pub const MENROLL_E_DEVICE_AUTHORIZATION_ERROR: ::windows_core_sys::HRESULT = -2145910781i32;
pub const MENROLL_E_DEVICE_CERTIFCATEREQUEST_ERROR: ::windows_core_sys::HRESULT = -2145910780i32;
pub const MENROLL_E_DEVICE_CERTIFICATEREQUEST_ERROR: ::windows_core_sys::HRESULT = -2145910780i32;
pub const MENROLL_E_DEVICE_CONFIGMGRSERVER_ERROR: ::windows_core_sys::HRESULT = -2145910779i32;
pub const MENROLL_E_DEVICE_INTERNALSERVICE_ERROR: ::windows_core_sys::HRESULT = -2145910778i32;
pub const MENROLL_E_DEVICE_INVALIDSECURITY_ERROR: ::windows_core_sys::HRESULT = -2145910777i32;
pub const MENROLL_E_DEVICE_MANAGEMENT_BLOCKED: ::windows_core_sys::HRESULT = -2145910746i32;
pub const MENROLL_E_DEVICE_MESSAGE_FORMAT_ERROR: ::windows_core_sys::HRESULT = -2145910783i32;
pub const MENROLL_E_DEVICE_NOT_ENROLLED: ::windows_core_sys::HRESULT = -2145910773i32;
pub const MENROLL_E_DEVICE_UNKNOWN_ERROR: ::windows_core_sys::HRESULT = -2145910776i32;
pub const MENROLL_E_DISCOVERY_SEC_CERT_DATE_INVALID: ::windows_core_sys::HRESULT = -2145910771i32;
pub const MENROLL_E_EMPTY_MESSAGE: ::windows_core_sys::HRESULT = -2145910743i32;
pub const MENROLL_E_ENROLLMENTDATAINVALID: ::windows_core_sys::HRESULT = -2145910759i32;
pub const MENROLL_E_ENROLLMENT_IN_PROGRESS: ::windows_core_sys::HRESULT = -2145910775i32;
pub const MENROLL_E_INMAINTENANCE: ::windows_core_sys::HRESULT = -2145910761i32;
pub const MENROLL_E_INSECUREREDIRECT: ::windows_core_sys::HRESULT = -2145910758i32;
pub const MENROLL_E_INVALIDSSLCERT: ::windows_core_sys::HRESULT = -2145910766i32;
pub const MENROLL_E_MDM_NOT_CONFIGURED: ::windows_core_sys::HRESULT = -2145910735i32;
pub const MENROLL_E_NOTELIGIBLETORENEW: ::windows_core_sys::HRESULT = -2145910762i32;
pub const MENROLL_E_NOTSUPPORTED: ::windows_core_sys::HRESULT = -2145910763i32;
pub const MENROLL_E_NOT_SUPPORTED: ::windows_core_sys::HRESULT = -2145910763i32;
pub const MENROLL_E_PASSWORD_NEEDED: ::windows_core_sys::HRESULT = -2145910770i32;
pub const MENROLL_E_PLATFORM_LICENSE_ERROR: ::windows_core_sys::HRESULT = -2145910756i32;
pub const MENROLL_E_PLATFORM_UNKNOWN_ERROR: ::windows_core_sys::HRESULT = -2145910755i32;
pub const MENROLL_E_PLATFORM_WRONG_STATE: ::windows_core_sys::HRESULT = -2145910757i32;
pub const MENROLL_E_PROV_CSP_APPMGMT: ::windows_core_sys::HRESULT = -2145910747i32;
pub const MENROLL_E_PROV_CSP_CERTSTORE: ::windows_core_sys::HRESULT = -2145910754i32;
pub const MENROLL_E_PROV_CSP_DMCLIENT: ::windows_core_sys::HRESULT = -2145910752i32;
pub const MENROLL_E_PROV_CSP_MISC: ::windows_core_sys::HRESULT = -2145910750i32;
pub const MENROLL_E_PROV_CSP_PFW: ::windows_core_sys::HRESULT = -2145910751i32;
pub const MENROLL_E_PROV_CSP_W7: ::windows_core_sys::HRESULT = -2145910753i32;
pub const MENROLL_E_PROV_SSLCERTNOTFOUND: ::windows_core_sys::HRESULT = -2145910748i32;
pub const MENROLL_E_PROV_UNKNOWN: ::windows_core_sys::HRESULT = -2145910749i32;
pub const MENROLL_E_USERLICENSE: ::windows_core_sys::HRESULT = -2145910760i32;
pub const MENROLL_E_USER_CANCELED: ::windows_core_sys::HRESULT = -2145910742i32;
pub const MENROLL_E_USER_CANCELLED: ::windows_core_sys::HRESULT = -2145910736i32;
pub const MENROLL_E_USER_LICENSE: ::windows_core_sys::HRESULT = -2145910760i32;
pub const MENROLL_E_WAB_ERROR: ::windows_core_sys::HRESULT = -2145910769i32;
pub const MREGISTER_E_DEVICE_ALREADY_REGISTERED: ::windows_core_sys::HRESULT = -2145845238i32;
pub const MREGISTER_E_DEVICE_AUTHENTICATION_ERROR: ::windows_core_sys::HRESULT = -2145845246i32;
pub const MREGISTER_E_DEVICE_AUTHORIZATION_ERROR: ::windows_core_sys::HRESULT = -2145845245i32;
pub const MREGISTER_E_DEVICE_CERTIFCATEREQUEST_ERROR: ::windows_core_sys::HRESULT = -2145845244i32;
pub const MREGISTER_E_DEVICE_CONFIGMGRSERVER_ERROR: ::windows_core_sys::HRESULT = -2145845243i32;
pub const MREGISTER_E_DEVICE_INTERNALSERVICE_ERROR: ::windows_core_sys::HRESULT = -2145845242i32;
pub const MREGISTER_E_DEVICE_INVALIDSECURITY_ERROR: ::windows_core_sys::HRESULT = -2145845241i32;
pub const MREGISTER_E_DEVICE_MESSAGE_FORMAT_ERROR: ::windows_core_sys::HRESULT = -2145845247i32;
pub const MREGISTER_E_DEVICE_NOT_AD_REGISTERED_ERROR: ::windows_core_sys::HRESULT = -2145845235i32;
pub const MREGISTER_E_DEVICE_NOT_REGISTERED: ::windows_core_sys::HRESULT = -2145845237i32;
pub const MREGISTER_E_DEVICE_UNKNOWN_ERROR: ::windows_core_sys::HRESULT = -2145845240i32;
pub const MREGISTER_E_DISCOVERY_FAILED: ::windows_core_sys::HRESULT = -2145845234i32;
pub const MREGISTER_E_DISCOVERY_REDIRECTED: ::windows_core_sys::HRESULT = -2145845236i32;
pub const MREGISTER_E_REGISTRATION_IN_PROGRESS: ::windows_core_sys::HRESULT = -2145845239i32;
pub type REGISTRATION_INFORMATION_CLASS = i32;
pub const DeviceRegistrationBasicInfo: REGISTRATION_INFORMATION_CLASS = 1i32;
pub const MaxDeviceInfoClass: REGISTRATION_INFORMATION_CLASS = 2i32;
