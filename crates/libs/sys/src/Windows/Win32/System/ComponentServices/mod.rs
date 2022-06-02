#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
    pub fn CoCreateActivity(piunknown: *mut *mut ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, ppobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
    pub fn CoEnterServiceDomain(pconfigobject: *mut *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn CoGetDefaultContext(apttype: super::Com::APTTYPE, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
    pub fn CoLeaveServiceDomain(punkstatus: *mut *mut ::windows_sys::core::IUnknown);
    #[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
    pub fn GetDispenserManager(param0: *mut *mut *mut IDispenserManager) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
    pub fn GetManagedExtensions(dwexts: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
    pub fn MTSCreateActivity(riid: *const ::windows_sys::core::GUID, ppobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
    pub fn RecycleSurrogate(lreasoncode: i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
    pub fn SafeRef(rid: *const ::windows_sys::core::GUID, punk: *mut *mut ::windows_sys::core::IUnknown) -> *mut ::core::ffi::c_void;
}
pub const AppDomainHelper: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4012177033, data2: 5368, data3: 19858, data4: [180, 175, 215, 177, 240, 231, 15, 212] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ApplicationProcessRecycleInfo {
    pub IsRecyclable: super::super::Foundation::BOOL,
    pub IsRecycled: super::super::Foundation::BOOL,
    pub TimeRecycled: super::super::Foundation::FILETIME,
    pub TimeToTerminate: super::super::Foundation::FILETIME,
    pub RecycleReasonCode: i32,
    pub IsPendingRecycle: super::super::Foundation::BOOL,
    pub HasAutomaticLifetimeRecycling: super::super::Foundation::BOOL,
    pub TimeForAutomaticRecycling: super::super::Foundation::FILETIME,
    pub MemoryLimitInKB: u32,
    pub MemoryUsageInKBLastCheck: u32,
    pub ActivationLimit: u32,
    pub NumActivationsLastReported: u32,
    pub CallLimit: u32,
    pub NumCallsLastReported: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ApplicationProcessRecycleInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ApplicationProcessRecycleInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct ApplicationProcessStatistics {
    pub NumCallsOutstanding: u32,
    pub NumTrackedComponents: u32,
    pub NumComponentInstances: u32,
    pub AvgCallsPerSecond: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: u32,
    pub Reserved4: u32,
}
impl ::core::marker::Copy for ApplicationProcessStatistics {}
impl ::core::clone::Clone for ApplicationProcessStatistics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ApplicationProcessSummary {
    pub PartitionIdPrimaryApplication: ::windows_sys::core::GUID,
    pub ApplicationIdPrimaryApplication: ::windows_sys::core::GUID,
    pub ApplicationInstanceId: ::windows_sys::core::GUID,
    pub ProcessId: u32,
    pub Type: COMPLUS_APPTYPE,
    pub ProcessExeName: ::windows_sys::core::PWSTR,
    pub IsService: super::super::Foundation::BOOL,
    pub IsPaused: super::super::Foundation::BOOL,
    pub IsRecycled: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ApplicationProcessSummary {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ApplicationProcessSummary {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct ApplicationSummary {
    pub ApplicationInstanceId: ::windows_sys::core::GUID,
    pub PartitionId: ::windows_sys::core::GUID,
    pub ApplicationId: ::windows_sys::core::GUID,
    pub Type: COMPLUS_APPTYPE,
    pub ApplicationName: ::windows_sys::core::PWSTR,
    pub NumTrackedComponents: u32,
    pub NumComponentInstances: u32,
}
impl ::core::marker::Copy for ApplicationSummary {}
impl ::core::clone::Clone for ApplicationSummary {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type AutoSvcs_Error_Constants = u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxAborted: AutoSvcs_Error_Constants = 2147803138u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxAborting: AutoSvcs_Error_Constants = 2147803139u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxNoContext: AutoSvcs_Error_Constants = 2147803140u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxNotRegistered: AutoSvcs_Error_Constants = 2147803141u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxSynchTimeout: AutoSvcs_Error_Constants = 2147803142u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxOldReference: AutoSvcs_Error_Constants = 2147803143u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxRoleNotFound: AutoSvcs_Error_Constants = 2147803148u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxNoSecurity: AutoSvcs_Error_Constants = 2147803149u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxWrongThread: AutoSvcs_Error_Constants = 2147803150u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxTMNotAvailable: AutoSvcs_Error_Constants = 2147803151u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comQCErrApplicationNotQueued: AutoSvcs_Error_Constants = 2148599296u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comQCErrNoQueueableInterfaces: AutoSvcs_Error_Constants = 2148599297u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comQCErrQueuingServiceNotAvailable: AutoSvcs_Error_Constants = 2148599298u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comQCErrQueueTransactMismatch: AutoSvcs_Error_Constants = 2148599299u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrRecorderMarshalled: AutoSvcs_Error_Constants = 2148599300u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrOutParam: AutoSvcs_Error_Constants = 2148599301u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrRecorderNotTrusted: AutoSvcs_Error_Constants = 2148599302u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrPSLoad: AutoSvcs_Error_Constants = 2148599303u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrMarshaledObjSameTxn: AutoSvcs_Error_Constants = 2148599304u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrInvalidMessage: AutoSvcs_Error_Constants = 2148599376u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrMsmqSidUnavailable: AutoSvcs_Error_Constants = 2148599377u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrWrongMsgExtension: AutoSvcs_Error_Constants = 2148599378u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrMsmqServiceUnavailable: AutoSvcs_Error_Constants = 2148599379u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrMsgNotAuthenticated: AutoSvcs_Error_Constants = 2148599380u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrMsmqConnectorUsed: AutoSvcs_Error_Constants = 2148599381u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrBadMarshaledObject: AutoSvcs_Error_Constants = 2148599382u32;
pub const ByotServerEx: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674858, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct CAppData {
    pub m_idApp: u32,
    pub m_szAppGuid: [u16; 40],
    pub m_dwAppProcessId: u32,
    pub m_AppStatistics: CAppStatistics,
}
impl ::core::marker::Copy for CAppData {}
impl ::core::clone::Clone for CAppData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct CAppStatistics {
    pub m_cTotalCalls: u32,
    pub m_cTotalInstances: u32,
    pub m_cTotalClasses: u32,
    pub m_cCallsPerSecond: u32,
}
impl ::core::marker::Copy for CAppStatistics {}
impl ::core::clone::Clone for CAppStatistics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct CCLSIDData {
    pub m_clsid: ::windows_sys::core::GUID,
    pub m_cReferences: u32,
    pub m_cBound: u32,
    pub m_cPooled: u32,
    pub m_cInCall: u32,
    pub m_dwRespTime: u32,
    pub m_cCallsCompleted: u32,
    pub m_cCallsFailed: u32,
}
impl ::core::marker::Copy for CCLSIDData {}
impl ::core::clone::Clone for CCLSIDData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct CCLSIDData2 {
    pub m_clsid: ::windows_sys::core::GUID,
    pub m_appid: ::windows_sys::core::GUID,
    pub m_partid: ::windows_sys::core::GUID,
    pub m_pwszAppName: ::windows_sys::core::PWSTR,
    pub m_pwszCtxName: ::windows_sys::core::PWSTR,
    pub m_eAppType: COMPLUS_APPTYPE,
    pub m_cReferences: u32,
    pub m_cBound: u32,
    pub m_cPooled: u32,
    pub m_cInCall: u32,
    pub m_dwRespTime: u32,
    pub m_cCallsCompleted: u32,
    pub m_cCallsFailed: u32,
}
impl ::core::marker::Copy for CCLSIDData2 {}
impl ::core::clone::Clone for CCLSIDData2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminAccessChecksLevelOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAccessChecksApplicationLevel: COMAdminAccessChecksLevelOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAccessChecksApplicationComponentLevel: COMAdminAccessChecksLevelOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminActivationOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminActivationInproc: COMAdminActivationOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminActivationLocal: COMAdminActivationOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminApplicationExportOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminExportNoUsers: COMAdminApplicationExportOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminExportUsers: COMAdminApplicationExportOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminExportApplicationProxy: COMAdminApplicationExportOptions = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminExportForceOverwriteOfFiles: COMAdminApplicationExportOptions = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminExportIn10Format: COMAdminApplicationExportOptions = 16i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminApplicationInstallOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminInstallNoUsers: COMAdminApplicationInstallOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminInstallUsers: COMAdminApplicationInstallOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminInstallForceOverwriteOfFiles: COMAdminApplicationInstallOptions = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminAuthenticationCapabilitiesOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationCapabilitiesNone: COMAdminAuthenticationCapabilitiesOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationCapabilitiesSecureReference: COMAdminAuthenticationCapabilitiesOptions = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationCapabilitiesStaticCloaking: COMAdminAuthenticationCapabilitiesOptions = 32i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationCapabilitiesDynamicCloaking: COMAdminAuthenticationCapabilitiesOptions = 64i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminAuthenticationLevelOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationDefault: COMAdminAuthenticationLevelOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationNone: COMAdminAuthenticationLevelOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationConnect: COMAdminAuthenticationLevelOptions = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationCall: COMAdminAuthenticationLevelOptions = 3i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationPacket: COMAdminAuthenticationLevelOptions = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationIntegrity: COMAdminAuthenticationLevelOptions = 5i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationPrivacy: COMAdminAuthenticationLevelOptions = 6i32;
pub const COMAdminCatalog: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4128818452, data2: 57272, data3: 4561, data4: [162, 207, 0, 128, 95, 199, 146, 53] };
pub const COMAdminCatalogCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4128818454, data2: 57272, data3: 4561, data4: [162, 207, 0, 128, 95, 199, 146, 53] };
pub const COMAdminCatalogObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4128818453, data2: 57272, data3: 4561, data4: [162, 207, 0, 128, 95, 199, 146, 53] };
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminComponentFlags = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminCompFlagTypeInfoFound: COMAdminComponentFlags = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminCompFlagCOMPlusPropertiesFound: COMAdminComponentFlags = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminCompFlagProxyFound: COMAdminComponentFlags = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminCompFlagInterfacesFound: COMAdminComponentFlags = 8i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminCompFlagAlreadyInstalled: COMAdminComponentFlags = 16i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminCompFlagNotInApplication: COMAdminComponentFlags = 32i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminComponentType = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdmin32BitComponent: COMAdminComponentType = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdmin64BitComponent: COMAdminComponentType = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminErrorCodes = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrObjectErrors: COMAdminErrorCodes = -2146368511i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrObjectInvalid: COMAdminErrorCodes = -2146368510i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrKeyMissing: COMAdminErrorCodes = -2146368509i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrAlreadyInstalled: COMAdminErrorCodes = -2146368508i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrAppFileWriteFail: COMAdminErrorCodes = -2146368505i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrAppFileReadFail: COMAdminErrorCodes = -2146368504i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrAppFileVersion: COMAdminErrorCodes = -2146368503i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrBadPath: COMAdminErrorCodes = -2146368502i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrApplicationExists: COMAdminErrorCodes = -2146368501i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRoleExists: COMAdminErrorCodes = -2146368500i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCantCopyFile: COMAdminErrorCodes = -2146368499i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrNoUser: COMAdminErrorCodes = -2146368497i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrInvalidUserids: COMAdminErrorCodes = -2146368496i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrNoRegistryCLSID: COMAdminErrorCodes = -2146368495i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrBadRegistryProgID: COMAdminErrorCodes = -2146368494i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrAuthenticationLevel: COMAdminErrorCodes = -2146368493i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrUserPasswdNotValid: COMAdminErrorCodes = -2146368492i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCLSIDOrIIDMismatch: COMAdminErrorCodes = -2146368488i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRemoteInterface: COMAdminErrorCodes = -2146368487i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrDllRegisterServer: COMAdminErrorCodes = -2146368486i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrNoServerShare: COMAdminErrorCodes = -2146368485i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrDllLoadFailed: COMAdminErrorCodes = -2146368483i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrBadRegistryLibID: COMAdminErrorCodes = -2146368482i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrAppDirNotFound: COMAdminErrorCodes = -2146368481i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRegistrarFailed: COMAdminErrorCodes = -2146368477i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompFileDoesNotExist: COMAdminErrorCodes = -2146368476i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompFileLoadDLLFail: COMAdminErrorCodes = -2146368475i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompFileGetClassObj: COMAdminErrorCodes = -2146368474i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompFileClassNotAvail: COMAdminErrorCodes = -2146368473i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompFileBadTLB: COMAdminErrorCodes = -2146368472i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompFileNotInstallable: COMAdminErrorCodes = -2146368471i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrNotChangeable: COMAdminErrorCodes = -2146368470i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrNotDeletable: COMAdminErrorCodes = -2146368469i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrSession: COMAdminErrorCodes = -2146368468i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompMoveLocked: COMAdminErrorCodes = -2146368467i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompMoveBadDest: COMAdminErrorCodes = -2146368466i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRegisterTLB: COMAdminErrorCodes = -2146368464i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrSystemApp: COMAdminErrorCodes = -2146368461i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompFileNoRegistrar: COMAdminErrorCodes = -2146368460i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCoReqCompInstalled: COMAdminErrorCodes = -2146368459i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrServiceNotInstalled: COMAdminErrorCodes = -2146368458i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrPropertySaveFailed: COMAdminErrorCodes = -2146368457i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrObjectExists: COMAdminErrorCodes = -2146368456i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrComponentExists: COMAdminErrorCodes = -2146368455i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRegFileCorrupt: COMAdminErrorCodes = -2146368453i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrPropertyOverflow: COMAdminErrorCodes = -2146368452i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrNotInRegistry: COMAdminErrorCodes = -2146368450i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrObjectNotPoolable: COMAdminErrorCodes = -2146368449i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrApplidMatchesClsid: COMAdminErrorCodes = -2146368442i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRoleDoesNotExist: COMAdminErrorCodes = -2146368441i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrStartAppNeedsComponents: COMAdminErrorCodes = -2146368440i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRequiresDifferentPlatform: COMAdminErrorCodes = -2146368439i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrQueuingServiceNotAvailable: COMAdminErrorCodes = -2146367998i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrObjectParentMissing: COMAdminErrorCodes = -2146367480i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrObjectDoesNotExist: COMAdminErrorCodes = -2146367479i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCanNotExportAppProxy: COMAdminErrorCodes = -2146368438i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCanNotStartApp: COMAdminErrorCodes = -2146368437i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCanNotExportSystemApp: COMAdminErrorCodes = -2146368436i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCanNotSubscribeToComponent: COMAdminErrorCodes = -2146368435i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrAppNotRunning: COMAdminErrorCodes = -2146367478i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrEventClassCannotBeSubscriber: COMAdminErrorCodes = -2146368434i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrLibAppProxyIncompatible: COMAdminErrorCodes = -2146368433i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrBasePartitionOnly: COMAdminErrorCodes = -2146368432i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrDuplicatePartitionName: COMAdminErrorCodes = -2146368425i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrPartitionInUse: COMAdminErrorCodes = -2146368423i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrImportedComponentsNotAllowed: COMAdminErrorCodes = -2146368421i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRegdbNotInitialized: COMAdminErrorCodes = -2146368398i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRegdbNotOpen: COMAdminErrorCodes = -2146368397i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRegdbSystemErr: COMAdminErrorCodes = -2146368396i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRegdbAlreadyRunning: COMAdminErrorCodes = -2146368395i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrMigVersionNotSupported: COMAdminErrorCodes = -2146368384i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrMigSchemaNotFound: COMAdminErrorCodes = -2146368383i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCatBitnessMismatch: COMAdminErrorCodes = -2146368382i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCatUnacceptableBitness: COMAdminErrorCodes = -2146368381i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCatWrongAppBitnessBitness: COMAdminErrorCodes = -2146368380i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCatPauseResumeNotSupported: COMAdminErrorCodes = -2146368379i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCatServerFault: COMAdminErrorCodes = -2146368378i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCantRecycleLibraryApps: COMAdminErrorCodes = -2146367473i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCantRecycleServiceApps: COMAdminErrorCodes = -2146367471i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrProcessAlreadyRecycled: COMAdminErrorCodes = -2146367470i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrPausedProcessMayNotBeRecycled: COMAdminErrorCodes = -2146367469i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrInvalidPartition: COMAdminErrorCodes = -2146367477i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrPartitionMsiOnly: COMAdminErrorCodes = -2146367463i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrStartAppDisabled: COMAdminErrorCodes = -2146368431i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompMoveSource: COMAdminErrorCodes = -2146367460i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompMoveDest: COMAdminErrorCodes = -2146367459i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompMovePrivate: COMAdminErrorCodes = -2146367458i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCannotCopyEventClass: COMAdminErrorCodes = -2146367456i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminFileFlags = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagLoadable: COMAdminFileFlags = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagCOM: COMAdminFileFlags = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagContainsPS: COMAdminFileFlags = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagContainsComp: COMAdminFileFlags = 8i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagContainsTLB: COMAdminFileFlags = 16i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagSelfReg: COMAdminFileFlags = 32i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagSelfUnReg: COMAdminFileFlags = 64i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagUnloadableDLL: COMAdminFileFlags = 128i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagDoesNotExist: COMAdminFileFlags = 256i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagAlreadyInstalled: COMAdminFileFlags = 512i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagBadTLB: COMAdminFileFlags = 1024i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagGetClassObjFailed: COMAdminFileFlags = 2048i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagClassNotAvailable: COMAdminFileFlags = 4096i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagRegistrar: COMAdminFileFlags = 8192i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagNoRegistrar: COMAdminFileFlags = 16384i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagDLLRegsvrFailed: COMAdminFileFlags = 32768i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagRegTLBFailed: COMAdminFileFlags = 65536i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagRegistrarFailed: COMAdminFileFlags = 131072i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagError: COMAdminFileFlags = 262144i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminImpersonationLevelOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminImpersonationAnonymous: COMAdminImpersonationLevelOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminImpersonationIdentify: COMAdminImpersonationLevelOptions = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminImpersonationImpersonate: COMAdminImpersonationLevelOptions = 3i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminImpersonationDelegate: COMAdminImpersonationLevelOptions = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminInUse = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminNotInUse: COMAdminInUse = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminInUseByCatalog: COMAdminInUse = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminInUseByRegistryUnknown: COMAdminInUse = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminInUseByRegistryProxyStub: COMAdminInUse = 3i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminInUseByRegistryTypeLib: COMAdminInUse = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminInUseByRegistryClsid: COMAdminInUse = 5i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminOS = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSNotInitialized: COMAdminOS = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows3_1: COMAdminOS = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows9x: COMAdminOS = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows2000: COMAdminOS = 3i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows2000AdvancedServer: COMAdminOS = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows2000Unknown: COMAdminOS = 5i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSUnknown: COMAdminOS = 6i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsXPPersonal: COMAdminOS = 11i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsXPProfessional: COMAdminOS = 12i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsNETStandardServer: COMAdminOS = 13i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsNETEnterpriseServer: COMAdminOS = 14i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsNETDatacenterServer: COMAdminOS = 15i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsNETWebServer: COMAdminOS = 16i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsLonghornPersonal: COMAdminOS = 17i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsLonghornProfessional: COMAdminOS = 18i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsLonghornStandardServer: COMAdminOS = 19i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsLonghornEnterpriseServer: COMAdminOS = 20i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsLonghornDatacenterServer: COMAdminOS = 21i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsLonghornWebServer: COMAdminOS = 22i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows7Personal: COMAdminOS = 23i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows7Professional: COMAdminOS = 24i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows7StandardServer: COMAdminOS = 25i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows7EnterpriseServer: COMAdminOS = 26i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows7DatacenterServer: COMAdminOS = 27i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows7WebServer: COMAdminOS = 28i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows8Personal: COMAdminOS = 29i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows8Professional: COMAdminOS = 30i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows8StandardServer: COMAdminOS = 31i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows8EnterpriseServer: COMAdminOS = 32i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows8DatacenterServer: COMAdminOS = 33i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows8WebServer: COMAdminOS = 34i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsBluePersonal: COMAdminOS = 35i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsBlueProfessional: COMAdminOS = 36i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsBlueStandardServer: COMAdminOS = 37i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsBlueEnterpriseServer: COMAdminOS = 38i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsBlueDatacenterServer: COMAdminOS = 39i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsBlueWebServer: COMAdminOS = 40i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminQCMessageAuthenticateOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminQCMessageAuthenticateSecureApps: COMAdminQCMessageAuthenticateOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminQCMessageAuthenticateOff: COMAdminQCMessageAuthenticateOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminQCMessageAuthenticateOn: COMAdminQCMessageAuthenticateOptions = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminServiceOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServiceLoadBalanceRouter: COMAdminServiceOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminServiceStatusOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServiceStopped: COMAdminServiceStatusOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServiceStartPending: COMAdminServiceStatusOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServiceStopPending: COMAdminServiceStatusOptions = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServiceRunning: COMAdminServiceStatusOptions = 3i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServiceContinuePending: COMAdminServiceStatusOptions = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServicePausePending: COMAdminServiceStatusOptions = 5i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServicePaused: COMAdminServiceStatusOptions = 6i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServiceUnknownState: COMAdminServiceStatusOptions = 7i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminSynchronizationOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminSynchronizationIgnored: COMAdminSynchronizationOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminSynchronizationNone: COMAdminSynchronizationOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminSynchronizationSupported: COMAdminSynchronizationOptions = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminSynchronizationRequired: COMAdminSynchronizationOptions = 3i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminSynchronizationRequiresNew: COMAdminSynchronizationOptions = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminThreadingModels = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminThreadingModelApartment: COMAdminThreadingModels = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminThreadingModelFree: COMAdminThreadingModels = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminThreadingModelMain: COMAdminThreadingModels = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminThreadingModelBoth: COMAdminThreadingModels = 3i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminThreadingModelNeutral: COMAdminThreadingModels = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminThreadingModelNotSpecified: COMAdminThreadingModels = 5i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminTransactionOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTransactionIgnored: COMAdminTransactionOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTransactionNone: COMAdminTransactionOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTransactionSupported: COMAdminTransactionOptions = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTransactionRequired: COMAdminTransactionOptions = 3i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTransactionRequiresNew: COMAdminTransactionOptions = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminTxIsolationLevelOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTxIsolationLevelAny: COMAdminTxIsolationLevelOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTxIsolationLevelReadUnCommitted: COMAdminTxIsolationLevelOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTxIsolationLevelReadCommitted: COMAdminTxIsolationLevelOptions = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTxIsolationLevelRepeatableRead: COMAdminTxIsolationLevelOptions = 3i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTxIsolationLevelSerializable: COMAdminTxIsolationLevelOptions = 4i32;
pub const COMEvents: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674859, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMPLUS_APPTYPE = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const APPTYPE_UNKNOWN: COMPLUS_APPTYPE = -1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const APPTYPE_SERVER: COMPLUS_APPTYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const APPTYPE_LIBRARY: COMPLUS_APPTYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const APPTYPE_SWC: COMPLUS_APPTYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct COMSVCSEVENTINFO {
    pub cbSize: u32,
    pub dwPid: u32,
    pub lTime: i64,
    pub lMicroTime: i32,
    pub perfCount: i64,
    pub guidApp: ::windows_sys::core::GUID,
    pub sMachineName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for COMSVCSEVENTINFO {}
impl ::core::clone::Clone for COMSVCSEVENTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CRMClerk: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674877, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CRMFLAGS = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMFLAG_FORGETTARGET: CRMFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMFLAG_WRITTENDURINGPREPARE: CRMFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMFLAG_WRITTENDURINGCOMMIT: CRMFLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMFLAG_WRITTENDURINGABORT: CRMFLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMFLAG_WRITTENDURINGRECOVERY: CRMFLAGS = 16i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMFLAG_WRITTENDURINGREPLAY: CRMFLAGS = 32i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMFLAG_REPLAYINPROGRESS: CRMFLAGS = 64i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CRMREGFLAGS = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMREGFLAG_PREPAREPHASE: CRMREGFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMREGFLAG_COMMITPHASE: CRMREGFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMREGFLAG_ABORTPHASE: CRMREGFLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMREGFLAG_ALLPHASES: CRMREGFLAGS = 7i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMREGFLAG_FAILIFINDOUBTSREMAIN: CRMREGFLAGS = 16i32;
pub const CRMRecoveryClerk: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674878, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRR_ACTIVATION_LIMIT: u32 = 4294967294u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRR_CALL_LIMIT: u32 = 4294967293u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRR_LIFETIME_LIMIT: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRR_MEMORY_LIMIT: u32 = 4294967292u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRR_NO_REASON_SUPPLIED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRR_RECYCLED_FROM_UI: u32 = 4294967291u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CSC_Binding = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NoBinding: CSC_Binding = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_BindToPoolThread: CSC_Binding = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CSC_COMTIIntrinsicsConfig = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NoCOMTIIntrinsics: CSC_COMTIIntrinsicsConfig = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_InheritCOMTIIntrinsics: CSC_COMTIIntrinsicsConfig = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CSC_IISIntrinsicsConfig = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NoIISIntrinsics: CSC_IISIntrinsicsConfig = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_InheritIISIntrinsics: CSC_IISIntrinsicsConfig = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CSC_InheritanceConfig = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_Inherit: CSC_InheritanceConfig = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_Ignore: CSC_InheritanceConfig = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CSC_PartitionConfig = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NoPartition: CSC_PartitionConfig = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_InheritPartition: CSC_PartitionConfig = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NewPartition: CSC_PartitionConfig = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CSC_SxsConfig = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NoSxs: CSC_SxsConfig = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_InheritSxs: CSC_SxsConfig = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NewSxs: CSC_SxsConfig = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CSC_SynchronizationConfig = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NoSynchronization: CSC_SynchronizationConfig = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_IfContainerIsSynchronized: CSC_SynchronizationConfig = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NewSynchronizationIfNecessary: CSC_SynchronizationConfig = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NewSynchronization: CSC_SynchronizationConfig = 3i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CSC_ThreadPool = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_ThreadPoolNone: CSC_ThreadPool = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_ThreadPoolInherit: CSC_ThreadPool = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_STAThreadPool: CSC_ThreadPool = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_MTAThreadPool: CSC_ThreadPool = 3i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CSC_TrackerConfig = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_DontUseTracker: CSC_TrackerConfig = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_UseTracker: CSC_TrackerConfig = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CSC_TransactionConfig = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NoTransaction: CSC_TransactionConfig = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_IfContainerIsTransactional: CSC_TransactionConfig = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_CreateTransactionIfNecessary: CSC_TransactionConfig = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NewTransaction: CSC_TransactionConfig = 3i32;
pub const CServiceConfig: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674888, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub const ClrAssemblyLocator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1166713781, data2: 9818, data3: 19317, data4: [188, 5, 155, 234, 70, 48, 207, 24] };
pub const CoMTSLocator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674860, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub const ComServiceEvents: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674883, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub const ComSystemAppEventData: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674886, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ComponentHangMonitorInfo {
    pub IsMonitored: super::super::Foundation::BOOL,
    pub TerminateOnHang: super::super::Foundation::BOOL,
    pub AvgCallThresholdInMs: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ComponentHangMonitorInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ComponentHangMonitorInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct ComponentStatistics {
    pub NumInstances: u32,
    pub NumBoundReferences: u32,
    pub NumPooledObjects: u32,
    pub NumObjectsInCall: u32,
    pub AvgResponseTimeInMs: u32,
    pub NumCallsCompletedRecent: u32,
    pub NumCallsFailedRecent: u32,
    pub NumCallsCompletedTotal: u32,
    pub NumCallsFailedTotal: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: u32,
    pub Reserved4: u32,
}
impl ::core::marker::Copy for ComponentStatistics {}
impl ::core::clone::Clone for ComponentStatistics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct ComponentSummary {
    pub ApplicationInstanceId: ::windows_sys::core::GUID,
    pub PartitionId: ::windows_sys::core::GUID,
    pub ApplicationId: ::windows_sys::core::GUID,
    pub Clsid: ::windows_sys::core::GUID,
    pub ClassName: ::windows_sys::core::PWSTR,
    pub ApplicationName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for ComponentSummary {}
impl ::core::clone::Clone for ComponentSummary {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ContextInfo {
    pub base__: super::Com::IDispatch,
    pub IsInTransaction: unsafe extern "system" fn(this: *mut *mut Self, pbisintx: *mut i16) -> ::windows_sys::core::HRESULT,
    pub GetTransaction: unsafe extern "system" fn(this: *mut *mut Self, pptx: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTransactionId: unsafe extern "system" fn(this: *mut *mut Self, pbstrtxid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTransactionId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetActivityId: unsafe extern "system" fn(this: *mut *mut Self, pbstractivityid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetActivityId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetContextId: unsafe extern "system" fn(this: *mut *mut Self, pbstrctxid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetContextId: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ContextInfo2 {
    pub base__: ContextInfo,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPartitionId: unsafe extern "system" fn(this: *mut *mut Self, __midl__contextinfo20000: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPartitionId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetApplicationId: unsafe extern "system" fn(this: *mut *mut Self, __midl__contextinfo20001: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetApplicationId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetApplicationInstanceId: unsafe extern "system" fn(this: *mut *mut Self, __midl__contextinfo20002: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetApplicationInstanceId: usize,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct CrmLogRecordRead {
    pub dwCrmFlags: u32,
    pub dwSequenceNumber: u32,
    pub blobUserData: super::Com::BLOB,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for CrmLogRecordRead {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for CrmLogRecordRead {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CrmTransactionState = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TxState_Active: CrmTransactionState = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TxState_Committed: CrmTransactionState = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TxState_Aborted: CrmTransactionState = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TxState_Indoubt: CrmTransactionState = 3i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const DATA_NOT_AVAILABLE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type DUMPTYPE = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const DUMPTYPE_FULL: DUMPTYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const DUMPTYPE_MINI: DUMPTYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const DUMPTYPE_NONE: DUMPTYPE = 2i32;
pub const DispenserManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674880, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub const Dummy30040732: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674857, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub const EventServer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674620, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const GUID_STRING_SIZE: u32 = 40u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type GetAppTrackerDataFlags = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const GATD_INCLUDE_PROCESS_EXE_NAME: GetAppTrackerDataFlags = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const GATD_INCLUDE_LIBRARY_APPS: GetAppTrackerDataFlags = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const GATD_INCLUDE_SWC: GetAppTrackerDataFlags = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const GATD_INCLUDE_CLASS_NAME: GetAppTrackerDataFlags = 8i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const GATD_INCLUDE_APPLICATION_NAME: GetAppTrackerDataFlags = 16i32;
pub const GetSecurityCallContextAppObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674856, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HANG_INFO {
    pub fAppHangMonitorEnabled: super::super::Foundation::BOOL,
    pub fTerminateOnHang: super::super::Foundation::BOOL,
    pub DumpType: DUMPTYPE,
    pub dwHangTimeout: u32,
    pub dwDumpCount: u32,
    pub dwInfoMsgCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HANG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HANG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAppDomainHelper {
    pub base__: super::Com::IDispatch,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, punkad: *mut ::core::ffi::c_void, __midl__iappdomainhelper0000: isize, ppool: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DoCallback: unsafe extern "system" fn(this: *mut *mut Self, punkad: *mut ::core::ffi::c_void, __midl__iappdomainhelper0001: isize, ppool: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAssemblyLocator {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetModules: unsafe extern "system" fn(this: *mut *mut Self, applicationdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, applicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, assemblyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pmodules: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetModules: usize,
}
#[repr(C)]
pub struct IAsyncErrorNotify {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnError: unsafe extern "system" fn(this: *mut *mut Self, hr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICOMAdminCatalog {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetCollection: unsafe extern "system" fn(this: *mut *mut Self, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetCollection: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Connect: unsafe extern "system" fn(this: *mut *mut Self, bstrcatalogservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Connect: usize,
    pub MajorVersion: unsafe extern "system" fn(this: *mut *mut Self, plmajorversion: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(this: *mut *mut Self, plminorversion: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetCollectionByQuery: unsafe extern "system" fn(this: *mut *mut Self, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarquery: *const *const super::Com::SAFEARRAY, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetCollectionByQuery: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ImportComponent: unsafe extern "system" fn(this: *mut *mut Self, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ImportComponent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InstallComponent: unsafe extern "system" fn(this: *mut *mut Self, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtlb: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpsdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstallComponent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShutdownApplication: unsafe extern "system" fn(this: *mut *mut Self, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShutdownApplication: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExportApplication: unsafe extern "system" fn(this: *mut *mut Self, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationExportOptions) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExportApplication: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InstallApplication: unsafe extern "system" fn(this: *mut *mut Self, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdestinationdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationInstallOptions, bstruserid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrsn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstallApplication: usize,
    pub StopRouter: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RefreshRouter: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub StartRouter: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Reserved1: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Reserved2: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub InstallMultipleComponents: unsafe extern "system" fn(this: *mut *mut Self, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    InstallMultipleComponents: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetMultipleComponentsInfo: unsafe extern "system" fn(this: *mut *mut Self, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetMultipleComponentsInfo: usize,
    pub RefreshComponents: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub BackupREGDB: unsafe extern "system" fn(this: *mut *mut Self, bstrbackupfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BackupREGDB: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RestoreREGDB: unsafe extern "system" fn(this: *mut *mut Self, bstrbackupfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RestoreREGDB: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub QueryApplicationFile: unsafe extern "system" fn(this: *mut *mut Self, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrapplicationname: *mut super::super::Foundation::BSTR, pbstrapplicationdescription: *mut super::super::Foundation::BSTR, pbhasusers: *mut i16, pbisproxy: *mut i16, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    QueryApplicationFile: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StartApplication: unsafe extern "system" fn(this: *mut *mut Self, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartApplication: usize,
    pub ServiceCheck: unsafe extern "system" fn(this: *mut *mut Self, lservice: i32, plstatus: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub InstallMultipleEventClasses: unsafe extern "system" fn(this: *mut *mut Self, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    InstallMultipleEventClasses: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InstallEventClass: unsafe extern "system" fn(this: *mut *mut Self, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtlb: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpsdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstallEventClass: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetEventClassesForIID: unsafe extern "system" fn(this: *mut *mut Self, bstriid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetEventClassesForIID: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICOMAdminCatalog2 {
    pub base__: ICOMAdminCatalog,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetCollectionByQuery2: unsafe extern "system" fn(this: *mut *mut Self, bstrcollectionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarquerystrings: *const super::Com::VARIANT, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetCollectionByQuery2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetApplicationInstanceIDFromProcessID: unsafe extern "system" fn(this: *mut *mut Self, lprocessid: i32, pbstrapplicationinstanceid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetApplicationInstanceIDFromProcessID: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ShutdownApplicationInstances: unsafe extern "system" fn(this: *mut *mut Self, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ShutdownApplicationInstances: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PauseApplicationInstances: unsafe extern "system" fn(this: *mut *mut Self, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PauseApplicationInstances: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ResumeApplicationInstances: unsafe extern "system" fn(this: *mut *mut Self, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ResumeApplicationInstances: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RecycleApplicationInstances: unsafe extern "system" fn(this: *mut *mut Self, pvarapplicationinstanceid: *const super::Com::VARIANT, lreasoncode: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RecycleApplicationInstances: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AreApplicationInstancesPaused: unsafe extern "system" fn(this: *mut *mut Self, pvarapplicationinstanceid: *const super::Com::VARIANT, pvarboolpaused: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AreApplicationInstancesPaused: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DumpApplicationInstance: unsafe extern "system" fn(this: *mut *mut Self, bstrapplicationinstanceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmaximages: i32, pbstrdumpfile: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DumpApplicationInstance: usize,
    pub IsApplicationInstanceDumpSupported: unsafe extern "system" fn(this: *mut *mut Self, pvarbooldumpsupported: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateServiceForApplication: unsafe extern "system" fn(this: *mut *mut Self, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrservicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrstarttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrerrorcontrol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdependencies: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrunas: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bdesktopok: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateServiceForApplication: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteServiceForApplication: unsafe extern "system" fn(this: *mut *mut Self, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteServiceForApplication: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPartitionID: unsafe extern "system" fn(this: *mut *mut Self, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrpartitionid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPartitionID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPartitionName: unsafe extern "system" fn(this: *mut *mut Self, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrpartitionname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPartitionName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCurrentPartition: unsafe extern "system" fn(this: *mut *mut Self, bstrpartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCurrentPartition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentPartitionID: unsafe extern "system" fn(this: *mut *mut Self, pbstrpartitionid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentPartitionID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentPartitionName: unsafe extern "system" fn(this: *mut *mut Self, pbstrpartitionname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentPartitionName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GlobalPartitionID: unsafe extern "system" fn(this: *mut *mut Self, pbstrglobalpartitionid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GlobalPartitionID: usize,
    pub FlushPartitionCache: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CopyApplications: unsafe extern "system" fn(this: *mut *mut Self, bstrsourcepartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarapplicationid: *const super::Com::VARIANT, bstrdestinationpartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CopyApplications: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CopyComponents: unsafe extern "system" fn(this: *mut *mut Self, bstrsourceapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, bstrdestinationapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CopyComponents: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub MoveComponents: unsafe extern "system" fn(this: *mut *mut Self, bstrsourceapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, bstrdestinationapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    MoveComponents: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AliasComponent: unsafe extern "system" fn(this: *mut *mut Self, bstrsrcapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdestapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnewprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnewclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AliasComponent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSafeToDelete: unsafe extern "system" fn(this: *mut *mut Self, bstrdllname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcomadmininuse: *mut COMAdminInUse) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSafeToDelete: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ImportUnconfiguredComponents: unsafe extern "system" fn(this: *mut *mut Self, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ImportUnconfiguredComponents: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PromoteUnconfiguredComponents: unsafe extern "system" fn(this: *mut *mut Self, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PromoteUnconfiguredComponents: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ImportComponents: unsafe extern "system" fn(this: *mut *mut Self, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ImportComponents: usize,
    pub Is64BitCatalogServer: unsafe extern "system" fn(this: *mut *mut Self, pbis64bit: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExportPartition: unsafe extern "system" fn(this: *mut *mut Self, bstrpartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpartitionfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationExportOptions) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExportPartition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InstallPartition: unsafe extern "system" fn(this: *mut *mut Self, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdestdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationInstallOptions, bstruserid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrsn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstallPartition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub QueryApplicationFile2: unsafe extern "system" fn(this: *mut *mut Self, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfilesforimport: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    QueryApplicationFile2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetComponentVersionCount: unsafe extern "system" fn(this: *mut *mut Self, bstrclsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plversioncount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetComponentVersionCount: usize,
}
#[repr(C)]
pub struct ICOMLBArguments {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCLSID: unsafe extern "system" fn(this: *mut *mut Self, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetCLSID: unsafe extern "system" fn(this: *mut *mut Self, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetMachineName: unsafe extern "system" fn(this: *mut *mut Self, cchsvr: u32, szservername: ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetMachineName: unsafe extern "system" fn(this: *mut *mut Self, cchsvr: u32, szservername: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICatalogCollection {
    pub base__: super::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, lindex: i32, ppcatalogobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plobjectcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, lindex: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, ppcatalogobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Populate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SaveChanges: unsafe extern "system" fn(this: *mut *mut Self, pcchanges: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetCollection: unsafe extern "system" fn(this: *mut *mut Self, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varobjectkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetCollection: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pvarnamel: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Name: usize,
    pub AddEnabled: unsafe extern "system" fn(this: *mut *mut Self, pvarbool: *mut i16) -> ::windows_sys::core::HRESULT,
    pub RemoveEnabled: unsafe extern "system" fn(this: *mut *mut Self, pvarbool: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetUtilInterface: unsafe extern "system" fn(this: *mut *mut Self, ppidispatch: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetUtilInterface: usize,
    pub DataStoreMajorVersion: unsafe extern "system" fn(this: *mut *mut Self, plmajorversion: *mut i32) -> ::windows_sys::core::HRESULT,
    pub DataStoreMinorVersion: unsafe extern "system" fn(this: *mut *mut Self, plminorversionl: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PopulateByKey: unsafe extern "system" fn(this: *mut *mut Self, psakeys: *const super::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PopulateByKey: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PopulateByQuery: unsafe extern "system" fn(this: *mut *mut Self, bstrquerystring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lquerytype: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PopulateByQuery: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICatalogObject {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Value: unsafe extern "system" fn(this: *mut *mut Self, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarretval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub put_Value: unsafe extern "system" fn(this: *mut *mut Self, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, val: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    put_Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Key: unsafe extern "system" fn(this: *mut *mut Self, pvarretval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Key: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pvarretval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPropertyReadOnly: unsafe extern "system" fn(this: *mut *mut Self, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbretval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPropertyReadOnly: usize,
    pub Valid: unsafe extern "system" fn(this: *mut *mut Self, pbretval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPropertyWriteOnly: unsafe extern "system" fn(this: *mut *mut Self, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbretval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPropertyWriteOnly: usize,
}
#[repr(C)]
pub struct ICheckSxsConfig {
    pub base__: ::windows_sys::core::IUnknown,
    pub IsSameSxsConfig: unsafe extern "system" fn(this: *mut *mut Self, wszsxsname: ::windows_sys::core::PCWSTR, wszsxsdirectory: ::windows_sys::core::PCWSTR, wszsxsappname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComActivityEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnActivityCreate: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnActivityDestroy: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnActivityEnter: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_sys::core::GUID, guidentered: *const ::windows_sys::core::GUID, dwthread: u32) -> ::windows_sys::core::HRESULT,
    pub OnActivityTimeout: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_sys::core::GUID, guidentered: *const ::windows_sys::core::GUID, dwthread: u32, dwtimeout: u32) -> ::windows_sys::core::HRESULT,
    pub OnActivityReenter: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_sys::core::GUID, dwthread: u32, dwcalldepth: u32) -> ::windows_sys::core::HRESULT,
    pub OnActivityLeave: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_sys::core::GUID, guidleft: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnActivityLeaveSame: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_sys::core::GUID, dwcalldepth: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComApp2Events {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnAppActivation2: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_sys::core::GUID, guidprocess: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnAppShutdown2: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnAppForceShutdown2: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnAppPaused2: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_sys::core::GUID, bpaused: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnAppPaused2: usize,
    pub OnAppRecycle2: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_sys::core::GUID, guidprocess: ::windows_sys::core::GUID, lreason: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComAppEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnAppActivation: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnAppShutdown: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnAppForceShutdown: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComCRMEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnCRMRecoveryStart: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnCRMRecoveryDone: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnCRMCheckpoint: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnCRMBegin: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_sys::core::GUID, guidactivity: ::windows_sys::core::GUID, guidtx: ::windows_sys::core::GUID, szprogidcompensator: ::windows_sys::core::PCWSTR, szdescription: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub OnCRMPrepare: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnCRMCommit: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnCRMAbort: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnCRMIndoubt: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnCRMDone: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnCRMRelease: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnCRMAnalyze: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_sys::core::GUID, dwcrmrecordtype: u32, dwrecordsize: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnCRMWrite: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_sys::core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnCRMWrite: usize,
    pub OnCRMForget: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnCRMForce: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnCRMDeliver: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_sys::core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnCRMDeliver: usize,
}
#[repr(C)]
pub struct IComExceptionEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnExceptionUser: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, code: u32, address: u64, pszstacktrace: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComIdentityEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnIISRequestInfo: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, objid: u64, pszclientip: ::windows_sys::core::PCWSTR, pszserverip: ::windows_sys::core::PCWSTR, pszurl: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComInstance2Events {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnObjectCreate2: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_sys::core::GUID, clsid: *const ::windows_sys::core::GUID, tsid: *const ::windows_sys::core::GUID, ctxtid: u64, objectid: u64, guidpartition: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnObjectDestroy2: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComInstanceEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnObjectCreate: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_sys::core::GUID, clsid: *const ::windows_sys::core::GUID, tsid: *const ::windows_sys::core::GUID, ctxtid: u64, objectid: u64) -> ::windows_sys::core::HRESULT,
    pub OnObjectDestroy: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComLTxEvents {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnLtxTransactionStart: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows_sys::core::GUID, tsid: ::windows_sys::core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnLtxTransactionStart: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnLtxTransactionPrepare: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows_sys::core::GUID, fvote: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnLtxTransactionPrepare: usize,
    pub OnLtxTransactionAbort: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnLtxTransactionCommit: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnLtxTransactionPromote: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows_sys::core::GUID, txnid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComMethod2Events {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnMethodCall2: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_sys::core::GUID, guidrid: *const ::windows_sys::core::GUID, dwthread: u32, imeth: u32) -> ::windows_sys::core::HRESULT,
    pub OnMethodReturn2: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_sys::core::GUID, guidrid: *const ::windows_sys::core::GUID, dwthread: u32, imeth: u32, hresult: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub OnMethodException2: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_sys::core::GUID, guidrid: *const ::windows_sys::core::GUID, dwthread: u32, imeth: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComMethodEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnMethodCall: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_sys::core::GUID, guidrid: *const ::windows_sys::core::GUID, imeth: u32) -> ::windows_sys::core::HRESULT,
    pub OnMethodReturn: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_sys::core::GUID, guidrid: *const ::windows_sys::core::GUID, imeth: u32, hresult: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub OnMethodException: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_sys::core::GUID, guidrid: *const ::windows_sys::core::GUID, imeth: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComMtaThreadPoolKnobs {
    pub base__: ::windows_sys::core::IUnknown,
    pub MTASetMaxThreadCount: unsafe extern "system" fn(this: *mut *mut Self, dwmaxthreads: u32) -> ::windows_sys::core::HRESULT,
    pub MTAGetMaxThreadCount: unsafe extern "system" fn(this: *mut *mut Self, pdwmaxthreads: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MTASetThrottleValue: unsafe extern "system" fn(this: *mut *mut Self, dwthrottle: u32) -> ::windows_sys::core::HRESULT,
    pub MTAGetThrottleValue: unsafe extern "system" fn(this: *mut *mut Self, pdwthrottle: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComObjectConstruction2Events {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnObjectConstruct2: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_sys::core::GUID, sconstructstring: ::windows_sys::core::PCWSTR, oid: u64, guidpartition: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComObjectConstructionEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnObjectConstruct: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_sys::core::GUID, sconstructstring: ::windows_sys::core::PCWSTR, oid: u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComObjectEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnObjectActivate: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows_sys::core::HRESULT,
    pub OnObjectDeactivate: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows_sys::core::HRESULT,
    pub OnDisableCommit: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_sys::core::HRESULT,
    pub OnEnableCommit: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_sys::core::HRESULT,
    pub OnSetComplete: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_sys::core::HRESULT,
    pub OnSetAbort: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComObjectPool2Events {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnObjPoolPutObject2: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_sys::core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows_sys::core::HRESULT,
    pub OnObjPoolGetObject2: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_sys::core::GUID, guidobject: *const ::windows_sys::core::GUID, dwavailable: u32, oid: u64, guidpartition: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnObjPoolRecycleToTx2: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_sys::core::GUID, guidobject: *const ::windows_sys::core::GUID, guidtx: *const ::windows_sys::core::GUID, objid: u64) -> ::windows_sys::core::HRESULT,
    pub OnObjPoolGetFromTx2: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_sys::core::GUID, guidobject: *const ::windows_sys::core::GUID, guidtx: *const ::windows_sys::core::GUID, objid: u64, guidpartition: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComObjectPoolEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnObjPoolPutObject: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_sys::core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows_sys::core::HRESULT,
    pub OnObjPoolGetObject: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_sys::core::GUID, guidobject: *const ::windows_sys::core::GUID, dwavailable: u32, oid: u64) -> ::windows_sys::core::HRESULT,
    pub OnObjPoolRecycleToTx: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_sys::core::GUID, guidobject: *const ::windows_sys::core::GUID, guidtx: *const ::windows_sys::core::GUID, objid: u64) -> ::windows_sys::core::HRESULT,
    pub OnObjPoolGetFromTx: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_sys::core::GUID, guidobject: *const ::windows_sys::core::GUID, guidtx: *const ::windows_sys::core::GUID, objid: u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComObjectPoolEvents2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnObjPoolCreateObject: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_sys::core::GUID, dwobjscreated: u32, oid: u64) -> ::windows_sys::core::HRESULT,
    pub OnObjPoolDestroyObject: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_sys::core::GUID, dwobjscreated: u32, oid: u64) -> ::windows_sys::core::HRESULT,
    pub OnObjPoolCreateDecision: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, dwthreadswaiting: u32, dwavail: u32, dwcreated: u32, dwmin: u32, dwmax: u32) -> ::windows_sys::core::HRESULT,
    pub OnObjPoolTimeout: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_sys::core::GUID, guidactivity: *const ::windows_sys::core::GUID, dwtimeout: u32) -> ::windows_sys::core::HRESULT,
    pub OnObjPoolCreatePool: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_sys::core::GUID, dwmin: u32, dwmax: u32, dwtimeout: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComQCEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnQCRecord: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, objid: u64, szqueue: ::windows_sys::core::PCWSTR, guidmsgid: *const ::windows_sys::core::GUID, guidworkflowid: *const ::windows_sys::core::GUID, msmqhr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub OnQCQueueOpen: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, szqueue: ::windows_sys::core::PCWSTR, queueid: u64, hr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub OnQCReceive: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, queueid: u64, guidmsgid: *const ::windows_sys::core::GUID, guidworkflowid: *const ::windows_sys::core::GUID, hr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub OnQCReceiveFail: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, queueid: u64, msmqhr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub OnQCMoveToReTryQueue: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows_sys::core::GUID, guidworkflowid: *const ::windows_sys::core::GUID, retryindex: u32) -> ::windows_sys::core::HRESULT,
    pub OnQCMoveToDeadQueue: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows_sys::core::GUID, guidworkflowid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnQCPlayback: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, objid: u64, guidmsgid: *const ::windows_sys::core::GUID, guidworkflowid: *const ::windows_sys::core::GUID, hr: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComResourceEvents {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnResourceCreate: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: ::windows_sys::core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnResourceCreate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnResourceAllocate: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: ::windows_sys::core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL, numrated: u32, rating: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnResourceAllocate: usize,
    pub OnResourceRecycle: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: ::windows_sys::core::PCWSTR, resid: u64) -> ::windows_sys::core::HRESULT,
    pub OnResourceDestroy: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, hr: ::windows_sys::core::HRESULT, psztype: ::windows_sys::core::PCWSTR, resid: u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnResourceTrack: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: ::windows_sys::core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnResourceTrack: usize,
}
#[repr(C)]
pub struct IComSecurityEvents {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnAuthenticate: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_sys::core::GUID, objectid: u64, guidiid: *const ::windows_sys::core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnAuthenticate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnAuthenticateFail: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_sys::core::GUID, objectid: u64, guidiid: *const ::windows_sys::core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnAuthenticateFail: usize,
}
#[repr(C)]
pub struct IComStaThreadPoolKnobs {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetMinThreadCount: unsafe extern "system" fn(this: *mut *mut Self, minthreads: u32) -> ::windows_sys::core::HRESULT,
    pub GetMinThreadCount: unsafe extern "system" fn(this: *mut *mut Self, minthreads: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMaxThreadCount: unsafe extern "system" fn(this: *mut *mut Self, maxthreads: u32) -> ::windows_sys::core::HRESULT,
    pub GetMaxThreadCount: unsafe extern "system" fn(this: *mut *mut Self, maxthreads: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetActivityPerThread: unsafe extern "system" fn(this: *mut *mut Self, activitiesperthread: u32) -> ::windows_sys::core::HRESULT,
    pub GetActivityPerThread: unsafe extern "system" fn(this: *mut *mut Self, activitiesperthread: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetActivityRatio: unsafe extern "system" fn(this: *mut *mut Self, activityratio: f64) -> ::windows_sys::core::HRESULT,
    pub GetActivityRatio: unsafe extern "system" fn(this: *mut *mut Self, activityratio: *mut f64) -> ::windows_sys::core::HRESULT,
    pub GetThreadCount: unsafe extern "system" fn(this: *mut *mut Self, pdwthreads: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetQueueDepth: unsafe extern "system" fn(this: *mut *mut Self, pdwqdepth: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetQueueDepth: unsafe extern "system" fn(this: *mut *mut Self, dwqdepth: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComStaThreadPoolKnobs2 {
    pub base__: IComStaThreadPoolKnobs,
    pub GetMaxCPULoad: unsafe extern "system" fn(this: *mut *mut Self, pdwload: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMaxCPULoad: unsafe extern "system" fn(this: *mut *mut Self, pdwload: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCPUMetricEnabled: unsafe extern "system" fn(this: *mut *mut Self, pbmetricenabled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCPUMetricEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCPUMetricEnabled: unsafe extern "system" fn(this: *mut *mut Self, bmetricenabled: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCPUMetricEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCreateThreadsAggressively: unsafe extern "system" fn(this: *mut *mut Self, pbmetricenabled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCreateThreadsAggressively: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCreateThreadsAggressively: unsafe extern "system" fn(this: *mut *mut Self, bmetricenabled: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCreateThreadsAggressively: usize,
    pub GetMaxCSR: unsafe extern "system" fn(this: *mut *mut Self, pdwcsr: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMaxCSR: unsafe extern "system" fn(this: *mut *mut Self, dwcsr: i32) -> ::windows_sys::core::HRESULT,
    pub GetWaitTimeForThreadCleanup: unsafe extern "system" fn(this: *mut *mut Self, pdwthreadcleanupwaittime: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetWaitTimeForThreadCleanup: unsafe extern "system" fn(this: *mut *mut Self, dwthreadcleanupwaittime: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComThreadEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnThreadStart: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows_sys::core::HRESULT,
    pub OnThreadTerminate: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows_sys::core::HRESULT,
    pub OnThreadBindToApartment: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32, dwlowcnt: u32) -> ::windows_sys::core::HRESULT,
    pub OnThreadUnBind: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32) -> ::windows_sys::core::HRESULT,
    pub OnThreadWorkEnque: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows_sys::core::HRESULT,
    pub OnThreadWorkPrivate: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64) -> ::windows_sys::core::HRESULT,
    pub OnThreadWorkPublic: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows_sys::core::HRESULT,
    pub OnThreadWorkRedirect: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32, threadnum: u64) -> ::windows_sys::core::HRESULT,
    pub OnThreadWorkReject: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows_sys::core::HRESULT,
    pub OnThreadAssignApartment: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_sys::core::GUID, aptid: u64) -> ::windows_sys::core::HRESULT,
    pub OnThreadUnassignApartment: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, aptid: u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComTrackingInfoCollection {
    pub base__: ::windows_sys::core::IUnknown,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, ptype: *mut TRACKING_COLL_TYPE) -> ::windows_sys::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, ulindex: u32, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComTrackingInfoEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnNewTrackingInfo: unsafe extern "system" fn(this: *mut *mut Self, ptoplevelcollection: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComTrackingInfoObject {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, szpropertyname: ::windows_sys::core::PCWSTR, pvarout: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetValue: usize,
}
#[repr(C)]
pub struct IComTrackingInfoProperties {
    pub base__: ::windows_sys::core::IUnknown,
    pub PropCount: unsafe extern "system" fn(this: *mut *mut Self, pcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetPropName: unsafe extern "system" fn(this: *mut *mut Self, ulindex: u32, ppszpropname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComTransaction2Events {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnTransactionStart2: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_sys::core::GUID, tsid: *const ::windows_sys::core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnTransactionStart2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnTransactionPrepare2: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_sys::core::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnTransactionPrepare2: usize,
    pub OnTransactionAbort2: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnTransactionCommit2: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComTransactionEvents {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub OnTransactionStart: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_sys::core::GUID, tsid: *const ::windows_sys::core::GUID, froot: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnTransactionStart: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnTransactionPrepare: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_sys::core::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnTransactionPrepare: usize,
    pub OnTransactionAbort: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OnTransactionCommit: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComUserEvent {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OnUserEvent: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *const COMSVCSEVENTINFO, pvarevent: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OnUserEvent: usize,
}
#[repr(C)]
pub struct IContextProperties {
    pub base__: ::windows_sys::core::IUnknown,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    pub EnumNames: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, property: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveProperty: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveProperty: usize,
}
#[repr(C)]
pub struct IContextSecurityPerimeter {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPerimeterFlag: unsafe extern "system" fn(this: *mut *mut Self, pflag: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPerimeterFlag: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPerimeterFlag: unsafe extern "system" fn(this: *mut *mut Self, fflag: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPerimeterFlag: usize,
}
#[repr(C)]
pub struct IContextState {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetDeactivateOnReturn: unsafe extern "system" fn(this: *mut *mut Self, bdeactivate: i16) -> ::windows_sys::core::HRESULT,
    pub GetDeactivateOnReturn: unsafe extern "system" fn(this: *mut *mut Self, pbdeactivate: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetMyTransactionVote: unsafe extern "system" fn(this: *mut *mut Self, txvote: TransactionVote) -> ::windows_sys::core::HRESULT,
    pub GetMyTransactionVote: unsafe extern "system" fn(this: *mut *mut Self, ptxvote: *mut TransactionVote) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICreateWithLocalTransaction {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateInstanceWithSysTx: unsafe extern "system" fn(this: *mut *mut Self, ptransaction: *mut ::core::ffi::c_void, rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICreateWithTipTransactionEx {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, bstrtipurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateInstance: usize,
}
#[repr(C)]
pub struct ICreateWithTransactionEx {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, ptransaction: *mut ::core::ffi::c_void, rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    CreateInstance: usize,
}
#[repr(C)]
pub struct ICrmCompensator {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetLogControl: unsafe extern "system" fn(this: *mut *mut Self, plogcontrol: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BeginPrepare: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub PrepareRecord: unsafe extern "system" fn(this: *mut *mut Self, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    PrepareRecord: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EndPrepare: unsafe extern "system" fn(this: *mut *mut Self, pfoktoprepare: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EndPrepare: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginCommit: unsafe extern "system" fn(this: *mut *mut Self, frecovery: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginCommit: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CommitRecord: unsafe extern "system" fn(this: *mut *mut Self, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CommitRecord: usize,
    pub EndCommit: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginAbort: unsafe extern "system" fn(this: *mut *mut Self, frecovery: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginAbort: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AbortRecord: unsafe extern "system" fn(this: *mut *mut Self, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AbortRecord: usize,
    pub EndAbort: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICrmCompensatorVariants {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetLogControlVariants: unsafe extern "system" fn(this: *mut *mut Self, plogcontrol: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BeginPrepareVariants: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PrepareRecordVariants: unsafe extern "system" fn(this: *mut *mut Self, plogrecord: *const super::Com::VARIANT, pbforget: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PrepareRecordVariants: usize,
    pub EndPrepareVariants: unsafe extern "system" fn(this: *mut *mut Self, pboktoprepare: *mut i16) -> ::windows_sys::core::HRESULT,
    pub BeginCommitVariants: unsafe extern "system" fn(this: *mut *mut Self, brecovery: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CommitRecordVariants: unsafe extern "system" fn(this: *mut *mut Self, plogrecord: *const super::Com::VARIANT, pbforget: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CommitRecordVariants: usize,
    pub EndCommitVariants: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub BeginAbortVariants: unsafe extern "system" fn(this: *mut *mut Self, brecovery: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AbortRecordVariants: unsafe extern "system" fn(this: *mut *mut Self, plogrecord: *const super::Com::VARIANT, pbforget: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AbortRecordVariants: usize,
    pub EndAbortVariants: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICrmFormatLogRecords {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetColumnCount: unsafe extern "system" fn(this: *mut *mut Self, plcolumncount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetColumnHeaders: unsafe extern "system" fn(this: *mut *mut Self, pheaders: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetColumnHeaders: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetColumn: unsafe extern "system" fn(this: *mut *mut Self, crmlogrec: CrmLogRecordRead, pformattedlogrecord: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetColumn: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetColumnVariants: unsafe extern "system" fn(this: *mut *mut Self, logrecord: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pformattedlogrecord: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetColumnVariants: usize,
}
#[repr(C)]
pub struct ICrmLogControl {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub TransactionUOW: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TransactionUOW: usize,
    pub RegisterCompensator: unsafe extern "system" fn(this: *mut *mut Self, lpcwstrprogidcompensator: ::windows_sys::core::PCWSTR, lpcwstrdescription: ::windows_sys::core::PCWSTR, lcrmregflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub WriteLogRecordVariants: unsafe extern "system" fn(this: *mut *mut Self, plogrecord: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    WriteLogRecordVariants: usize,
    pub ForceLog: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ForgetLogRecord: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ForceTransactionToAbort: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub WriteLogRecord: unsafe extern "system" fn(this: *mut *mut Self, rgblob: *const super::Com::BLOB, cblob: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WriteLogRecord: usize,
}
#[repr(C)]
pub struct ICrmMonitor {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub GetClerks: unsafe extern "system" fn(this: *mut *mut Self, pclerks: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetClerks: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub HoldClerk: unsafe extern "system" fn(this: *mut *mut Self, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    HoldClerk: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICrmMonitorClerks {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ProgIdCompensator: unsafe extern "system" fn(this: *mut *mut Self, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ProgIdCompensator: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Description: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub TransactionUOW: unsafe extern "system" fn(this: *mut *mut Self, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    TransactionUOW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ActivityId: unsafe extern "system" fn(this: *mut *mut Self, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ActivityId: usize,
}
#[repr(C)]
pub struct ICrmMonitorLogRecords {
    pub base__: ::windows_sys::core::IUnknown,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub TransactionState: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut CrmTransactionState) -> ::windows_sys::core::HRESULT,
    pub StructuredRecords: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetLogRecord: unsafe extern "system" fn(this: *mut *mut Self, dwindex: u32, pcrmlogrec: *mut CrmLogRecordRead) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetLogRecord: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetLogRecordVariants: unsafe extern "system" fn(this: *mut *mut Self, indexnumber: ::core::mem::ManuallyDrop<super::Com::VARIANT>, plogrecord: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetLogRecordVariants: usize,
}
#[repr(C)]
pub struct IDispenserDriver {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateResource: unsafe extern "system" fn(this: *mut *mut Self, restypid: usize, presid: *mut usize, psecsfreebeforedestroy: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RateResource: unsafe extern "system" fn(this: *mut *mut Self, restypid: usize, resid: usize, frequirestransactionenlistment: super::super::Foundation::BOOL, prating: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RateResource: usize,
    pub EnlistResource: unsafe extern "system" fn(this: *mut *mut Self, resid: usize, transid: usize) -> ::windows_sys::core::HRESULT,
    pub ResetResource: unsafe extern "system" fn(this: *mut *mut Self, resid: usize) -> ::windows_sys::core::HRESULT,
    pub DestroyResource: unsafe extern "system" fn(this: *mut *mut Self, resid: usize) -> ::windows_sys::core::HRESULT,
    pub DestroyResourceS: unsafe extern "system" fn(this: *mut *mut Self, resid: *mut u16) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDispenserManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub RegisterDispenser: unsafe extern "system" fn(this: *mut *mut Self, __midl__idispensermanager0000: *mut ::core::ffi::c_void, szdispensername: ::windows_sys::core::PCWSTR, __midl__idispensermanager0001: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetContext: unsafe extern "system" fn(this: *mut *mut Self, __midl__idispensermanager0002: *mut usize, __midl__idispensermanager0003: *mut usize) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnumNames {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, celt: u32, rgname: *mut super::super::Foundation::BSTR, pceltfetched: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: u32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IEventServerTrace {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub StartTraceGuid: unsafe extern "system" fn(this: *mut *mut Self, bstrguidevent: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrguidfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpidfilter: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartTraceGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StopTraceGuid: unsafe extern "system" fn(this: *mut *mut Self, bstrguidevent: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrguidfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpidfilter: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StopTraceGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumTraceGuid: unsafe extern "system" fn(this: *mut *mut Self, plcntguids: *mut i32, pbstrguidlist: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumTraceGuid: usize,
}
#[repr(C)]
pub struct IGetAppTrackerData {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetApplicationProcesses: unsafe extern "system" fn(this: *mut *mut Self, partitionid: *const ::windows_sys::core::GUID, applicationid: *const ::windows_sys::core::GUID, flags: u32, numapplicationprocesses: *mut u32, applicationprocesses: *mut *mut ApplicationProcessSummary) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetApplicationProcesses: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetApplicationProcessDetails: unsafe extern "system" fn(this: *mut *mut Self, applicationinstanceid: *const ::windows_sys::core::GUID, processid: u32, flags: u32, summary: *mut ApplicationProcessSummary, statistics: *mut ApplicationProcessStatistics, recycleinfo: *mut ApplicationProcessRecycleInfo, anycomponentshangmonitored: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetApplicationProcessDetails: usize,
    pub GetApplicationsInProcess: unsafe extern "system" fn(this: *mut *mut Self, applicationinstanceid: *const ::windows_sys::core::GUID, processid: u32, partitionid: *const ::windows_sys::core::GUID, flags: u32, numapplicationsinprocess: *mut u32, applications: *mut *mut ApplicationSummary) -> ::windows_sys::core::HRESULT,
    pub GetComponentsInProcess: unsafe extern "system" fn(this: *mut *mut Self, applicationinstanceid: *const ::windows_sys::core::GUID, processid: u32, partitionid: *const ::windows_sys::core::GUID, applicationid: *const ::windows_sys::core::GUID, flags: u32, numcomponentsinprocess: *mut u32, components: *mut *mut ComponentSummary) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetComponentDetails: unsafe extern "system" fn(this: *mut *mut Self, applicationinstanceid: *const ::windows_sys::core::GUID, processid: u32, clsid: *const ::windows_sys::core::GUID, flags: u32, summary: *mut ComponentSummary, statistics: *mut ComponentStatistics, hangmonitorinfo: *mut ComponentHangMonitorInfo) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetComponentDetails: usize,
    pub GetTrackerDataAsCollectionObject: unsafe extern "system" fn(this: *mut *mut Self, toplevelcollection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSuggestedPollingInterval: unsafe extern "system" fn(this: *mut *mut Self, pollingintervalinseconds: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGetContextProperties {
    pub base__: ::windows_sys::core::IUnknown,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    pub EnumNames: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IGetSecurityCallContext {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSecurityCallContext: unsafe extern "system" fn(this: *mut *mut Self, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSecurityCallContext: usize,
}
#[repr(C)]
pub struct IHolder {
    pub base__: ::windows_sys::core::IUnknown,
    pub AllocResource: unsafe extern "system" fn(this: *mut *mut Self, __midl__iholder0000: usize, __midl__iholder0001: *mut usize) -> ::windows_sys::core::HRESULT,
    pub FreeResource: unsafe extern "system" fn(this: *mut *mut Self, __midl__iholder0002: usize) -> ::windows_sys::core::HRESULT,
    pub TrackResource: unsafe extern "system" fn(this: *mut *mut Self, __midl__iholder0003: usize) -> ::windows_sys::core::HRESULT,
    pub TrackResourceS: unsafe extern "system" fn(this: *mut *mut Self, __midl__iholder0004: *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UntrackResource: unsafe extern "system" fn(this: *mut *mut Self, __midl__iholder0005: usize, __midl__iholder0006: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UntrackResource: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UntrackResourceS: unsafe extern "system" fn(this: *mut *mut Self, __midl__iholder0007: *mut u16, __midl__iholder0008: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UntrackResourceS: usize,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RequestDestroyResource: unsafe extern "system" fn(this: *mut *mut Self, __midl__iholder0009: usize) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILBEvents {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub TargetUp: unsafe extern "system" fn(this: *mut *mut Self, bstrservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsideng: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TargetUp: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TargetDown: unsafe extern "system" fn(this: *mut *mut Self, bstrservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsideng: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TargetDown: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EngineDefined: unsafe extern "system" fn(this: *mut *mut Self, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varpropvalue: *const super::Com::VARIANT, bstrclsideng: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EngineDefined: usize,
}
#[repr(C)]
pub struct IMTSActivity {
    pub base__: ::windows_sys::core::IUnknown,
    pub SynchronousCall: unsafe extern "system" fn(this: *mut *mut Self, pcall: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AsyncCall: unsafe extern "system" fn(this: *mut *mut Self, pcall: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Reserved1: unsafe extern "system" fn(this: *mut *mut Self),
    pub BindToCurrentThread: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub UnbindFromThread: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMTSCall {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnCall: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMTSLocator {
    pub base__: super::Com::IDispatch,
    pub GetEventDispatcher: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IManagedActivationEvents {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateManagedStub: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *mut ::core::ffi::c_void, fdist: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateManagedStub: usize,
    pub DestroyManagedStub: unsafe extern "system" fn(this: *mut *mut Self, pinfo: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IManagedObjectInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetIUnknown: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIObjectControl: unsafe extern "system" fn(this: *mut *mut Self, pctrl: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetInPool: unsafe extern "system" fn(this: *mut *mut Self, binpool: super::super::Foundation::BOOL, ppooledobj: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetInPool: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWrapperStrength: unsafe extern "system" fn(this: *mut *mut Self, bstrong: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWrapperStrength: usize,
}
#[repr(C)]
pub struct IManagedPoolAction {
    pub base__: ::windows_sys::core::IUnknown,
    pub LastRelease: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IManagedPooledObj {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub SetHeld: unsafe extern "system" fn(this: *mut *mut Self, m_bheld: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetHeld: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMessageMover {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub SourcePath: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SourcePath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSourcePath: unsafe extern "system" fn(this: *mut *mut Self, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSourcePath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DestPath: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DestPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDestPath: unsafe extern "system" fn(this: *mut *mut Self, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDestPath: usize,
    pub CommitBatchSize: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetCommitBatchSize: unsafe extern "system" fn(this: *mut *mut Self, newval: i32) -> ::windows_sys::core::HRESULT,
    pub MoveMessages: unsafe extern "system" fn(this: *mut *mut Self, plmessagesmoved: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMtsEventInfo {
    pub base__: super::Com::IDispatch,
    pub Names: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, sdisplayname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EventID: unsafe extern "system" fn(this: *mut *mut Self, sguideventid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EventID: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, lcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Value: unsafe extern "system" fn(this: *mut *mut Self, skey: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Value: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMtsEvents {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub PackageName: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PackageName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PackageGuid: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PackageGuid: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PostEvent: unsafe extern "system" fn(this: *mut *mut Self, vevent: *const super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PostEvent: usize,
    pub FireEvents: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub GetProcessID: unsafe extern "system" fn(this: *mut *mut Self, id: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMtsGrp {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, lindex: i32, ppunkdispatcher: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IObjPool {
    pub base__: ::windows_sys::core::IUnknown,
    pub Reserved1: unsafe extern "system" fn(this: *mut *mut Self),
    pub Reserved2: unsafe extern "system" fn(this: *mut *mut Self),
    pub Reserved3: unsafe extern "system" fn(this: *mut *mut Self),
    pub Reserved4: unsafe extern "system" fn(this: *mut *mut Self),
    pub PutEndTx: unsafe extern "system" fn(this: *mut *mut Self, pobj: *mut ::core::ffi::c_void),
    pub Reserved5: unsafe extern "system" fn(this: *mut *mut Self),
    pub Reserved6: unsafe extern "system" fn(this: *mut *mut Self),
}
#[repr(C)]
pub struct IObjectConstruct {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub Construct: unsafe extern "system" fn(this: *mut *mut Self, pctorobj: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Construct: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IObjectConstructString {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub ConstructString: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConstructString: usize,
}
#[repr(C)]
pub struct IObjectContext {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetComplete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetAbort: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EnableCommit: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub DisableCommit: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsInTransaction: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsInTransaction: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSecurityEnabled: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSecurityEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCallerInRole: unsafe extern "system" fn(this: *mut *mut Self, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCallerInRole: usize,
}
#[repr(C)]
pub struct IObjectContextActivity {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetActivityId: unsafe extern "system" fn(this: *mut *mut Self, pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IObjectContextInfo {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub IsInTransaction: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsInTransaction: usize,
    pub GetTransaction: unsafe extern "system" fn(this: *mut *mut Self, pptrans: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetTransactionId: unsafe extern "system" fn(this: *mut *mut Self, pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetActivityId: unsafe extern "system" fn(this: *mut *mut Self, pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetContextId: unsafe extern "system" fn(this: *mut *mut Self, pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IObjectContextInfo2 {
    pub base__: IObjectContextInfo,
    pub GetPartitionId: unsafe extern "system" fn(this: *mut *mut Self, pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetApplicationId: unsafe extern "system" fn(this: *mut *mut Self, pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetApplicationInstanceId: unsafe extern "system" fn(this: *mut *mut Self, pguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IObjectContextTip {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTipUrl: unsafe extern "system" fn(this: *mut *mut Self, ptipurl: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTipUrl: usize,
}
#[repr(C)]
pub struct IObjectControl {
    pub base__: ::windows_sys::core::IUnknown,
    pub Activate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(this: *mut *mut Self),
    #[cfg(feature = "Win32_Foundation")]
    pub CanBePooled: unsafe extern "system" fn(this: *mut *mut Self) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanBePooled: usize,
}
#[repr(C)]
pub struct IPlaybackControl {
    pub base__: ::windows_sys::core::IUnknown,
    pub FinalClientRetry: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub FinalServerRetry: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPoolManager {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub ShutdownPool: unsafe extern "system" fn(this: *mut *mut Self, clsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShutdownPool: usize,
}
#[repr(C)]
pub struct IProcessInitializer {
    pub base__: ::windows_sys::core::IUnknown,
    pub Startup: unsafe extern "system" fn(this: *mut *mut Self, punkprocesscontrol: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISecurityCallContext {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pitem: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCallerInRole: unsafe extern "system" fn(this: *mut *mut Self, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfinrole: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCallerInRole: usize,
    pub IsSecurityEnabled: unsafe extern "system" fn(this: *mut *mut Self, pfisenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub IsUserInRole: unsafe extern "system" fn(this: *mut *mut Self, puser: *const super::Com::VARIANT, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfinrole: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    IsUserInRole: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISecurityCallersColl {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, lindex: i32, pobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISecurityIdentityColl {
    pub base__: super::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pitem: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISecurityProperty {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDirectCreatorSID: unsafe extern "system" fn(this: *mut *mut Self, psid: *mut super::super::Foundation::PSID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDirectCreatorSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOriginalCreatorSID: unsafe extern "system" fn(this: *mut *mut Self, psid: *mut super::super::Foundation::PSID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOriginalCreatorSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDirectCallerSID: unsafe extern "system" fn(this: *mut *mut Self, psid: *mut super::super::Foundation::PSID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDirectCallerSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOriginalCallerSID: unsafe extern "system" fn(this: *mut *mut Self, psid: *mut super::super::Foundation::PSID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOriginalCallerSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReleaseSID: unsafe extern "system" fn(this: *mut *mut Self, psid: super::super::Foundation::PSID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReleaseSID: usize,
}
#[repr(C)]
pub struct ISelectCOMLBServer {
    pub base__: ::windows_sys::core::IUnknown,
    pub Init: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetLBServer: unsafe extern "system" fn(this: *mut *mut Self, punk: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISendMethodEvents {
    pub base__: ::windows_sys::core::IUnknown,
    pub SendMethodCall: unsafe extern "system" fn(this: *mut *mut Self, pidentity: *const ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, dwmeth: u32) -> ::windows_sys::core::HRESULT,
    pub SendMethodReturn: unsafe extern "system" fn(this: *mut *mut Self, pidentity: *const ::core::ffi::c_void, riid: *const ::windows_sys::core::GUID, dwmeth: u32, hrcall: ::windows_sys::core::HRESULT, hrserver: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IServiceActivity {
    pub base__: ::windows_sys::core::IUnknown,
    pub SynchronousCall: unsafe extern "system" fn(this: *mut *mut Self, piservicecall: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AsynchronousCall: unsafe extern "system" fn(this: *mut *mut Self, piservicecall: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BindToCurrentThread: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub UnbindFromThread: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IServiceCall {
    pub base__: ::windows_sys::core::IUnknown,
    pub OnCall: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IServiceComTIIntrinsicsConfig {
    pub base__: ::windows_sys::core::IUnknown,
    pub ComTIIntrinsicsConfig: unsafe extern "system" fn(this: *mut *mut Self, comtiintrinsicsconfig: CSC_COMTIIntrinsicsConfig) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IServiceIISIntrinsicsConfig {
    pub base__: ::windows_sys::core::IUnknown,
    pub IISIntrinsicsConfig: unsafe extern "system" fn(this: *mut *mut Self, iisintrinsicsconfig: CSC_IISIntrinsicsConfig) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IServiceInheritanceConfig {
    pub base__: ::windows_sys::core::IUnknown,
    pub ContainingContextTreatment: unsafe extern "system" fn(this: *mut *mut Self, inheritanceconfig: CSC_InheritanceConfig) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IServicePartitionConfig {
    pub base__: ::windows_sys::core::IUnknown,
    pub PartitionConfig: unsafe extern "system" fn(this: *mut *mut Self, partitionconfig: CSC_PartitionConfig) -> ::windows_sys::core::HRESULT,
    pub PartitionID: unsafe extern "system" fn(this: *mut *mut Self, guidpartitionid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IServicePool {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, ppoolconfig: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetObject: unsafe extern "system" fn(this: *mut *mut Self, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IServicePoolConfig {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetMaxPoolSize: unsafe extern "system" fn(this: *mut *mut Self, dwmaxpool: u32) -> ::windows_sys::core::HRESULT,
    pub MaxPoolSize: unsafe extern "system" fn(this: *mut *mut Self, pdwmaxpool: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetMinPoolSize: unsafe extern "system" fn(this: *mut *mut Self, dwminpool: u32) -> ::windows_sys::core::HRESULT,
    pub MinPoolSize: unsafe extern "system" fn(this: *mut *mut Self, pdwminpool: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetCreationTimeout: unsafe extern "system" fn(this: *mut *mut Self, dwcreationtimeout: u32) -> ::windows_sys::core::HRESULT,
    pub CreationTimeout: unsafe extern "system" fn(this: *mut *mut Self, pdwcreationtimeout: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTransactionAffinity: unsafe extern "system" fn(this: *mut *mut Self, ftxaffinity: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTransactionAffinity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TransactionAffinity: unsafe extern "system" fn(this: *mut *mut Self, pftxaffinity: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TransactionAffinity: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetClassFactory: unsafe extern "system" fn(this: *mut *mut Self, pfactory: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetClassFactory: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ClassFactory: unsafe extern "system" fn(this: *mut *mut Self, pfactory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ClassFactory: usize,
}
#[repr(C)]
pub struct IServiceSxsConfig {
    pub base__: ::windows_sys::core::IUnknown,
    pub SxsConfig: unsafe extern "system" fn(this: *mut *mut Self, scsconfig: CSC_SxsConfig) -> ::windows_sys::core::HRESULT,
    pub SxsName: unsafe extern "system" fn(this: *mut *mut Self, szsxsname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub SxsDirectory: unsafe extern "system" fn(this: *mut *mut Self, szsxsdirectory: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IServiceSynchronizationConfig {
    pub base__: ::windows_sys::core::IUnknown,
    pub ConfigureSynchronization: unsafe extern "system" fn(this: *mut *mut Self, synchconfig: CSC_SynchronizationConfig) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IServiceSysTxnConfig {
    pub base__: IServiceTransactionConfig,
    pub ConfigureBYOTSysTxn: unsafe extern "system" fn(this: *mut *mut Self, ptxproxy: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IServiceThreadPoolConfig {
    pub base__: ::windows_sys::core::IUnknown,
    pub SelectThreadPool: unsafe extern "system" fn(this: *mut *mut Self, threadpool: CSC_ThreadPool) -> ::windows_sys::core::HRESULT,
    pub SetBindingInfo: unsafe extern "system" fn(this: *mut *mut Self, binding: CSC_Binding) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IServiceTrackerConfig {
    pub base__: ::windows_sys::core::IUnknown,
    pub TrackerConfig: unsafe extern "system" fn(this: *mut *mut Self, trackerconfig: CSC_TrackerConfig, sztrackerappname: ::windows_sys::core::PCWSTR, sztrackerctxname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IServiceTransactionConfig {
    pub base__: IServiceTransactionConfigBase,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub ConfigureBYOT: unsafe extern "system" fn(this: *mut *mut Self, pitxbyot: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    ConfigureBYOT: usize,
}
#[repr(C)]
pub struct IServiceTransactionConfigBase {
    pub base__: ::windows_sys::core::IUnknown,
    pub ConfigureTransaction: unsafe extern "system" fn(this: *mut *mut Self, transactionconfig: CSC_TransactionConfig) -> ::windows_sys::core::HRESULT,
    pub IsolationLevel: unsafe extern "system" fn(this: *mut *mut Self, option: COMAdminTxIsolationLevelOptions) -> ::windows_sys::core::HRESULT,
    pub TransactionTimeout: unsafe extern "system" fn(this: *mut *mut Self, ultimeoutsec: u32) -> ::windows_sys::core::HRESULT,
    pub BringYourOwnTransaction: unsafe extern "system" fn(this: *mut *mut Self, sztipurl: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub NewTransactionDescription: unsafe extern "system" fn(this: *mut *mut Self, sztxdesc: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISharedProperty {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, val: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISharedPropertyGroup {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePropertyByPosition: unsafe extern "system" fn(this: *mut *mut Self, index: i32, fexists: *mut i16, ppprop: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePropertyByPosition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_PropertyByPosition: unsafe extern "system" fn(this: *mut *mut Self, index: i32, ppproperty: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_PropertyByPosition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateProperty: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fexists: *mut i16, ppprop: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub get_Property: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppproperty: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    get_Property: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISharedPropertyGroupManager {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreatePropertyGroup: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwisomode: *mut i32, dwrelmode: *mut i32, fexists: *mut i16, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreatePropertyGroup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub get_Group: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    get_Group: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, retval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISystemAppEventData {
    pub base__: ::windows_sys::core::IUnknown,
    pub Startup: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnDataChanged: unsafe extern "system" fn(this: *mut *mut Self, dwpid: u32, dwmask: u32, dwnumbersinks: u32, bstrdwmethodmask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwreason: u32, u64tracehandle: u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnDataChanged: usize,
}
#[repr(C)]
pub struct IThreadPoolKnobs {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetMaxThreads: unsafe extern "system" fn(this: *mut *mut Self, plcmaxthreads: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetCurrentThreads: unsafe extern "system" fn(this: *mut *mut Self, plccurrentthreads: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxThreads: unsafe extern "system" fn(this: *mut *mut Self, lcmaxthreads: i32) -> ::windows_sys::core::HRESULT,
    pub GetDeleteDelay: unsafe extern "system" fn(this: *mut *mut Self, pmsecdeletedelay: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDeleteDelay: unsafe extern "system" fn(this: *mut *mut Self, msecdeletedelay: i32) -> ::windows_sys::core::HRESULT,
    pub GetMaxQueuedRequests: unsafe extern "system" fn(this: *mut *mut Self, plcmaxqueuedrequests: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetCurrentQueuedRequests: unsafe extern "system" fn(this: *mut *mut Self, plccurrentqueuedrequests: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxQueuedRequests: unsafe extern "system" fn(this: *mut *mut Self, lcmaxqueuedrequests: i32) -> ::windows_sys::core::HRESULT,
    pub SetMinThreads: unsafe extern "system" fn(this: *mut *mut Self, lcminthreads: i32) -> ::windows_sys::core::HRESULT,
    pub SetQueueDepth: unsafe extern "system" fn(this: *mut *mut Self, lcqueuedepth: i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITransactionContext {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, pszprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pobject: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateInstance: usize,
    pub Commit: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITransactionContextEx {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITransactionProperty {
    pub base__: ::windows_sys::core::IUnknown,
    pub Reserved1: unsafe extern "system" fn(this: *mut *mut Self),
    pub Reserved2: unsafe extern "system" fn(this: *mut *mut Self),
    pub Reserved3: unsafe extern "system" fn(this: *mut *mut Self),
    pub Reserved4: unsafe extern "system" fn(this: *mut *mut Self),
    pub Reserved5: unsafe extern "system" fn(this: *mut *mut Self),
    pub Reserved6: unsafe extern "system" fn(this: *mut *mut Self),
    pub Reserved7: unsafe extern "system" fn(this: *mut *mut Self),
    pub Reserved8: unsafe extern "system" fn(this: *mut *mut Self),
    pub Reserved9: unsafe extern "system" fn(this: *mut *mut Self),
    pub GetTransactionResourcePool: unsafe extern "system" fn(this: *mut *mut Self, pptxpool: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Reserved10: unsafe extern "system" fn(this: *mut *mut Self),
    pub Reserved11: unsafe extern "system" fn(this: *mut *mut Self),
    pub Reserved12: unsafe extern "system" fn(this: *mut *mut Self),
    pub Reserved13: unsafe extern "system" fn(this: *mut *mut Self),
    pub Reserved14: unsafe extern "system" fn(this: *mut *mut Self),
    pub Reserved15: unsafe extern "system" fn(this: *mut *mut Self),
    pub Reserved16: unsafe extern "system" fn(this: *mut *mut Self),
    pub Reserved17: unsafe extern "system" fn(this: *mut *mut Self),
}
#[repr(C)]
pub struct ITransactionProxy {
    pub base__: ::windows_sys::core::IUnknown,
    pub Commit: unsafe extern "system" fn(this: *mut *mut Self, guid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub Promote: unsafe extern "system" fn(this: *mut *mut Self, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    Promote: usize,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub CreateVoter: unsafe extern "system" fn(this: *mut *mut Self, ptxasync: *mut ::core::ffi::c_void, ppballot: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    CreateVoter: usize,
    pub GetIsolationLevel: unsafe extern "system" fn(this: *mut *mut Self, __midl__itransactionproxy0000: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetIdentifier: unsafe extern "system" fn(this: *mut *mut Self, pbstridentifier: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsReusable: unsafe extern "system" fn(this: *mut *mut Self, pfisreusable: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsReusable: usize,
}
#[repr(C)]
pub struct ITransactionResourcePool {
    pub base__: ::windows_sys::core::IUnknown,
    pub PutResource: unsafe extern "system" fn(this: *mut *mut Self, ppool: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetResource: unsafe extern "system" fn(this: *mut *mut Self, ppool: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITransactionStatus {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetTransactionStatus: unsafe extern "system" fn(this: *mut *mut Self, hrstatus: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub GetTransactionStatus: unsafe extern "system" fn(this: *mut *mut Self, phrstatus: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITxProxyHolder {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetIdentifier: unsafe extern "system" fn(this: *mut *mut Self, pguidltx: *mut ::windows_sys::core::GUID),
}
pub const LBEvents: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674881, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type LockModes = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const LockSetGet: LockModes = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const LockMethod: LockModes = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const MTXDM_E_ENLISTRESOURCEFAILED: u32 = 2147803392u32;
pub const MessageMover: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674879, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub const MtsGrp: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1261344141, data2: 915, data3: 4561, data4: [177, 171, 0, 170, 0, 186, 50, 88] };
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ObjectContext {
    pub base__: super::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, bstrprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pobject: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateInstance: usize,
    pub SetComplete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetAbort: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EnableCommit: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub DisableCommit: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub IsInTransaction: unsafe extern "system" fn(this: *mut *mut Self, pbisintx: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsSecurityEnabled: unsafe extern "system" fn(this: *mut *mut Self, pbisenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCallerInRole: unsafe extern "system" fn(this: *mut *mut Self, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbinrole: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCallerInRole: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pitem: *mut super::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Security: unsafe extern "system" fn(this: *mut *mut Self, ppsecurityproperty: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ContextInfo: unsafe extern "system" fn(this: *mut *mut Self, ppcontextinfo: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ContextInfo: usize,
}
#[repr(C)]
pub struct ObjectControl {
    pub base__: ::windows_sys::core::IUnknown,
    pub Activate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub CanBePooled: unsafe extern "system" fn(this: *mut *mut Self, pbpoolable: *mut i16) -> ::windows_sys::core::HRESULT,
}
pub const PoolMgr: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674613, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct RECYCLE_INFO {
    pub guidCombaseProcessIdentifier: ::windows_sys::core::GUID,
    pub ProcessStartTime: i64,
    pub dwRecycleLifetimeLimit: u32,
    pub dwRecycleMemoryLimit: u32,
    pub dwRecycleExpirationTimeout: u32,
}
impl ::core::marker::Copy for RECYCLE_INFO {}
impl ::core::clone::Clone for RECYCLE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type ReleaseModes = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const Standard: ReleaseModes = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const Process: ReleaseModes = 1i32;
pub const SecurityCallContext: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674855, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub const SecurityCallers: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674854, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub const SecurityIdentity: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674853, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct SecurityProperty {
    pub base__: super::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDirectCallerName: unsafe extern "system" fn(this: *mut *mut Self, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDirectCallerName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDirectCreatorName: unsafe extern "system" fn(this: *mut *mut Self, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDirectCreatorName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOriginalCallerName: unsafe extern "system" fn(this: *mut *mut Self, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOriginalCallerName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOriginalCreatorName: unsafe extern "system" fn(this: *mut *mut Self, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOriginalCreatorName: usize,
}
pub const ServicePool: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674889, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub const ServicePoolConfig: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674890, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub const SharedProperty: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 704666629, data2: 42462, data3: 4559, data4: [158, 102, 0, 170, 0, 163, 244, 100] };
pub const SharedPropertyGroup: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 704666635, data2: 42462, data3: 4559, data4: [158, 102, 0, 170, 0, 163, 244, 100] };
pub const SharedPropertyGroupManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 704666641, data2: 42462, data3: 4559, data4: [158, 102, 0, 170, 0, 163, 244, 100] };
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TRACKER_INIT_EVENT: &str = "Global\\COM+ Tracker Init Event";
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TRACKER_STARTSTOP_EVENT: &str = "Global\\COM+ Tracker Push Event";
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type TRACKING_COLL_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TRKCOLL_PROCESSES: TRACKING_COLL_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TRKCOLL_APPLICATIONS: TRACKING_COLL_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TRKCOLL_COMPONENTS: TRACKING_COLL_TYPE = 2i32;
pub const TrackerServer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3970674617, data2: 32537, data3: 4562, data4: [151, 142, 0, 0, 248, 117, 126, 42] };
pub const TransactionContext: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2040134693, data2: 54214, data3: 4559, data4: [172, 171, 0, 160, 36, 165, 90, 239] };
pub const TransactionContextEx: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1555457648, data2: 54228, data3: 4559, data4: [172, 171, 0, 160, 36, 165, 90, 239] };
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type TransactionVote = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TxCommit: TransactionVote = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TxAbort: TransactionVote = 1i32;
