pub const AppDomainHelper: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef24f689_14f8_4d92_b4af_d7b1f0e70fd4);
#[repr(C)]
pub struct ApplicationProcessRecycleInfo {
    pub IsRecyclable: ::win32_foundation::BOOL,
    pub IsRecycled: ::win32_foundation::BOOL,
    pub TimeRecycled: ::win32_foundation::FILETIME,
    pub TimeToTerminate: ::win32_foundation::FILETIME,
    pub RecycleReasonCode: i32,
    pub IsPendingRecycle: ::win32_foundation::BOOL,
    pub HasAutomaticLifetimeRecycling: ::win32_foundation::BOOL,
    pub TimeForAutomaticRecycling: ::win32_foundation::FILETIME,
    pub MemoryLimitInKB: u32,
    pub MemoryUsageInKBLastCheck: u32,
    pub ActivationLimit: u32,
    pub NumActivationsLastReported: u32,
    pub CallLimit: u32,
    pub NumCallsLastReported: u32,
}
impl ::core::marker::Copy for ApplicationProcessRecycleInfo {}
impl ::core::clone::Clone for ApplicationProcessRecycleInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ApplicationProcessRecycleInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ApplicationProcessRecycleInfo")
            .field("IsRecyclable", &self.IsRecyclable)
            .field("IsRecycled", &self.IsRecycled)
            .field("TimeRecycled", &self.TimeRecycled)
            .field("TimeToTerminate", &self.TimeToTerminate)
            .field("RecycleReasonCode", &self.RecycleReasonCode)
            .field("IsPendingRecycle", &self.IsPendingRecycle)
            .field("HasAutomaticLifetimeRecycling", &self.HasAutomaticLifetimeRecycling)
            .field("TimeForAutomaticRecycling", &self.TimeForAutomaticRecycling)
            .field("MemoryLimitInKB", &self.MemoryLimitInKB)
            .field("MemoryUsageInKBLastCheck", &self.MemoryUsageInKBLastCheck)
            .field("ActivationLimit", &self.ActivationLimit)
            .field("NumActivationsLastReported", &self.NumActivationsLastReported)
            .field("CallLimit", &self.CallLimit)
            .field("NumCallsLastReported", &self.NumCallsLastReported)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for ApplicationProcessRecycleInfo {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ApplicationProcessRecycleInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ApplicationProcessRecycleInfo>()) == 0 }
    }
}
impl ::core::cmp::Eq for ApplicationProcessRecycleInfo {}
impl ::core::default::Default for ApplicationProcessRecycleInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl ::core::fmt::Debug for ApplicationProcessStatistics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ApplicationProcessStatistics").field("NumCallsOutstanding", &self.NumCallsOutstanding).field("NumTrackedComponents", &self.NumTrackedComponents).field("NumComponentInstances", &self.NumComponentInstances).field("AvgCallsPerSecond", &self.AvgCallsPerSecond).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("Reserved3", &self.Reserved3).field("Reserved4", &self.Reserved4).finish()
    }
}
unsafe impl ::windows_core::Abi for ApplicationProcessStatistics {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ApplicationProcessStatistics {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ApplicationProcessStatistics>()) == 0 }
    }
}
impl ::core::cmp::Eq for ApplicationProcessStatistics {}
impl ::core::default::Default for ApplicationProcessStatistics {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ApplicationProcessSummary {
    pub PartitionIdPrimaryApplication: ::windows_core::GUID,
    pub ApplicationIdPrimaryApplication: ::windows_core::GUID,
    pub ApplicationInstanceId: ::windows_core::GUID,
    pub ProcessId: u32,
    pub Type: COMPLUS_APPTYPE,
    pub ProcessExeName: ::windows_core::PWSTR,
    pub IsService: ::win32_foundation::BOOL,
    pub IsPaused: ::win32_foundation::BOOL,
    pub IsRecycled: ::win32_foundation::BOOL,
}
impl ::core::marker::Copy for ApplicationProcessSummary {}
impl ::core::clone::Clone for ApplicationProcessSummary {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ApplicationProcessSummary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ApplicationProcessSummary")
            .field("PartitionIdPrimaryApplication", &self.PartitionIdPrimaryApplication)
            .field("ApplicationIdPrimaryApplication", &self.ApplicationIdPrimaryApplication)
            .field("ApplicationInstanceId", &self.ApplicationInstanceId)
            .field("ProcessId", &self.ProcessId)
            .field("Type", &self.Type)
            .field("ProcessExeName", &self.ProcessExeName)
            .field("IsService", &self.IsService)
            .field("IsPaused", &self.IsPaused)
            .field("IsRecycled", &self.IsRecycled)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for ApplicationProcessSummary {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ApplicationProcessSummary {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ApplicationProcessSummary>()) == 0 }
    }
}
impl ::core::cmp::Eq for ApplicationProcessSummary {}
impl ::core::default::Default for ApplicationProcessSummary {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ApplicationSummary {
    pub ApplicationInstanceId: ::windows_core::GUID,
    pub PartitionId: ::windows_core::GUID,
    pub ApplicationId: ::windows_core::GUID,
    pub Type: COMPLUS_APPTYPE,
    pub ApplicationName: ::windows_core::PWSTR,
    pub NumTrackedComponents: u32,
    pub NumComponentInstances: u32,
}
impl ::core::marker::Copy for ApplicationSummary {}
impl ::core::clone::Clone for ApplicationSummary {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ApplicationSummary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ApplicationSummary").field("ApplicationInstanceId", &self.ApplicationInstanceId).field("PartitionId", &self.PartitionId).field("ApplicationId", &self.ApplicationId).field("Type", &self.Type).field("ApplicationName", &self.ApplicationName).field("NumTrackedComponents", &self.NumTrackedComponents).field("NumComponentInstances", &self.NumComponentInstances).finish()
    }
}
unsafe impl ::windows_core::Abi for ApplicationSummary {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ApplicationSummary {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ApplicationSummary>()) == 0 }
    }
}
impl ::core::cmp::Eq for ApplicationSummary {}
impl ::core::default::Default for ApplicationSummary {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutoSvcs_Error_Constants(pub u32);
pub const mtsErrCtxAborted: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803138u32);
pub const mtsErrCtxAborting: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803139u32);
pub const mtsErrCtxNoContext: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803140u32);
pub const mtsErrCtxNotRegistered: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803141u32);
pub const mtsErrCtxSynchTimeout: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803142u32);
pub const mtsErrCtxOldReference: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803143u32);
pub const mtsErrCtxRoleNotFound: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803148u32);
pub const mtsErrCtxNoSecurity: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803149u32);
pub const mtsErrCtxWrongThread: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803150u32);
pub const mtsErrCtxTMNotAvailable: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803151u32);
pub const comQCErrApplicationNotQueued: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599296u32);
pub const comQCErrNoQueueableInterfaces: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599297u32);
pub const comQCErrQueuingServiceNotAvailable: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599298u32);
pub const comQCErrQueueTransactMismatch: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599299u32);
pub const comqcErrRecorderMarshalled: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599300u32);
pub const comqcErrOutParam: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599301u32);
pub const comqcErrRecorderNotTrusted: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599302u32);
pub const comqcErrPSLoad: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599303u32);
pub const comqcErrMarshaledObjSameTxn: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599304u32);
pub const comqcErrInvalidMessage: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599376u32);
pub const comqcErrMsmqSidUnavailable: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599377u32);
pub const comqcErrWrongMsgExtension: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599378u32);
pub const comqcErrMsmqServiceUnavailable: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599379u32);
pub const comqcErrMsgNotAuthenticated: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599380u32);
pub const comqcErrMsmqConnectorUsed: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599381u32);
pub const comqcErrBadMarshaledObject: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599382u32);
impl ::core::marker::Copy for AutoSvcs_Error_Constants {}
impl ::core::clone::Clone for AutoSvcs_Error_Constants {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutoSvcs_Error_Constants {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AutoSvcs_Error_Constants {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutoSvcs_Error_Constants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutoSvcs_Error_Constants").field(&self.0).finish()
    }
}
pub const ByotServerEx: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecabb0aa_7f19_11d2_978e_0000f8757e2a);
#[repr(C)]
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
impl ::core::fmt::Debug for CAppData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAppData").field("m_idApp", &self.m_idApp).field("m_szAppGuid", &self.m_szAppGuid).field("m_dwAppProcessId", &self.m_dwAppProcessId).field("m_AppStatistics", &self.m_AppStatistics).finish()
    }
}
unsafe impl ::windows_core::Abi for CAppData {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CAppData {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CAppData>()) == 0 }
    }
}
impl ::core::cmp::Eq for CAppData {}
impl ::core::default::Default for CAppData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl ::core::fmt::Debug for CAppStatistics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAppStatistics").field("m_cTotalCalls", &self.m_cTotalCalls).field("m_cTotalInstances", &self.m_cTotalInstances).field("m_cTotalClasses", &self.m_cTotalClasses).field("m_cCallsPerSecond", &self.m_cCallsPerSecond).finish()
    }
}
unsafe impl ::windows_core::Abi for CAppStatistics {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CAppStatistics {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CAppStatistics>()) == 0 }
    }
}
impl ::core::cmp::Eq for CAppStatistics {}
impl ::core::default::Default for CAppStatistics {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CCLSIDData {
    pub m_clsid: ::windows_core::GUID,
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
impl ::core::fmt::Debug for CCLSIDData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CCLSIDData").field("m_clsid", &self.m_clsid).field("m_cReferences", &self.m_cReferences).field("m_cBound", &self.m_cBound).field("m_cPooled", &self.m_cPooled).field("m_cInCall", &self.m_cInCall).field("m_dwRespTime", &self.m_dwRespTime).field("m_cCallsCompleted", &self.m_cCallsCompleted).field("m_cCallsFailed", &self.m_cCallsFailed).finish()
    }
}
unsafe impl ::windows_core::Abi for CCLSIDData {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CCLSIDData {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CCLSIDData>()) == 0 }
    }
}
impl ::core::cmp::Eq for CCLSIDData {}
impl ::core::default::Default for CCLSIDData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CCLSIDData2 {
    pub m_clsid: ::windows_core::GUID,
    pub m_appid: ::windows_core::GUID,
    pub m_partid: ::windows_core::GUID,
    pub m_pwszAppName: ::windows_core::PWSTR,
    pub m_pwszCtxName: ::windows_core::PWSTR,
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
impl ::core::fmt::Debug for CCLSIDData2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CCLSIDData2")
            .field("m_clsid", &self.m_clsid)
            .field("m_appid", &self.m_appid)
            .field("m_partid", &self.m_partid)
            .field("m_pwszAppName", &self.m_pwszAppName)
            .field("m_pwszCtxName", &self.m_pwszCtxName)
            .field("m_eAppType", &self.m_eAppType)
            .field("m_cReferences", &self.m_cReferences)
            .field("m_cBound", &self.m_cBound)
            .field("m_cPooled", &self.m_cPooled)
            .field("m_cInCall", &self.m_cInCall)
            .field("m_dwRespTime", &self.m_dwRespTime)
            .field("m_cCallsCompleted", &self.m_cCallsCompleted)
            .field("m_cCallsFailed", &self.m_cCallsFailed)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for CCLSIDData2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CCLSIDData2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CCLSIDData2>()) == 0 }
    }
}
impl ::core::cmp::Eq for CCLSIDData2 {}
impl ::core::default::Default for CCLSIDData2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMAdminAccessChecksLevelOptions(pub i32);
pub const COMAdminAccessChecksApplicationLevel: COMAdminAccessChecksLevelOptions = COMAdminAccessChecksLevelOptions(0i32);
pub const COMAdminAccessChecksApplicationComponentLevel: COMAdminAccessChecksLevelOptions = COMAdminAccessChecksLevelOptions(1i32);
impl ::core::marker::Copy for COMAdminAccessChecksLevelOptions {}
impl ::core::clone::Clone for COMAdminAccessChecksLevelOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminAccessChecksLevelOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COMAdminAccessChecksLevelOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminAccessChecksLevelOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminAccessChecksLevelOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMAdminActivationOptions(pub i32);
pub const COMAdminActivationInproc: COMAdminActivationOptions = COMAdminActivationOptions(0i32);
pub const COMAdminActivationLocal: COMAdminActivationOptions = COMAdminActivationOptions(1i32);
impl ::core::marker::Copy for COMAdminActivationOptions {}
impl ::core::clone::Clone for COMAdminActivationOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminActivationOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COMAdminActivationOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminActivationOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminActivationOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMAdminApplicationExportOptions(pub i32);
pub const COMAdminExportNoUsers: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(0i32);
pub const COMAdminExportUsers: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(1i32);
pub const COMAdminExportApplicationProxy: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(2i32);
pub const COMAdminExportForceOverwriteOfFiles: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(4i32);
pub const COMAdminExportIn10Format: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(16i32);
impl ::core::marker::Copy for COMAdminApplicationExportOptions {}
impl ::core::clone::Clone for COMAdminApplicationExportOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminApplicationExportOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COMAdminApplicationExportOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminApplicationExportOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminApplicationExportOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMAdminApplicationInstallOptions(pub i32);
pub const COMAdminInstallNoUsers: COMAdminApplicationInstallOptions = COMAdminApplicationInstallOptions(0i32);
pub const COMAdminInstallUsers: COMAdminApplicationInstallOptions = COMAdminApplicationInstallOptions(1i32);
pub const COMAdminInstallForceOverwriteOfFiles: COMAdminApplicationInstallOptions = COMAdminApplicationInstallOptions(2i32);
impl ::core::marker::Copy for COMAdminApplicationInstallOptions {}
impl ::core::clone::Clone for COMAdminApplicationInstallOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminApplicationInstallOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COMAdminApplicationInstallOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminApplicationInstallOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminApplicationInstallOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMAdminAuthenticationCapabilitiesOptions(pub i32);
pub const COMAdminAuthenticationCapabilitiesNone: COMAdminAuthenticationCapabilitiesOptions = COMAdminAuthenticationCapabilitiesOptions(0i32);
pub const COMAdminAuthenticationCapabilitiesSecureReference: COMAdminAuthenticationCapabilitiesOptions = COMAdminAuthenticationCapabilitiesOptions(2i32);
pub const COMAdminAuthenticationCapabilitiesStaticCloaking: COMAdminAuthenticationCapabilitiesOptions = COMAdminAuthenticationCapabilitiesOptions(32i32);
pub const COMAdminAuthenticationCapabilitiesDynamicCloaking: COMAdminAuthenticationCapabilitiesOptions = COMAdminAuthenticationCapabilitiesOptions(64i32);
impl ::core::marker::Copy for COMAdminAuthenticationCapabilitiesOptions {}
impl ::core::clone::Clone for COMAdminAuthenticationCapabilitiesOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminAuthenticationCapabilitiesOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COMAdminAuthenticationCapabilitiesOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminAuthenticationCapabilitiesOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminAuthenticationCapabilitiesOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMAdminAuthenticationLevelOptions(pub i32);
pub const COMAdminAuthenticationDefault: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(0i32);
pub const COMAdminAuthenticationNone: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(1i32);
pub const COMAdminAuthenticationConnect: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(2i32);
pub const COMAdminAuthenticationCall: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(3i32);
pub const COMAdminAuthenticationPacket: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(4i32);
pub const COMAdminAuthenticationIntegrity: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(5i32);
pub const COMAdminAuthenticationPrivacy: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(6i32);
impl ::core::marker::Copy for COMAdminAuthenticationLevelOptions {}
impl ::core::clone::Clone for COMAdminAuthenticationLevelOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminAuthenticationLevelOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COMAdminAuthenticationLevelOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminAuthenticationLevelOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminAuthenticationLevelOptions").field(&self.0).finish()
    }
}
pub const COMAdminCatalog: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf618c514_dfb8_11d1_a2cf_00805fc79235);
pub const COMAdminCatalogCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf618c516_dfb8_11d1_a2cf_00805fc79235);
pub const COMAdminCatalogObject: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf618c515_dfb8_11d1_a2cf_00805fc79235);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMAdminComponentFlags(pub i32);
pub const COMAdminCompFlagTypeInfoFound: COMAdminComponentFlags = COMAdminComponentFlags(1i32);
pub const COMAdminCompFlagCOMPlusPropertiesFound: COMAdminComponentFlags = COMAdminComponentFlags(2i32);
pub const COMAdminCompFlagProxyFound: COMAdminComponentFlags = COMAdminComponentFlags(4i32);
pub const COMAdminCompFlagInterfacesFound: COMAdminComponentFlags = COMAdminComponentFlags(8i32);
pub const COMAdminCompFlagAlreadyInstalled: COMAdminComponentFlags = COMAdminComponentFlags(16i32);
pub const COMAdminCompFlagNotInApplication: COMAdminComponentFlags = COMAdminComponentFlags(32i32);
impl ::core::marker::Copy for COMAdminComponentFlags {}
impl ::core::clone::Clone for COMAdminComponentFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminComponentFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COMAdminComponentFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminComponentFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminComponentFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMAdminComponentType(pub i32);
pub const COMAdmin32BitComponent: COMAdminComponentType = COMAdminComponentType(1i32);
pub const COMAdmin64BitComponent: COMAdminComponentType = COMAdminComponentType(2i32);
impl ::core::marker::Copy for COMAdminComponentType {}
impl ::core::clone::Clone for COMAdminComponentType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminComponentType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COMAdminComponentType {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminComponentType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminComponentType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMAdminErrorCodes(pub i32);
pub const COMAdminErrObjectErrors: COMAdminErrorCodes = COMAdminErrorCodes(-2146368511i32);
pub const COMAdminErrObjectInvalid: COMAdminErrorCodes = COMAdminErrorCodes(-2146368510i32);
pub const COMAdminErrKeyMissing: COMAdminErrorCodes = COMAdminErrorCodes(-2146368509i32);
pub const COMAdminErrAlreadyInstalled: COMAdminErrorCodes = COMAdminErrorCodes(-2146368508i32);
pub const COMAdminErrAppFileWriteFail: COMAdminErrorCodes = COMAdminErrorCodes(-2146368505i32);
pub const COMAdminErrAppFileReadFail: COMAdminErrorCodes = COMAdminErrorCodes(-2146368504i32);
pub const COMAdminErrAppFileVersion: COMAdminErrorCodes = COMAdminErrorCodes(-2146368503i32);
pub const COMAdminErrBadPath: COMAdminErrorCodes = COMAdminErrorCodes(-2146368502i32);
pub const COMAdminErrApplicationExists: COMAdminErrorCodes = COMAdminErrorCodes(-2146368501i32);
pub const COMAdminErrRoleExists: COMAdminErrorCodes = COMAdminErrorCodes(-2146368500i32);
pub const COMAdminErrCantCopyFile: COMAdminErrorCodes = COMAdminErrorCodes(-2146368499i32);
pub const COMAdminErrNoUser: COMAdminErrorCodes = COMAdminErrorCodes(-2146368497i32);
pub const COMAdminErrInvalidUserids: COMAdminErrorCodes = COMAdminErrorCodes(-2146368496i32);
pub const COMAdminErrNoRegistryCLSID: COMAdminErrorCodes = COMAdminErrorCodes(-2146368495i32);
pub const COMAdminErrBadRegistryProgID: COMAdminErrorCodes = COMAdminErrorCodes(-2146368494i32);
pub const COMAdminErrAuthenticationLevel: COMAdminErrorCodes = COMAdminErrorCodes(-2146368493i32);
pub const COMAdminErrUserPasswdNotValid: COMAdminErrorCodes = COMAdminErrorCodes(-2146368492i32);
pub const COMAdminErrCLSIDOrIIDMismatch: COMAdminErrorCodes = COMAdminErrorCodes(-2146368488i32);
pub const COMAdminErrRemoteInterface: COMAdminErrorCodes = COMAdminErrorCodes(-2146368487i32);
pub const COMAdminErrDllRegisterServer: COMAdminErrorCodes = COMAdminErrorCodes(-2146368486i32);
pub const COMAdminErrNoServerShare: COMAdminErrorCodes = COMAdminErrorCodes(-2146368485i32);
pub const COMAdminErrDllLoadFailed: COMAdminErrorCodes = COMAdminErrorCodes(-2146368483i32);
pub const COMAdminErrBadRegistryLibID: COMAdminErrorCodes = COMAdminErrorCodes(-2146368482i32);
pub const COMAdminErrAppDirNotFound: COMAdminErrorCodes = COMAdminErrorCodes(-2146368481i32);
pub const COMAdminErrRegistrarFailed: COMAdminErrorCodes = COMAdminErrorCodes(-2146368477i32);
pub const COMAdminErrCompFileDoesNotExist: COMAdminErrorCodes = COMAdminErrorCodes(-2146368476i32);
pub const COMAdminErrCompFileLoadDLLFail: COMAdminErrorCodes = COMAdminErrorCodes(-2146368475i32);
pub const COMAdminErrCompFileGetClassObj: COMAdminErrorCodes = COMAdminErrorCodes(-2146368474i32);
pub const COMAdminErrCompFileClassNotAvail: COMAdminErrorCodes = COMAdminErrorCodes(-2146368473i32);
pub const COMAdminErrCompFileBadTLB: COMAdminErrorCodes = COMAdminErrorCodes(-2146368472i32);
pub const COMAdminErrCompFileNotInstallable: COMAdminErrorCodes = COMAdminErrorCodes(-2146368471i32);
pub const COMAdminErrNotChangeable: COMAdminErrorCodes = COMAdminErrorCodes(-2146368470i32);
pub const COMAdminErrNotDeletable: COMAdminErrorCodes = COMAdminErrorCodes(-2146368469i32);
pub const COMAdminErrSession: COMAdminErrorCodes = COMAdminErrorCodes(-2146368468i32);
pub const COMAdminErrCompMoveLocked: COMAdminErrorCodes = COMAdminErrorCodes(-2146368467i32);
pub const COMAdminErrCompMoveBadDest: COMAdminErrorCodes = COMAdminErrorCodes(-2146368466i32);
pub const COMAdminErrRegisterTLB: COMAdminErrorCodes = COMAdminErrorCodes(-2146368464i32);
pub const COMAdminErrSystemApp: COMAdminErrorCodes = COMAdminErrorCodes(-2146368461i32);
pub const COMAdminErrCompFileNoRegistrar: COMAdminErrorCodes = COMAdminErrorCodes(-2146368460i32);
pub const COMAdminErrCoReqCompInstalled: COMAdminErrorCodes = COMAdminErrorCodes(-2146368459i32);
pub const COMAdminErrServiceNotInstalled: COMAdminErrorCodes = COMAdminErrorCodes(-2146368458i32);
pub const COMAdminErrPropertySaveFailed: COMAdminErrorCodes = COMAdminErrorCodes(-2146368457i32);
pub const COMAdminErrObjectExists: COMAdminErrorCodes = COMAdminErrorCodes(-2146368456i32);
pub const COMAdminErrComponentExists: COMAdminErrorCodes = COMAdminErrorCodes(-2146368455i32);
pub const COMAdminErrRegFileCorrupt: COMAdminErrorCodes = COMAdminErrorCodes(-2146368453i32);
pub const COMAdminErrPropertyOverflow: COMAdminErrorCodes = COMAdminErrorCodes(-2146368452i32);
pub const COMAdminErrNotInRegistry: COMAdminErrorCodes = COMAdminErrorCodes(-2146368450i32);
pub const COMAdminErrObjectNotPoolable: COMAdminErrorCodes = COMAdminErrorCodes(-2146368449i32);
pub const COMAdminErrApplidMatchesClsid: COMAdminErrorCodes = COMAdminErrorCodes(-2146368442i32);
pub const COMAdminErrRoleDoesNotExist: COMAdminErrorCodes = COMAdminErrorCodes(-2146368441i32);
pub const COMAdminErrStartAppNeedsComponents: COMAdminErrorCodes = COMAdminErrorCodes(-2146368440i32);
pub const COMAdminErrRequiresDifferentPlatform: COMAdminErrorCodes = COMAdminErrorCodes(-2146368439i32);
pub const COMAdminErrQueuingServiceNotAvailable: COMAdminErrorCodes = COMAdminErrorCodes(-2146367998i32);
pub const COMAdminErrObjectParentMissing: COMAdminErrorCodes = COMAdminErrorCodes(-2146367480i32);
pub const COMAdminErrObjectDoesNotExist: COMAdminErrorCodes = COMAdminErrorCodes(-2146367479i32);
pub const COMAdminErrCanNotExportAppProxy: COMAdminErrorCodes = COMAdminErrorCodes(-2146368438i32);
pub const COMAdminErrCanNotStartApp: COMAdminErrorCodes = COMAdminErrorCodes(-2146368437i32);
pub const COMAdminErrCanNotExportSystemApp: COMAdminErrorCodes = COMAdminErrorCodes(-2146368436i32);
pub const COMAdminErrCanNotSubscribeToComponent: COMAdminErrorCodes = COMAdminErrorCodes(-2146368435i32);
pub const COMAdminErrAppNotRunning: COMAdminErrorCodes = COMAdminErrorCodes(-2146367478i32);
pub const COMAdminErrEventClassCannotBeSubscriber: COMAdminErrorCodes = COMAdminErrorCodes(-2146368434i32);
pub const COMAdminErrLibAppProxyIncompatible: COMAdminErrorCodes = COMAdminErrorCodes(-2146368433i32);
pub const COMAdminErrBasePartitionOnly: COMAdminErrorCodes = COMAdminErrorCodes(-2146368432i32);
pub const COMAdminErrDuplicatePartitionName: COMAdminErrorCodes = COMAdminErrorCodes(-2146368425i32);
pub const COMAdminErrPartitionInUse: COMAdminErrorCodes = COMAdminErrorCodes(-2146368423i32);
pub const COMAdminErrImportedComponentsNotAllowed: COMAdminErrorCodes = COMAdminErrorCodes(-2146368421i32);
pub const COMAdminErrRegdbNotInitialized: COMAdminErrorCodes = COMAdminErrorCodes(-2146368398i32);
pub const COMAdminErrRegdbNotOpen: COMAdminErrorCodes = COMAdminErrorCodes(-2146368397i32);
pub const COMAdminErrRegdbSystemErr: COMAdminErrorCodes = COMAdminErrorCodes(-2146368396i32);
pub const COMAdminErrRegdbAlreadyRunning: COMAdminErrorCodes = COMAdminErrorCodes(-2146368395i32);
pub const COMAdminErrMigVersionNotSupported: COMAdminErrorCodes = COMAdminErrorCodes(-2146368384i32);
pub const COMAdminErrMigSchemaNotFound: COMAdminErrorCodes = COMAdminErrorCodes(-2146368383i32);
pub const COMAdminErrCatBitnessMismatch: COMAdminErrorCodes = COMAdminErrorCodes(-2146368382i32);
pub const COMAdminErrCatUnacceptableBitness: COMAdminErrorCodes = COMAdminErrorCodes(-2146368381i32);
pub const COMAdminErrCatWrongAppBitnessBitness: COMAdminErrorCodes = COMAdminErrorCodes(-2146368380i32);
pub const COMAdminErrCatPauseResumeNotSupported: COMAdminErrorCodes = COMAdminErrorCodes(-2146368379i32);
pub const COMAdminErrCatServerFault: COMAdminErrorCodes = COMAdminErrorCodes(-2146368378i32);
pub const COMAdminErrCantRecycleLibraryApps: COMAdminErrorCodes = COMAdminErrorCodes(-2146367473i32);
pub const COMAdminErrCantRecycleServiceApps: COMAdminErrorCodes = COMAdminErrorCodes(-2146367471i32);
pub const COMAdminErrProcessAlreadyRecycled: COMAdminErrorCodes = COMAdminErrorCodes(-2146367470i32);
pub const COMAdminErrPausedProcessMayNotBeRecycled: COMAdminErrorCodes = COMAdminErrorCodes(-2146367469i32);
pub const COMAdminErrInvalidPartition: COMAdminErrorCodes = COMAdminErrorCodes(-2146367477i32);
pub const COMAdminErrPartitionMsiOnly: COMAdminErrorCodes = COMAdminErrorCodes(-2146367463i32);
pub const COMAdminErrStartAppDisabled: COMAdminErrorCodes = COMAdminErrorCodes(-2146368431i32);
pub const COMAdminErrCompMoveSource: COMAdminErrorCodes = COMAdminErrorCodes(-2146367460i32);
pub const COMAdminErrCompMoveDest: COMAdminErrorCodes = COMAdminErrorCodes(-2146367459i32);
pub const COMAdminErrCompMovePrivate: COMAdminErrorCodes = COMAdminErrorCodes(-2146367458i32);
pub const COMAdminErrCannotCopyEventClass: COMAdminErrorCodes = COMAdminErrorCodes(-2146367456i32);
impl ::core::marker::Copy for COMAdminErrorCodes {}
impl ::core::clone::Clone for COMAdminErrorCodes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminErrorCodes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COMAdminErrorCodes {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminErrorCodes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminErrorCodes").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMAdminFileFlags(pub i32);
pub const COMAdminFileFlagLoadable: COMAdminFileFlags = COMAdminFileFlags(1i32);
pub const COMAdminFileFlagCOM: COMAdminFileFlags = COMAdminFileFlags(2i32);
pub const COMAdminFileFlagContainsPS: COMAdminFileFlags = COMAdminFileFlags(4i32);
pub const COMAdminFileFlagContainsComp: COMAdminFileFlags = COMAdminFileFlags(8i32);
pub const COMAdminFileFlagContainsTLB: COMAdminFileFlags = COMAdminFileFlags(16i32);
pub const COMAdminFileFlagSelfReg: COMAdminFileFlags = COMAdminFileFlags(32i32);
pub const COMAdminFileFlagSelfUnReg: COMAdminFileFlags = COMAdminFileFlags(64i32);
pub const COMAdminFileFlagUnloadableDLL: COMAdminFileFlags = COMAdminFileFlags(128i32);
pub const COMAdminFileFlagDoesNotExist: COMAdminFileFlags = COMAdminFileFlags(256i32);
pub const COMAdminFileFlagAlreadyInstalled: COMAdminFileFlags = COMAdminFileFlags(512i32);
pub const COMAdminFileFlagBadTLB: COMAdminFileFlags = COMAdminFileFlags(1024i32);
pub const COMAdminFileFlagGetClassObjFailed: COMAdminFileFlags = COMAdminFileFlags(2048i32);
pub const COMAdminFileFlagClassNotAvailable: COMAdminFileFlags = COMAdminFileFlags(4096i32);
pub const COMAdminFileFlagRegistrar: COMAdminFileFlags = COMAdminFileFlags(8192i32);
pub const COMAdminFileFlagNoRegistrar: COMAdminFileFlags = COMAdminFileFlags(16384i32);
pub const COMAdminFileFlagDLLRegsvrFailed: COMAdminFileFlags = COMAdminFileFlags(32768i32);
pub const COMAdminFileFlagRegTLBFailed: COMAdminFileFlags = COMAdminFileFlags(65536i32);
pub const COMAdminFileFlagRegistrarFailed: COMAdminFileFlags = COMAdminFileFlags(131072i32);
pub const COMAdminFileFlagError: COMAdminFileFlags = COMAdminFileFlags(262144i32);
impl ::core::marker::Copy for COMAdminFileFlags {}
impl ::core::clone::Clone for COMAdminFileFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminFileFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COMAdminFileFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminFileFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminFileFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMAdminImpersonationLevelOptions(pub i32);
pub const COMAdminImpersonationAnonymous: COMAdminImpersonationLevelOptions = COMAdminImpersonationLevelOptions(1i32);
pub const COMAdminImpersonationIdentify: COMAdminImpersonationLevelOptions = COMAdminImpersonationLevelOptions(2i32);
pub const COMAdminImpersonationImpersonate: COMAdminImpersonationLevelOptions = COMAdminImpersonationLevelOptions(3i32);
pub const COMAdminImpersonationDelegate: COMAdminImpersonationLevelOptions = COMAdminImpersonationLevelOptions(4i32);
impl ::core::marker::Copy for COMAdminImpersonationLevelOptions {}
impl ::core::clone::Clone for COMAdminImpersonationLevelOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminImpersonationLevelOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COMAdminImpersonationLevelOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminImpersonationLevelOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminImpersonationLevelOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMAdminInUse(pub i32);
pub const COMAdminNotInUse: COMAdminInUse = COMAdminInUse(0i32);
pub const COMAdminInUseByCatalog: COMAdminInUse = COMAdminInUse(1i32);
pub const COMAdminInUseByRegistryUnknown: COMAdminInUse = COMAdminInUse(2i32);
pub const COMAdminInUseByRegistryProxyStub: COMAdminInUse = COMAdminInUse(3i32);
pub const COMAdminInUseByRegistryTypeLib: COMAdminInUse = COMAdminInUse(4i32);
pub const COMAdminInUseByRegistryClsid: COMAdminInUse = COMAdminInUse(5i32);
impl ::core::marker::Copy for COMAdminInUse {}
impl ::core::clone::Clone for COMAdminInUse {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminInUse {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COMAdminInUse {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminInUse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminInUse").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMAdminOS(pub i32);
pub const COMAdminOSNotInitialized: COMAdminOS = COMAdminOS(0i32);
pub const COMAdminOSWindows3_1: COMAdminOS = COMAdminOS(1i32);
pub const COMAdminOSWindows9x: COMAdminOS = COMAdminOS(2i32);
pub const COMAdminOSWindows2000: COMAdminOS = COMAdminOS(3i32);
pub const COMAdminOSWindows2000AdvancedServer: COMAdminOS = COMAdminOS(4i32);
pub const COMAdminOSWindows2000Unknown: COMAdminOS = COMAdminOS(5i32);
pub const COMAdminOSUnknown: COMAdminOS = COMAdminOS(6i32);
pub const COMAdminOSWindowsXPPersonal: COMAdminOS = COMAdminOS(11i32);
pub const COMAdminOSWindowsXPProfessional: COMAdminOS = COMAdminOS(12i32);
pub const COMAdminOSWindowsNETStandardServer: COMAdminOS = COMAdminOS(13i32);
pub const COMAdminOSWindowsNETEnterpriseServer: COMAdminOS = COMAdminOS(14i32);
pub const COMAdminOSWindowsNETDatacenterServer: COMAdminOS = COMAdminOS(15i32);
pub const COMAdminOSWindowsNETWebServer: COMAdminOS = COMAdminOS(16i32);
pub const COMAdminOSWindowsLonghornPersonal: COMAdminOS = COMAdminOS(17i32);
pub const COMAdminOSWindowsLonghornProfessional: COMAdminOS = COMAdminOS(18i32);
pub const COMAdminOSWindowsLonghornStandardServer: COMAdminOS = COMAdminOS(19i32);
pub const COMAdminOSWindowsLonghornEnterpriseServer: COMAdminOS = COMAdminOS(20i32);
pub const COMAdminOSWindowsLonghornDatacenterServer: COMAdminOS = COMAdminOS(21i32);
pub const COMAdminOSWindowsLonghornWebServer: COMAdminOS = COMAdminOS(22i32);
pub const COMAdminOSWindows7Personal: COMAdminOS = COMAdminOS(23i32);
pub const COMAdminOSWindows7Professional: COMAdminOS = COMAdminOS(24i32);
pub const COMAdminOSWindows7StandardServer: COMAdminOS = COMAdminOS(25i32);
pub const COMAdminOSWindows7EnterpriseServer: COMAdminOS = COMAdminOS(26i32);
pub const COMAdminOSWindows7DatacenterServer: COMAdminOS = COMAdminOS(27i32);
pub const COMAdminOSWindows7WebServer: COMAdminOS = COMAdminOS(28i32);
pub const COMAdminOSWindows8Personal: COMAdminOS = COMAdminOS(29i32);
pub const COMAdminOSWindows8Professional: COMAdminOS = COMAdminOS(30i32);
pub const COMAdminOSWindows8StandardServer: COMAdminOS = COMAdminOS(31i32);
pub const COMAdminOSWindows8EnterpriseServer: COMAdminOS = COMAdminOS(32i32);
pub const COMAdminOSWindows8DatacenterServer: COMAdminOS = COMAdminOS(33i32);
pub const COMAdminOSWindows8WebServer: COMAdminOS = COMAdminOS(34i32);
pub const COMAdminOSWindowsBluePersonal: COMAdminOS = COMAdminOS(35i32);
pub const COMAdminOSWindowsBlueProfessional: COMAdminOS = COMAdminOS(36i32);
pub const COMAdminOSWindowsBlueStandardServer: COMAdminOS = COMAdminOS(37i32);
pub const COMAdminOSWindowsBlueEnterpriseServer: COMAdminOS = COMAdminOS(38i32);
pub const COMAdminOSWindowsBlueDatacenterServer: COMAdminOS = COMAdminOS(39i32);
pub const COMAdminOSWindowsBlueWebServer: COMAdminOS = COMAdminOS(40i32);
impl ::core::marker::Copy for COMAdminOS {}
impl ::core::clone::Clone for COMAdminOS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminOS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COMAdminOS {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminOS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminOS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMAdminQCMessageAuthenticateOptions(pub i32);
pub const COMAdminQCMessageAuthenticateSecureApps: COMAdminQCMessageAuthenticateOptions = COMAdminQCMessageAuthenticateOptions(0i32);
pub const COMAdminQCMessageAuthenticateOff: COMAdminQCMessageAuthenticateOptions = COMAdminQCMessageAuthenticateOptions(1i32);
pub const COMAdminQCMessageAuthenticateOn: COMAdminQCMessageAuthenticateOptions = COMAdminQCMessageAuthenticateOptions(2i32);
impl ::core::marker::Copy for COMAdminQCMessageAuthenticateOptions {}
impl ::core::clone::Clone for COMAdminQCMessageAuthenticateOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminQCMessageAuthenticateOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COMAdminQCMessageAuthenticateOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminQCMessageAuthenticateOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminQCMessageAuthenticateOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMAdminServiceOptions(pub i32);
pub const COMAdminServiceLoadBalanceRouter: COMAdminServiceOptions = COMAdminServiceOptions(1i32);
impl ::core::marker::Copy for COMAdminServiceOptions {}
impl ::core::clone::Clone for COMAdminServiceOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminServiceOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COMAdminServiceOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminServiceOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminServiceOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMAdminServiceStatusOptions(pub i32);
pub const COMAdminServiceStopped: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(0i32);
pub const COMAdminServiceStartPending: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(1i32);
pub const COMAdminServiceStopPending: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(2i32);
pub const COMAdminServiceRunning: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(3i32);
pub const COMAdminServiceContinuePending: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(4i32);
pub const COMAdminServicePausePending: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(5i32);
pub const COMAdminServicePaused: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(6i32);
pub const COMAdminServiceUnknownState: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(7i32);
impl ::core::marker::Copy for COMAdminServiceStatusOptions {}
impl ::core::clone::Clone for COMAdminServiceStatusOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminServiceStatusOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COMAdminServiceStatusOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminServiceStatusOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminServiceStatusOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMAdminSynchronizationOptions(pub i32);
pub const COMAdminSynchronizationIgnored: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(0i32);
pub const COMAdminSynchronizationNone: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(1i32);
pub const COMAdminSynchronizationSupported: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(2i32);
pub const COMAdminSynchronizationRequired: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(3i32);
pub const COMAdminSynchronizationRequiresNew: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(4i32);
impl ::core::marker::Copy for COMAdminSynchronizationOptions {}
impl ::core::clone::Clone for COMAdminSynchronizationOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminSynchronizationOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COMAdminSynchronizationOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminSynchronizationOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminSynchronizationOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMAdminThreadingModels(pub i32);
pub const COMAdminThreadingModelApartment: COMAdminThreadingModels = COMAdminThreadingModels(0i32);
pub const COMAdminThreadingModelFree: COMAdminThreadingModels = COMAdminThreadingModels(1i32);
pub const COMAdminThreadingModelMain: COMAdminThreadingModels = COMAdminThreadingModels(2i32);
pub const COMAdminThreadingModelBoth: COMAdminThreadingModels = COMAdminThreadingModels(3i32);
pub const COMAdminThreadingModelNeutral: COMAdminThreadingModels = COMAdminThreadingModels(4i32);
pub const COMAdminThreadingModelNotSpecified: COMAdminThreadingModels = COMAdminThreadingModels(5i32);
impl ::core::marker::Copy for COMAdminThreadingModels {}
impl ::core::clone::Clone for COMAdminThreadingModels {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminThreadingModels {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COMAdminThreadingModels {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminThreadingModels {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminThreadingModels").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMAdminTransactionOptions(pub i32);
pub const COMAdminTransactionIgnored: COMAdminTransactionOptions = COMAdminTransactionOptions(0i32);
pub const COMAdminTransactionNone: COMAdminTransactionOptions = COMAdminTransactionOptions(1i32);
pub const COMAdminTransactionSupported: COMAdminTransactionOptions = COMAdminTransactionOptions(2i32);
pub const COMAdminTransactionRequired: COMAdminTransactionOptions = COMAdminTransactionOptions(3i32);
pub const COMAdminTransactionRequiresNew: COMAdminTransactionOptions = COMAdminTransactionOptions(4i32);
impl ::core::marker::Copy for COMAdminTransactionOptions {}
impl ::core::clone::Clone for COMAdminTransactionOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminTransactionOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COMAdminTransactionOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminTransactionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminTransactionOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMAdminTxIsolationLevelOptions(pub i32);
pub const COMAdminTxIsolationLevelAny: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(0i32);
pub const COMAdminTxIsolationLevelReadUnCommitted: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(1i32);
pub const COMAdminTxIsolationLevelReadCommitted: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(2i32);
pub const COMAdminTxIsolationLevelRepeatableRead: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(3i32);
pub const COMAdminTxIsolationLevelSerializable: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(4i32);
impl ::core::marker::Copy for COMAdminTxIsolationLevelOptions {}
impl ::core::clone::Clone for COMAdminTxIsolationLevelOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMAdminTxIsolationLevelOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COMAdminTxIsolationLevelOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMAdminTxIsolationLevelOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMAdminTxIsolationLevelOptions").field(&self.0).finish()
    }
}
pub const COMEvents: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecabb0ab_7f19_11d2_978e_0000f8757e2a);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMPLUS_APPTYPE(pub i32);
pub const APPTYPE_UNKNOWN: COMPLUS_APPTYPE = COMPLUS_APPTYPE(-1i32);
pub const APPTYPE_SERVER: COMPLUS_APPTYPE = COMPLUS_APPTYPE(1i32);
pub const APPTYPE_LIBRARY: COMPLUS_APPTYPE = COMPLUS_APPTYPE(0i32);
pub const APPTYPE_SWC: COMPLUS_APPTYPE = COMPLUS_APPTYPE(2i32);
impl ::core::marker::Copy for COMPLUS_APPTYPE {}
impl ::core::clone::Clone for COMPLUS_APPTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMPLUS_APPTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COMPLUS_APPTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMPLUS_APPTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMPLUS_APPTYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct COMSVCSEVENTINFO {
    pub cbSize: u32,
    pub dwPid: u32,
    pub lTime: i64,
    pub lMicroTime: i32,
    pub perfCount: i64,
    pub guidApp: ::windows_core::GUID,
    pub sMachineName: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for COMSVCSEVENTINFO {}
impl ::core::clone::Clone for COMSVCSEVENTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMSVCSEVENTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMSVCSEVENTINFO").field("cbSize", &self.cbSize).field("dwPid", &self.dwPid).field("lTime", &self.lTime).field("lMicroTime", &self.lMicroTime).field("perfCount", &self.perfCount).field("guidApp", &self.guidApp).field("sMachineName", &self.sMachineName).finish()
    }
}
unsafe impl ::windows_core::Abi for COMSVCSEVENTINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COMSVCSEVENTINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMSVCSEVENTINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for COMSVCSEVENTINFO {}
impl ::core::default::Default for COMSVCSEVENTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const CRMClerk: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecabb0bd_7f19_11d2_978e_0000f8757e2a);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CRMFLAGS(pub i32);
pub const CRMFLAG_FORGETTARGET: CRMFLAGS = CRMFLAGS(1i32);
pub const CRMFLAG_WRITTENDURINGPREPARE: CRMFLAGS = CRMFLAGS(2i32);
pub const CRMFLAG_WRITTENDURINGCOMMIT: CRMFLAGS = CRMFLAGS(4i32);
pub const CRMFLAG_WRITTENDURINGABORT: CRMFLAGS = CRMFLAGS(8i32);
pub const CRMFLAG_WRITTENDURINGRECOVERY: CRMFLAGS = CRMFLAGS(16i32);
pub const CRMFLAG_WRITTENDURINGREPLAY: CRMFLAGS = CRMFLAGS(32i32);
pub const CRMFLAG_REPLAYINPROGRESS: CRMFLAGS = CRMFLAGS(64i32);
impl ::core::marker::Copy for CRMFLAGS {}
impl ::core::clone::Clone for CRMFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CRMFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CRMFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CRMFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRMFLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CRMREGFLAGS(pub i32);
pub const CRMREGFLAG_PREPAREPHASE: CRMREGFLAGS = CRMREGFLAGS(1i32);
pub const CRMREGFLAG_COMMITPHASE: CRMREGFLAGS = CRMREGFLAGS(2i32);
pub const CRMREGFLAG_ABORTPHASE: CRMREGFLAGS = CRMREGFLAGS(4i32);
pub const CRMREGFLAG_ALLPHASES: CRMREGFLAGS = CRMREGFLAGS(7i32);
pub const CRMREGFLAG_FAILIFINDOUBTSREMAIN: CRMREGFLAGS = CRMREGFLAGS(16i32);
impl ::core::marker::Copy for CRMREGFLAGS {}
impl ::core::clone::Clone for CRMREGFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CRMREGFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CRMREGFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CRMREGFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRMREGFLAGS").field(&self.0).finish()
    }
}
pub const CRMRecoveryClerk: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecabb0be_7f19_11d2_978e_0000f8757e2a);
pub const CRR_ACTIVATION_LIMIT: u32 = 4294967294u32;
pub const CRR_CALL_LIMIT: u32 = 4294967293u32;
pub const CRR_LIFETIME_LIMIT: u32 = 4294967295u32;
pub const CRR_MEMORY_LIMIT: u32 = 4294967292u32;
pub const CRR_NO_REASON_SUPPLIED: u32 = 0u32;
pub const CRR_RECYCLED_FROM_UI: u32 = 4294967291u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CSC_Binding(pub i32);
pub const CSC_NoBinding: CSC_Binding = CSC_Binding(0i32);
pub const CSC_BindToPoolThread: CSC_Binding = CSC_Binding(1i32);
impl ::core::marker::Copy for CSC_Binding {}
impl ::core::clone::Clone for CSC_Binding {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CSC_Binding {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CSC_Binding {
    type Abi = Self;
}
impl ::core::fmt::Debug for CSC_Binding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CSC_Binding").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CSC_COMTIIntrinsicsConfig(pub i32);
pub const CSC_NoCOMTIIntrinsics: CSC_COMTIIntrinsicsConfig = CSC_COMTIIntrinsicsConfig(0i32);
pub const CSC_InheritCOMTIIntrinsics: CSC_COMTIIntrinsicsConfig = CSC_COMTIIntrinsicsConfig(1i32);
impl ::core::marker::Copy for CSC_COMTIIntrinsicsConfig {}
impl ::core::clone::Clone for CSC_COMTIIntrinsicsConfig {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CSC_COMTIIntrinsicsConfig {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CSC_COMTIIntrinsicsConfig {
    type Abi = Self;
}
impl ::core::fmt::Debug for CSC_COMTIIntrinsicsConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CSC_COMTIIntrinsicsConfig").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CSC_IISIntrinsicsConfig(pub i32);
pub const CSC_NoIISIntrinsics: CSC_IISIntrinsicsConfig = CSC_IISIntrinsicsConfig(0i32);
pub const CSC_InheritIISIntrinsics: CSC_IISIntrinsicsConfig = CSC_IISIntrinsicsConfig(1i32);
impl ::core::marker::Copy for CSC_IISIntrinsicsConfig {}
impl ::core::clone::Clone for CSC_IISIntrinsicsConfig {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CSC_IISIntrinsicsConfig {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CSC_IISIntrinsicsConfig {
    type Abi = Self;
}
impl ::core::fmt::Debug for CSC_IISIntrinsicsConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CSC_IISIntrinsicsConfig").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CSC_InheritanceConfig(pub i32);
pub const CSC_Inherit: CSC_InheritanceConfig = CSC_InheritanceConfig(0i32);
pub const CSC_Ignore: CSC_InheritanceConfig = CSC_InheritanceConfig(1i32);
impl ::core::marker::Copy for CSC_InheritanceConfig {}
impl ::core::clone::Clone for CSC_InheritanceConfig {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CSC_InheritanceConfig {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CSC_InheritanceConfig {
    type Abi = Self;
}
impl ::core::fmt::Debug for CSC_InheritanceConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CSC_InheritanceConfig").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CSC_PartitionConfig(pub i32);
pub const CSC_NoPartition: CSC_PartitionConfig = CSC_PartitionConfig(0i32);
pub const CSC_InheritPartition: CSC_PartitionConfig = CSC_PartitionConfig(1i32);
pub const CSC_NewPartition: CSC_PartitionConfig = CSC_PartitionConfig(2i32);
impl ::core::marker::Copy for CSC_PartitionConfig {}
impl ::core::clone::Clone for CSC_PartitionConfig {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CSC_PartitionConfig {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CSC_PartitionConfig {
    type Abi = Self;
}
impl ::core::fmt::Debug for CSC_PartitionConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CSC_PartitionConfig").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CSC_SxsConfig(pub i32);
pub const CSC_NoSxs: CSC_SxsConfig = CSC_SxsConfig(0i32);
pub const CSC_InheritSxs: CSC_SxsConfig = CSC_SxsConfig(1i32);
pub const CSC_NewSxs: CSC_SxsConfig = CSC_SxsConfig(2i32);
impl ::core::marker::Copy for CSC_SxsConfig {}
impl ::core::clone::Clone for CSC_SxsConfig {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CSC_SxsConfig {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CSC_SxsConfig {
    type Abi = Self;
}
impl ::core::fmt::Debug for CSC_SxsConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CSC_SxsConfig").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CSC_SynchronizationConfig(pub i32);
pub const CSC_NoSynchronization: CSC_SynchronizationConfig = CSC_SynchronizationConfig(0i32);
pub const CSC_IfContainerIsSynchronized: CSC_SynchronizationConfig = CSC_SynchronizationConfig(1i32);
pub const CSC_NewSynchronizationIfNecessary: CSC_SynchronizationConfig = CSC_SynchronizationConfig(2i32);
pub const CSC_NewSynchronization: CSC_SynchronizationConfig = CSC_SynchronizationConfig(3i32);
impl ::core::marker::Copy for CSC_SynchronizationConfig {}
impl ::core::clone::Clone for CSC_SynchronizationConfig {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CSC_SynchronizationConfig {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CSC_SynchronizationConfig {
    type Abi = Self;
}
impl ::core::fmt::Debug for CSC_SynchronizationConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CSC_SynchronizationConfig").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CSC_ThreadPool(pub i32);
pub const CSC_ThreadPoolNone: CSC_ThreadPool = CSC_ThreadPool(0i32);
pub const CSC_ThreadPoolInherit: CSC_ThreadPool = CSC_ThreadPool(1i32);
pub const CSC_STAThreadPool: CSC_ThreadPool = CSC_ThreadPool(2i32);
pub const CSC_MTAThreadPool: CSC_ThreadPool = CSC_ThreadPool(3i32);
impl ::core::marker::Copy for CSC_ThreadPool {}
impl ::core::clone::Clone for CSC_ThreadPool {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CSC_ThreadPool {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CSC_ThreadPool {
    type Abi = Self;
}
impl ::core::fmt::Debug for CSC_ThreadPool {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CSC_ThreadPool").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CSC_TrackerConfig(pub i32);
pub const CSC_DontUseTracker: CSC_TrackerConfig = CSC_TrackerConfig(0i32);
pub const CSC_UseTracker: CSC_TrackerConfig = CSC_TrackerConfig(1i32);
impl ::core::marker::Copy for CSC_TrackerConfig {}
impl ::core::clone::Clone for CSC_TrackerConfig {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CSC_TrackerConfig {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CSC_TrackerConfig {
    type Abi = Self;
}
impl ::core::fmt::Debug for CSC_TrackerConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CSC_TrackerConfig").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CSC_TransactionConfig(pub i32);
pub const CSC_NoTransaction: CSC_TransactionConfig = CSC_TransactionConfig(0i32);
pub const CSC_IfContainerIsTransactional: CSC_TransactionConfig = CSC_TransactionConfig(1i32);
pub const CSC_CreateTransactionIfNecessary: CSC_TransactionConfig = CSC_TransactionConfig(2i32);
pub const CSC_NewTransaction: CSC_TransactionConfig = CSC_TransactionConfig(3i32);
impl ::core::marker::Copy for CSC_TransactionConfig {}
impl ::core::clone::Clone for CSC_TransactionConfig {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CSC_TransactionConfig {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CSC_TransactionConfig {
    type Abi = Self;
}
impl ::core::fmt::Debug for CSC_TransactionConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CSC_TransactionConfig").field(&self.0).finish()
    }
}
pub const CServiceConfig: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecabb0c8_7f19_11d2_978e_0000f8757e2a);
pub const ClrAssemblyLocator: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x458aa3b5_265a_4b75_bc05_9bea4630cf18);
#[inline]
pub unsafe fn CoCreateActivity<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(piunknown: Param0, riid: *const ::windows_core::GUID, ppobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateActivity(piunknown: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        CoCreateActivity(piunknown.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppobj)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoEnterServiceDomain<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(pconfigobject: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoEnterServiceDomain(pconfigobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        CoEnterServiceDomain(pconfigobject.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CoGetDefaultContext(apttype: super::Com::APTTYPE, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetDefaultContext(apttype: super::Com::APTTYPE, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        CoGetDefaultContext(::core::mem::transmute(apttype), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoLeaveServiceDomain<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(punkstatus: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoLeaveServiceDomain(punkstatus: *mut ::core::ffi::c_void);
        }
        CoLeaveServiceDomain(punkstatus.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const CoMTSLocator: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecabb0ac_7f19_11d2_978e_0000f8757e2a);
pub const ComServiceEvents: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecabb0c3_7f19_11d2_978e_0000f8757e2a);
pub const ComSystemAppEventData: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecabb0c6_7f19_11d2_978e_0000f8757e2a);
#[repr(C)]
pub struct ComponentHangMonitorInfo {
    pub IsMonitored: ::win32_foundation::BOOL,
    pub TerminateOnHang: ::win32_foundation::BOOL,
    pub AvgCallThresholdInMs: u32,
}
impl ::core::marker::Copy for ComponentHangMonitorInfo {}
impl ::core::clone::Clone for ComponentHangMonitorInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ComponentHangMonitorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ComponentHangMonitorInfo").field("IsMonitored", &self.IsMonitored).field("TerminateOnHang", &self.TerminateOnHang).field("AvgCallThresholdInMs", &self.AvgCallThresholdInMs).finish()
    }
}
unsafe impl ::windows_core::Abi for ComponentHangMonitorInfo {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ComponentHangMonitorInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ComponentHangMonitorInfo>()) == 0 }
    }
}
impl ::core::cmp::Eq for ComponentHangMonitorInfo {}
impl ::core::default::Default for ComponentHangMonitorInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl ::core::fmt::Debug for ComponentStatistics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ComponentStatistics")
            .field("NumInstances", &self.NumInstances)
            .field("NumBoundReferences", &self.NumBoundReferences)
            .field("NumPooledObjects", &self.NumPooledObjects)
            .field("NumObjectsInCall", &self.NumObjectsInCall)
            .field("AvgResponseTimeInMs", &self.AvgResponseTimeInMs)
            .field("NumCallsCompletedRecent", &self.NumCallsCompletedRecent)
            .field("NumCallsFailedRecent", &self.NumCallsFailedRecent)
            .field("NumCallsCompletedTotal", &self.NumCallsCompletedTotal)
            .field("NumCallsFailedTotal", &self.NumCallsFailedTotal)
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .field("Reserved3", &self.Reserved3)
            .field("Reserved4", &self.Reserved4)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for ComponentStatistics {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ComponentStatistics {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ComponentStatistics>()) == 0 }
    }
}
impl ::core::cmp::Eq for ComponentStatistics {}
impl ::core::default::Default for ComponentStatistics {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ComponentSummary {
    pub ApplicationInstanceId: ::windows_core::GUID,
    pub PartitionId: ::windows_core::GUID,
    pub ApplicationId: ::windows_core::GUID,
    pub Clsid: ::windows_core::GUID,
    pub ClassName: ::windows_core::PWSTR,
    pub ApplicationName: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for ComponentSummary {}
impl ::core::clone::Clone for ComponentSummary {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ComponentSummary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ComponentSummary").field("ApplicationInstanceId", &self.ApplicationInstanceId).field("PartitionId", &self.PartitionId).field("ApplicationId", &self.ApplicationId).field("Clsid", &self.Clsid).field("ClassName", &self.ClassName).field("ApplicationName", &self.ApplicationName).finish()
    }
}
unsafe impl ::windows_core::Abi for ComponentSummary {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ComponentSummary {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ComponentSummary>()) == 0 }
    }
}
impl ::core::cmp::Eq for ComponentSummary {}
impl ::core::default::Default for ComponentSummary {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ContextInfo(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ContextInfo {
    pub unsafe fn IsInTransaction(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsInTransaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn GetTransaction(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).GetTransaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn GetTransactionId(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetTransactionId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetActivityId(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetActivityId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetContextId(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetContextId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ContextInfo> for ::windows_core::IUnknown {
    fn from(value: ContextInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ContextInfo> for ::windows_core::IUnknown {
    fn from(value: &ContextInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ContextInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ContextInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ContextInfo> for super::Com::IDispatch {
    fn from(value: ContextInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ContextInfo> for super::Com::IDispatch {
    fn from(value: &ContextInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ContextInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ContextInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ContextInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ContextInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ContextInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ContextInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContextInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ContextInfo {
    type Vtable = ContextInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x19a5a02c_0ac8_11d2_b286_00c04f8ef934);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ContextInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub IsInTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisintx: *mut i16) -> ::windows_core::HRESULT,
    pub GetTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptx: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetTransactionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtxid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstractivityid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetContextId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrctxid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ContextInfo2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ContextInfo2 {
    pub unsafe fn IsInTransaction(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsInTransaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn GetTransaction(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn GetTransactionId(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransactionId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetActivityId(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetActivityId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetContextId(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetContextId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetPartitionId(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetPartitionId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetApplicationId(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetApplicationId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetApplicationInstanceId(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetApplicationInstanceId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ContextInfo2> for ::windows_core::IUnknown {
    fn from(value: ContextInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ContextInfo2> for ::windows_core::IUnknown {
    fn from(value: &ContextInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ContextInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ContextInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ContextInfo2> for super::Com::IDispatch {
    fn from(value: ContextInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ContextInfo2> for super::Com::IDispatch {
    fn from(value: &ContextInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ContextInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ContextInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ContextInfo2> for ContextInfo {
    fn from(value: ContextInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ContextInfo2> for ContextInfo {
    fn from(value: &ContextInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ContextInfo> for ContextInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ContextInfo> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ContextInfo> for &'a ContextInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ContextInfo> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ContextInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ContextInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ContextInfo2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ContextInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContextInfo2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ContextInfo2 {
    type Vtable = ContextInfo2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc99d6e75_2375_11d4_8331_00c04f605588);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ContextInfo2_Vtbl {
    pub base__: ContextInfo_Vtbl,
    pub GetPartitionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__contextinfo20000: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetApplicationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__contextinfo20001: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetApplicationInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__contextinfo20002: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(C)]
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
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for CrmLogRecordRead {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CrmLogRecordRead").field("dwCrmFlags", &self.dwCrmFlags).field("dwSequenceNumber", &self.dwSequenceNumber).field("blobUserData", &self.blobUserData).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Abi for CrmLogRecordRead {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for CrmLogRecordRead {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CrmLogRecordRead>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for CrmLogRecordRead {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for CrmLogRecordRead {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CrmTransactionState(pub i32);
pub const TxState_Active: CrmTransactionState = CrmTransactionState(0i32);
pub const TxState_Committed: CrmTransactionState = CrmTransactionState(1i32);
pub const TxState_Aborted: CrmTransactionState = CrmTransactionState(2i32);
pub const TxState_Indoubt: CrmTransactionState = CrmTransactionState(3i32);
impl ::core::marker::Copy for CrmTransactionState {}
impl ::core::clone::Clone for CrmTransactionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CrmTransactionState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CrmTransactionState {
    type Abi = Self;
}
impl ::core::fmt::Debug for CrmTransactionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CrmTransactionState").field(&self.0).finish()
    }
}
pub const DATA_NOT_AVAILABLE: u32 = 4294967295u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DUMPTYPE(pub i32);
pub const DUMPTYPE_FULL: DUMPTYPE = DUMPTYPE(0i32);
pub const DUMPTYPE_MINI: DUMPTYPE = DUMPTYPE(1i32);
pub const DUMPTYPE_NONE: DUMPTYPE = DUMPTYPE(2i32);
impl ::core::marker::Copy for DUMPTYPE {}
impl ::core::clone::Clone for DUMPTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DUMPTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DUMPTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DUMPTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DUMPTYPE").field(&self.0).finish()
    }
}
pub const DispenserManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecabb0c0_7f19_11d2_978e_0000f8757e2a);
pub const Dummy30040732: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecabb0a9_7f19_11d2_978e_0000f8757e2a);
pub const EventServer: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecabafbc_7f19_11d2_978e_0000f8757e2a);
pub const GUID_STRING_SIZE: u32 = 40u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GetAppTrackerDataFlags(pub i32);
pub const GATD_INCLUDE_PROCESS_EXE_NAME: GetAppTrackerDataFlags = GetAppTrackerDataFlags(1i32);
pub const GATD_INCLUDE_LIBRARY_APPS: GetAppTrackerDataFlags = GetAppTrackerDataFlags(2i32);
pub const GATD_INCLUDE_SWC: GetAppTrackerDataFlags = GetAppTrackerDataFlags(4i32);
pub const GATD_INCLUDE_CLASS_NAME: GetAppTrackerDataFlags = GetAppTrackerDataFlags(8i32);
pub const GATD_INCLUDE_APPLICATION_NAME: GetAppTrackerDataFlags = GetAppTrackerDataFlags(16i32);
impl ::core::marker::Copy for GetAppTrackerDataFlags {}
impl ::core::clone::Clone for GetAppTrackerDataFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GetAppTrackerDataFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GetAppTrackerDataFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for GetAppTrackerDataFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GetAppTrackerDataFlags").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn GetDispenserManager() -> ::windows_core::Result<IDispenserManager> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDispenserManager(param0: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        GetDispenserManager(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDispenserManager>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetManagedExtensions(dwexts: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetManagedExtensions(dwexts: *mut u32) -> ::windows_core::HRESULT;
        }
        GetManagedExtensions(::core::mem::transmute(dwexts)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const GetSecurityCallContextAppObject: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecabb0a8_7f19_11d2_978e_0000f8757e2a);
#[repr(C)]
pub struct HANG_INFO {
    pub fAppHangMonitorEnabled: ::win32_foundation::BOOL,
    pub fTerminateOnHang: ::win32_foundation::BOOL,
    pub DumpType: DUMPTYPE,
    pub dwHangTimeout: u32,
    pub dwDumpCount: u32,
    pub dwInfoMsgCount: u32,
}
impl ::core::marker::Copy for HANG_INFO {}
impl ::core::clone::Clone for HANG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HANG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HANG_INFO").field("fAppHangMonitorEnabled", &self.fAppHangMonitorEnabled).field("fTerminateOnHang", &self.fTerminateOnHang).field("DumpType", &self.DumpType).field("dwHangTimeout", &self.dwHangTimeout).field("dwDumpCount", &self.dwDumpCount).field("dwInfoMsgCount", &self.dwInfoMsgCount).finish()
    }
}
unsafe impl ::windows_core::Abi for HANG_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HANG_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HANG_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for HANG_INFO {}
impl ::core::default::Default for HANG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAppDomainHelper(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAppDomainHelper {
    pub unsafe fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, punkad: Param0, __midl__iappdomainhelper0000: isize, ppool: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), punkad.into_param().abi(), ::core::mem::transmute(__midl__iappdomainhelper0000), ::core::mem::transmute(ppool)).ok()
    }
    pub unsafe fn DoCallback<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, punkad: Param0, __midl__iappdomainhelper0001: isize, ppool: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DoCallback)(::windows_core::Interface::as_raw(self), punkad.into_param().abi(), ::core::mem::transmute(__midl__iappdomainhelper0001), ::core::mem::transmute(ppool)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAppDomainHelper> for ::windows_core::IUnknown {
    fn from(value: IAppDomainHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAppDomainHelper> for ::windows_core::IUnknown {
    fn from(value: &IAppDomainHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAppDomainHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAppDomainHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAppDomainHelper> for super::Com::IDispatch {
    fn from(value: IAppDomainHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAppDomainHelper> for super::Com::IDispatch {
    fn from(value: &IAppDomainHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IAppDomainHelper {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IAppDomainHelper {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAppDomainHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAppDomainHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAppDomainHelper {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAppDomainHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppDomainHelper").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAppDomainHelper {
    type Vtable = IAppDomainHelper_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc7b67079_8255_42c6_9ec0_6994a3548780);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAppDomainHelper_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkad: *mut ::core::ffi::c_void, __midl__iappdomainhelper0000: isize, ppool: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DoCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkad: *mut ::core::ffi::c_void, __midl__iappdomainhelper0001: isize, ppool: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAssemblyLocator(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAssemblyLocator {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetModules<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, applicationdir: Param0, applicationname: Param1, assemblyname: Param2) -> ::windows_core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut super::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).GetModules)(::windows_core::Interface::as_raw(self), applicationdir.into_param().abi(), applicationname.into_param().abi(), assemblyname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAssemblyLocator> for ::windows_core::IUnknown {
    fn from(value: IAssemblyLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAssemblyLocator> for ::windows_core::IUnknown {
    fn from(value: &IAssemblyLocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAssemblyLocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAssemblyLocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAssemblyLocator> for super::Com::IDispatch {
    fn from(value: IAssemblyLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAssemblyLocator> for super::Com::IDispatch {
    fn from(value: &IAssemblyLocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IAssemblyLocator {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IAssemblyLocator {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAssemblyLocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAssemblyLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAssemblyLocator {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAssemblyLocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAssemblyLocator").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAssemblyLocator {
    type Vtable = IAssemblyLocator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x391ffbb9_a8ee_432a_abc8_baa238dab90f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAssemblyLocator_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetModules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationdir: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, applicationname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, assemblyname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pmodules: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetModules: usize,
}
#[repr(transparent)]
pub struct IAsyncErrorNotify(::windows_core::IUnknown);
impl IAsyncErrorNotify {
    pub unsafe fn OnError(&self, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnError)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hr)).ok()
    }
}
impl ::core::convert::From<IAsyncErrorNotify> for ::windows_core::IUnknown {
    fn from(value: IAsyncErrorNotify) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAsyncErrorNotify> for ::windows_core::IUnknown {
    fn from(value: &IAsyncErrorNotify) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAsyncErrorNotify {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAsyncErrorNotify {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAsyncErrorNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAsyncErrorNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAsyncErrorNotify {}
impl ::core::fmt::Debug for IAsyncErrorNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsyncErrorNotify").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAsyncErrorNotify {
    type Vtable = IAsyncErrorNotify_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfe6777fb_a674_4177_8f32_6d707e113484);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncErrorNotify_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ICOMAdminCatalog(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ICOMAdminCatalog {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCollection<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrcollname: Param0) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetCollection)(::windows_core::Interface::as_raw(self), bstrcollname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Connect<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrcatalogservername: Param0) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Connect)(::windows_core::Interface::as_raw(self), bstrcatalogservername.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    pub unsafe fn MajorVersion(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MajorVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn MinorVersion(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MinorVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCollectionByQuery<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrcollname: Param0, ppsavarquery: *const *const super::Com::SAFEARRAY) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetCollectionByQuery)(::windows_core::Interface::as_raw(self), bstrcollname.into_param().abi(), ::core::mem::transmute(ppsavarquery), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    pub unsafe fn ImportComponent<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplidorname: Param0, bstrclsidorprogid: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ImportComponent)(::windows_core::Interface::as_raw(self), bstrapplidorname.into_param().abi(), bstrclsidorprogid.into_param().abi()).ok()
    }
    pub unsafe fn InstallComponent<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplidorname: Param0, bstrdll: Param1, bstrtlb: Param2, bstrpsdll: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InstallComponent)(::windows_core::Interface::as_raw(self), bstrapplidorname.into_param().abi(), bstrdll.into_param().abi(), bstrtlb.into_param().abi(), bstrpsdll.into_param().abi()).ok()
    }
    pub unsafe fn ShutdownApplication<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplidorname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ShutdownApplication)(::windows_core::Interface::as_raw(self), bstrapplidorname.into_param().abi()).ok()
    }
    pub unsafe fn ExportApplication<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplidorname: Param0, bstrapplicationfile: Param1, loptions: COMAdminApplicationExportOptions) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExportApplication)(::windows_core::Interface::as_raw(self), bstrapplidorname.into_param().abi(), bstrapplicationfile.into_param().abi(), ::core::mem::transmute(loptions)).ok()
    }
    pub unsafe fn InstallApplication<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationfile: Param0, bstrdestinationdirectory: Param1, loptions: COMAdminApplicationInstallOptions, bstruserid: Param3, bstrpassword: Param4, bstrrsn: Param5) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InstallApplication)(::windows_core::Interface::as_raw(self), bstrapplicationfile.into_param().abi(), bstrdestinationdirectory.into_param().abi(), ::core::mem::transmute(loptions), bstruserid.into_param().abi(), bstrpassword.into_param().abi(), bstrrsn.into_param().abi()).ok()
    }
    pub unsafe fn StopRouter(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StopRouter)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RefreshRouter(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RefreshRouter)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StartRouter(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StartRouter)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reserved1(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reserved1)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reserved2(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reserved2)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstallMultipleComponents<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplidorname: Param0, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InstallMultipleComponents)(::windows_core::Interface::as_raw(self), bstrapplidorname.into_param().abi(), ::core::mem::transmute(ppsavarfilenames), ::core::mem::transmute(ppsavarclsids)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMultipleComponentsInfo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplidorname: Param0, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetMultipleComponentsInfo)(::windows_core::Interface::as_raw(self), bstrapplidorname.into_param().abi(), ::core::mem::transmute(ppsavarfilenames), ::core::mem::transmute(ppsavarclsids), ::core::mem::transmute(ppsavarclassnames), ::core::mem::transmute(ppsavarfileflags), ::core::mem::transmute(ppsavarcomponentflags)).ok()
    }
    pub unsafe fn RefreshComponents(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RefreshComponents)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackupREGDB<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrbackupfilepath: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BackupREGDB)(::windows_core::Interface::as_raw(self), bstrbackupfilepath.into_param().abi()).ok()
    }
    pub unsafe fn RestoreREGDB<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrbackupfilepath: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RestoreREGDB)(::windows_core::Interface::as_raw(self), bstrbackupfilepath.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryApplicationFile<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationfile: Param0, pbstrapplicationname: *mut ::win32_foundation::BSTR, pbstrapplicationdescription: *mut ::win32_foundation::BSTR, pbhasusers: *mut i16, pbisproxy: *mut i16, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).QueryApplicationFile)(::windows_core::Interface::as_raw(self), bstrapplicationfile.into_param().abi(), ::core::mem::transmute(pbstrapplicationname), ::core::mem::transmute(pbstrapplicationdescription), ::core::mem::transmute(pbhasusers), ::core::mem::transmute(pbisproxy), ::core::mem::transmute(ppsavarfilenames)).ok()
    }
    pub unsafe fn StartApplication<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplidorname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StartApplication)(::windows_core::Interface::as_raw(self), bstrapplidorname.into_param().abi()).ok()
    }
    pub unsafe fn ServiceCheck(&self, lservice: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).ServiceCheck)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lservice), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstallMultipleEventClasses<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplidorname: Param0, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InstallMultipleEventClasses)(::windows_core::Interface::as_raw(self), bstrapplidorname.into_param().abi(), ::core::mem::transmute(ppsavarfilenames), ::core::mem::transmute(ppsavarclsids)).ok()
    }
    pub unsafe fn InstallEventClass<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplidorname: Param0, bstrdll: Param1, bstrtlb: Param2, bstrpsdll: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InstallEventClass)(::windows_core::Interface::as_raw(self), bstrapplidorname.into_param().abi(), bstrdll.into_param().abi(), bstrtlb.into_param().abi(), bstrpsdll.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEventClassesForIID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstriid: Param0, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetEventClassesForIID)(::windows_core::Interface::as_raw(self), bstriid.into_param().abi(), ::core::mem::transmute(ppsavarclsids), ::core::mem::transmute(ppsavarprogids), ::core::mem::transmute(ppsavardescriptions)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICOMAdminCatalog> for ::windows_core::IUnknown {
    fn from(value: ICOMAdminCatalog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICOMAdminCatalog> for ::windows_core::IUnknown {
    fn from(value: &ICOMAdminCatalog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICOMAdminCatalog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICOMAdminCatalog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICOMAdminCatalog> for super::Com::IDispatch {
    fn from(value: ICOMAdminCatalog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICOMAdminCatalog> for super::Com::IDispatch {
    fn from(value: &ICOMAdminCatalog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ICOMAdminCatalog {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ICOMAdminCatalog {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ICOMAdminCatalog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICOMAdminCatalog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICOMAdminCatalog {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICOMAdminCatalog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICOMAdminCatalog").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ICOMAdminCatalog {
    type Vtable = ICOMAdminCatalog_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdd662187_dfc2_11d1_a2cf_00805fc79235);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICOMAdminCatalog_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcollname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppcatalogcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCollection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcatalogservername: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppcatalogcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Connect: usize,
    pub MajorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows_core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCollectionByQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcollname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppsavarquery: *const *const super::Com::SAFEARRAY, ppcatalogcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCollectionByQuery: usize,
    pub ImportComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrclsidorprogid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub InstallComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrdll: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrtlb: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrpsdll: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ShutdownApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ExportApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrapplicationfile: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, loptions: COMAdminApplicationExportOptions) -> ::windows_core::HRESULT,
    pub InstallApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationfile: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrdestinationdirectory: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, loptions: COMAdminApplicationInstallOptions, bstruserid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrrsn: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub StopRouter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RefreshRouter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StartRouter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Reserved1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Reserved2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InstallMultipleComponents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstallMultipleComponents: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMultipleComponentsInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMultipleComponentsInfo: usize,
    pub RefreshComponents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub BackupREGDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbackupfilepath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub RestoreREGDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbackupfilepath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryApplicationFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationfile: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pbstrapplicationname: *mut ::win32_foundation::BSTR, pbstrapplicationdescription: *mut ::win32_foundation::BSTR, pbhasusers: *mut i16, pbisproxy: *mut i16, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryApplicationFile: usize,
    pub StartApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ServiceCheck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lservice: i32, plstatus: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InstallMultipleEventClasses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstallMultipleEventClasses: usize,
    pub InstallEventClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrdll: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrtlb: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrpsdll: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetEventClassesForIID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstriid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetEventClassesForIID: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ICOMAdminCatalog2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ICOMAdminCatalog2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCollection<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrcollname: Param0) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCollection)(::windows_core::Interface::as_raw(self), bstrcollname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Connect<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrcatalogservername: Param0) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Connect)(::windows_core::Interface::as_raw(self), bstrcatalogservername.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    pub unsafe fn MajorVersion(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MajorVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn MinorVersion(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MinorVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCollectionByQuery<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrcollname: Param0, ppsavarquery: *const *const super::Com::SAFEARRAY) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCollectionByQuery)(::windows_core::Interface::as_raw(self), bstrcollname.into_param().abi(), ::core::mem::transmute(ppsavarquery), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    pub unsafe fn ImportComponent<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplidorname: Param0, bstrclsidorprogid: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ImportComponent)(::windows_core::Interface::as_raw(self), bstrapplidorname.into_param().abi(), bstrclsidorprogid.into_param().abi()).ok()
    }
    pub unsafe fn InstallComponent<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplidorname: Param0, bstrdll: Param1, bstrtlb: Param2, bstrpsdll: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.InstallComponent)(::windows_core::Interface::as_raw(self), bstrapplidorname.into_param().abi(), bstrdll.into_param().abi(), bstrtlb.into_param().abi(), bstrpsdll.into_param().abi()).ok()
    }
    pub unsafe fn ShutdownApplication<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplidorname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ShutdownApplication)(::windows_core::Interface::as_raw(self), bstrapplidorname.into_param().abi()).ok()
    }
    pub unsafe fn ExportApplication<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplidorname: Param0, bstrapplicationfile: Param1, loptions: COMAdminApplicationExportOptions) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ExportApplication)(::windows_core::Interface::as_raw(self), bstrapplidorname.into_param().abi(), bstrapplicationfile.into_param().abi(), ::core::mem::transmute(loptions)).ok()
    }
    pub unsafe fn InstallApplication<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationfile: Param0, bstrdestinationdirectory: Param1, loptions: COMAdminApplicationInstallOptions, bstruserid: Param3, bstrpassword: Param4, bstrrsn: Param5) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.InstallApplication)(::windows_core::Interface::as_raw(self), bstrapplicationfile.into_param().abi(), bstrdestinationdirectory.into_param().abi(), ::core::mem::transmute(loptions), bstruserid.into_param().abi(), bstrpassword.into_param().abi(), bstrrsn.into_param().abi()).ok()
    }
    pub unsafe fn StopRouter(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StopRouter)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RefreshRouter(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RefreshRouter)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StartRouter(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StartRouter)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reserved1(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Reserved1)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reserved2(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Reserved2)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstallMultipleComponents<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplidorname: Param0, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.InstallMultipleComponents)(::windows_core::Interface::as_raw(self), bstrapplidorname.into_param().abi(), ::core::mem::transmute(ppsavarfilenames), ::core::mem::transmute(ppsavarclsids)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMultipleComponentsInfo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplidorname: Param0, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetMultipleComponentsInfo)(::windows_core::Interface::as_raw(self), bstrapplidorname.into_param().abi(), ::core::mem::transmute(ppsavarfilenames), ::core::mem::transmute(ppsavarclsids), ::core::mem::transmute(ppsavarclassnames), ::core::mem::transmute(ppsavarfileflags), ::core::mem::transmute(ppsavarcomponentflags)).ok()
    }
    pub unsafe fn RefreshComponents(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RefreshComponents)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackupREGDB<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrbackupfilepath: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BackupREGDB)(::windows_core::Interface::as_raw(self), bstrbackupfilepath.into_param().abi()).ok()
    }
    pub unsafe fn RestoreREGDB<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrbackupfilepath: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RestoreREGDB)(::windows_core::Interface::as_raw(self), bstrbackupfilepath.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryApplicationFile<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationfile: Param0, pbstrapplicationname: *mut ::win32_foundation::BSTR, pbstrapplicationdescription: *mut ::win32_foundation::BSTR, pbhasusers: *mut i16, pbisproxy: *mut i16, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.QueryApplicationFile)(::windows_core::Interface::as_raw(self), bstrapplicationfile.into_param().abi(), ::core::mem::transmute(pbstrapplicationname), ::core::mem::transmute(pbstrapplicationdescription), ::core::mem::transmute(pbhasusers), ::core::mem::transmute(pbisproxy), ::core::mem::transmute(ppsavarfilenames)).ok()
    }
    pub unsafe fn StartApplication<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplidorname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StartApplication)(::windows_core::Interface::as_raw(self), bstrapplidorname.into_param().abi()).ok()
    }
    pub unsafe fn ServiceCheck(&self, lservice: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ServiceCheck)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lservice), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstallMultipleEventClasses<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplidorname: Param0, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.InstallMultipleEventClasses)(::windows_core::Interface::as_raw(self), bstrapplidorname.into_param().abi(), ::core::mem::transmute(ppsavarfilenames), ::core::mem::transmute(ppsavarclsids)).ok()
    }
    pub unsafe fn InstallEventClass<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplidorname: Param0, bstrdll: Param1, bstrtlb: Param2, bstrpsdll: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.InstallEventClass)(::windows_core::Interface::as_raw(self), bstrapplidorname.into_param().abi(), bstrdll.into_param().abi(), bstrtlb.into_param().abi(), bstrpsdll.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEventClassesForIID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstriid: Param0, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetEventClassesForIID)(::windows_core::Interface::as_raw(self), bstriid.into_param().abi(), ::core::mem::transmute(ppsavarclsids), ::core::mem::transmute(ppsavarprogids), ::core::mem::transmute(ppsavardescriptions)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCollectionByQuery2<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrcollectionname: Param0, pvarquerystrings: *const super::Com::VARIANT) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetCollectionByQuery2)(::windows_core::Interface::as_raw(self), bstrcollectionname.into_param().abi(), ::core::mem::transmute(pvarquerystrings), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    pub unsafe fn GetApplicationInstanceIDFromProcessID(&self, lprocessid: i32) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetApplicationInstanceIDFromProcessID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lprocessid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ShutdownApplicationInstances(&self, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ShutdownApplicationInstances)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pvarapplicationinstanceid)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PauseApplicationInstances(&self, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PauseApplicationInstances)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pvarapplicationinstanceid)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ResumeApplicationInstances(&self, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ResumeApplicationInstances)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pvarapplicationinstanceid)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RecycleApplicationInstances(&self, pvarapplicationinstanceid: *const super::Com::VARIANT, lreasoncode: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RecycleApplicationInstances)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pvarapplicationinstanceid), ::core::mem::transmute(lreasoncode)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AreApplicationInstancesPaused(&self, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).AreApplicationInstancesPaused)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pvarapplicationinstanceid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn DumpApplicationInstance<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationinstanceid: Param0, bstrdirectory: Param1, lmaximages: i32) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DumpApplicationInstance)(::windows_core::Interface::as_raw(self), bstrapplicationinstanceid.into_param().abi(), bstrdirectory.into_param().abi(), ::core::mem::transmute(lmaximages), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn IsApplicationInstanceDumpSupported(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsApplicationInstanceDumpSupported)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn CreateServiceForApplication<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param6: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationidorname: Param0, bstrservicename: Param1, bstrstarttype: Param2, bstrerrorcontrol: Param3, bstrdependencies: Param4, bstrrunas: Param5, bstrpassword: Param6, bdesktopok: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateServiceForApplication)(::windows_core::Interface::as_raw(self), bstrapplicationidorname.into_param().abi(), bstrservicename.into_param().abi(), bstrstarttype.into_param().abi(), bstrerrorcontrol.into_param().abi(), bstrdependencies.into_param().abi(), bstrrunas.into_param().abi(), bstrpassword.into_param().abi(), ::core::mem::transmute(bdesktopok)).ok()
    }
    pub unsafe fn DeleteServiceForApplication<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationidorname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteServiceForApplication)(::windows_core::Interface::as_raw(self), bstrapplicationidorname.into_param().abi()).ok()
    }
    pub unsafe fn GetPartitionID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationidorname: Param0) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetPartitionID)(::windows_core::Interface::as_raw(self), bstrapplicationidorname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetPartitionName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationidorname: Param0) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetPartitionName)(::windows_core::Interface::as_raw(self), bstrapplicationidorname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetCurrentPartition<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpartitionidorname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCurrentPartition)(::windows_core::Interface::as_raw(self), bstrpartitionidorname.into_param().abi()).ok()
    }
    pub unsafe fn CurrentPartitionID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).CurrentPartitionID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn CurrentPartitionName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).CurrentPartitionName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GlobalPartitionID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GlobalPartitionID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn FlushPartitionCache(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FlushPartitionCache)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CopyApplications<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrsourcepartitionidorname: Param0, pvarapplicationid: *const super::Com::VARIANT, bstrdestinationpartitionidorname: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CopyApplications)(::windows_core::Interface::as_raw(self), bstrsourcepartitionidorname.into_param().abi(), ::core::mem::transmute(pvarapplicationid), bstrdestinationpartitionidorname.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CopyComponents<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrsourceapplicationidorname: Param0, pvarclsidorprogid: *const super::Com::VARIANT, bstrdestinationapplicationidorname: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CopyComponents)(::windows_core::Interface::as_raw(self), bstrsourceapplicationidorname.into_param().abi(), ::core::mem::transmute(pvarclsidorprogid), bstrdestinationapplicationidorname.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MoveComponents<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrsourceapplicationidorname: Param0, pvarclsidorprogid: *const super::Com::VARIANT, bstrdestinationapplicationidorname: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MoveComponents)(::windows_core::Interface::as_raw(self), bstrsourceapplicationidorname.into_param().abi(), ::core::mem::transmute(pvarclsidorprogid), bstrdestinationapplicationidorname.into_param().abi()).ok()
    }
    pub unsafe fn AliasComponent<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrsrcapplicationidorname: Param0, bstrclsidorprogid: Param1, bstrdestapplicationidorname: Param2, bstrnewprogid: Param3, bstrnewclsid: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AliasComponent)(::windows_core::Interface::as_raw(self), bstrsrcapplicationidorname.into_param().abi(), bstrclsidorprogid.into_param().abi(), bstrdestapplicationidorname.into_param().abi(), bstrnewprogid.into_param().abi(), bstrnewclsid.into_param().abi()).ok()
    }
    pub unsafe fn IsSafeToDelete<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdllname: Param0) -> ::windows_core::Result<COMAdminInUse> {
        let mut result__ = ::core::mem::MaybeUninit::<COMAdminInUse>::zeroed();
        (::windows_core::Interface::vtable(self).IsSafeToDelete)(::windows_core::Interface::as_raw(self), bstrdllname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<COMAdminInUse>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ImportUnconfiguredComponents<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationidorname: Param0, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ImportUnconfiguredComponents)(::windows_core::Interface::as_raw(self), bstrapplicationidorname.into_param().abi(), ::core::mem::transmute(pvarclsidorprogid), ::core::mem::transmute(pvarcomponenttype)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PromoteUnconfiguredComponents<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationidorname: Param0, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PromoteUnconfiguredComponents)(::windows_core::Interface::as_raw(self), bstrapplicationidorname.into_param().abi(), ::core::mem::transmute(pvarclsidorprogid), ::core::mem::transmute(pvarcomponenttype)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ImportComponents<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationidorname: Param0, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ImportComponents)(::windows_core::Interface::as_raw(self), bstrapplicationidorname.into_param().abi(), ::core::mem::transmute(pvarclsidorprogid), ::core::mem::transmute(pvarcomponenttype)).ok()
    }
    pub unsafe fn Is64BitCatalogServer(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Is64BitCatalogServer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn ExportPartition<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpartitionidorname: Param0, bstrpartitionfilename: Param1, loptions: COMAdminApplicationExportOptions) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExportPartition)(::windows_core::Interface::as_raw(self), bstrpartitionidorname.into_param().abi(), bstrpartitionfilename.into_param().abi(), ::core::mem::transmute(loptions)).ok()
    }
    pub unsafe fn InstallPartition<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrfilename: Param0, bstrdestdirectory: Param1, loptions: COMAdminApplicationInstallOptions, bstruserid: Param3, bstrpassword: Param4, bstrrsn: Param5) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InstallPartition)(::windows_core::Interface::as_raw(self), bstrfilename.into_param().abi(), bstrdestdirectory.into_param().abi(), ::core::mem::transmute(loptions), bstruserid.into_param().abi(), bstrpassword.into_param().abi(), bstrrsn.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryApplicationFile2<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationfile: Param0) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).QueryApplicationFile2)(::windows_core::Interface::as_raw(self), bstrapplicationfile.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    pub unsafe fn GetComponentVersionCount<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrclsidorprogid: Param0) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).GetComponentVersionCount)(::windows_core::Interface::as_raw(self), bstrclsidorprogid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICOMAdminCatalog2> for ::windows_core::IUnknown {
    fn from(value: ICOMAdminCatalog2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICOMAdminCatalog2> for ::windows_core::IUnknown {
    fn from(value: &ICOMAdminCatalog2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICOMAdminCatalog2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICOMAdminCatalog2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICOMAdminCatalog2> for super::Com::IDispatch {
    fn from(value: ICOMAdminCatalog2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICOMAdminCatalog2> for super::Com::IDispatch {
    fn from(value: &ICOMAdminCatalog2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ICOMAdminCatalog2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ICOMAdminCatalog2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICOMAdminCatalog2> for ICOMAdminCatalog {
    fn from(value: ICOMAdminCatalog2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICOMAdminCatalog2> for ICOMAdminCatalog {
    fn from(value: &ICOMAdminCatalog2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ICOMAdminCatalog> for ICOMAdminCatalog2 {
    fn into_param(self) -> ::windows_core::Param<'a, ICOMAdminCatalog> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ICOMAdminCatalog> for &'a ICOMAdminCatalog2 {
    fn into_param(self) -> ::windows_core::Param<'a, ICOMAdminCatalog> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ICOMAdminCatalog2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICOMAdminCatalog2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICOMAdminCatalog2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICOMAdminCatalog2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICOMAdminCatalog2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ICOMAdminCatalog2 {
    type Vtable = ICOMAdminCatalog2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x790c6e0b_9194_4cc9_9426_a48a63185696);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICOMAdminCatalog2_Vtbl {
    pub base__: ICOMAdminCatalog_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetCollectionByQuery2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcollectionname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pvarquerystrings: *const super::Com::VARIANT, ppcatalogcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetCollectionByQuery2: usize,
    pub GetApplicationInstanceIDFromProcessID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprocessid: i32, pbstrapplicationinstanceid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ShutdownApplicationInstances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ShutdownApplicationInstances: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PauseApplicationInstances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PauseApplicationInstances: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ResumeApplicationInstances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ResumeApplicationInstances: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RecycleApplicationInstances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT, lreasoncode: i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RecycleApplicationInstances: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AreApplicationInstancesPaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT, pvarboolpaused: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AreApplicationInstancesPaused: usize,
    pub DumpApplicationInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationinstanceid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrdirectory: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lmaximages: i32, pbstrdumpfile: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub IsApplicationInstanceDumpSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarbooldumpsupported: *mut i16) -> ::windows_core::HRESULT,
    pub CreateServiceForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrservicename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrstarttype: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrerrorcontrol: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrdependencies: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrrunas: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bdesktopok: i16) -> ::windows_core::HRESULT,
    pub DeleteServiceForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub GetPartitionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pbstrpartitionid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetPartitionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pbstrpartitionname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetCurrentPartition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpartitionidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub CurrentPartitionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpartitionid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub CurrentPartitionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpartitionname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GlobalPartitionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrglobalpartitionid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub FlushPartitionCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CopyApplications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsourcepartitionidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pvarapplicationid: *const super::Com::VARIANT, bstrdestinationpartitionidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CopyApplications: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CopyComponents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsourceapplicationidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, bstrdestinationapplicationidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CopyComponents: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub MoveComponents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsourceapplicationidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, bstrdestinationapplicationidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    MoveComponents: usize,
    pub AliasComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsrcapplicationidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrclsidorprogid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrdestapplicationidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrnewprogid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrnewclsid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub IsSafeToDelete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdllname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pcomadmininuse: *mut COMAdminInUse) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ImportUnconfiguredComponents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ImportUnconfiguredComponents: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PromoteUnconfiguredComponents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PromoteUnconfiguredComponents: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ImportComponents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ImportComponents: usize,
    pub Is64BitCatalogServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbis64bit: *mut i16) -> ::windows_core::HRESULT,
    pub ExportPartition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpartitionidorname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrpartitionfilename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, loptions: COMAdminApplicationExportOptions) -> ::windows_core::HRESULT,
    pub InstallPartition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrdestdirectory: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, loptions: COMAdminApplicationInstallOptions, bstruserid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrrsn: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryApplicationFile2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationfile: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppfilesforimport: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryApplicationFile2: usize,
    pub GetComponentVersionCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrclsidorprogid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, plversioncount: *mut i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICOMLBArguments(::windows_core::IUnknown);
impl ICOMLBArguments {
    pub unsafe fn GetCLSID(&self, pclsid: *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCLSID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pclsid)).ok()
    }
    pub unsafe fn SetCLSID(&self, pclsid: *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCLSID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pclsid)).ok()
    }
    pub unsafe fn GetMachineName(&self, szservername: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetMachineName)(::windows_core::Interface::as_raw(self), szservername.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(szservername))).ok()
    }
    pub unsafe fn SetMachineName(&self, szservername: &[u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMachineName)(::windows_core::Interface::as_raw(self), szservername.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(szservername))).ok()
    }
}
impl ::core::convert::From<ICOMLBArguments> for ::windows_core::IUnknown {
    fn from(value: ICOMLBArguments) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICOMLBArguments> for ::windows_core::IUnknown {
    fn from(value: &ICOMLBArguments) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICOMLBArguments {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICOMLBArguments {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICOMLBArguments {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICOMLBArguments {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICOMLBArguments {}
impl ::core::fmt::Debug for ICOMLBArguments {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICOMLBArguments").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICOMLBArguments {
    type Vtable = ICOMLBArguments_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a0f150f_8ee5_4b94_b40e_aef2f9e42ed2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICOMLBArguments_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetMachineName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchsvr: u32, szservername: ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetMachineName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchsvr: u32, szservername: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ICatalogCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ICatalogCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Remove(&self, lindex: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lindex)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    pub unsafe fn Populate(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Populate)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SaveChanges(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).SaveChanges)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCollection<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, bstrcollname: Param0, varobjectkey: Param1) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetCollection)(::windows_core::Interface::as_raw(self), bstrcollname.into_param().abi(), varobjectkey.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Name(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn AddEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).AddEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn RemoveEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).RemoveEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUtilInterface(&self) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetUtilInterface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    pub unsafe fn DataStoreMajorVersion(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).DataStoreMajorVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn DataStoreMinorVersion(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).DataStoreMinorVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PopulateByKey(&self, psakeys: *const super::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PopulateByKey)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(psakeys)).ok()
    }
    pub unsafe fn PopulateByQuery<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrquerystring: Param0, lquerytype: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PopulateByQuery)(::windows_core::Interface::as_raw(self), bstrquerystring.into_param().abi(), ::core::mem::transmute(lquerytype)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICatalogCollection> for ::windows_core::IUnknown {
    fn from(value: ICatalogCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICatalogCollection> for ::windows_core::IUnknown {
    fn from(value: &ICatalogCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICatalogCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICatalogCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICatalogCollection> for super::Com::IDispatch {
    fn from(value: ICatalogCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICatalogCollection> for super::Com::IDispatch {
    fn from(value: &ICatalogCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ICatalogCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ICatalogCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ICatalogCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICatalogCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICatalogCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICatalogCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICatalogCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ICatalogCollection {
    type Vtable = ICatalogCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6eb22872_8a19_11d0_81b6_00a0c9231c29);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICatalogCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, ppcatalogobject: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plobjectcount: *mut i32) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcatalogobject: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Populate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SaveChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchanges: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcollname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varobjectkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppcatalogcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetCollection: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarnamel: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Name: usize,
    pub AddEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarbool: *mut i16) -> ::windows_core::HRESULT,
    pub RemoveEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarbool: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetUtilInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppidispatch: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetUtilInterface: usize,
    pub DataStoreMajorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows_core::HRESULT,
    pub DataStoreMinorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plminorversionl: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PopulateByKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psakeys: *const super::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PopulateByKey: usize,
    pub PopulateByQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrquerystring: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lquerytype: i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ICatalogObject(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ICatalogObject {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Value<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpropname: Param0) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Value)(::windows_core::Interface::as_raw(self), bstrpropname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn put_Value<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, bstrpropname: Param0, val: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_Value)(::windows_core::Interface::as_raw(self), bstrpropname.into_param().abi(), val.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Key(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Key)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Name(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn IsPropertyReadOnly<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpropname: Param0) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsPropertyReadOnly)(::windows_core::Interface::as_raw(self), bstrpropname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Valid(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Valid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsPropertyWriteOnly<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpropname: Param0) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsPropertyWriteOnly)(::windows_core::Interface::as_raw(self), bstrpropname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICatalogObject> for ::windows_core::IUnknown {
    fn from(value: ICatalogObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICatalogObject> for ::windows_core::IUnknown {
    fn from(value: &ICatalogObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICatalogObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICatalogObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICatalogObject> for super::Com::IDispatch {
    fn from(value: ICatalogObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICatalogObject> for super::Com::IDispatch {
    fn from(value: &ICatalogObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ICatalogObject {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ICatalogObject {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ICatalogObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICatalogObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICatalogObject {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICatalogObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICatalogObject").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ICatalogObject {
    type Vtable = ICatalogObject_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6eb22871_8a19_11d0_81b6_00a0c9231c29);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICatalogObject_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pvarretval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Value: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub put_Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, val: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    put_Value: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Key: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarretval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Key: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarretval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Name: usize,
    pub IsPropertyReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pbretval: *mut i16) -> ::windows_core::HRESULT,
    pub Valid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbretval: *mut i16) -> ::windows_core::HRESULT,
    pub IsPropertyWriteOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pbretval: *mut i16) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICheckSxsConfig(::windows_core::IUnknown);
impl ICheckSxsConfig {
    pub unsafe fn IsSameSxsConfig<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, wszsxsname: Param0, wszsxsdirectory: Param1, wszsxsappname: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsSameSxsConfig)(::windows_core::Interface::as_raw(self), wszsxsname.into_param().abi(), wszsxsdirectory.into_param().abi(), wszsxsappname.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ICheckSxsConfig> for ::windows_core::IUnknown {
    fn from(value: ICheckSxsConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICheckSxsConfig> for ::windows_core::IUnknown {
    fn from(value: &ICheckSxsConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICheckSxsConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICheckSxsConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICheckSxsConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICheckSxsConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICheckSxsConfig {}
impl ::core::fmt::Debug for ICheckSxsConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICheckSxsConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICheckSxsConfig {
    type Vtable = ICheckSxsConfig_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0ff5a96f_11fc_47d1_baa6_25dd347e7242);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICheckSxsConfig_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub IsSameSxsConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszsxsname: ::windows_core::PCWSTR, wszsxsdirectory: ::windows_core::PCWSTR, wszsxsappname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComActivityEvents(::windows_core::IUnknown);
impl IComActivityEvents {
    pub unsafe fn OnActivityCreate(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnActivityCreate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity)).ok()
    }
    pub unsafe fn OnActivityDestroy(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnActivityDestroy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity)).ok()
    }
    pub unsafe fn OnActivityEnter(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, guidentered: *const ::windows_core::GUID, dwthread: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnActivityEnter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidcurrent), ::core::mem::transmute(guidentered), ::core::mem::transmute(dwthread)).ok()
    }
    pub unsafe fn OnActivityTimeout(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, guidentered: *const ::windows_core::GUID, dwthread: u32, dwtimeout: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnActivityTimeout)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidcurrent), ::core::mem::transmute(guidentered), ::core::mem::transmute(dwthread), ::core::mem::transmute(dwtimeout)).ok()
    }
    pub unsafe fn OnActivityReenter(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, dwthread: u32, dwcalldepth: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnActivityReenter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidcurrent), ::core::mem::transmute(dwthread), ::core::mem::transmute(dwcalldepth)).ok()
    }
    pub unsafe fn OnActivityLeave(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, guidleft: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnActivityLeave)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidcurrent), ::core::mem::transmute(guidleft)).ok()
    }
    pub unsafe fn OnActivityLeaveSame(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, dwcalldepth: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnActivityLeaveSame)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidcurrent), ::core::mem::transmute(dwcalldepth)).ok()
    }
}
impl ::core::convert::From<IComActivityEvents> for ::windows_core::IUnknown {
    fn from(value: IComActivityEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComActivityEvents> for ::windows_core::IUnknown {
    fn from(value: &IComActivityEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComActivityEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComActivityEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComActivityEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComActivityEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComActivityEvents {}
impl ::core::fmt::Debug for IComActivityEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComActivityEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComActivityEvents {
    type Vtable = IComActivityEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x683130b0_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComActivityEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnActivityCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnActivityDestroy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnActivityEnter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, guidentered: *const ::windows_core::GUID, dwthread: u32) -> ::windows_core::HRESULT,
    pub OnActivityTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, guidentered: *const ::windows_core::GUID, dwthread: u32, dwtimeout: u32) -> ::windows_core::HRESULT,
    pub OnActivityReenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, dwthread: u32, dwcalldepth: u32) -> ::windows_core::HRESULT,
    pub OnActivityLeave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, guidleft: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnActivityLeaveSame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, dwcalldepth: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComApp2Events(::windows_core::IUnknown);
impl IComApp2Events {
    pub unsafe fn OnAppActivation2<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param2: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: Param1, guidprocess: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnAppActivation2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidapp.into_param().abi(), guidprocess.into_param().abi()).ok()
    }
    pub unsafe fn OnAppShutdown2<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnAppShutdown2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidapp.into_param().abi()).ok()
    }
    pub unsafe fn OnAppForceShutdown2<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnAppForceShutdown2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidapp.into_param().abi()).ok()
    }
    pub unsafe fn OnAppPaused2<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: Param1, bpaused: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnAppPaused2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidapp.into_param().abi(), bpaused.into_param().abi()).ok()
    }
    pub unsafe fn OnAppRecycle2<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param2: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: Param1, guidprocess: Param2, lreason: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnAppRecycle2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidapp.into_param().abi(), guidprocess.into_param().abi(), ::core::mem::transmute(lreason)).ok()
    }
}
impl ::core::convert::From<IComApp2Events> for ::windows_core::IUnknown {
    fn from(value: IComApp2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComApp2Events> for ::windows_core::IUnknown {
    fn from(value: &IComApp2Events) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComApp2Events {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComApp2Events {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComApp2Events {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComApp2Events {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComApp2Events {}
impl ::core::fmt::Debug for IComApp2Events {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComApp2Events").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComApp2Events {
    type Vtable = IComApp2Events_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1290bc1a_b219_418d_b078_5934ded08242);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComApp2Events_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnAppActivation2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID, guidprocess: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnAppShutdown2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnAppForceShutdown2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnAppPaused2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID, bpaused: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub OnAppRecycle2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID, guidprocess: ::windows_core::GUID, lreason: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComAppEvents(::windows_core::IUnknown);
impl IComAppEvents {
    pub unsafe fn OnAppActivation<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnAppActivation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidapp.into_param().abi()).ok()
    }
    pub unsafe fn OnAppShutdown<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnAppShutdown)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidapp.into_param().abi()).ok()
    }
    pub unsafe fn OnAppForceShutdown<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnAppForceShutdown)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidapp.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IComAppEvents> for ::windows_core::IUnknown {
    fn from(value: IComAppEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComAppEvents> for ::windows_core::IUnknown {
    fn from(value: &IComAppEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComAppEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComAppEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComAppEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComAppEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComAppEvents {}
impl ::core::fmt::Debug for IComAppEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComAppEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComAppEvents {
    type Vtable = IComAppEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x683130a6_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComAppEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnAppActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnAppForceShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComCRMEvents(::windows_core::IUnknown);
impl IComCRMEvents {
    pub unsafe fn OnCRMRecoveryStart<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnCRMRecoveryStart)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidapp.into_param().abi()).ok()
    }
    pub unsafe fn OnCRMRecoveryDone<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnCRMRecoveryDone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidapp.into_param().abi()).ok()
    }
    pub unsafe fn OnCRMCheckpoint<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnCRMCheckpoint)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidapp.into_param().abi()).ok()
    }
    pub unsafe fn OnCRMBegin<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param2: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param3: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: Param1, guidactivity: Param2, guidtx: Param3, szprogidcompensator: &[u16; 64], szdescription: &[u16; 64]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnCRMBegin)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidclerkclsid.into_param().abi(), guidactivity.into_param().abi(), guidtx.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(szprogidcompensator)), ::core::mem::transmute(::windows_core::as_ptr_or_null(szdescription))).ok()
    }
    pub unsafe fn OnCRMPrepare<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnCRMPrepare)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidclerkclsid.into_param().abi()).ok()
    }
    pub unsafe fn OnCRMCommit<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnCRMCommit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidclerkclsid.into_param().abi()).ok()
    }
    pub unsafe fn OnCRMAbort<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnCRMAbort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidclerkclsid.into_param().abi()).ok()
    }
    pub unsafe fn OnCRMIndoubt<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnCRMIndoubt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidclerkclsid.into_param().abi()).ok()
    }
    pub unsafe fn OnCRMDone<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnCRMDone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidclerkclsid.into_param().abi()).ok()
    }
    pub unsafe fn OnCRMRelease<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnCRMRelease)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidclerkclsid.into_param().abi()).ok()
    }
    pub unsafe fn OnCRMAnalyze<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: Param1, dwcrmrecordtype: u32, dwrecordsize: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnCRMAnalyze)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidclerkclsid.into_param().abi(), ::core::mem::transmute(dwcrmrecordtype), ::core::mem::transmute(dwrecordsize)).ok()
    }
    pub unsafe fn OnCRMWrite<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: Param1, fvariants: Param2, dwrecordsize: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnCRMWrite)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidclerkclsid.into_param().abi(), fvariants.into_param().abi(), ::core::mem::transmute(dwrecordsize)).ok()
    }
    pub unsafe fn OnCRMForget<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnCRMForget)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidclerkclsid.into_param().abi()).ok()
    }
    pub unsafe fn OnCRMForce<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnCRMForce)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidclerkclsid.into_param().abi()).ok()
    }
    pub unsafe fn OnCRMDeliver<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: Param1, fvariants: Param2, dwrecordsize: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnCRMDeliver)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidclerkclsid.into_param().abi(), fvariants.into_param().abi(), ::core::mem::transmute(dwrecordsize)).ok()
    }
}
impl ::core::convert::From<IComCRMEvents> for ::windows_core::IUnknown {
    fn from(value: IComCRMEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComCRMEvents> for ::windows_core::IUnknown {
    fn from(value: &IComCRMEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComCRMEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComCRMEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComCRMEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComCRMEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComCRMEvents {}
impl ::core::fmt::Debug for IComCRMEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComCRMEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComCRMEvents {
    type Vtable = IComCRMEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x683130b5_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComCRMEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnCRMRecoveryStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnCRMRecoveryDone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnCRMCheckpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnCRMBegin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID, guidactivity: ::windows_core::GUID, guidtx: ::windows_core::GUID, szprogidcompensator: ::windows_core::PCWSTR, szdescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub OnCRMPrepare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnCRMCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnCRMAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnCRMIndoubt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnCRMDone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnCRMRelease: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnCRMAnalyze: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID, dwcrmrecordtype: u32, dwrecordsize: u32) -> ::windows_core::HRESULT,
    pub OnCRMWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID, fvariants: ::win32_foundation::BOOL, dwrecordsize: u32) -> ::windows_core::HRESULT,
    pub OnCRMForget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnCRMForce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnCRMDeliver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID, fvariants: ::win32_foundation::BOOL, dwrecordsize: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComExceptionEvents(::windows_core::IUnknown);
impl IComExceptionEvents {
    pub unsafe fn OnExceptionUser<'a, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pinfo: *const COMSVCSEVENTINFO, code: u32, address: u64, pszstacktrace: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnExceptionUser)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(code), ::core::mem::transmute(address), pszstacktrace.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IComExceptionEvents> for ::windows_core::IUnknown {
    fn from(value: IComExceptionEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComExceptionEvents> for ::windows_core::IUnknown {
    fn from(value: &IComExceptionEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComExceptionEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComExceptionEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComExceptionEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComExceptionEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComExceptionEvents {}
impl ::core::fmt::Debug for IComExceptionEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComExceptionEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComExceptionEvents {
    type Vtable = IComExceptionEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x683130b3_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComExceptionEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnExceptionUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, code: u32, address: u64, pszstacktrace: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComIdentityEvents(::windows_core::IUnknown);
impl IComIdentityEvents {
    pub unsafe fn OnIISRequestInfo<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param4: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, pszclientip: Param2, pszserverip: Param3, pszurl: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnIISRequestInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(objid), pszclientip.into_param().abi(), pszserverip.into_param().abi(), pszurl.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IComIdentityEvents> for ::windows_core::IUnknown {
    fn from(value: IComIdentityEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComIdentityEvents> for ::windows_core::IUnknown {
    fn from(value: &IComIdentityEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComIdentityEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComIdentityEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComIdentityEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComIdentityEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComIdentityEvents {}
impl ::core::fmt::Debug for IComIdentityEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComIdentityEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComIdentityEvents {
    type Vtable = IComIdentityEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x683130b1_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComIdentityEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnIISRequestInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, pszclientip: ::windows_core::PCWSTR, pszserverip: ::windows_core::PCWSTR, pszurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComInstance2Events(::windows_core::IUnknown);
impl IComInstance2Events {
    pub unsafe fn OnObjectCreate2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, clsid: *const ::windows_core::GUID, tsid: *const ::windows_core::GUID, ctxtid: u64, objectid: u64, guidpartition: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnObjectCreate2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(clsid), ::core::mem::transmute(tsid), ::core::mem::transmute(ctxtid), ::core::mem::transmute(objectid), ::core::mem::transmute(guidpartition)).ok()
    }
    pub unsafe fn OnObjectDestroy2(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnObjectDestroy2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(ctxtid)).ok()
    }
}
impl ::core::convert::From<IComInstance2Events> for ::windows_core::IUnknown {
    fn from(value: IComInstance2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComInstance2Events> for ::windows_core::IUnknown {
    fn from(value: &IComInstance2Events) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComInstance2Events {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComInstance2Events {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComInstance2Events {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComInstance2Events {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComInstance2Events {}
impl ::core::fmt::Debug for IComInstance2Events {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComInstance2Events").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComInstance2Events {
    type Vtable = IComInstance2Events_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20e3bf07_b506_4ad5_a50c_d2ca5b9c158e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComInstance2Events_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnObjectCreate2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, clsid: *const ::windows_core::GUID, tsid: *const ::windows_core::GUID, ctxtid: u64, objectid: u64, guidpartition: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnObjectDestroy2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComInstanceEvents(::windows_core::IUnknown);
impl IComInstanceEvents {
    pub unsafe fn OnObjectCreate(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, clsid: *const ::windows_core::GUID, tsid: *const ::windows_core::GUID, ctxtid: u64, objectid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnObjectCreate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(clsid), ::core::mem::transmute(tsid), ::core::mem::transmute(ctxtid), ::core::mem::transmute(objectid)).ok()
    }
    pub unsafe fn OnObjectDestroy(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnObjectDestroy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(ctxtid)).ok()
    }
}
impl ::core::convert::From<IComInstanceEvents> for ::windows_core::IUnknown {
    fn from(value: IComInstanceEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComInstanceEvents> for ::windows_core::IUnknown {
    fn from(value: &IComInstanceEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComInstanceEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComInstanceEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComInstanceEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComInstanceEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComInstanceEvents {}
impl ::core::fmt::Debug for IComInstanceEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComInstanceEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComInstanceEvents {
    type Vtable = IComInstanceEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x683130a7_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComInstanceEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnObjectCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, clsid: *const ::windows_core::GUID, tsid: *const ::windows_core::GUID, ctxtid: u64, objectid: u64) -> ::windows_core::HRESULT,
    pub OnObjectDestroy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComLTxEvents(::windows_core::IUnknown);
impl IComLTxEvents {
    pub unsafe fn OnLtxTransactionStart<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param2: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: Param1, tsid: Param2, froot: Param3, nisolationlevel: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnLtxTransactionStart)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidltx.into_param().abi(), tsid.into_param().abi(), froot.into_param().abi(), ::core::mem::transmute(nisolationlevel)).ok()
    }
    pub unsafe fn OnLtxTransactionPrepare<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: Param1, fvote: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnLtxTransactionPrepare)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidltx.into_param().abi(), fvote.into_param().abi()).ok()
    }
    pub unsafe fn OnLtxTransactionAbort<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnLtxTransactionAbort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidltx.into_param().abi()).ok()
    }
    pub unsafe fn OnLtxTransactionCommit<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnLtxTransactionCommit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidltx.into_param().abi()).ok()
    }
    pub unsafe fn OnLtxTransactionPromote<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param2: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: Param1, txnid: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnLtxTransactionPromote)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), guidltx.into_param().abi(), txnid.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IComLTxEvents> for ::windows_core::IUnknown {
    fn from(value: IComLTxEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComLTxEvents> for ::windows_core::IUnknown {
    fn from(value: &IComLTxEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComLTxEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComLTxEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComLTxEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComLTxEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComLTxEvents {}
impl ::core::fmt::Debug for IComLTxEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComLTxEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComLTxEvents {
    type Vtable = IComLTxEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x605cf82c_578e_4298_975d_82babcd9e053);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComLTxEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnLtxTransactionStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows_core::GUID, tsid: ::windows_core::GUID, froot: ::win32_foundation::BOOL, nisolationlevel: i32) -> ::windows_core::HRESULT,
    pub OnLtxTransactionPrepare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows_core::GUID, fvote: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub OnLtxTransactionAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnLtxTransactionCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnLtxTransactionPromote: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows_core::GUID, txnid: ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComMethod2Events(::windows_core::IUnknown);
impl IComMethod2Events {
    pub unsafe fn OnMethodCall2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, dwthread: u32, imeth: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnMethodCall2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(oid), ::core::mem::transmute(guidcid), ::core::mem::transmute(guidrid), ::core::mem::transmute(dwthread), ::core::mem::transmute(imeth)).ok()
    }
    pub unsafe fn OnMethodReturn2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, dwthread: u32, imeth: u32, hresult: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnMethodReturn2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(oid), ::core::mem::transmute(guidcid), ::core::mem::transmute(guidrid), ::core::mem::transmute(dwthread), ::core::mem::transmute(imeth), ::core::mem::transmute(hresult)).ok()
    }
    pub unsafe fn OnMethodException2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, dwthread: u32, imeth: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnMethodException2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(oid), ::core::mem::transmute(guidcid), ::core::mem::transmute(guidrid), ::core::mem::transmute(dwthread), ::core::mem::transmute(imeth)).ok()
    }
}
impl ::core::convert::From<IComMethod2Events> for ::windows_core::IUnknown {
    fn from(value: IComMethod2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComMethod2Events> for ::windows_core::IUnknown {
    fn from(value: &IComMethod2Events) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComMethod2Events {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComMethod2Events {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComMethod2Events {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComMethod2Events {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComMethod2Events {}
impl ::core::fmt::Debug for IComMethod2Events {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComMethod2Events").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComMethod2Events {
    type Vtable = IComMethod2Events_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb388aaa_567d_4024_af8e_6e93ee748573);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComMethod2Events_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnMethodCall2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, dwthread: u32, imeth: u32) -> ::windows_core::HRESULT,
    pub OnMethodReturn2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, dwthread: u32, imeth: u32, hresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub OnMethodException2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, dwthread: u32, imeth: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComMethodEvents(::windows_core::IUnknown);
impl IComMethodEvents {
    pub unsafe fn OnMethodCall(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, imeth: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnMethodCall)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(oid), ::core::mem::transmute(guidcid), ::core::mem::transmute(guidrid), ::core::mem::transmute(imeth)).ok()
    }
    pub unsafe fn OnMethodReturn(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, imeth: u32, hresult: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnMethodReturn)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(oid), ::core::mem::transmute(guidcid), ::core::mem::transmute(guidrid), ::core::mem::transmute(imeth), ::core::mem::transmute(hresult)).ok()
    }
    pub unsafe fn OnMethodException(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, imeth: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnMethodException)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(oid), ::core::mem::transmute(guidcid), ::core::mem::transmute(guidrid), ::core::mem::transmute(imeth)).ok()
    }
}
impl ::core::convert::From<IComMethodEvents> for ::windows_core::IUnknown {
    fn from(value: IComMethodEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComMethodEvents> for ::windows_core::IUnknown {
    fn from(value: &IComMethodEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComMethodEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComMethodEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComMethodEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComMethodEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComMethodEvents {}
impl ::core::fmt::Debug for IComMethodEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComMethodEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComMethodEvents {
    type Vtable = IComMethodEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x683130a9_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComMethodEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnMethodCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, imeth: u32) -> ::windows_core::HRESULT,
    pub OnMethodReturn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, imeth: u32, hresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub OnMethodException: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, imeth: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComMtaThreadPoolKnobs(::windows_core::IUnknown);
impl IComMtaThreadPoolKnobs {
    pub unsafe fn MTASetMaxThreadCount(&self, dwmaxthreads: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MTASetMaxThreadCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwmaxthreads)).ok()
    }
    pub unsafe fn MTAGetMaxThreadCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).MTAGetMaxThreadCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn MTASetThrottleValue(&self, dwthrottle: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MTASetThrottleValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwthrottle)).ok()
    }
    pub unsafe fn MTAGetThrottleValue(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).MTAGetThrottleValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IComMtaThreadPoolKnobs> for ::windows_core::IUnknown {
    fn from(value: IComMtaThreadPoolKnobs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComMtaThreadPoolKnobs> for ::windows_core::IUnknown {
    fn from(value: &IComMtaThreadPoolKnobs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComMtaThreadPoolKnobs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComMtaThreadPoolKnobs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComMtaThreadPoolKnobs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComMtaThreadPoolKnobs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComMtaThreadPoolKnobs {}
impl ::core::fmt::Debug for IComMtaThreadPoolKnobs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComMtaThreadPoolKnobs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComMtaThreadPoolKnobs {
    type Vtable = IComMtaThreadPoolKnobs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf9a76d2e_76a5_43eb_a0c4_49bec8e48480);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComMtaThreadPoolKnobs_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub MTASetMaxThreadCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxthreads: u32) -> ::windows_core::HRESULT,
    pub MTAGetMaxThreadCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmaxthreads: *mut u32) -> ::windows_core::HRESULT,
    pub MTASetThrottleValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwthrottle: u32) -> ::windows_core::HRESULT,
    pub MTAGetThrottleValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwthrottle: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComObjectConstruction2Events(::windows_core::IUnknown);
impl IComObjectConstruction2Events {
    pub unsafe fn OnObjectConstruct2<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, sconstructstring: Param2, oid: u64, guidpartition: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnObjectConstruct2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidobject), sconstructstring.into_param().abi(), ::core::mem::transmute(oid), ::core::mem::transmute(guidpartition)).ok()
    }
}
impl ::core::convert::From<IComObjectConstruction2Events> for ::windows_core::IUnknown {
    fn from(value: IComObjectConstruction2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComObjectConstruction2Events> for ::windows_core::IUnknown {
    fn from(value: &IComObjectConstruction2Events) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComObjectConstruction2Events {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComObjectConstruction2Events {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComObjectConstruction2Events {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComObjectConstruction2Events {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComObjectConstruction2Events {}
impl ::core::fmt::Debug for IComObjectConstruction2Events {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComObjectConstruction2Events").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComObjectConstruction2Events {
    type Vtable = IComObjectConstruction2Events_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4b5a7827_8df2_45c0_8f6f_57ea1f856a9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComObjectConstruction2Events_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnObjectConstruct2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, sconstructstring: ::windows_core::PCWSTR, oid: u64, guidpartition: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComObjectConstructionEvents(::windows_core::IUnknown);
impl IComObjectConstructionEvents {
    pub unsafe fn OnObjectConstruct<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, sconstructstring: Param2, oid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnObjectConstruct)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidobject), sconstructstring.into_param().abi(), ::core::mem::transmute(oid)).ok()
    }
}
impl ::core::convert::From<IComObjectConstructionEvents> for ::windows_core::IUnknown {
    fn from(value: IComObjectConstructionEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComObjectConstructionEvents> for ::windows_core::IUnknown {
    fn from(value: &IComObjectConstructionEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComObjectConstructionEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComObjectConstructionEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComObjectConstructionEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComObjectConstructionEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComObjectConstructionEvents {}
impl ::core::fmt::Debug for IComObjectConstructionEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComObjectConstructionEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComObjectConstructionEvents {
    type Vtable = IComObjectConstructionEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x683130af_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComObjectConstructionEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnObjectConstruct: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, sconstructstring: ::windows_core::PCWSTR, oid: u64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComObjectEvents(::windows_core::IUnknown);
impl IComObjectEvents {
    pub unsafe fn OnObjectActivate(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnObjectActivate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(ctxtid), ::core::mem::transmute(objectid)).ok()
    }
    pub unsafe fn OnObjectDeactivate(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnObjectDeactivate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(ctxtid), ::core::mem::transmute(objectid)).ok()
    }
    pub unsafe fn OnDisableCommit(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnDisableCommit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(ctxtid)).ok()
    }
    pub unsafe fn OnEnableCommit(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnEnableCommit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(ctxtid)).ok()
    }
    pub unsafe fn OnSetComplete(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnSetComplete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(ctxtid)).ok()
    }
    pub unsafe fn OnSetAbort(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnSetAbort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(ctxtid)).ok()
    }
}
impl ::core::convert::From<IComObjectEvents> for ::windows_core::IUnknown {
    fn from(value: IComObjectEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComObjectEvents> for ::windows_core::IUnknown {
    fn from(value: &IComObjectEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComObjectEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComObjectEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComObjectEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComObjectEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComObjectEvents {}
impl ::core::fmt::Debug for IComObjectEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComObjectEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComObjectEvents {
    type Vtable = IComObjectEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x683130aa_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComObjectEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnObjectActivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows_core::HRESULT,
    pub OnObjectDeactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows_core::HRESULT,
    pub OnDisableCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::HRESULT,
    pub OnEnableCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::HRESULT,
    pub OnSetComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::HRESULT,
    pub OnSetAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComObjectPool2Events(::windows_core::IUnknown);
impl IComObjectPool2Events {
    pub unsafe fn OnObjPoolPutObject2(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnObjPoolPutObject2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidobject), ::core::mem::transmute(nreason), ::core::mem::transmute(dwavailable), ::core::mem::transmute(oid)).ok()
    }
    pub unsafe fn OnObjPoolGetObject2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, dwavailable: u32, oid: u64, guidpartition: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnObjPoolGetObject2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(guidobject), ::core::mem::transmute(dwavailable), ::core::mem::transmute(oid), ::core::mem::transmute(guidpartition)).ok()
    }
    pub unsafe fn OnObjPoolRecycleToTx2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, guidtx: *const ::windows_core::GUID, objid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnObjPoolRecycleToTx2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(guidobject), ::core::mem::transmute(guidtx), ::core::mem::transmute(objid)).ok()
    }
    pub unsafe fn OnObjPoolGetFromTx2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, guidtx: *const ::windows_core::GUID, objid: u64, guidpartition: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnObjPoolGetFromTx2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(guidobject), ::core::mem::transmute(guidtx), ::core::mem::transmute(objid), ::core::mem::transmute(guidpartition)).ok()
    }
}
impl ::core::convert::From<IComObjectPool2Events> for ::windows_core::IUnknown {
    fn from(value: IComObjectPool2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComObjectPool2Events> for ::windows_core::IUnknown {
    fn from(value: &IComObjectPool2Events) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComObjectPool2Events {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComObjectPool2Events {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComObjectPool2Events {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComObjectPool2Events {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComObjectPool2Events {}
impl ::core::fmt::Debug for IComObjectPool2Events {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComObjectPool2Events").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComObjectPool2Events {
    type Vtable = IComObjectPool2Events_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x65bf6534_85ea_4f64_8cf4_3d974b2ab1cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComObjectPool2Events_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnObjPoolPutObject2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows_core::HRESULT,
    pub OnObjPoolGetObject2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, dwavailable: u32, oid: u64, guidpartition: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnObjPoolRecycleToTx2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, guidtx: *const ::windows_core::GUID, objid: u64) -> ::windows_core::HRESULT,
    pub OnObjPoolGetFromTx2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, guidtx: *const ::windows_core::GUID, objid: u64, guidpartition: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComObjectPoolEvents(::windows_core::IUnknown);
impl IComObjectPoolEvents {
    pub unsafe fn OnObjPoolPutObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnObjPoolPutObject)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidobject), ::core::mem::transmute(nreason), ::core::mem::transmute(dwavailable), ::core::mem::transmute(oid)).ok()
    }
    pub unsafe fn OnObjPoolGetObject(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, dwavailable: u32, oid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnObjPoolGetObject)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(guidobject), ::core::mem::transmute(dwavailable), ::core::mem::transmute(oid)).ok()
    }
    pub unsafe fn OnObjPoolRecycleToTx(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, guidtx: *const ::windows_core::GUID, objid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnObjPoolRecycleToTx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(guidobject), ::core::mem::transmute(guidtx), ::core::mem::transmute(objid)).ok()
    }
    pub unsafe fn OnObjPoolGetFromTx(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, guidtx: *const ::windows_core::GUID, objid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnObjPoolGetFromTx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(guidobject), ::core::mem::transmute(guidtx), ::core::mem::transmute(objid)).ok()
    }
}
impl ::core::convert::From<IComObjectPoolEvents> for ::windows_core::IUnknown {
    fn from(value: IComObjectPoolEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComObjectPoolEvents> for ::windows_core::IUnknown {
    fn from(value: &IComObjectPoolEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComObjectPoolEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComObjectPoolEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComObjectPoolEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComObjectPoolEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComObjectPoolEvents {}
impl ::core::fmt::Debug for IComObjectPoolEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComObjectPoolEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComObjectPoolEvents {
    type Vtable = IComObjectPoolEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x683130ad_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComObjectPoolEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnObjPoolPutObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows_core::HRESULT,
    pub OnObjPoolGetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, dwavailable: u32, oid: u64) -> ::windows_core::HRESULT,
    pub OnObjPoolRecycleToTx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, guidtx: *const ::windows_core::GUID, objid: u64) -> ::windows_core::HRESULT,
    pub OnObjPoolGetFromTx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, guidtx: *const ::windows_core::GUID, objid: u64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComObjectPoolEvents2(::windows_core::IUnknown);
impl IComObjectPoolEvents2 {
    pub unsafe fn OnObjPoolCreateObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, dwobjscreated: u32, oid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnObjPoolCreateObject)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidobject), ::core::mem::transmute(dwobjscreated), ::core::mem::transmute(oid)).ok()
    }
    pub unsafe fn OnObjPoolDestroyObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, dwobjscreated: u32, oid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnObjPoolDestroyObject)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidobject), ::core::mem::transmute(dwobjscreated), ::core::mem::transmute(oid)).ok()
    }
    pub unsafe fn OnObjPoolCreateDecision(&self, pinfo: *const COMSVCSEVENTINFO, dwthreadswaiting: u32, dwavail: u32, dwcreated: u32, dwmin: u32, dwmax: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnObjPoolCreateDecision)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(dwthreadswaiting), ::core::mem::transmute(dwavail), ::core::mem::transmute(dwcreated), ::core::mem::transmute(dwmin), ::core::mem::transmute(dwmax)).ok()
    }
    pub unsafe fn OnObjPoolTimeout(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, guidactivity: *const ::windows_core::GUID, dwtimeout: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnObjPoolTimeout)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidobject), ::core::mem::transmute(guidactivity), ::core::mem::transmute(dwtimeout)).ok()
    }
    pub unsafe fn OnObjPoolCreatePool(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, dwmin: u32, dwmax: u32, dwtimeout: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnObjPoolCreatePool)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidobject), ::core::mem::transmute(dwmin), ::core::mem::transmute(dwmax), ::core::mem::transmute(dwtimeout)).ok()
    }
}
impl ::core::convert::From<IComObjectPoolEvents2> for ::windows_core::IUnknown {
    fn from(value: IComObjectPoolEvents2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComObjectPoolEvents2> for ::windows_core::IUnknown {
    fn from(value: &IComObjectPoolEvents2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComObjectPoolEvents2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComObjectPoolEvents2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComObjectPoolEvents2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComObjectPoolEvents2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComObjectPoolEvents2 {}
impl ::core::fmt::Debug for IComObjectPoolEvents2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComObjectPoolEvents2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComObjectPoolEvents2 {
    type Vtable = IComObjectPoolEvents2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x683130ae_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComObjectPoolEvents2_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnObjPoolCreateObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, dwobjscreated: u32, oid: u64) -> ::windows_core::HRESULT,
    pub OnObjPoolDestroyObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, dwobjscreated: u32, oid: u64) -> ::windows_core::HRESULT,
    pub OnObjPoolCreateDecision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, dwthreadswaiting: u32, dwavail: u32, dwcreated: u32, dwmin: u32, dwmax: u32) -> ::windows_core::HRESULT,
    pub OnObjPoolTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, guidactivity: *const ::windows_core::GUID, dwtimeout: u32) -> ::windows_core::HRESULT,
    pub OnObjPoolCreatePool: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, dwmin: u32, dwmax: u32, dwtimeout: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComQCEvents(::windows_core::IUnknown);
impl IComQCEvents {
    pub unsafe fn OnQCRecord(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, szqueue: &[u16; 60], guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID, msmqhr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnQCRecord)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(objid), ::core::mem::transmute(::windows_core::as_ptr_or_null(szqueue)), ::core::mem::transmute(guidmsgid), ::core::mem::transmute(guidworkflowid), ::core::mem::transmute(msmqhr)).ok()
    }
    pub unsafe fn OnQCQueueOpen(&self, pinfo: *const COMSVCSEVENTINFO, szqueue: &[u16; 60], queueid: u64, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnQCQueueOpen)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(::windows_core::as_ptr_or_null(szqueue)), ::core::mem::transmute(queueid), ::core::mem::transmute(hr)).ok()
    }
    pub unsafe fn OnQCReceive(&self, pinfo: *const COMSVCSEVENTINFO, queueid: u64, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnQCReceive)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(queueid), ::core::mem::transmute(guidmsgid), ::core::mem::transmute(guidworkflowid), ::core::mem::transmute(hr)).ok()
    }
    pub unsafe fn OnQCReceiveFail(&self, pinfo: *const COMSVCSEVENTINFO, queueid: u64, msmqhr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnQCReceiveFail)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(queueid), ::core::mem::transmute(msmqhr)).ok()
    }
    pub unsafe fn OnQCMoveToReTryQueue(&self, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID, retryindex: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnQCMoveToReTryQueue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidmsgid), ::core::mem::transmute(guidworkflowid), ::core::mem::transmute(retryindex)).ok()
    }
    pub unsafe fn OnQCMoveToDeadQueue(&self, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnQCMoveToDeadQueue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidmsgid), ::core::mem::transmute(guidworkflowid)).ok()
    }
    pub unsafe fn OnQCPlayback(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnQCPlayback)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(objid), ::core::mem::transmute(guidmsgid), ::core::mem::transmute(guidworkflowid), ::core::mem::transmute(hr)).ok()
    }
}
impl ::core::convert::From<IComQCEvents> for ::windows_core::IUnknown {
    fn from(value: IComQCEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComQCEvents> for ::windows_core::IUnknown {
    fn from(value: &IComQCEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComQCEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComQCEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComQCEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComQCEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComQCEvents {}
impl ::core::fmt::Debug for IComQCEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComQCEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComQCEvents {
    type Vtable = IComQCEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x683130b2_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComQCEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnQCRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, szqueue: ::windows_core::PCWSTR, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID, msmqhr: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub OnQCQueueOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, szqueue: ::windows_core::PCWSTR, queueid: u64, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub OnQCReceive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, queueid: u64, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub OnQCReceiveFail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, queueid: u64, msmqhr: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub OnQCMoveToReTryQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID, retryindex: u32) -> ::windows_core::HRESULT,
    pub OnQCMoveToDeadQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnQCPlayback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComResourceEvents(::windows_core::IUnknown);
impl IComResourceEvents {
    pub unsafe fn OnResourceCreate<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: Param2, resid: u64, enlisted: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnResourceCreate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(objectid), psztype.into_param().abi(), ::core::mem::transmute(resid), enlisted.into_param().abi()).ok()
    }
    pub unsafe fn OnResourceAllocate<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: Param2, resid: u64, enlisted: Param4, numrated: u32, rating: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnResourceAllocate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(objectid), psztype.into_param().abi(), ::core::mem::transmute(resid), enlisted.into_param().abi(), ::core::mem::transmute(numrated), ::core::mem::transmute(rating)).ok()
    }
    pub unsafe fn OnResourceRecycle<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: Param2, resid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnResourceRecycle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(objectid), psztype.into_param().abi(), ::core::mem::transmute(resid)).ok()
    }
    pub unsafe fn OnResourceDestroy<'a, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, hr: ::windows_core::HRESULT, psztype: Param3, resid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnResourceDestroy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(objectid), ::core::mem::transmute(hr), psztype.into_param().abi(), ::core::mem::transmute(resid)).ok()
    }
    pub unsafe fn OnResourceTrack<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: Param2, resid: u64, enlisted: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnResourceTrack)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(objectid), psztype.into_param().abi(), ::core::mem::transmute(resid), enlisted.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IComResourceEvents> for ::windows_core::IUnknown {
    fn from(value: IComResourceEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComResourceEvents> for ::windows_core::IUnknown {
    fn from(value: &IComResourceEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComResourceEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComResourceEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComResourceEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComResourceEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComResourceEvents {}
impl ::core::fmt::Debug for IComResourceEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComResourceEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComResourceEvents {
    type Vtable = IComResourceEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x683130ab_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComResourceEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnResourceCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: ::windows_core::PCWSTR, resid: u64, enlisted: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub OnResourceAllocate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: ::windows_core::PCWSTR, resid: u64, enlisted: ::win32_foundation::BOOL, numrated: u32, rating: u32) -> ::windows_core::HRESULT,
    pub OnResourceRecycle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: ::windows_core::PCWSTR, resid: u64) -> ::windows_core::HRESULT,
    pub OnResourceDestroy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, hr: ::windows_core::HRESULT, psztype: ::windows_core::PCWSTR, resid: u64) -> ::windows_core::HRESULT,
    pub OnResourceTrack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: ::windows_core::PCWSTR, resid: u64, enlisted: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComSecurityEvents(::windows_core::IUnknown);
impl IComSecurityEvents {
    pub unsafe fn OnAuthenticate<'a, Param9: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, objectid: u64, guidiid: *const ::windows_core::GUID, imeth: u32, psidoriginaluser: &[u8], psidcurrentuser: &[u8], bcurrentuserinpersonatinginproc: Param9) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnAuthenticate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(objectid), ::core::mem::transmute(guidiid), ::core::mem::transmute(imeth), psidoriginaluser.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(psidoriginaluser)), psidcurrentuser.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(psidcurrentuser)), bcurrentuserinpersonatinginproc.into_param().abi()).ok()
    }
    pub unsafe fn OnAuthenticateFail<'a, Param9: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, objectid: u64, guidiid: *const ::windows_core::GUID, imeth: u32, psidoriginaluser: &[u8], psidcurrentuser: &[u8], bcurrentuserinpersonatinginproc: Param9) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnAuthenticateFail)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(objectid), ::core::mem::transmute(guidiid), ::core::mem::transmute(imeth), psidoriginaluser.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(psidoriginaluser)), psidcurrentuser.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(psidcurrentuser)), bcurrentuserinpersonatinginproc.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IComSecurityEvents> for ::windows_core::IUnknown {
    fn from(value: IComSecurityEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComSecurityEvents> for ::windows_core::IUnknown {
    fn from(value: &IComSecurityEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComSecurityEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComSecurityEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComSecurityEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComSecurityEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComSecurityEvents {}
impl ::core::fmt::Debug for IComSecurityEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComSecurityEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComSecurityEvents {
    type Vtable = IComSecurityEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x683130ac_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComSecurityEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnAuthenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, objectid: u64, guidiid: *const ::windows_core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub OnAuthenticateFail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, objectid: u64, guidiid: *const ::windows_core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComStaThreadPoolKnobs(::windows_core::IUnknown);
impl IComStaThreadPoolKnobs {
    pub unsafe fn SetMinThreadCount(&self, minthreads: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMinThreadCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(minthreads)).ok()
    }
    pub unsafe fn GetMinThreadCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMinThreadCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMaxThreadCount(&self, maxthreads: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxThreadCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(maxthreads)).ok()
    }
    pub unsafe fn GetMaxThreadCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxThreadCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetActivityPerThread(&self, activitiesperthread: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetActivityPerThread)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(activitiesperthread)).ok()
    }
    pub unsafe fn GetActivityPerThread(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetActivityPerThread)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetActivityRatio(&self, activityratio: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetActivityRatio)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(activityratio)).ok()
    }
    pub unsafe fn GetActivityRatio(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).GetActivityRatio)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn GetThreadCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetThreadCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetQueueDepth(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetQueueDepth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetQueueDepth(&self, dwqdepth: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetQueueDepth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwqdepth)).ok()
    }
}
impl ::core::convert::From<IComStaThreadPoolKnobs> for ::windows_core::IUnknown {
    fn from(value: IComStaThreadPoolKnobs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComStaThreadPoolKnobs> for ::windows_core::IUnknown {
    fn from(value: &IComStaThreadPoolKnobs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComStaThreadPoolKnobs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComStaThreadPoolKnobs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComStaThreadPoolKnobs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComStaThreadPoolKnobs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComStaThreadPoolKnobs {}
impl ::core::fmt::Debug for IComStaThreadPoolKnobs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComStaThreadPoolKnobs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComStaThreadPoolKnobs {
    type Vtable = IComStaThreadPoolKnobs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x324b64fa_33b6_11d2_98b7_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComStaThreadPoolKnobs_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetMinThreadCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minthreads: u32) -> ::windows_core::HRESULT,
    pub GetMinThreadCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minthreads: *mut u32) -> ::windows_core::HRESULT,
    pub SetMaxThreadCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxthreads: u32) -> ::windows_core::HRESULT,
    pub GetMaxThreadCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxthreads: *mut u32) -> ::windows_core::HRESULT,
    pub SetActivityPerThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activitiesperthread: u32) -> ::windows_core::HRESULT,
    pub GetActivityPerThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activitiesperthread: *mut u32) -> ::windows_core::HRESULT,
    pub SetActivityRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityratio: f64) -> ::windows_core::HRESULT,
    pub GetActivityRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activityratio: *mut f64) -> ::windows_core::HRESULT,
    pub GetThreadCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwthreads: *mut u32) -> ::windows_core::HRESULT,
    pub GetQueueDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwqdepth: *mut u32) -> ::windows_core::HRESULT,
    pub SetQueueDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwqdepth: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComStaThreadPoolKnobs2(::windows_core::IUnknown);
impl IComStaThreadPoolKnobs2 {
    pub unsafe fn SetMinThreadCount(&self, minthreads: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMinThreadCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(minthreads)).ok()
    }
    pub unsafe fn GetMinThreadCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMinThreadCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMaxThreadCount(&self, maxthreads: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMaxThreadCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(maxthreads)).ok()
    }
    pub unsafe fn GetMaxThreadCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMaxThreadCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetActivityPerThread(&self, activitiesperthread: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetActivityPerThread)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(activitiesperthread)).ok()
    }
    pub unsafe fn GetActivityPerThread(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetActivityPerThread)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetActivityRatio(&self, activityratio: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetActivityRatio)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(activityratio)).ok()
    }
    pub unsafe fn GetActivityRatio(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetActivityRatio)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn GetThreadCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetThreadCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetQueueDepth(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetQueueDepth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetQueueDepth(&self, dwqdepth: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetQueueDepth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwqdepth)).ok()
    }
    pub unsafe fn GetMaxCPULoad(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxCPULoad)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMaxCPULoad(&self, pdwload: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxCPULoad)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwload)).ok()
    }
    pub unsafe fn GetCPUMetricEnabled(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetCPUMetricEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetCPUMetricEnabled<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bmetricenabled: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCPUMetricEnabled)(::windows_core::Interface::as_raw(self), bmetricenabled.into_param().abi()).ok()
    }
    pub unsafe fn GetCreateThreadsAggressively(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GetCreateThreadsAggressively)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetCreateThreadsAggressively<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bmetricenabled: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCreateThreadsAggressively)(::windows_core::Interface::as_raw(self), bmetricenabled.into_param().abi()).ok()
    }
    pub unsafe fn GetMaxCSR(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxCSR)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMaxCSR(&self, dwcsr: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxCSR)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwcsr)).ok()
    }
    pub unsafe fn GetWaitTimeForThreadCleanup(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetWaitTimeForThreadCleanup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetWaitTimeForThreadCleanup(&self, dwthreadcleanupwaittime: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWaitTimeForThreadCleanup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwthreadcleanupwaittime)).ok()
    }
}
impl ::core::convert::From<IComStaThreadPoolKnobs2> for ::windows_core::IUnknown {
    fn from(value: IComStaThreadPoolKnobs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComStaThreadPoolKnobs2> for ::windows_core::IUnknown {
    fn from(value: &IComStaThreadPoolKnobs2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComStaThreadPoolKnobs2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComStaThreadPoolKnobs2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IComStaThreadPoolKnobs2> for IComStaThreadPoolKnobs {
    fn from(value: IComStaThreadPoolKnobs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComStaThreadPoolKnobs2> for IComStaThreadPoolKnobs {
    fn from(value: &IComStaThreadPoolKnobs2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IComStaThreadPoolKnobs> for IComStaThreadPoolKnobs2 {
    fn into_param(self) -> ::windows_core::Param<'a, IComStaThreadPoolKnobs> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IComStaThreadPoolKnobs> for &'a IComStaThreadPoolKnobs2 {
    fn into_param(self) -> ::windows_core::Param<'a, IComStaThreadPoolKnobs> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComStaThreadPoolKnobs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComStaThreadPoolKnobs2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComStaThreadPoolKnobs2 {}
impl ::core::fmt::Debug for IComStaThreadPoolKnobs2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComStaThreadPoolKnobs2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComStaThreadPoolKnobs2 {
    type Vtable = IComStaThreadPoolKnobs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73707523_ff9a_4974_bf84_2108dc213740);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComStaThreadPoolKnobs2_Vtbl {
    pub base__: IComStaThreadPoolKnobs_Vtbl,
    pub GetMaxCPULoad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwload: *mut u32) -> ::windows_core::HRESULT,
    pub SetMaxCPULoad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwload: i32) -> ::windows_core::HRESULT,
    pub GetCPUMetricEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbmetricenabled: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetCPUMetricEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmetricenabled: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetCreateThreadsAggressively: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbmetricenabled: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetCreateThreadsAggressively: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmetricenabled: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetMaxCSR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcsr: *mut u32) -> ::windows_core::HRESULT,
    pub SetMaxCSR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcsr: i32) -> ::windows_core::HRESULT,
    pub GetWaitTimeForThreadCleanup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwthreadcleanupwaittime: *mut u32) -> ::windows_core::HRESULT,
    pub SetWaitTimeForThreadCleanup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwthreadcleanupwaittime: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComThreadEvents(::windows_core::IUnknown);
impl IComThreadEvents {
    pub unsafe fn OnThreadStart(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnThreadStart)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(threadid), ::core::mem::transmute(dwthread), ::core::mem::transmute(dwtheadcnt)).ok()
    }
    pub unsafe fn OnThreadTerminate(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnThreadTerminate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(threadid), ::core::mem::transmute(dwthread), ::core::mem::transmute(dwtheadcnt)).ok()
    }
    pub unsafe fn OnThreadBindToApartment(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32, dwlowcnt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnThreadBindToApartment)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(threadid), ::core::mem::transmute(aptid), ::core::mem::transmute(dwactcnt), ::core::mem::transmute(dwlowcnt)).ok()
    }
    pub unsafe fn OnThreadUnBind(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnThreadUnBind)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(threadid), ::core::mem::transmute(aptid), ::core::mem::transmute(dwactcnt)).ok()
    }
    pub unsafe fn OnThreadWorkEnque(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnThreadWorkEnque)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(threadid), ::core::mem::transmute(msgworkid), ::core::mem::transmute(queuelen)).ok()
    }
    pub unsafe fn OnThreadWorkPrivate(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnThreadWorkPrivate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(threadid), ::core::mem::transmute(msgworkid)).ok()
    }
    pub unsafe fn OnThreadWorkPublic(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnThreadWorkPublic)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(threadid), ::core::mem::transmute(msgworkid), ::core::mem::transmute(queuelen)).ok()
    }
    pub unsafe fn OnThreadWorkRedirect(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32, threadnum: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnThreadWorkRedirect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(threadid), ::core::mem::transmute(msgworkid), ::core::mem::transmute(queuelen), ::core::mem::transmute(threadnum)).ok()
    }
    pub unsafe fn OnThreadWorkReject(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnThreadWorkReject)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(threadid), ::core::mem::transmute(msgworkid), ::core::mem::transmute(queuelen)).ok()
    }
    pub unsafe fn OnThreadAssignApartment(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, aptid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnThreadAssignApartment)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidactivity), ::core::mem::transmute(aptid)).ok()
    }
    pub unsafe fn OnThreadUnassignApartment(&self, pinfo: *const COMSVCSEVENTINFO, aptid: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnThreadUnassignApartment)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(aptid)).ok()
    }
}
impl ::core::convert::From<IComThreadEvents> for ::windows_core::IUnknown {
    fn from(value: IComThreadEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComThreadEvents> for ::windows_core::IUnknown {
    fn from(value: &IComThreadEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComThreadEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComThreadEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComThreadEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComThreadEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComThreadEvents {}
impl ::core::fmt::Debug for IComThreadEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComThreadEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComThreadEvents {
    type Vtable = IComThreadEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x683130a5_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComThreadEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnThreadStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows_core::HRESULT,
    pub OnThreadTerminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows_core::HRESULT,
    pub OnThreadBindToApartment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32, dwlowcnt: u32) -> ::windows_core::HRESULT,
    pub OnThreadUnBind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32) -> ::windows_core::HRESULT,
    pub OnThreadWorkEnque: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows_core::HRESULT,
    pub OnThreadWorkPrivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64) -> ::windows_core::HRESULT,
    pub OnThreadWorkPublic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows_core::HRESULT,
    pub OnThreadWorkRedirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32, threadnum: u64) -> ::windows_core::HRESULT,
    pub OnThreadWorkReject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows_core::HRESULT,
    pub OnThreadAssignApartment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, aptid: u64) -> ::windows_core::HRESULT,
    pub OnThreadUnassignApartment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, aptid: u64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComTrackingInfoCollection(::windows_core::IUnknown);
impl IComTrackingInfoCollection {
    pub unsafe fn Type(&self) -> ::windows_core::Result<TRACKING_COLL_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::<TRACKING_COLL_TYPE>::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<TRACKING_COLL_TYPE>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn Item(&self, ulindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ulindex), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
}
impl ::core::convert::From<IComTrackingInfoCollection> for ::windows_core::IUnknown {
    fn from(value: IComTrackingInfoCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComTrackingInfoCollection> for ::windows_core::IUnknown {
    fn from(value: &IComTrackingInfoCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComTrackingInfoCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComTrackingInfoCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComTrackingInfoCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComTrackingInfoCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComTrackingInfoCollection {}
impl ::core::fmt::Debug for IComTrackingInfoCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComTrackingInfoCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComTrackingInfoCollection {
    type Vtable = IComTrackingInfoCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc266c677_c9ad_49ab_9fd9_d9661078588a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComTrackingInfoCollection_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut TRACKING_COLL_TYPE) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComTrackingInfoEvents(::windows_core::IUnknown);
impl IComTrackingInfoEvents {
    pub unsafe fn OnNewTrackingInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, ptoplevelcollection: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnNewTrackingInfo)(::windows_core::Interface::as_raw(self), ptoplevelcollection.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IComTrackingInfoEvents> for ::windows_core::IUnknown {
    fn from(value: IComTrackingInfoEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComTrackingInfoEvents> for ::windows_core::IUnknown {
    fn from(value: &IComTrackingInfoEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComTrackingInfoEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComTrackingInfoEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComTrackingInfoEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComTrackingInfoEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComTrackingInfoEvents {}
impl ::core::fmt::Debug for IComTrackingInfoEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComTrackingInfoEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComTrackingInfoEvents {
    type Vtable = IComTrackingInfoEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e6cdcc9_fb25_4fd5_9cc5_c9f4b6559cec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComTrackingInfoEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnNewTrackingInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptoplevelcollection: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComTrackingInfoObject(::windows_core::IUnknown);
impl IComTrackingInfoObject {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, szpropertyname: Param0) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetValue)(::windows_core::Interface::as_raw(self), szpropertyname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
}
impl ::core::convert::From<IComTrackingInfoObject> for ::windows_core::IUnknown {
    fn from(value: IComTrackingInfoObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComTrackingInfoObject> for ::windows_core::IUnknown {
    fn from(value: &IComTrackingInfoObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComTrackingInfoObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComTrackingInfoObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComTrackingInfoObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComTrackingInfoObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComTrackingInfoObject {}
impl ::core::fmt::Debug for IComTrackingInfoObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComTrackingInfoObject").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComTrackingInfoObject {
    type Vtable = IComTrackingInfoObject_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x116e42c5_d8b1_47bf_ab1e_c895ed3e2372);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComTrackingInfoObject_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szpropertyname: ::windows_core::PCWSTR, pvarout: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetValue: usize,
}
#[repr(transparent)]
pub struct IComTrackingInfoProperties(::windows_core::IUnknown);
impl IComTrackingInfoProperties {
    pub unsafe fn PropCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).PropCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPropName(&self, ulindex: u32) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetPropName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ulindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IComTrackingInfoProperties> for ::windows_core::IUnknown {
    fn from(value: IComTrackingInfoProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComTrackingInfoProperties> for ::windows_core::IUnknown {
    fn from(value: &IComTrackingInfoProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComTrackingInfoProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComTrackingInfoProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComTrackingInfoProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComTrackingInfoProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComTrackingInfoProperties {}
impl ::core::fmt::Debug for IComTrackingInfoProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComTrackingInfoProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComTrackingInfoProperties {
    type Vtable = IComTrackingInfoProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x789b42be_6f6b_443a_898e_67abf390aa14);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComTrackingInfoProperties_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub PropCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetPropName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulindex: u32, ppszpropname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComTransaction2Events(::windows_core::IUnknown);
impl IComTransaction2Events {
    pub unsafe fn OnTransactionStart2<'a, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID, tsid: *const ::windows_core::GUID, froot: Param3, nisolationlevel: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnTransactionStart2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidtx), ::core::mem::transmute(tsid), froot.into_param().abi(), ::core::mem::transmute(nisolationlevel)).ok()
    }
    pub unsafe fn OnTransactionPrepare2<'a, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID, fvoteyes: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnTransactionPrepare2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidtx), fvoteyes.into_param().abi()).ok()
    }
    pub unsafe fn OnTransactionAbort2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnTransactionAbort2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidtx)).ok()
    }
    pub unsafe fn OnTransactionCommit2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnTransactionCommit2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidtx)).ok()
    }
}
impl ::core::convert::From<IComTransaction2Events> for ::windows_core::IUnknown {
    fn from(value: IComTransaction2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComTransaction2Events> for ::windows_core::IUnknown {
    fn from(value: &IComTransaction2Events) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComTransaction2Events {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComTransaction2Events {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComTransaction2Events {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComTransaction2Events {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComTransaction2Events {}
impl ::core::fmt::Debug for IComTransaction2Events {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComTransaction2Events").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComTransaction2Events {
    type Vtable = IComTransaction2Events_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa136f62a_2f94_4288_86e0_d8a1fa4c0299);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComTransaction2Events_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnTransactionStart2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID, tsid: *const ::windows_core::GUID, froot: ::win32_foundation::BOOL, nisolationlevel: i32) -> ::windows_core::HRESULT,
    pub OnTransactionPrepare2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID, fvoteyes: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub OnTransactionAbort2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnTransactionCommit2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComTransactionEvents(::windows_core::IUnknown);
impl IComTransactionEvents {
    pub unsafe fn OnTransactionStart<'a, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID, tsid: *const ::windows_core::GUID, froot: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnTransactionStart)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidtx), ::core::mem::transmute(tsid), froot.into_param().abi()).ok()
    }
    pub unsafe fn OnTransactionPrepare<'a, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID, fvoteyes: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnTransactionPrepare)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidtx), fvoteyes.into_param().abi()).ok()
    }
    pub unsafe fn OnTransactionAbort(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnTransactionAbort)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidtx)).ok()
    }
    pub unsafe fn OnTransactionCommit(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnTransactionCommit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(guidtx)).ok()
    }
}
impl ::core::convert::From<IComTransactionEvents> for ::windows_core::IUnknown {
    fn from(value: IComTransactionEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComTransactionEvents> for ::windows_core::IUnknown {
    fn from(value: &IComTransactionEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComTransactionEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComTransactionEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComTransactionEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComTransactionEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComTransactionEvents {}
impl ::core::fmt::Debug for IComTransactionEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComTransactionEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComTransactionEvents {
    type Vtable = IComTransactionEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x683130a8_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComTransactionEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnTransactionStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID, tsid: *const ::windows_core::GUID, froot: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub OnTransactionPrepare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID, fvoteyes: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub OnTransactionAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnTransactionCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IComUserEvent(::windows_core::IUnknown);
impl IComUserEvent {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OnUserEvent(&self, pinfo: *const COMSVCSEVENTINFO, pvarevent: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnUserEvent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(pvarevent)).ok()
    }
}
impl ::core::convert::From<IComUserEvent> for ::windows_core::IUnknown {
    fn from(value: IComUserEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComUserEvent> for ::windows_core::IUnknown {
    fn from(value: &IComUserEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IComUserEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IComUserEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComUserEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComUserEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComUserEvent {}
impl ::core::fmt::Debug for IComUserEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComUserEvent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IComUserEvent {
    type Vtable = IComUserEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x683130a4_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComUserEvent_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OnUserEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, pvarevent: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OnUserEvent: usize,
}
#[repr(transparent)]
pub struct IContextProperties(::windows_core::IUnknown);
impl IContextProperties {
    pub unsafe fn Count(&self, plcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(plcount)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0, pproperty: *mut super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(pproperty)).ok()
    }
    pub unsafe fn EnumNames(&self) -> ::windows_core::Result<IEnumNames> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumNames)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumNames>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, name: Param0, property: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), name.into_param().abi(), property.into_param().abi()).ok()
    }
    pub unsafe fn RemoveProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveProperty)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IContextProperties> for ::windows_core::IUnknown {
    fn from(value: IContextProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContextProperties> for ::windows_core::IUnknown {
    fn from(value: &IContextProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IContextProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IContextProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IContextProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContextProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContextProperties {}
impl ::core::fmt::Debug for IContextProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContextProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IContextProperties {
    type Vtable = IContextProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd396da85_bf8f_11d1_bbae_00c04fc2fa5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextProperties_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pproperty: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    pub EnumNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, property: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    pub RemoveProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IContextSecurityPerimeter(::windows_core::IUnknown);
impl IContextSecurityPerimeter {
    pub unsafe fn GetPerimeterFlag(&self, pflag: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPerimeterFlag)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pflag)).ok()
    }
    pub unsafe fn SetPerimeterFlag<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fflag: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPerimeterFlag)(::windows_core::Interface::as_raw(self), fflag.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IContextSecurityPerimeter> for ::windows_core::IUnknown {
    fn from(value: IContextSecurityPerimeter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContextSecurityPerimeter> for ::windows_core::IUnknown {
    fn from(value: &IContextSecurityPerimeter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IContextSecurityPerimeter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IContextSecurityPerimeter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IContextSecurityPerimeter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContextSecurityPerimeter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContextSecurityPerimeter {}
impl ::core::fmt::Debug for IContextSecurityPerimeter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContextSecurityPerimeter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IContextSecurityPerimeter {
    type Vtable = IContextSecurityPerimeter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa7549a29_a7c4_42e1_8dc1_7e3d748dc24a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextSecurityPerimeter_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetPerimeterFlag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflag: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetPerimeterFlag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fflag: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IContextState(::windows_core::IUnknown);
impl IContextState {
    pub unsafe fn SetDeactivateOnReturn(&self, bdeactivate: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDeactivateOnReturn)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bdeactivate)).ok()
    }
    pub unsafe fn GetDeactivateOnReturn(&self, pbdeactivate: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeactivateOnReturn)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbdeactivate)).ok()
    }
    pub unsafe fn SetMyTransactionVote(&self, txvote: TransactionVote) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMyTransactionVote)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(txvote)).ok()
    }
    pub unsafe fn GetMyTransactionVote(&self, ptxvote: *mut TransactionVote) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetMyTransactionVote)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptxvote)).ok()
    }
}
impl ::core::convert::From<IContextState> for ::windows_core::IUnknown {
    fn from(value: IContextState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContextState> for ::windows_core::IUnknown {
    fn from(value: &IContextState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IContextState {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IContextState {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IContextState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContextState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContextState {}
impl ::core::fmt::Debug for IContextState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContextState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IContextState {
    type Vtable = IContextState_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3c05e54b_a42a_11d2_afc4_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextState_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetDeactivateOnReturn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bdeactivate: i16) -> ::windows_core::HRESULT,
    pub GetDeactivateOnReturn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdeactivate: *mut i16) -> ::windows_core::HRESULT,
    pub SetMyTransactionVote: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, txvote: TransactionVote) -> ::windows_core::HRESULT,
    pub GetMyTransactionVote: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptxvote: *mut TransactionVote) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICreateWithLocalTransaction(::windows_core::IUnknown);
impl ICreateWithLocalTransaction {
    pub unsafe fn CreateInstanceWithSysTx<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, ptransaction: Param0, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateInstanceWithSysTx)(::windows_core::Interface::as_raw(self), ptransaction.into_param().abi(), ::core::mem::transmute(rclsid), ::core::mem::transmute(riid), ::core::mem::transmute(pobject)).ok()
    }
}
impl ::core::convert::From<ICreateWithLocalTransaction> for ::windows_core::IUnknown {
    fn from(value: ICreateWithLocalTransaction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICreateWithLocalTransaction> for ::windows_core::IUnknown {
    fn from(value: &ICreateWithLocalTransaction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICreateWithLocalTransaction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICreateWithLocalTransaction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICreateWithLocalTransaction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICreateWithLocalTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICreateWithLocalTransaction {}
impl ::core::fmt::Debug for ICreateWithLocalTransaction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICreateWithLocalTransaction").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICreateWithLocalTransaction {
    type Vtable = ICreateWithLocalTransaction_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x227ac7a8_8423_42ce_b7cf_03061ec9aaa3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateWithLocalTransaction_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreateInstanceWithSysTx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICreateWithTipTransactionEx(::windows_core::IUnknown);
impl ICreateWithTipTransactionEx {
    pub unsafe fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, T: ::windows_core::Interface>(&self, bstrtipurl: Param0, rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).CreateInstance)(::windows_core::Interface::as_raw(self), bstrtipurl.into_param().abi(), ::core::mem::transmute(rclsid), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<ICreateWithTipTransactionEx> for ::windows_core::IUnknown {
    fn from(value: ICreateWithTipTransactionEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICreateWithTipTransactionEx> for ::windows_core::IUnknown {
    fn from(value: &ICreateWithTipTransactionEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICreateWithTipTransactionEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICreateWithTipTransactionEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICreateWithTipTransactionEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICreateWithTipTransactionEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICreateWithTipTransactionEx {}
impl ::core::fmt::Debug for ICreateWithTipTransactionEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICreateWithTipTransactionEx").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICreateWithTipTransactionEx {
    type Vtable = ICreateWithTipTransactionEx_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x455acf59_5345_11d2_99cf_00c04f797bc9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateWithTipTransactionEx_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtipurl: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICreateWithTransactionEx(::windows_core::IUnknown);
impl ICreateWithTransactionEx {
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub unsafe fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, super::DistributedTransactionCoordinator::ITransaction>, T: ::windows_core::Interface>(&self, ptransaction: Param0, rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).CreateInstance)(::windows_core::Interface::as_raw(self), ptransaction.into_param().abi(), ::core::mem::transmute(rclsid), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<ICreateWithTransactionEx> for ::windows_core::IUnknown {
    fn from(value: ICreateWithTransactionEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICreateWithTransactionEx> for ::windows_core::IUnknown {
    fn from(value: &ICreateWithTransactionEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICreateWithTransactionEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICreateWithTransactionEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICreateWithTransactionEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICreateWithTransactionEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICreateWithTransactionEx {}
impl ::core::fmt::Debug for ICreateWithTransactionEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICreateWithTransactionEx").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICreateWithTransactionEx {
    type Vtable = ICreateWithTransactionEx_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x455acf57_5345_11d2_99cf_00c04f797bc9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateWithTransactionEx_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: ::windows_core::RawPtr, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    CreateInstance: usize,
}
#[repr(transparent)]
pub struct ICrmCompensator(::windows_core::IUnknown);
impl ICrmCompensator {
    pub unsafe fn SetLogControl<'a, Param0: ::windows_core::IntoParam<'a, ICrmLogControl>>(&self, plogcontrol: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLogControl)(::windows_core::Interface::as_raw(self), plogcontrol.into_param().abi()).ok()
    }
    pub unsafe fn BeginPrepare(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginPrepare)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrepareRecord<'a, Param0: ::windows_core::IntoParam<'a, CrmLogRecordRead>>(&self, crmlogrec: Param0) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).PrepareRecord)(::windows_core::Interface::as_raw(self), crmlogrec.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn EndPrepare(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).EndPrepare)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn BeginCommit<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, frecovery: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginCommit)(::windows_core::Interface::as_raw(self), frecovery.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommitRecord<'a, Param0: ::windows_core::IntoParam<'a, CrmLogRecordRead>>(&self, crmlogrec: Param0) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).CommitRecord)(::windows_core::Interface::as_raw(self), crmlogrec.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn EndCommit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndCommit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BeginAbort<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, frecovery: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginAbort)(::windows_core::Interface::as_raw(self), frecovery.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AbortRecord<'a, Param0: ::windows_core::IntoParam<'a, CrmLogRecordRead>>(&self, crmlogrec: Param0) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).AbortRecord)(::windows_core::Interface::as_raw(self), crmlogrec.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn EndAbort(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndAbort)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ICrmCompensator> for ::windows_core::IUnknown {
    fn from(value: ICrmCompensator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICrmCompensator> for ::windows_core::IUnknown {
    fn from(value: &ICrmCompensator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICrmCompensator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICrmCompensator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICrmCompensator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICrmCompensator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICrmCompensator {}
impl ::core::fmt::Debug for ICrmCompensator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICrmCompensator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICrmCompensator {
    type Vtable = ICrmCompensator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbbc01830_8d3b_11d1_82ec_00a0c91eede9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrmCompensator_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetLogControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plogcontrol: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub BeginPrepare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PrepareRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrepareRecord: usize,
    pub EndPrepare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfoktoprepare: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub BeginCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frecovery: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CommitRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommitRecord: usize,
    pub EndCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub BeginAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frecovery: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AbortRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AbortRecord: usize,
    pub EndAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICrmCompensatorVariants(::windows_core::IUnknown);
impl ICrmCompensatorVariants {
    pub unsafe fn SetLogControlVariants<'a, Param0: ::windows_core::IntoParam<'a, ICrmLogControl>>(&self, plogcontrol: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLogControlVariants)(::windows_core::Interface::as_raw(self), plogcontrol.into_param().abi()).ok()
    }
    pub unsafe fn BeginPrepareVariants(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginPrepareVariants)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PrepareRecordVariants(&self, plogrecord: *const super::Com::VARIANT) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).PrepareRecordVariants)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(plogrecord), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn EndPrepareVariants(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).EndPrepareVariants)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn BeginCommitVariants(&self, brecovery: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginCommitVariants)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(brecovery)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CommitRecordVariants(&self, plogrecord: *const super::Com::VARIANT) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).CommitRecordVariants)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(plogrecord), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn EndCommitVariants(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndCommitVariants)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BeginAbortVariants(&self, brecovery: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginAbortVariants)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(brecovery)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AbortRecordVariants(&self, plogrecord: *const super::Com::VARIANT) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).AbortRecordVariants)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(plogrecord), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn EndAbortVariants(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndAbortVariants)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ICrmCompensatorVariants> for ::windows_core::IUnknown {
    fn from(value: ICrmCompensatorVariants) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICrmCompensatorVariants> for ::windows_core::IUnknown {
    fn from(value: &ICrmCompensatorVariants) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICrmCompensatorVariants {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICrmCompensatorVariants {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICrmCompensatorVariants {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICrmCompensatorVariants {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICrmCompensatorVariants {}
impl ::core::fmt::Debug for ICrmCompensatorVariants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICrmCompensatorVariants").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICrmCompensatorVariants {
    type Vtable = ICrmCompensatorVariants_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf0baf8e4_7804_11d1_82e9_00a0c91eede9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrmCompensatorVariants_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetLogControlVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plogcontrol: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub BeginPrepareVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PrepareRecordVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plogrecord: *const super::Com::VARIANT, pbforget: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PrepareRecordVariants: usize,
    pub EndPrepareVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pboktoprepare: *mut i16) -> ::windows_core::HRESULT,
    pub BeginCommitVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brecovery: i16) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CommitRecordVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plogrecord: *const super::Com::VARIANT, pbforget: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CommitRecordVariants: usize,
    pub EndCommitVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub BeginAbortVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brecovery: i16) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AbortRecordVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plogrecord: *const super::Com::VARIANT, pbforget: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AbortRecordVariants: usize,
    pub EndAbortVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICrmFormatLogRecords(::windows_core::IUnknown);
impl ICrmFormatLogRecords {
    pub unsafe fn GetColumnCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).GetColumnCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetColumnHeaders(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetColumnHeaders)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetColumn<'a, Param0: ::windows_core::IntoParam<'a, CrmLogRecordRead>>(&self, crmlogrec: Param0) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetColumn)(::windows_core::Interface::as_raw(self), crmlogrec.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetColumnVariants<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, logrecord: Param0) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetColumnVariants)(::windows_core::Interface::as_raw(self), logrecord.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
}
impl ::core::convert::From<ICrmFormatLogRecords> for ::windows_core::IUnknown {
    fn from(value: ICrmFormatLogRecords) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICrmFormatLogRecords> for ::windows_core::IUnknown {
    fn from(value: &ICrmFormatLogRecords) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICrmFormatLogRecords {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICrmFormatLogRecords {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICrmFormatLogRecords {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICrmFormatLogRecords {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICrmFormatLogRecords {}
impl ::core::fmt::Debug for ICrmFormatLogRecords {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICrmFormatLogRecords").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICrmFormatLogRecords {
    type Vtable = ICrmFormatLogRecords_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9c51d821_c98b_11d1_82fb_00a0c91eede9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrmFormatLogRecords_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetColumnCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcolumncount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetColumnHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheaders: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetColumnHeaders: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetColumn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pformattedlogrecord: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetColumn: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetColumnVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logrecord: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pformattedlogrecord: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetColumnVariants: usize,
}
#[repr(transparent)]
pub struct ICrmLogControl(::windows_core::IUnknown);
impl ICrmLogControl {
    pub unsafe fn TransactionUOW(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).TransactionUOW)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn RegisterCompensator<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, lpcwstrprogidcompensator: Param0, lpcwstrdescription: Param1, lcrmregflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterCompensator)(::windows_core::Interface::as_raw(self), lpcwstrprogidcompensator.into_param().abi(), lpcwstrdescription.into_param().abi(), ::core::mem::transmute(lcrmregflags)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn WriteLogRecordVariants(&self, plogrecord: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteLogRecordVariants)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(plogrecord)).ok()
    }
    pub unsafe fn ForceLog(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ForceLog)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ForgetLogRecord(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ForgetLogRecord)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ForceTransactionToAbort(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ForceTransactionToAbort)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WriteLogRecord(&self, rgblob: &[super::Com::BLOB]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteLogRecord)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_ptr_or_null(rgblob)), rgblob.len() as _).ok()
    }
}
impl ::core::convert::From<ICrmLogControl> for ::windows_core::IUnknown {
    fn from(value: ICrmLogControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICrmLogControl> for ::windows_core::IUnknown {
    fn from(value: &ICrmLogControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICrmLogControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICrmLogControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICrmLogControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICrmLogControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICrmLogControl {}
impl ::core::fmt::Debug for ICrmLogControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICrmLogControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICrmLogControl {
    type Vtable = ICrmLogControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa0e174b3_d26e_11d2_8f84_00805fc7bcd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrmLogControl_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub TransactionUOW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub RegisterCompensator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpcwstrprogidcompensator: ::windows_core::PCWSTR, lpcwstrdescription: ::windows_core::PCWSTR, lcrmregflags: i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub WriteLogRecordVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plogrecord: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    WriteLogRecordVariants: usize,
    pub ForceLog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ForgetLogRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ForceTransactionToAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub WriteLogRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rgblob: *const super::Com::BLOB, cblob: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WriteLogRecord: usize,
}
#[repr(transparent)]
pub struct ICrmMonitor(::windows_core::IUnknown);
impl ICrmMonitor {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClerks(&self) -> ::windows_core::Result<ICrmMonitorClerks> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetClerks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ICrmMonitorClerks>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn HoldClerk<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).HoldClerk)(::windows_core::Interface::as_raw(self), index.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
}
impl ::core::convert::From<ICrmMonitor> for ::windows_core::IUnknown {
    fn from(value: ICrmMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICrmMonitor> for ::windows_core::IUnknown {
    fn from(value: &ICrmMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICrmMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICrmMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICrmMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICrmMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICrmMonitor {}
impl ::core::fmt::Debug for ICrmMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICrmMonitor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICrmMonitor {
    type Vtable = ICrmMonitor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70c8e443_c7ed_11d1_82fb_00a0c91eede9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrmMonitor_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetClerks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclerks: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetClerks: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub HoldClerk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    HoldClerk: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ICrmMonitorClerks(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ICrmMonitorClerks {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), index.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ProgIdCompensator<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).ProgIdCompensator)(::windows_core::Interface::as_raw(self), index.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Description<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), index.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn TransactionUOW<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).TransactionUOW)(::windows_core::Interface::as_raw(self), index.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ActivityId<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).ActivityId)(::windows_core::Interface::as_raw(self), index.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICrmMonitorClerks> for ::windows_core::IUnknown {
    fn from(value: ICrmMonitorClerks) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICrmMonitorClerks> for ::windows_core::IUnknown {
    fn from(value: &ICrmMonitorClerks) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICrmMonitorClerks {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICrmMonitorClerks {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICrmMonitorClerks> for super::Com::IDispatch {
    fn from(value: ICrmMonitorClerks) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICrmMonitorClerks> for super::Com::IDispatch {
    fn from(value: &ICrmMonitorClerks) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ICrmMonitorClerks {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ICrmMonitorClerks {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ICrmMonitorClerks {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICrmMonitorClerks {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICrmMonitorClerks {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICrmMonitorClerks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICrmMonitorClerks").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ICrmMonitorClerks {
    type Vtable = ICrmMonitorClerks_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70c8e442_c7ed_11d1_82fb_00a0c91eede9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICrmMonitorClerks_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ProgIdCompensator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ProgIdCompensator: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Description: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub TransactionUOW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    TransactionUOW: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ActivityId: usize,
}
#[repr(transparent)]
pub struct ICrmMonitorLogRecords(::windows_core::IUnknown);
impl ICrmMonitorLogRecords {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn TransactionState(&self) -> ::windows_core::Result<CrmTransactionState> {
        let mut result__ = ::core::mem::MaybeUninit::<CrmTransactionState>::zeroed();
        (::windows_core::Interface::vtable(self).TransactionState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<CrmTransactionState>(result__)
    }
    pub unsafe fn StructuredRecords(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).StructuredRecords)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetLogRecord(&self, dwindex: u32, pcrmlogrec: *mut CrmLogRecordRead) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetLogRecord)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pcrmlogrec)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetLogRecordVariants<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, indexnumber: Param0) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetLogRecordVariants)(::windows_core::Interface::as_raw(self), indexnumber.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
}
impl ::core::convert::From<ICrmMonitorLogRecords> for ::windows_core::IUnknown {
    fn from(value: ICrmMonitorLogRecords) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICrmMonitorLogRecords> for ::windows_core::IUnknown {
    fn from(value: &ICrmMonitorLogRecords) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICrmMonitorLogRecords {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICrmMonitorLogRecords {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICrmMonitorLogRecords {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICrmMonitorLogRecords {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICrmMonitorLogRecords {}
impl ::core::fmt::Debug for ICrmMonitorLogRecords {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICrmMonitorLogRecords").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICrmMonitorLogRecords {
    type Vtable = ICrmMonitorLogRecords_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70c8e441_c7ed_11d1_82fb_00a0c91eede9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrmMonitorLogRecords_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub TransactionState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut CrmTransactionState) -> ::windows_core::HRESULT,
    pub StructuredRecords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetLogRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pcrmlogrec: *mut CrmLogRecordRead) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetLogRecord: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetLogRecordVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexnumber: ::core::mem::ManuallyDrop<super::Com::VARIANT>, plogrecord: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetLogRecordVariants: usize,
}
#[repr(transparent)]
pub struct IDispenserDriver(::windows_core::IUnknown);
impl IDispenserDriver {
    pub unsafe fn CreateResource(&self, restypid: usize, presid: *mut usize, psecsfreebeforedestroy: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateResource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(restypid), ::core::mem::transmute(presid), ::core::mem::transmute(psecsfreebeforedestroy)).ok()
    }
    pub unsafe fn RateResource<'a, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, restypid: usize, resid: usize, frequirestransactionenlistment: Param2, prating: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RateResource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(restypid), ::core::mem::transmute(resid), frequirestransactionenlistment.into_param().abi(), ::core::mem::transmute(prating)).ok()
    }
    pub unsafe fn EnlistResource(&self, resid: usize, transid: usize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnlistResource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(resid), ::core::mem::transmute(transid)).ok()
    }
    pub unsafe fn ResetResource(&self, resid: usize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ResetResource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(resid)).ok()
    }
    pub unsafe fn DestroyResource(&self, resid: usize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DestroyResource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(resid)).ok()
    }
    pub unsafe fn DestroyResourceS(&self, resid: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DestroyResourceS)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(resid)).ok()
    }
}
impl ::core::convert::From<IDispenserDriver> for ::windows_core::IUnknown {
    fn from(value: IDispenserDriver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDispenserDriver> for ::windows_core::IUnknown {
    fn from(value: &IDispenserDriver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDispenserDriver {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDispenserDriver {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDispenserDriver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDispenserDriver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDispenserDriver {}
impl ::core::fmt::Debug for IDispenserDriver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDispenserDriver").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDispenserDriver {
    type Vtable = IDispenserDriver_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x208b3651_2b48_11cf_be10_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispenserDriver_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreateResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restypid: usize, presid: *mut usize, psecsfreebeforedestroy: *mut i32) -> ::windows_core::HRESULT,
    pub RateResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restypid: usize, resid: usize, frequirestransactionenlistment: ::win32_foundation::BOOL, prating: *mut u32) -> ::windows_core::HRESULT,
    pub EnlistResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resid: usize, transid: usize) -> ::windows_core::HRESULT,
    pub ResetResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resid: usize) -> ::windows_core::HRESULT,
    pub DestroyResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resid: usize) -> ::windows_core::HRESULT,
    pub DestroyResourceS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resid: *mut u16) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDispenserManager(::windows_core::IUnknown);
impl IDispenserManager {
    pub unsafe fn RegisterDispenser<'a, Param0: ::windows_core::IntoParam<'a, IDispenserDriver>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, __midl__idispensermanager0000: Param0, szdispensername: Param1) -> ::windows_core::Result<IHolder> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RegisterDispenser)(::windows_core::Interface::as_raw(self), __midl__idispensermanager0000.into_param().abi(), szdispensername.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IHolder>(result__)
    }
    pub unsafe fn GetContext(&self, __midl__idispensermanager0002: *mut usize, __midl__idispensermanager0003: *mut usize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetContext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__idispensermanager0002), ::core::mem::transmute(__midl__idispensermanager0003)).ok()
    }
}
impl ::core::convert::From<IDispenserManager> for ::windows_core::IUnknown {
    fn from(value: IDispenserManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDispenserManager> for ::windows_core::IUnknown {
    fn from(value: &IDispenserManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDispenserManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDispenserManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDispenserManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDispenserManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDispenserManager {}
impl ::core::fmt::Debug for IDispenserManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDispenserManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDispenserManager {
    type Vtable = IDispenserManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5cb31e10_2b5f_11cf_be10_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispenserManager_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub RegisterDispenser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__idispensermanager0000: ::windows_core::RawPtr, szdispensername: ::windows_core::PCWSTR, __midl__idispensermanager0001: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__idispensermanager0002: *mut usize, __midl__idispensermanager0003: *mut usize) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnumNames(::windows_core::IUnknown);
impl IEnumNames {
    pub unsafe fn Next(&self, celt: u32, rgname: *mut ::win32_foundation::BSTR, pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgname), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumNames> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumNames>(result__)
    }
}
impl ::core::convert::From<IEnumNames> for ::windows_core::IUnknown {
    fn from(value: IEnumNames) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumNames> for ::windows_core::IUnknown {
    fn from(value: &IEnumNames) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnumNames {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnumNames {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumNames {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumNames {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumNames {}
impl ::core::fmt::Debug for IEnumNames {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumNames").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumNames {
    type Vtable = IEnumNames_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x51372af2_cae7_11cf_be81_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNames_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgname: *mut ::win32_foundation::BSTR, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IEventServerTrace(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IEventServerTrace {
    pub unsafe fn StartTraceGuid<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrguidevent: Param0, bstrguidfilter: Param1, lpidfilter: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StartTraceGuid)(::windows_core::Interface::as_raw(self), bstrguidevent.into_param().abi(), bstrguidfilter.into_param().abi(), ::core::mem::transmute(lpidfilter)).ok()
    }
    pub unsafe fn StopTraceGuid<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrguidevent: Param0, bstrguidfilter: Param1, lpidfilter: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StopTraceGuid)(::windows_core::Interface::as_raw(self), bstrguidevent.into_param().abi(), bstrguidfilter.into_param().abi(), ::core::mem::transmute(lpidfilter)).ok()
    }
    pub unsafe fn EnumTraceGuid(&self, plcntguids: *mut i32, pbstrguidlist: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnumTraceGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(plcntguids), ::core::mem::transmute(pbstrguidlist)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IEventServerTrace> for ::windows_core::IUnknown {
    fn from(value: IEventServerTrace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IEventServerTrace> for ::windows_core::IUnknown {
    fn from(value: &IEventServerTrace) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEventServerTrace {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEventServerTrace {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IEventServerTrace> for super::Com::IDispatch {
    fn from(value: IEventServerTrace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IEventServerTrace> for super::Com::IDispatch {
    fn from(value: &IEventServerTrace) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IEventServerTrace {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IEventServerTrace {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IEventServerTrace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IEventServerTrace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IEventServerTrace {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IEventServerTrace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventServerTrace").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IEventServerTrace {
    type Vtable = IEventServerTrace_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a9f12b8_80af_47ab_a579_35ea57725370);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IEventServerTrace_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub StartTraceGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguidevent: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrguidfilter: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lpidfilter: i32) -> ::windows_core::HRESULT,
    pub StopTraceGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguidevent: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrguidfilter: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lpidfilter: i32) -> ::windows_core::HRESULT,
    pub EnumTraceGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcntguids: *mut i32, pbstrguidlist: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IGetAppTrackerData(::windows_core::IUnknown);
impl IGetAppTrackerData {
    pub unsafe fn GetApplicationProcesses(&self, partitionid: *const ::windows_core::GUID, applicationid: *const ::windows_core::GUID, flags: u32, numapplicationprocesses: *mut u32, applicationprocesses: *mut *mut ApplicationProcessSummary) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetApplicationProcesses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(partitionid), ::core::mem::transmute(applicationid), ::core::mem::transmute(flags), ::core::mem::transmute(numapplicationprocesses), ::core::mem::transmute(applicationprocesses)).ok()
    }
    pub unsafe fn GetApplicationProcessDetails(&self, applicationinstanceid: *const ::windows_core::GUID, processid: u32, flags: u32, summary: *mut ApplicationProcessSummary, statistics: *mut ApplicationProcessStatistics, recycleinfo: *mut ApplicationProcessRecycleInfo, anycomponentshangmonitored: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetApplicationProcessDetails)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(applicationinstanceid), ::core::mem::transmute(processid), ::core::mem::transmute(flags), ::core::mem::transmute(summary), ::core::mem::transmute(statistics), ::core::mem::transmute(recycleinfo), ::core::mem::transmute(anycomponentshangmonitored)).ok()
    }
    pub unsafe fn GetApplicationsInProcess(&self, applicationinstanceid: *const ::windows_core::GUID, processid: u32, partitionid: *const ::windows_core::GUID, flags: u32, numapplicationsinprocess: *mut u32, applications: *mut *mut ApplicationSummary) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetApplicationsInProcess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(applicationinstanceid), ::core::mem::transmute(processid), ::core::mem::transmute(partitionid), ::core::mem::transmute(flags), ::core::mem::transmute(numapplicationsinprocess), ::core::mem::transmute(applications)).ok()
    }
    pub unsafe fn GetComponentsInProcess(&self, applicationinstanceid: *const ::windows_core::GUID, processid: u32, partitionid: *const ::windows_core::GUID, applicationid: *const ::windows_core::GUID, flags: u32, numcomponentsinprocess: *mut u32, components: *mut *mut ComponentSummary) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetComponentsInProcess)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(applicationinstanceid), ::core::mem::transmute(processid), ::core::mem::transmute(partitionid), ::core::mem::transmute(applicationid), ::core::mem::transmute(flags), ::core::mem::transmute(numcomponentsinprocess), ::core::mem::transmute(components)).ok()
    }
    pub unsafe fn GetComponentDetails(&self, applicationinstanceid: *const ::windows_core::GUID, processid: u32, clsid: *const ::windows_core::GUID, flags: u32, summary: *mut ComponentSummary, statistics: *mut ComponentStatistics, hangmonitorinfo: *mut ComponentHangMonitorInfo) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetComponentDetails)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(applicationinstanceid), ::core::mem::transmute(processid), ::core::mem::transmute(clsid), ::core::mem::transmute(flags), ::core::mem::transmute(summary), ::core::mem::transmute(statistics), ::core::mem::transmute(hangmonitorinfo)).ok()
    }
    pub unsafe fn GetTrackerDataAsCollectionObject(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).GetTrackerDataAsCollectionObject)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn GetSuggestedPollingInterval(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetSuggestedPollingInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IGetAppTrackerData> for ::windows_core::IUnknown {
    fn from(value: IGetAppTrackerData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGetAppTrackerData> for ::windows_core::IUnknown {
    fn from(value: &IGetAppTrackerData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGetAppTrackerData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGetAppTrackerData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGetAppTrackerData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGetAppTrackerData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetAppTrackerData {}
impl ::core::fmt::Debug for IGetAppTrackerData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetAppTrackerData").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IGetAppTrackerData {
    type Vtable = IGetAppTrackerData_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x507c3ac8_3e12_4cb0_9366_653d3e050638);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetAppTrackerData_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetApplicationProcesses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partitionid: *const ::windows_core::GUID, applicationid: *const ::windows_core::GUID, flags: u32, numapplicationprocesses: *mut u32, applicationprocesses: *mut *mut ApplicationProcessSummary) -> ::windows_core::HRESULT,
    pub GetApplicationProcessDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows_core::GUID, processid: u32, flags: u32, summary: *mut ApplicationProcessSummary, statistics: *mut ApplicationProcessStatistics, recycleinfo: *mut ApplicationProcessRecycleInfo, anycomponentshangmonitored: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetApplicationsInProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows_core::GUID, processid: u32, partitionid: *const ::windows_core::GUID, flags: u32, numapplicationsinprocess: *mut u32, applications: *mut *mut ApplicationSummary) -> ::windows_core::HRESULT,
    pub GetComponentsInProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows_core::GUID, processid: u32, partitionid: *const ::windows_core::GUID, applicationid: *const ::windows_core::GUID, flags: u32, numcomponentsinprocess: *mut u32, components: *mut *mut ComponentSummary) -> ::windows_core::HRESULT,
    pub GetComponentDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows_core::GUID, processid: u32, clsid: *const ::windows_core::GUID, flags: u32, summary: *mut ComponentSummary, statistics: *mut ComponentStatistics, hangmonitorinfo: *mut ComponentHangMonitorInfo) -> ::windows_core::HRESULT,
    pub GetTrackerDataAsCollectionObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, toplevelcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSuggestedPollingInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pollingintervalinseconds: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IGetContextProperties(::windows_core::IUnknown);
impl IGetContextProperties {
    pub unsafe fn Count(&self, plcount: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(plcount)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0, pproperty: *mut super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(pproperty)).ok()
    }
    pub unsafe fn EnumNames(&self) -> ::windows_core::Result<IEnumNames> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumNames)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumNames>(result__)
    }
}
impl ::core::convert::From<IGetContextProperties> for ::windows_core::IUnknown {
    fn from(value: IGetContextProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGetContextProperties> for ::windows_core::IUnknown {
    fn from(value: &IGetContextProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGetContextProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGetContextProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGetContextProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGetContextProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetContextProperties {}
impl ::core::fmt::Debug for IGetContextProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetContextProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IGetContextProperties {
    type Vtable = IGetContextProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x51372af4_cae7_11cf_be81_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetContextProperties_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pproperty: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    pub EnumNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IGetSecurityCallContext(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGetSecurityCallContext {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSecurityCallContext(&self) -> ::windows_core::Result<ISecurityCallContext> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSecurityCallContext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISecurityCallContext>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGetSecurityCallContext> for ::windows_core::IUnknown {
    fn from(value: IGetSecurityCallContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGetSecurityCallContext> for ::windows_core::IUnknown {
    fn from(value: &IGetSecurityCallContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGetSecurityCallContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGetSecurityCallContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IGetSecurityCallContext> for super::Com::IDispatch {
    fn from(value: IGetSecurityCallContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IGetSecurityCallContext> for super::Com::IDispatch {
    fn from(value: &IGetSecurityCallContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGetSecurityCallContext {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGetSecurityCallContext {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IGetSecurityCallContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IGetSecurityCallContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IGetSecurityCallContext {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IGetSecurityCallContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetSecurityCallContext").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGetSecurityCallContext {
    type Vtable = IGetSecurityCallContext_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcafc823f_b441_11d1_b82b_0000f8757e2a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGetSecurityCallContext_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSecurityCallContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppobject: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSecurityCallContext: usize,
}
#[repr(transparent)]
pub struct IHolder(::windows_core::IUnknown);
impl IHolder {
    pub unsafe fn AllocResource(&self, __midl__iholder0000: usize, __midl__iholder0001: *mut usize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AllocResource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iholder0000), ::core::mem::transmute(__midl__iholder0001)).ok()
    }
    pub unsafe fn FreeResource(&self, __midl__iholder0002: usize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FreeResource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iholder0002)).ok()
    }
    pub unsafe fn TrackResource(&self, __midl__iholder0003: usize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TrackResource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iholder0003)).ok()
    }
    pub unsafe fn TrackResourceS(&self, __midl__iholder0004: *mut u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TrackResourceS)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iholder0004)).ok()
    }
    pub unsafe fn UntrackResource<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, __midl__iholder0005: usize, __midl__iholder0006: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UntrackResource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iholder0005), __midl__iholder0006.into_param().abi()).ok()
    }
    pub unsafe fn UntrackResourceS<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, __midl__iholder0007: *mut u16, __midl__iholder0008: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UntrackResourceS)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iholder0007), __midl__iholder0008.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RequestDestroyResource(&self, __midl__iholder0009: usize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RequestDestroyResource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__iholder0009)).ok()
    }
}
impl ::core::convert::From<IHolder> for ::windows_core::IUnknown {
    fn from(value: IHolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHolder> for ::windows_core::IUnknown {
    fn from(value: &IHolder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IHolder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IHolder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IHolder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IHolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHolder {}
impl ::core::fmt::Debug for IHolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHolder").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IHolder {
    type Vtable = IHolder_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbf6a1850_2b45_11cf_be10_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolder_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AllocResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iholder0000: usize, __midl__iholder0001: *mut usize) -> ::windows_core::HRESULT,
    pub FreeResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iholder0002: usize) -> ::windows_core::HRESULT,
    pub TrackResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iholder0003: usize) -> ::windows_core::HRESULT,
    pub TrackResourceS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iholder0004: *mut u16) -> ::windows_core::HRESULT,
    pub UntrackResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iholder0005: usize, __midl__iholder0006: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub UntrackResourceS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iholder0007: *mut u16, __midl__iholder0008: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RequestDestroyResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iholder0009: usize) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ILBEvents(::windows_core::IUnknown);
impl ILBEvents {
    pub unsafe fn TargetUp<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrservername: Param0, bstrclsideng: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TargetUp)(::windows_core::Interface::as_raw(self), bstrservername.into_param().abi(), bstrclsideng.into_param().abi()).ok()
    }
    pub unsafe fn TargetDown<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrservername: Param0, bstrclsideng: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TargetDown)(::windows_core::Interface::as_raw(self), bstrservername.into_param().abi(), bstrclsideng.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EngineDefined<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpropname: Param0, varpropvalue: *const super::Com::VARIANT, bstrclsideng: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EngineDefined)(::windows_core::Interface::as_raw(self), bstrpropname.into_param().abi(), ::core::mem::transmute(varpropvalue), bstrclsideng.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ILBEvents> for ::windows_core::IUnknown {
    fn from(value: ILBEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILBEvents> for ::windows_core::IUnknown {
    fn from(value: &ILBEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ILBEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ILBEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILBEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILBEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILBEvents {}
impl ::core::fmt::Debug for ILBEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILBEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ILBEvents {
    type Vtable = ILBEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x683130b4_2e50_11d2_98a5_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILBEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub TargetUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrclsideng: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub TargetDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrclsideng: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EngineDefined: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varpropvalue: *const super::Com::VARIANT, bstrclsideng: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EngineDefined: usize,
}
#[repr(transparent)]
pub struct IMTSActivity(::windows_core::IUnknown);
impl IMTSActivity {
    pub unsafe fn SynchronousCall<'a, Param0: ::windows_core::IntoParam<'a, IMTSCall>>(&self, pcall: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SynchronousCall)(::windows_core::Interface::as_raw(self), pcall.into_param().abi()).ok()
    }
    pub unsafe fn AsyncCall<'a, Param0: ::windows_core::IntoParam<'a, IMTSCall>>(&self, pcall: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AsyncCall)(::windows_core::Interface::as_raw(self), pcall.into_param().abi()).ok()
    }
    pub unsafe fn Reserved1(&self) {
        (::windows_core::Interface::vtable(self).Reserved1)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn BindToCurrentThread(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BindToCurrentThread)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn UnbindFromThread(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnbindFromThread)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IMTSActivity> for ::windows_core::IUnknown {
    fn from(value: IMTSActivity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMTSActivity> for ::windows_core::IUnknown {
    fn from(value: &IMTSActivity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMTSActivity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMTSActivity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMTSActivity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMTSActivity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMTSActivity {}
impl ::core::fmt::Debug for IMTSActivity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMTSActivity").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IMTSActivity {
    type Vtable = IMTSActivity_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x51372af0_cae7_11cf_be81_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMTSActivity_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SynchronousCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcall: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AsyncCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcall: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Reserved1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub BindToCurrentThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UnbindFromThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IMTSCall(::windows_core::IUnknown);
impl IMTSCall {
    pub unsafe fn OnCall(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnCall)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IMTSCall> for ::windows_core::IUnknown {
    fn from(value: IMTSCall) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMTSCall> for ::windows_core::IUnknown {
    fn from(value: &IMTSCall) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMTSCall {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMTSCall {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMTSCall {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMTSCall {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMTSCall {}
impl ::core::fmt::Debug for IMTSCall {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMTSCall").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IMTSCall {
    type Vtable = IMTSCall_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x51372aef_cae7_11cf_be81_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMTSCall_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMTSLocator(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMTSLocator {
    pub unsafe fn GetEventDispatcher(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).GetEventDispatcher)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMTSLocator> for ::windows_core::IUnknown {
    fn from(value: IMTSLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMTSLocator> for ::windows_core::IUnknown {
    fn from(value: &IMTSLocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMTSLocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMTSLocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMTSLocator> for super::Com::IDispatch {
    fn from(value: IMTSLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMTSLocator> for super::Com::IDispatch {
    fn from(value: &IMTSLocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMTSLocator {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMTSLocator {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMTSLocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMTSLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMTSLocator {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMTSLocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMTSLocator").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IMTSLocator {
    type Vtable = IMTSLocator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd19b8bfd_7f88_11d0_b16e_00aa00ba3258);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMTSLocator_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub GetEventDispatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IManagedActivationEvents(::windows_core::IUnknown);
impl IManagedActivationEvents {
    pub unsafe fn CreateManagedStub<'a, Param0: ::windows_core::IntoParam<'a, IManagedObjectInfo>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, pinfo: Param0, fdist: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateManagedStub)(::windows_core::Interface::as_raw(self), pinfo.into_param().abi(), fdist.into_param().abi()).ok()
    }
    pub unsafe fn DestroyManagedStub<'a, Param0: ::windows_core::IntoParam<'a, IManagedObjectInfo>>(&self, pinfo: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DestroyManagedStub)(::windows_core::Interface::as_raw(self), pinfo.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IManagedActivationEvents> for ::windows_core::IUnknown {
    fn from(value: IManagedActivationEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IManagedActivationEvents> for ::windows_core::IUnknown {
    fn from(value: &IManagedActivationEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IManagedActivationEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IManagedActivationEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IManagedActivationEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IManagedActivationEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IManagedActivationEvents {}
impl ::core::fmt::Debug for IManagedActivationEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IManagedActivationEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IManagedActivationEvents {
    type Vtable = IManagedActivationEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa5f325af_572f_46da_b8ab_827c3d95d99e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManagedActivationEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreateManagedStub: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: ::windows_core::RawPtr, fdist: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub DestroyManagedStub: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IManagedObjectInfo(::windows_core::IUnknown);
impl IManagedObjectInfo {
    pub unsafe fn GetIUnknown(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).GetIUnknown)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn GetIObjectControl(&self) -> ::windows_core::Result<IObjectControl> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetIObjectControl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IObjectControl>(result__)
    }
    pub unsafe fn SetInPool<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param1: ::windows_core::IntoParam<'a, IManagedPooledObj>>(&self, binpool: Param0, ppooledobj: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInPool)(::windows_core::Interface::as_raw(self), binpool.into_param().abi(), ppooledobj.into_param().abi()).ok()
    }
    pub unsafe fn SetWrapperStrength<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bstrong: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWrapperStrength)(::windows_core::Interface::as_raw(self), bstrong.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IManagedObjectInfo> for ::windows_core::IUnknown {
    fn from(value: IManagedObjectInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IManagedObjectInfo> for ::windows_core::IUnknown {
    fn from(value: &IManagedObjectInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IManagedObjectInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IManagedObjectInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IManagedObjectInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IManagedObjectInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IManagedObjectInfo {}
impl ::core::fmt::Debug for IManagedObjectInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IManagedObjectInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IManagedObjectInfo {
    type Vtable = IManagedObjectInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1427c51a_4584_49d8_90a0_c50d8086cbe9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManagedObjectInfo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetIUnknown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetIObjectControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctrl: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetInPool: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binpool: ::win32_foundation::BOOL, ppooledobj: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetWrapperStrength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrong: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IManagedPoolAction(::windows_core::IUnknown);
impl IManagedPoolAction {
    pub unsafe fn LastRelease(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LastRelease)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IManagedPoolAction> for ::windows_core::IUnknown {
    fn from(value: IManagedPoolAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IManagedPoolAction> for ::windows_core::IUnknown {
    fn from(value: &IManagedPoolAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IManagedPoolAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IManagedPoolAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IManagedPoolAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IManagedPoolAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IManagedPoolAction {}
impl ::core::fmt::Debug for IManagedPoolAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IManagedPoolAction").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IManagedPoolAction {
    type Vtable = IManagedPoolAction_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda91b74e_5388_4783_949d_c1cd5fb00506);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManagedPoolAction_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub LastRelease: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IManagedPooledObj(::windows_core::IUnknown);
impl IManagedPooledObj {
    pub unsafe fn SetHeld<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, m_bheld: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHeld)(::windows_core::Interface::as_raw(self), m_bheld.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IManagedPooledObj> for ::windows_core::IUnknown {
    fn from(value: IManagedPooledObj) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IManagedPooledObj> for ::windows_core::IUnknown {
    fn from(value: &IManagedPooledObj) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IManagedPooledObj {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IManagedPooledObj {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IManagedPooledObj {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IManagedPooledObj {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IManagedPooledObj {}
impl ::core::fmt::Debug for IManagedPooledObj {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IManagedPooledObj").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IManagedPooledObj {
    type Vtable = IManagedPooledObj_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc5da4bea_1b42_4437_8926_b6a38860a770);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManagedPooledObj_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetHeld: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, m_bheld: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMessageMover(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMessageMover {
    pub unsafe fn SourcePath(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).SourcePath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetSourcePath<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, newval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSourcePath)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn DestPath(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DestPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDestPath<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, newval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDestPath)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn CommitBatchSize(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).CommitBatchSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetCommitBatchSize(&self, newval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCommitBatchSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newval)).ok()
    }
    pub unsafe fn MoveMessages(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MoveMessages)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMessageMover> for ::windows_core::IUnknown {
    fn from(value: IMessageMover) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMessageMover> for ::windows_core::IUnknown {
    fn from(value: &IMessageMover) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMessageMover {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMessageMover {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMessageMover> for super::Com::IDispatch {
    fn from(value: IMessageMover) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMessageMover> for super::Com::IDispatch {
    fn from(value: &IMessageMover) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMessageMover {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMessageMover {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMessageMover {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMessageMover {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMessageMover {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMessageMover {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMessageMover").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IMessageMover {
    type Vtable = IMessageMover_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x588a085a_b795_11d1_8054_00c04fc340ee);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMessageMover_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub SourcePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetSourcePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub DestPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDestPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub CommitBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub SetCommitBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT,
    pub MoveMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmessagesmoved: *mut i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMtsEventInfo(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMtsEventInfo {
    pub unsafe fn Names(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).Names)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn DisplayName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn EventID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).EventID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Value<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, skey: Param0) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Value)(::windows_core::Interface::as_raw(self), skey.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMtsEventInfo> for ::windows_core::IUnknown {
    fn from(value: IMtsEventInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMtsEventInfo> for ::windows_core::IUnknown {
    fn from(value: &IMtsEventInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMtsEventInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMtsEventInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMtsEventInfo> for super::Com::IDispatch {
    fn from(value: IMtsEventInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMtsEventInfo> for super::Com::IDispatch {
    fn from(value: &IMtsEventInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMtsEventInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMtsEventInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMtsEventInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMtsEventInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMtsEventInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMtsEventInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMtsEventInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IMtsEventInfo {
    type Vtable = IMtsEventInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd56c3dc1_8482_11d0_b170_00aa00ba3258);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMtsEventInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Names: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sdisplayname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub EventID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sguideventid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, skey: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Value: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMtsEvents(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMtsEvents {
    pub unsafe fn PackageName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PackageName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn PackageGuid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PackageGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PostEvent(&self, vevent: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PostEvent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vevent)).ok()
    }
    pub unsafe fn FireEvents(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).FireEvents)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn GetProcessID(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).GetProcessID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMtsEvents> for ::windows_core::IUnknown {
    fn from(value: IMtsEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMtsEvents> for ::windows_core::IUnknown {
    fn from(value: &IMtsEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMtsEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMtsEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMtsEvents> for super::Com::IDispatch {
    fn from(value: IMtsEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMtsEvents> for super::Com::IDispatch {
    fn from(value: &IMtsEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMtsEvents {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMtsEvents {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMtsEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMtsEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMtsEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMtsEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMtsEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IMtsEvents {
    type Vtable = IMtsEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbacedf4d_74ab_11d0_b162_00aa00ba3258);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMtsEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub PackageName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub PackageGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PostEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vevent: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PostEvent: usize,
    pub FireEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows_core::HRESULT,
    pub GetProcessID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMtsGrp(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMtsGrp {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Item(&self, lindex: i32) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMtsGrp> for ::windows_core::IUnknown {
    fn from(value: IMtsGrp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMtsGrp> for ::windows_core::IUnknown {
    fn from(value: &IMtsGrp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IMtsGrp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IMtsGrp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMtsGrp> for super::Com::IDispatch {
    fn from(value: IMtsGrp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMtsGrp> for super::Com::IDispatch {
    fn from(value: &IMtsGrp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IMtsGrp {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IMtsGrp {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMtsGrp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMtsGrp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMtsGrp {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMtsGrp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMtsGrp").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IMtsGrp {
    type Vtable = IMtsGrp_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4b2e958c_0393_11d1_b1ab_00aa00ba3258);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMtsGrp_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, ppunkdispatcher: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IObjPool(::windows_core::IUnknown);
impl IObjPool {
    pub unsafe fn Reserved1(&self) {
        (::windows_core::Interface::vtable(self).Reserved1)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved2(&self) {
        (::windows_core::Interface::vtable(self).Reserved2)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved3(&self) {
        (::windows_core::Interface::vtable(self).Reserved3)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved4(&self) {
        (::windows_core::Interface::vtable(self).Reserved4)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn PutEndTx<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, pobj: Param0) {
        (::windows_core::Interface::vtable(self).PutEndTx)(::windows_core::Interface::as_raw(self), pobj.into_param().abi())
    }
    pub unsafe fn Reserved5(&self) {
        (::windows_core::Interface::vtable(self).Reserved5)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved6(&self) {
        (::windows_core::Interface::vtable(self).Reserved6)(::windows_core::Interface::as_raw(self))
    }
}
impl ::core::convert::From<IObjPool> for ::windows_core::IUnknown {
    fn from(value: IObjPool) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjPool> for ::windows_core::IUnknown {
    fn from(value: &IObjPool) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IObjPool {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IObjPool {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IObjPool {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IObjPool {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjPool {}
impl ::core::fmt::Debug for IObjPool {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjPool").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IObjPool {
    type Vtable = IObjPool_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d8805a0_2ea7_11d1_b1cc_00aa00ba3258);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjPool_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Reserved1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub PutEndTx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobj: *mut ::core::ffi::c_void),
    pub Reserved5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved6: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[repr(transparent)]
pub struct IObjectConstruct(::windows_core::IUnknown);
impl IObjectConstruct {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Construct<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, pctorobj: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Construct)(::windows_core::Interface::as_raw(self), pctorobj.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IObjectConstruct> for ::windows_core::IUnknown {
    fn from(value: IObjectConstruct) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectConstruct> for ::windows_core::IUnknown {
    fn from(value: &IObjectConstruct) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IObjectConstruct {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IObjectConstruct {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IObjectConstruct {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IObjectConstruct {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectConstruct {}
impl ::core::fmt::Debug for IObjectConstruct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectConstruct").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IObjectConstruct {
    type Vtable = IObjectConstruct_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x41c4f8b3_7439_11d2_98cb_00c04f8ee1c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectConstruct_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Construct: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctorobj: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Construct: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IObjectConstructString(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IObjectConstructString {
    pub unsafe fn ConstructString(&self, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ConstructString)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pval)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IObjectConstructString> for ::windows_core::IUnknown {
    fn from(value: IObjectConstructString) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IObjectConstructString> for ::windows_core::IUnknown {
    fn from(value: &IObjectConstructString) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IObjectConstructString {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IObjectConstructString {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IObjectConstructString> for super::Com::IDispatch {
    fn from(value: IObjectConstructString) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IObjectConstructString> for super::Com::IDispatch {
    fn from(value: &IObjectConstructString) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IObjectConstructString {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IObjectConstructString {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IObjectConstructString {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IObjectConstructString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IObjectConstructString {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IObjectConstructString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectConstructString").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IObjectConstructString {
    type Vtable = IObjectConstructString_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x41c4f8b2_7439_11d2_98cb_00c04f8ee1c4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IObjectConstructString_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ConstructString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IObjectContext(::windows_core::IUnknown);
impl IObjectContext {
    pub unsafe fn CreateInstance(&self, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateInstance)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    pub unsafe fn SetComplete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetComplete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetAbort(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAbort)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnableCommit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnableCommit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisableCommit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisableCommit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsInTransaction(&self) -> ::win32_foundation::BOOL {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).IsInTransaction)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn IsSecurityEnabled(&self) -> ::win32_foundation::BOOL {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).IsSecurityEnabled)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn IsCallerInRole<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrrole: Param0, pfisinrole: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsCallerInRole)(::windows_core::Interface::as_raw(self), bstrrole.into_param().abi(), ::core::mem::transmute(pfisinrole)).ok()
    }
}
impl ::core::convert::From<IObjectContext> for ::windows_core::IUnknown {
    fn from(value: IObjectContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectContext> for ::windows_core::IUnknown {
    fn from(value: &IObjectContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IObjectContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IObjectContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IObjectContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IObjectContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectContext {}
impl ::core::fmt::Debug for IObjectContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectContext").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IObjectContext {
    type Vtable = IObjectContext_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x51372ae0_cae7_11cf_be81_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectContext_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnableCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DisableCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsInTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL,
    pub IsSecurityEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL,
    pub IsCallerInRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrole: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pfisinrole: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IObjectContextActivity(::windows_core::IUnknown);
impl IObjectContextActivity {
    pub unsafe fn GetActivityId(&self, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetActivityId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguid)).ok()
    }
}
impl ::core::convert::From<IObjectContextActivity> for ::windows_core::IUnknown {
    fn from(value: IObjectContextActivity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectContextActivity> for ::windows_core::IUnknown {
    fn from(value: &IObjectContextActivity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IObjectContextActivity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IObjectContextActivity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IObjectContextActivity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IObjectContextActivity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectContextActivity {}
impl ::core::fmt::Debug for IObjectContextActivity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectContextActivity").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IObjectContextActivity {
    type Vtable = IObjectContextActivity_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x51372afc_cae7_11cf_be81_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectContextActivity_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IObjectContextInfo(::windows_core::IUnknown);
impl IObjectContextInfo {
    pub unsafe fn IsInTransaction(&self) -> ::win32_foundation::BOOL {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).IsInTransaction)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn GetTransaction(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).GetTransaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn GetTransactionId(&self, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetTransactionId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguid)).ok()
    }
    pub unsafe fn GetActivityId(&self, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetActivityId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguid)).ok()
    }
    pub unsafe fn GetContextId(&self, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetContextId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguid)).ok()
    }
}
impl ::core::convert::From<IObjectContextInfo> for ::windows_core::IUnknown {
    fn from(value: IObjectContextInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectContextInfo> for ::windows_core::IUnknown {
    fn from(value: &IObjectContextInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IObjectContextInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IObjectContextInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IObjectContextInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IObjectContextInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectContextInfo {}
impl ::core::fmt::Debug for IObjectContextInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectContextInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IObjectContextInfo {
    type Vtable = IObjectContextInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75b52ddb_e8ed_11d1_93ad_00aa00ba3258);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectContextInfo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub IsInTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL,
    pub GetTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptrans: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetTransactionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetContextId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IObjectContextInfo2(::windows_core::IUnknown);
impl IObjectContextInfo2 {
    pub unsafe fn IsInTransaction(&self) -> ::win32_foundation::BOOL {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).base__.IsInTransaction)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn GetTransaction(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn GetTransactionId(&self, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetTransactionId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguid)).ok()
    }
    pub unsafe fn GetActivityId(&self, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetActivityId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguid)).ok()
    }
    pub unsafe fn GetContextId(&self, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetContextId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguid)).ok()
    }
    pub unsafe fn GetPartitionId(&self, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPartitionId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguid)).ok()
    }
    pub unsafe fn GetApplicationId(&self, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetApplicationId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguid)).ok()
    }
    pub unsafe fn GetApplicationInstanceId(&self, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetApplicationInstanceId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguid)).ok()
    }
}
impl ::core::convert::From<IObjectContextInfo2> for ::windows_core::IUnknown {
    fn from(value: IObjectContextInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectContextInfo2> for ::windows_core::IUnknown {
    fn from(value: &IObjectContextInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IObjectContextInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IObjectContextInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IObjectContextInfo2> for IObjectContextInfo {
    fn from(value: IObjectContextInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectContextInfo2> for IObjectContextInfo {
    fn from(value: &IObjectContextInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IObjectContextInfo> for IObjectContextInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, IObjectContextInfo> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IObjectContextInfo> for &'a IObjectContextInfo2 {
    fn into_param(self) -> ::windows_core::Param<'a, IObjectContextInfo> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IObjectContextInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IObjectContextInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectContextInfo2 {}
impl ::core::fmt::Debug for IObjectContextInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectContextInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IObjectContextInfo2 {
    type Vtable = IObjectContextInfo2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x594be71a_4bc4_438b_9197_cfd176248b09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectContextInfo2_Vtbl {
    pub base__: IObjectContextInfo_Vtbl,
    pub GetPartitionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetApplicationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetApplicationInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IObjectContextTip(::windows_core::IUnknown);
impl IObjectContextTip {
    pub unsafe fn GetTipUrl(&self, ptipurl: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetTipUrl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptipurl)).ok()
    }
}
impl ::core::convert::From<IObjectContextTip> for ::windows_core::IUnknown {
    fn from(value: IObjectContextTip) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectContextTip> for ::windows_core::IUnknown {
    fn from(value: &IObjectContextTip) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IObjectContextTip {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IObjectContextTip {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IObjectContextTip {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IObjectContextTip {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectContextTip {}
impl ::core::fmt::Debug for IObjectContextTip {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectContextTip").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IObjectContextTip {
    type Vtable = IObjectContextTip_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92fd41ca_bad9_11d2_9a2d_00c04f797bc9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectContextTip_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetTipUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptipurl: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IObjectControl(::windows_core::IUnknown);
impl IObjectControl {
    pub unsafe fn Activate(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Activate)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Deactivate(&self) {
        (::windows_core::Interface::vtable(self).Deactivate)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn CanBePooled(&self) -> ::win32_foundation::BOOL {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).CanBePooled)(::windows_core::Interface::as_raw(self)))
    }
}
impl ::core::convert::From<IObjectControl> for ::windows_core::IUnknown {
    fn from(value: IObjectControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectControl> for ::windows_core::IUnknown {
    fn from(value: &IObjectControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IObjectControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IObjectControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IObjectControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IObjectControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectControl {}
impl ::core::fmt::Debug for IObjectControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IObjectControl {
    type Vtable = IObjectControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x51372aec_cae7_11cf_be81_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectControl_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub CanBePooled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL,
}
#[repr(transparent)]
pub struct IPlaybackControl(::windows_core::IUnknown);
impl IPlaybackControl {
    pub unsafe fn FinalClientRetry(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FinalClientRetry)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FinalServerRetry(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FinalServerRetry)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IPlaybackControl> for ::windows_core::IUnknown {
    fn from(value: IPlaybackControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPlaybackControl> for ::windows_core::IUnknown {
    fn from(value: &IPlaybackControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPlaybackControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPlaybackControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPlaybackControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPlaybackControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPlaybackControl {}
impl ::core::fmt::Debug for IPlaybackControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPlaybackControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPlaybackControl {
    type Vtable = IPlaybackControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x51372afd_cae7_11cf_be81_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackControl_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub FinalClientRetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FinalServerRetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPoolManager(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPoolManager {
    pub unsafe fn ShutdownPool<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, clsidorprogid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ShutdownPool)(::windows_core::Interface::as_raw(self), clsidorprogid.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IPoolManager> for ::windows_core::IUnknown {
    fn from(value: IPoolManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IPoolManager> for ::windows_core::IUnknown {
    fn from(value: &IPoolManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPoolManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPoolManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IPoolManager> for super::Com::IDispatch {
    fn from(value: IPoolManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IPoolManager> for super::Com::IDispatch {
    fn from(value: &IPoolManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IPoolManager {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IPoolManager {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPoolManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPoolManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPoolManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPoolManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPoolManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IPoolManager {
    type Vtable = IPoolManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0a469861_5a91_43a0_99b6_d5e179bb0631);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPoolManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ShutdownPool: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsidorprogid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IProcessInitializer(::windows_core::IUnknown);
impl IProcessInitializer {
    pub unsafe fn Startup<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, punkprocesscontrol: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Startup)(::windows_core::Interface::as_raw(self), punkprocesscontrol.into_param().abi()).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Shutdown)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IProcessInitializer> for ::windows_core::IUnknown {
    fn from(value: IProcessInitializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProcessInitializer> for ::windows_core::IUnknown {
    fn from(value: &IProcessInitializer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IProcessInitializer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IProcessInitializer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IProcessInitializer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProcessInitializer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProcessInitializer {}
impl ::core::fmt::Debug for IProcessInitializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProcessInitializer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IProcessInitializer {
    type Vtable = IProcessInitializer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1113f52d_dc7f_4943_aed6_88d04027e32a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessInitializer_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Startup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkprocesscontrol: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISecurityCallContext(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISecurityCallContext {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn IsCallerInRole<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrrole: Param0) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsCallerInRole)(::windows_core::Interface::as_raw(self), bstrrole.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsSecurityEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsSecurityEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn IsUserInRole<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, puser: *const super::Com::VARIANT, bstrrole: Param1) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsUserInRole)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(puser), bstrrole.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISecurityCallContext> for ::windows_core::IUnknown {
    fn from(value: ISecurityCallContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISecurityCallContext> for ::windows_core::IUnknown {
    fn from(value: &ISecurityCallContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISecurityCallContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISecurityCallContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISecurityCallContext> for super::Com::IDispatch {
    fn from(value: ISecurityCallContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISecurityCallContext> for super::Com::IDispatch {
    fn from(value: &ISecurityCallContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISecurityCallContext {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISecurityCallContext {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISecurityCallContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISecurityCallContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISecurityCallContext {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISecurityCallContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISecurityCallContext").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ISecurityCallContext {
    type Vtable = ISecurityCallContext_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcafc823e_b441_11d1_b82b_0000f8757e2a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityCallContext_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pitem: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsCallerInRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrole: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pfinrole: *mut i16) -> ::windows_core::HRESULT,
    pub IsSecurityEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisenabled: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub IsUserInRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puser: *const super::Com::VARIANT, bstrrole: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pfinrole: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    IsUserInRole: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISecurityCallersColl(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISecurityCallersColl {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<ISecurityIdentityColl> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISecurityIdentityColl>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISecurityCallersColl> for ::windows_core::IUnknown {
    fn from(value: ISecurityCallersColl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISecurityCallersColl> for ::windows_core::IUnknown {
    fn from(value: &ISecurityCallersColl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISecurityCallersColl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISecurityCallersColl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISecurityCallersColl> for super::Com::IDispatch {
    fn from(value: ISecurityCallersColl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISecurityCallersColl> for super::Com::IDispatch {
    fn from(value: &ISecurityCallersColl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISecurityCallersColl {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISecurityCallersColl {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISecurityCallersColl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISecurityCallersColl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISecurityCallersColl {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISecurityCallersColl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISecurityCallersColl").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ISecurityCallersColl {
    type Vtable = ISecurityCallersColl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcafc823d_b441_11d1_b82b_0000f8757e2a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityCallersColl_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pobj: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISecurityIdentityColl(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISecurityIdentityColl {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISecurityIdentityColl> for ::windows_core::IUnknown {
    fn from(value: ISecurityIdentityColl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISecurityIdentityColl> for ::windows_core::IUnknown {
    fn from(value: &ISecurityIdentityColl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISecurityIdentityColl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISecurityIdentityColl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISecurityIdentityColl> for super::Com::IDispatch {
    fn from(value: ISecurityIdentityColl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISecurityIdentityColl> for super::Com::IDispatch {
    fn from(value: &ISecurityIdentityColl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISecurityIdentityColl {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISecurityIdentityColl {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISecurityIdentityColl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISecurityIdentityColl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISecurityIdentityColl {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISecurityIdentityColl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISecurityIdentityColl").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ISecurityIdentityColl {
    type Vtable = ISecurityIdentityColl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcafc823c_b441_11d1_b82b_0000f8757e2a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityIdentityColl_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pitem: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISecurityProperty(::windows_core::IUnknown);
impl ISecurityProperty {
    pub unsafe fn GetDirectCreatorSID(&self, psid: *mut ::win32_foundation::PSID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDirectCreatorSID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(psid)).ok()
    }
    pub unsafe fn GetOriginalCreatorSID(&self, psid: *mut ::win32_foundation::PSID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOriginalCreatorSID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(psid)).ok()
    }
    pub unsafe fn GetDirectCallerSID(&self, psid: *mut ::win32_foundation::PSID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDirectCallerSID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(psid)).ok()
    }
    pub unsafe fn GetOriginalCallerSID(&self, psid: *mut ::win32_foundation::PSID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOriginalCallerSID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(psid)).ok()
    }
    pub unsafe fn ReleaseSID<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>>(&self, psid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseSID)(::windows_core::Interface::as_raw(self), psid.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ISecurityProperty> for ::windows_core::IUnknown {
    fn from(value: ISecurityProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISecurityProperty> for ::windows_core::IUnknown {
    fn from(value: &ISecurityProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISecurityProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISecurityProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISecurityProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISecurityProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISecurityProperty {}
impl ::core::fmt::Debug for ISecurityProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISecurityProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISecurityProperty {
    type Vtable = ISecurityProperty_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x51372aea_cae7_11cf_be81_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityProperty_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetDirectCreatorSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psid: *mut ::win32_foundation::PSID) -> ::windows_core::HRESULT,
    pub GetOriginalCreatorSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psid: *mut ::win32_foundation::PSID) -> ::windows_core::HRESULT,
    pub GetDirectCallerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psid: *mut ::win32_foundation::PSID) -> ::windows_core::HRESULT,
    pub GetOriginalCallerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psid: *mut ::win32_foundation::PSID) -> ::windows_core::HRESULT,
    pub ReleaseSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psid: ::win32_foundation::PSID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISelectCOMLBServer(::windows_core::IUnknown);
impl ISelectCOMLBServer {
    pub unsafe fn Init(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Init)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetLBServer<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, punk: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetLBServer)(::windows_core::Interface::as_raw(self), punk.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ISelectCOMLBServer> for ::windows_core::IUnknown {
    fn from(value: ISelectCOMLBServer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISelectCOMLBServer> for ::windows_core::IUnknown {
    fn from(value: &ISelectCOMLBServer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISelectCOMLBServer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISelectCOMLBServer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISelectCOMLBServer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISelectCOMLBServer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISelectCOMLBServer {}
impl ::core::fmt::Debug for ISelectCOMLBServer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISelectCOMLBServer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISelectCOMLBServer {
    type Vtable = ISelectCOMLBServer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdcf443f4_3f8a_4872_b9f0_369a796d12d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectCOMLBServer_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetLBServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISendMethodEvents(::windows_core::IUnknown);
impl ISendMethodEvents {
    pub unsafe fn SendMethodCall(&self, pidentity: *const ::core::ffi::c_void, riid: *const ::windows_core::GUID, dwmeth: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendMethodCall)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pidentity), ::core::mem::transmute(riid), ::core::mem::transmute(dwmeth)).ok()
    }
    pub unsafe fn SendMethodReturn(&self, pidentity: *const ::core::ffi::c_void, riid: *const ::windows_core::GUID, dwmeth: u32, hrcall: ::windows_core::HRESULT, hrserver: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendMethodReturn)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pidentity), ::core::mem::transmute(riid), ::core::mem::transmute(dwmeth), ::core::mem::transmute(hrcall), ::core::mem::transmute(hrserver)).ok()
    }
}
impl ::core::convert::From<ISendMethodEvents> for ::windows_core::IUnknown {
    fn from(value: ISendMethodEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISendMethodEvents> for ::windows_core::IUnknown {
    fn from(value: &ISendMethodEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISendMethodEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISendMethodEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISendMethodEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISendMethodEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISendMethodEvents {}
impl ::core::fmt::Debug for ISendMethodEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISendMethodEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISendMethodEvents {
    type Vtable = ISendMethodEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2732fd59_b2b4_4d44_878c_8b8f09626008);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISendMethodEvents_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SendMethodCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidentity: *const ::core::ffi::c_void, riid: *const ::windows_core::GUID, dwmeth: u32) -> ::windows_core::HRESULT,
    pub SendMethodReturn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidentity: *const ::core::ffi::c_void, riid: *const ::windows_core::GUID, dwmeth: u32, hrcall: ::windows_core::HRESULT, hrserver: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IServiceActivity(::windows_core::IUnknown);
impl IServiceActivity {
    pub unsafe fn SynchronousCall<'a, Param0: ::windows_core::IntoParam<'a, IServiceCall>>(&self, piservicecall: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SynchronousCall)(::windows_core::Interface::as_raw(self), piservicecall.into_param().abi()).ok()
    }
    pub unsafe fn AsynchronousCall<'a, Param0: ::windows_core::IntoParam<'a, IServiceCall>>(&self, piservicecall: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AsynchronousCall)(::windows_core::Interface::as_raw(self), piservicecall.into_param().abi()).ok()
    }
    pub unsafe fn BindToCurrentThread(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BindToCurrentThread)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn UnbindFromThread(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnbindFromThread)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IServiceActivity> for ::windows_core::IUnknown {
    fn from(value: IServiceActivity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceActivity> for ::windows_core::IUnknown {
    fn from(value: &IServiceActivity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IServiceActivity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IServiceActivity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IServiceActivity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceActivity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceActivity {}
impl ::core::fmt::Debug for IServiceActivity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceActivity").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IServiceActivity {
    type Vtable = IServiceActivity_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x67532e0c_9e2f_4450_a354_035633944e17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceActivity_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SynchronousCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piservicecall: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AsynchronousCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piservicecall: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub BindToCurrentThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UnbindFromThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IServiceCall(::windows_core::IUnknown);
impl IServiceCall {
    pub unsafe fn OnCall(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnCall)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IServiceCall> for ::windows_core::IUnknown {
    fn from(value: IServiceCall) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceCall> for ::windows_core::IUnknown {
    fn from(value: &IServiceCall) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IServiceCall {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IServiceCall {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IServiceCall {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceCall {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceCall {}
impl ::core::fmt::Debug for IServiceCall {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceCall").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IServiceCall {
    type Vtable = IServiceCall_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbd3e2e12_42dd_40f4_a09a_95a50c58304b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceCall_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub OnCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IServiceComTIIntrinsicsConfig(::windows_core::IUnknown);
impl IServiceComTIIntrinsicsConfig {
    pub unsafe fn ComTIIntrinsicsConfig(&self, comtiintrinsicsconfig: CSC_COMTIIntrinsicsConfig) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ComTIIntrinsicsConfig)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(comtiintrinsicsconfig)).ok()
    }
}
impl ::core::convert::From<IServiceComTIIntrinsicsConfig> for ::windows_core::IUnknown {
    fn from(value: IServiceComTIIntrinsicsConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceComTIIntrinsicsConfig> for ::windows_core::IUnknown {
    fn from(value: &IServiceComTIIntrinsicsConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IServiceComTIIntrinsicsConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IServiceComTIIntrinsicsConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IServiceComTIIntrinsicsConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceComTIIntrinsicsConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceComTIIntrinsicsConfig {}
impl ::core::fmt::Debug for IServiceComTIIntrinsicsConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceComTIIntrinsicsConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IServiceComTIIntrinsicsConfig {
    type Vtable = IServiceComTIIntrinsicsConfig_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x09e6831e_04e1_4ed4_9d0f_e8b168bafeaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceComTIIntrinsicsConfig_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ComTIIntrinsicsConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comtiintrinsicsconfig: CSC_COMTIIntrinsicsConfig) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IServiceIISIntrinsicsConfig(::windows_core::IUnknown);
impl IServiceIISIntrinsicsConfig {
    pub unsafe fn IISIntrinsicsConfig(&self, iisintrinsicsconfig: CSC_IISIntrinsicsConfig) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IISIntrinsicsConfig)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iisintrinsicsconfig)).ok()
    }
}
impl ::core::convert::From<IServiceIISIntrinsicsConfig> for ::windows_core::IUnknown {
    fn from(value: IServiceIISIntrinsicsConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceIISIntrinsicsConfig> for ::windows_core::IUnknown {
    fn from(value: &IServiceIISIntrinsicsConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IServiceIISIntrinsicsConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IServiceIISIntrinsicsConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IServiceIISIntrinsicsConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceIISIntrinsicsConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceIISIntrinsicsConfig {}
impl ::core::fmt::Debug for IServiceIISIntrinsicsConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceIISIntrinsicsConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IServiceIISIntrinsicsConfig {
    type Vtable = IServiceIISIntrinsicsConfig_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1a0cf920_d452_46f4_bc36_48118d54ea52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceIISIntrinsicsConfig_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub IISIntrinsicsConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iisintrinsicsconfig: CSC_IISIntrinsicsConfig) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IServiceInheritanceConfig(::windows_core::IUnknown);
impl IServiceInheritanceConfig {
    pub unsafe fn ContainingContextTreatment(&self, inheritanceconfig: CSC_InheritanceConfig) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ContainingContextTreatment)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(inheritanceconfig)).ok()
    }
}
impl ::core::convert::From<IServiceInheritanceConfig> for ::windows_core::IUnknown {
    fn from(value: IServiceInheritanceConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceInheritanceConfig> for ::windows_core::IUnknown {
    fn from(value: &IServiceInheritanceConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IServiceInheritanceConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IServiceInheritanceConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IServiceInheritanceConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceInheritanceConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceInheritanceConfig {}
impl ::core::fmt::Debug for IServiceInheritanceConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceInheritanceConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IServiceInheritanceConfig {
    type Vtable = IServiceInheritanceConfig_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92186771_d3b4_4d77_a8ea_ee842d586f35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceInheritanceConfig_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ContainingContextTreatment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inheritanceconfig: CSC_InheritanceConfig) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IServicePartitionConfig(::windows_core::IUnknown);
impl IServicePartitionConfig {
    pub unsafe fn PartitionConfig(&self, partitionconfig: CSC_PartitionConfig) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PartitionConfig)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(partitionconfig)).ok()
    }
    pub unsafe fn PartitionID(&self, guidpartitionid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PartitionID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(guidpartitionid)).ok()
    }
}
impl ::core::convert::From<IServicePartitionConfig> for ::windows_core::IUnknown {
    fn from(value: IServicePartitionConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServicePartitionConfig> for ::windows_core::IUnknown {
    fn from(value: &IServicePartitionConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IServicePartitionConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IServicePartitionConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IServicePartitionConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServicePartitionConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServicePartitionConfig {}
impl ::core::fmt::Debug for IServicePartitionConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServicePartitionConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IServicePartitionConfig {
    type Vtable = IServicePartitionConfig_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80182d03_5ea4_4831_ae97_55beffc2e590);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServicePartitionConfig_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub PartitionConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partitionconfig: CSC_PartitionConfig) -> ::windows_core::HRESULT,
    pub PartitionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidpartitionid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IServicePool(::windows_core::IUnknown);
impl IServicePool {
    pub unsafe fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, ppoolconfig: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), ppoolconfig.into_param().abi()).ok()
    }
    pub unsafe fn GetObject(&self, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetObject)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Shutdown)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IServicePool> for ::windows_core::IUnknown {
    fn from(value: IServicePool) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServicePool> for ::windows_core::IUnknown {
    fn from(value: &IServicePool) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IServicePool {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IServicePool {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IServicePool {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServicePool {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServicePool {}
impl ::core::fmt::Debug for IServicePool {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServicePool").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IServicePool {
    type Vtable = IServicePool_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb302df81_ea45_451e_99a2_09f9fd1b1e13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServicePool_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppoolconfig: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IServicePoolConfig(::windows_core::IUnknown);
impl IServicePoolConfig {
    pub unsafe fn SetMaxPoolSize(&self, dwmaxpool: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxPoolSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwmaxpool)).ok()
    }
    pub unsafe fn MaxPoolSize(&self, pdwmaxpool: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MaxPoolSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwmaxpool)).ok()
    }
    pub unsafe fn SetMinPoolSize(&self, dwminpool: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMinPoolSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwminpool)).ok()
    }
    pub unsafe fn MinPoolSize(&self, pdwminpool: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MinPoolSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwminpool)).ok()
    }
    pub unsafe fn SetCreationTimeout(&self, dwcreationtimeout: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCreationTimeout)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwcreationtimeout)).ok()
    }
    pub unsafe fn CreationTimeout(&self, pdwcreationtimeout: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreationTimeout)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwcreationtimeout)).ok()
    }
    pub unsafe fn SetTransactionAffinity<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, ftxaffinity: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTransactionAffinity)(::windows_core::Interface::as_raw(self), ftxaffinity.into_param().abi()).ok()
    }
    pub unsafe fn TransactionAffinity(&self, pftxaffinity: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TransactionAffinity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pftxaffinity)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetClassFactory<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IClassFactory>>(&self, pfactory: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClassFactory)(::windows_core::Interface::as_raw(self), pfactory.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ClassFactory(&self) -> ::windows_core::Result<super::Com::IClassFactory> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ClassFactory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IClassFactory>(result__)
    }
}
impl ::core::convert::From<IServicePoolConfig> for ::windows_core::IUnknown {
    fn from(value: IServicePoolConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServicePoolConfig> for ::windows_core::IUnknown {
    fn from(value: &IServicePoolConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IServicePoolConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IServicePoolConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IServicePoolConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServicePoolConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServicePoolConfig {}
impl ::core::fmt::Debug for IServicePoolConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServicePoolConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IServicePoolConfig {
    type Vtable = IServicePoolConfig_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9690656_5bca_470c_8451_250c1f43a33e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServicePoolConfig_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetMaxPoolSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxpool: u32) -> ::windows_core::HRESULT,
    pub MaxPoolSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmaxpool: *mut u32) -> ::windows_core::HRESULT,
    pub SetMinPoolSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwminpool: u32) -> ::windows_core::HRESULT,
    pub MinPoolSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwminpool: *mut u32) -> ::windows_core::HRESULT,
    pub SetCreationTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcreationtimeout: u32) -> ::windows_core::HRESULT,
    pub CreationTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcreationtimeout: *mut u32) -> ::windows_core::HRESULT,
    pub SetTransactionAffinity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ftxaffinity: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub TransactionAffinity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pftxaffinity: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetClassFactory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfactory: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetClassFactory: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ClassFactory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfactory: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ClassFactory: usize,
}
#[repr(transparent)]
pub struct IServiceSxsConfig(::windows_core::IUnknown);
impl IServiceSxsConfig {
    pub unsafe fn SxsConfig(&self, scsconfig: CSC_SxsConfig) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SxsConfig)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(scsconfig)).ok()
    }
    pub unsafe fn SxsName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, szsxsname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SxsName)(::windows_core::Interface::as_raw(self), szsxsname.into_param().abi()).ok()
    }
    pub unsafe fn SxsDirectory<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, szsxsdirectory: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SxsDirectory)(::windows_core::Interface::as_raw(self), szsxsdirectory.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IServiceSxsConfig> for ::windows_core::IUnknown {
    fn from(value: IServiceSxsConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceSxsConfig> for ::windows_core::IUnknown {
    fn from(value: &IServiceSxsConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IServiceSxsConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IServiceSxsConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IServiceSxsConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceSxsConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceSxsConfig {}
impl ::core::fmt::Debug for IServiceSxsConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceSxsConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IServiceSxsConfig {
    type Vtable = IServiceSxsConfig_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc7cd7379_f3f2_4634_811b_703281d73e08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceSxsConfig_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SxsConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scsconfig: CSC_SxsConfig) -> ::windows_core::HRESULT,
    pub SxsName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szsxsname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SxsDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szsxsdirectory: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IServiceSynchronizationConfig(::windows_core::IUnknown);
impl IServiceSynchronizationConfig {
    pub unsafe fn ConfigureSynchronization(&self, synchconfig: CSC_SynchronizationConfig) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ConfigureSynchronization)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(synchconfig)).ok()
    }
}
impl ::core::convert::From<IServiceSynchronizationConfig> for ::windows_core::IUnknown {
    fn from(value: IServiceSynchronizationConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceSynchronizationConfig> for ::windows_core::IUnknown {
    fn from(value: &IServiceSynchronizationConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IServiceSynchronizationConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IServiceSynchronizationConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IServiceSynchronizationConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceSynchronizationConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceSynchronizationConfig {}
impl ::core::fmt::Debug for IServiceSynchronizationConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceSynchronizationConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IServiceSynchronizationConfig {
    type Vtable = IServiceSynchronizationConfig_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd880e81_6dce_4c58_af83_a208846c0030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceSynchronizationConfig_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ConfigureSynchronization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, synchconfig: CSC_SynchronizationConfig) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IServiceSysTxnConfig(::windows_core::IUnknown);
impl IServiceSysTxnConfig {
    pub unsafe fn ConfigureTransaction(&self, transactionconfig: CSC_TransactionConfig) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.ConfigureTransaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(transactionconfig)).ok()
    }
    pub unsafe fn IsolationLevel(&self, option: COMAdminTxIsolationLevelOptions) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.IsolationLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(option)).ok()
    }
    pub unsafe fn TransactionTimeout(&self, ultimeoutsec: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.TransactionTimeout)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ultimeoutsec)).ok()
    }
    pub unsafe fn BringYourOwnTransaction<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, sztipurl: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.BringYourOwnTransaction)(::windows_core::Interface::as_raw(self), sztipurl.into_param().abi()).ok()
    }
    pub unsafe fn NewTransactionDescription<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, sztxdesc: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.NewTransactionDescription)(::windows_core::Interface::as_raw(self), sztxdesc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub unsafe fn ConfigureBYOT<'a, Param0: ::windows_core::IntoParam<'a, super::DistributedTransactionCoordinator::ITransaction>>(&self, pitxbyot: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ConfigureBYOT)(::windows_core::Interface::as_raw(self), pitxbyot.into_param().abi()).ok()
    }
    pub unsafe fn ConfigureBYOTSysTxn<'a, Param0: ::windows_core::IntoParam<'a, ITransactionProxy>>(&self, ptxproxy: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ConfigureBYOTSysTxn)(::windows_core::Interface::as_raw(self), ptxproxy.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IServiceSysTxnConfig> for ::windows_core::IUnknown {
    fn from(value: IServiceSysTxnConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceSysTxnConfig> for ::windows_core::IUnknown {
    fn from(value: &IServiceSysTxnConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IServiceSysTxnConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IServiceSysTxnConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IServiceSysTxnConfig> for IServiceTransactionConfigBase {
    fn from(value: IServiceSysTxnConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceSysTxnConfig> for IServiceTransactionConfigBase {
    fn from(value: &IServiceSysTxnConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IServiceTransactionConfigBase> for IServiceSysTxnConfig {
    fn into_param(self) -> ::windows_core::Param<'a, IServiceTransactionConfigBase> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IServiceTransactionConfigBase> for &'a IServiceSysTxnConfig {
    fn into_param(self) -> ::windows_core::Param<'a, IServiceTransactionConfigBase> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IServiceSysTxnConfig> for IServiceTransactionConfig {
    fn from(value: IServiceSysTxnConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceSysTxnConfig> for IServiceTransactionConfig {
    fn from(value: &IServiceSysTxnConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IServiceTransactionConfig> for IServiceSysTxnConfig {
    fn into_param(self) -> ::windows_core::Param<'a, IServiceTransactionConfig> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IServiceTransactionConfig> for &'a IServiceSysTxnConfig {
    fn into_param(self) -> ::windows_core::Param<'a, IServiceTransactionConfig> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IServiceSysTxnConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceSysTxnConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceSysTxnConfig {}
impl ::core::fmt::Debug for IServiceSysTxnConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceSysTxnConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IServiceSysTxnConfig {
    type Vtable = IServiceSysTxnConfig_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33caf1a1_fcb8_472b_b45e_967448ded6d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceSysTxnConfig_Vtbl {
    pub base__: IServiceTransactionConfig_Vtbl,
    pub ConfigureBYOTSysTxn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptxproxy: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IServiceThreadPoolConfig(::windows_core::IUnknown);
impl IServiceThreadPoolConfig {
    pub unsafe fn SelectThreadPool(&self, threadpool: CSC_ThreadPool) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SelectThreadPool)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threadpool)).ok()
    }
    pub unsafe fn SetBindingInfo(&self, binding: CSC_Binding) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBindingInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(binding)).ok()
    }
}
impl ::core::convert::From<IServiceThreadPoolConfig> for ::windows_core::IUnknown {
    fn from(value: IServiceThreadPoolConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceThreadPoolConfig> for ::windows_core::IUnknown {
    fn from(value: &IServiceThreadPoolConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IServiceThreadPoolConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IServiceThreadPoolConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IServiceThreadPoolConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceThreadPoolConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceThreadPoolConfig {}
impl ::core::fmt::Debug for IServiceThreadPoolConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceThreadPoolConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IServiceThreadPoolConfig {
    type Vtable = IServiceThreadPoolConfig_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x186d89bc_f277_4bcc_80d5_4df7b836ef4a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceThreadPoolConfig_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SelectThreadPool: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadpool: CSC_ThreadPool) -> ::windows_core::HRESULT,
    pub SetBindingInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binding: CSC_Binding) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IServiceTrackerConfig(::windows_core::IUnknown);
impl IServiceTrackerConfig {
    pub unsafe fn TrackerConfig<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, trackerconfig: CSC_TrackerConfig, sztrackerappname: Param1, sztrackerctxname: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TrackerConfig)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(trackerconfig), sztrackerappname.into_param().abi(), sztrackerctxname.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IServiceTrackerConfig> for ::windows_core::IUnknown {
    fn from(value: IServiceTrackerConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceTrackerConfig> for ::windows_core::IUnknown {
    fn from(value: &IServiceTrackerConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IServiceTrackerConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IServiceTrackerConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IServiceTrackerConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceTrackerConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceTrackerConfig {}
impl ::core::fmt::Debug for IServiceTrackerConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceTrackerConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IServiceTrackerConfig {
    type Vtable = IServiceTrackerConfig_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6c3a3e1d_0ba6_4036_b76f_d0404db816c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceTrackerConfig_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub TrackerConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, trackerconfig: CSC_TrackerConfig, sztrackerappname: ::windows_core::PCWSTR, sztrackerctxname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IServiceTransactionConfig(::windows_core::IUnknown);
impl IServiceTransactionConfig {
    pub unsafe fn ConfigureTransaction(&self, transactionconfig: CSC_TransactionConfig) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ConfigureTransaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(transactionconfig)).ok()
    }
    pub unsafe fn IsolationLevel(&self, option: COMAdminTxIsolationLevelOptions) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.IsolationLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(option)).ok()
    }
    pub unsafe fn TransactionTimeout(&self, ultimeoutsec: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TransactionTimeout)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ultimeoutsec)).ok()
    }
    pub unsafe fn BringYourOwnTransaction<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, sztipurl: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BringYourOwnTransaction)(::windows_core::Interface::as_raw(self), sztipurl.into_param().abi()).ok()
    }
    pub unsafe fn NewTransactionDescription<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, sztxdesc: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.NewTransactionDescription)(::windows_core::Interface::as_raw(self), sztxdesc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub unsafe fn ConfigureBYOT<'a, Param0: ::windows_core::IntoParam<'a, super::DistributedTransactionCoordinator::ITransaction>>(&self, pitxbyot: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ConfigureBYOT)(::windows_core::Interface::as_raw(self), pitxbyot.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IServiceTransactionConfig> for ::windows_core::IUnknown {
    fn from(value: IServiceTransactionConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceTransactionConfig> for ::windows_core::IUnknown {
    fn from(value: &IServiceTransactionConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IServiceTransactionConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IServiceTransactionConfig {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IServiceTransactionConfig> for IServiceTransactionConfigBase {
    fn from(value: IServiceTransactionConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceTransactionConfig> for IServiceTransactionConfigBase {
    fn from(value: &IServiceTransactionConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IServiceTransactionConfigBase> for IServiceTransactionConfig {
    fn into_param(self) -> ::windows_core::Param<'a, IServiceTransactionConfigBase> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IServiceTransactionConfigBase> for &'a IServiceTransactionConfig {
    fn into_param(self) -> ::windows_core::Param<'a, IServiceTransactionConfigBase> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IServiceTransactionConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceTransactionConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceTransactionConfig {}
impl ::core::fmt::Debug for IServiceTransactionConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceTransactionConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IServiceTransactionConfig {
    type Vtable = IServiceTransactionConfig_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x59f4c2a3_d3d7_4a31_b6e4_6ab3177c50b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceTransactionConfig_Vtbl {
    pub base__: IServiceTransactionConfigBase_Vtbl,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub ConfigureBYOT: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitxbyot: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    ConfigureBYOT: usize,
}
#[repr(transparent)]
pub struct IServiceTransactionConfigBase(::windows_core::IUnknown);
impl IServiceTransactionConfigBase {
    pub unsafe fn ConfigureTransaction(&self, transactionconfig: CSC_TransactionConfig) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ConfigureTransaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(transactionconfig)).ok()
    }
    pub unsafe fn IsolationLevel(&self, option: COMAdminTxIsolationLevelOptions) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsolationLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(option)).ok()
    }
    pub unsafe fn TransactionTimeout(&self, ultimeoutsec: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TransactionTimeout)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ultimeoutsec)).ok()
    }
    pub unsafe fn BringYourOwnTransaction<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, sztipurl: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BringYourOwnTransaction)(::windows_core::Interface::as_raw(self), sztipurl.into_param().abi()).ok()
    }
    pub unsafe fn NewTransactionDescription<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, sztxdesc: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NewTransactionDescription)(::windows_core::Interface::as_raw(self), sztxdesc.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IServiceTransactionConfigBase> for ::windows_core::IUnknown {
    fn from(value: IServiceTransactionConfigBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceTransactionConfigBase> for ::windows_core::IUnknown {
    fn from(value: &IServiceTransactionConfigBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IServiceTransactionConfigBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IServiceTransactionConfigBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IServiceTransactionConfigBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceTransactionConfigBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceTransactionConfigBase {}
impl ::core::fmt::Debug for IServiceTransactionConfigBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceTransactionConfigBase").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IServiceTransactionConfigBase {
    type Vtable = IServiceTransactionConfigBase_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x772b3fbe_6ffd_42fb_b5f8_8f9b260f3810);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceTransactionConfigBase_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ConfigureTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transactionconfig: CSC_TransactionConfig) -> ::windows_core::HRESULT,
    pub IsolationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, option: COMAdminTxIsolationLevelOptions) -> ::windows_core::HRESULT,
    pub TransactionTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ultimeoutsec: u32) -> ::windows_core::HRESULT,
    pub BringYourOwnTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sztipurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub NewTransactionDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sztxdesc: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISharedProperty(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISharedProperty {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Value)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, val: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValue)(::windows_core::Interface::as_raw(self), val.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISharedProperty> for ::windows_core::IUnknown {
    fn from(value: ISharedProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISharedProperty> for ::windows_core::IUnknown {
    fn from(value: &ISharedProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISharedProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISharedProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISharedProperty> for super::Com::IDispatch {
    fn from(value: ISharedProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISharedProperty> for super::Com::IDispatch {
    fn from(value: &ISharedProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISharedProperty {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISharedProperty {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISharedProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISharedProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISharedProperty {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISharedProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISharedProperty").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ISharedProperty {
    type Vtable = ISharedProperty_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a005c01_a5de_11cf_9e66_00aa00a3f464);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISharedProperty_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISharedPropertyGroup(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISharedPropertyGroup {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertyByPosition(&self, index: i32, fexists: *mut i16, ppprop: *mut ::core::option::Option<ISharedProperty>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreatePropertyByPosition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(fexists), ::core::mem::transmute(ppprop)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_PropertyByPosition(&self, index: i32) -> ::windows_core::Result<ISharedProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_PropertyByPosition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISharedProperty>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0, fexists: *mut i16, ppprop: *mut ::core::option::Option<ISharedProperty>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateProperty)(::windows_core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(fexists), ::core::mem::transmute(ppprop)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Property<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<ISharedProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Property)(::windows_core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISharedProperty>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISharedPropertyGroup> for ::windows_core::IUnknown {
    fn from(value: ISharedPropertyGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISharedPropertyGroup> for ::windows_core::IUnknown {
    fn from(value: &ISharedPropertyGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISharedPropertyGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISharedPropertyGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISharedPropertyGroup> for super::Com::IDispatch {
    fn from(value: ISharedPropertyGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISharedPropertyGroup> for super::Com::IDispatch {
    fn from(value: &ISharedPropertyGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISharedPropertyGroup {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISharedPropertyGroup {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISharedPropertyGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISharedPropertyGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISharedPropertyGroup {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISharedPropertyGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISharedPropertyGroup").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ISharedPropertyGroup {
    type Vtable = ISharedPropertyGroup_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a005c07_a5de_11cf_9e66_00aa00a3f464);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPropertyGroup_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePropertyByPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, fexists: *mut i16, ppprop: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePropertyByPosition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_PropertyByPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, ppproperty: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_PropertyByPosition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, fexists: *mut i16, ppprop: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateProperty: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppproperty: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Property: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISharedPropertyGroupManager(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISharedPropertyGroupManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertyGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0, dwisomode: *mut i32, dwrelmode: *mut i32, fexists: *mut i16, ppgroup: *mut ::core::option::Option<ISharedPropertyGroup>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreatePropertyGroup)(::windows_core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(dwisomode), ::core::mem::transmute(dwrelmode), ::core::mem::transmute(fexists), ::core::mem::transmute(ppgroup)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Group<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<ISharedPropertyGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Group)(::windows_core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISharedPropertyGroup>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISharedPropertyGroupManager> for ::windows_core::IUnknown {
    fn from(value: ISharedPropertyGroupManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISharedPropertyGroupManager> for ::windows_core::IUnknown {
    fn from(value: &ISharedPropertyGroupManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISharedPropertyGroupManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISharedPropertyGroupManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISharedPropertyGroupManager> for super::Com::IDispatch {
    fn from(value: ISharedPropertyGroupManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISharedPropertyGroupManager> for super::Com::IDispatch {
    fn from(value: &ISharedPropertyGroupManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISharedPropertyGroupManager {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISharedPropertyGroupManager {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISharedPropertyGroupManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISharedPropertyGroupManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISharedPropertyGroupManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISharedPropertyGroupManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISharedPropertyGroupManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ISharedPropertyGroupManager {
    type Vtable = ISharedPropertyGroupManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a005c0d_a5de_11cf_9e66_00aa00a3f464);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPropertyGroupManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePropertyGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, dwisomode: *mut i32, dwrelmode: *mut i32, fexists: *mut i16, ppgroup: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePropertyGroup: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Group: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppgroup: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Group: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISystemAppEventData(::windows_core::IUnknown);
impl ISystemAppEventData {
    pub unsafe fn Startup(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Startup)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnDataChanged<'a, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, dwpid: u32, dwmask: u32, dwnumbersinks: u32, bstrdwmethodmask: Param3, dwreason: u32, u64tracehandle: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnDataChanged)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwpid), ::core::mem::transmute(dwmask), ::core::mem::transmute(dwnumbersinks), bstrdwmethodmask.into_param().abi(), ::core::mem::transmute(dwreason), ::core::mem::transmute(u64tracehandle)).ok()
    }
}
impl ::core::convert::From<ISystemAppEventData> for ::windows_core::IUnknown {
    fn from(value: ISystemAppEventData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISystemAppEventData> for ::windows_core::IUnknown {
    fn from(value: &ISystemAppEventData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISystemAppEventData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISystemAppEventData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISystemAppEventData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISystemAppEventData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISystemAppEventData {}
impl ::core::fmt::Debug for ISystemAppEventData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISystemAppEventData").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISystemAppEventData {
    type Vtable = ISystemAppEventData_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd6d48a3c_d5c5_49e7_8c74_99e4889ed52f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemAppEventData_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Startup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OnDataChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwpid: u32, dwmask: u32, dwnumbersinks: u32, bstrdwmethodmask: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, dwreason: u32, u64tracehandle: u64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IThreadPoolKnobs(::windows_core::IUnknown);
impl IThreadPoolKnobs {
    pub unsafe fn GetMaxThreads(&self, plcmaxthreads: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetMaxThreads)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(plcmaxthreads)).ok()
    }
    pub unsafe fn GetCurrentThreads(&self, plccurrentthreads: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCurrentThreads)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(plccurrentthreads)).ok()
    }
    pub unsafe fn SetMaxThreads(&self, lcmaxthreads: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxThreads)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lcmaxthreads)).ok()
    }
    pub unsafe fn GetDeleteDelay(&self, pmsecdeletedelay: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeleteDelay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pmsecdeletedelay)).ok()
    }
    pub unsafe fn SetDeleteDelay(&self, msecdeletedelay: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDeleteDelay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(msecdeletedelay)).ok()
    }
    pub unsafe fn GetMaxQueuedRequests(&self, plcmaxqueuedrequests: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetMaxQueuedRequests)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(plcmaxqueuedrequests)).ok()
    }
    pub unsafe fn GetCurrentQueuedRequests(&self, plccurrentqueuedrequests: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCurrentQueuedRequests)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(plccurrentqueuedrequests)).ok()
    }
    pub unsafe fn SetMaxQueuedRequests(&self, lcmaxqueuedrequests: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxQueuedRequests)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lcmaxqueuedrequests)).ok()
    }
    pub unsafe fn SetMinThreads(&self, lcminthreads: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMinThreads)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lcminthreads)).ok()
    }
    pub unsafe fn SetQueueDepth(&self, lcqueuedepth: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetQueueDepth)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lcqueuedepth)).ok()
    }
}
impl ::core::convert::From<IThreadPoolKnobs> for ::windows_core::IUnknown {
    fn from(value: IThreadPoolKnobs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IThreadPoolKnobs> for ::windows_core::IUnknown {
    fn from(value: &IThreadPoolKnobs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IThreadPoolKnobs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IThreadPoolKnobs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IThreadPoolKnobs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IThreadPoolKnobs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IThreadPoolKnobs {}
impl ::core::fmt::Debug for IThreadPoolKnobs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IThreadPoolKnobs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IThreadPoolKnobs {
    type Vtable = IThreadPoolKnobs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x51372af7_cae7_11cf_be81_00aa00a2fa25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThreadPoolKnobs_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetMaxThreads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcmaxthreads: *mut i32) -> ::windows_core::HRESULT,
    pub GetCurrentThreads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plccurrentthreads: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxThreads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcmaxthreads: i32) -> ::windows_core::HRESULT,
    pub GetDeleteDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsecdeletedelay: *mut i32) -> ::windows_core::HRESULT,
    pub SetDeleteDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msecdeletedelay: i32) -> ::windows_core::HRESULT,
    pub GetMaxQueuedRequests: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcmaxqueuedrequests: *mut i32) -> ::windows_core::HRESULT,
    pub GetCurrentQueuedRequests: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plccurrentqueuedrequests: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxQueuedRequests: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcmaxqueuedrequests: i32) -> ::windows_core::HRESULT,
    pub SetMinThreads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcminthreads: i32) -> ::windows_core::HRESULT,
    pub SetQueueDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcqueuedepth: i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITransactionContext(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITransactionContext {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, pszprogid: Param0) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).CreateInstance)(::windows_core::Interface::as_raw(self), pszprogid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Abort(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Abort)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITransactionContext> for ::windows_core::IUnknown {
    fn from(value: ITransactionContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITransactionContext> for ::windows_core::IUnknown {
    fn from(value: &ITransactionContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITransactionContext> for super::Com::IDispatch {
    fn from(value: ITransactionContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITransactionContext> for super::Com::IDispatch {
    fn from(value: &ITransactionContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ITransactionContext {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ITransactionContext {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITransactionContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITransactionContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITransactionContext {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITransactionContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionContext").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ITransactionContext {
    type Vtable = ITransactionContext_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7999fc21_d3c6_11cf_acab_00a024a55aef);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionContext_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszprogid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pobject: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateInstance: usize,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionContextEx(::windows_core::IUnknown);
impl ITransactionContextEx {
    pub unsafe fn CreateInstance<T: ::windows_core::Interface>(&self, rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows_core::Interface::vtable(self).CreateInstance)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rclsid), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Abort(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Abort)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ITransactionContextEx> for ::windows_core::IUnknown {
    fn from(value: ITransactionContextEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionContextEx> for ::windows_core::IUnknown {
    fn from(value: &ITransactionContextEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionContextEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionContextEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionContextEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionContextEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionContextEx {}
impl ::core::fmt::Debug for ITransactionContextEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionContextEx").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionContextEx {
    type Vtable = ITransactionContextEx_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7999fc22_d3c6_11cf_acab_00a024a55aef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionContextEx_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionProperty(::windows_core::IUnknown);
impl ITransactionProperty {
    pub unsafe fn Reserved1(&self) {
        (::windows_core::Interface::vtable(self).Reserved1)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved2(&self) {
        (::windows_core::Interface::vtable(self).Reserved2)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved3(&self) {
        (::windows_core::Interface::vtable(self).Reserved3)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved4(&self) {
        (::windows_core::Interface::vtable(self).Reserved4)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved5(&self) {
        (::windows_core::Interface::vtable(self).Reserved5)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved6(&self) {
        (::windows_core::Interface::vtable(self).Reserved6)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved7(&self) {
        (::windows_core::Interface::vtable(self).Reserved7)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved8(&self) {
        (::windows_core::Interface::vtable(self).Reserved8)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved9(&self) {
        (::windows_core::Interface::vtable(self).Reserved9)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetTransactionResourcePool(&self) -> ::windows_core::Result<ITransactionResourcePool> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetTransactionResourcePool)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITransactionResourcePool>(result__)
    }
    pub unsafe fn Reserved10(&self) {
        (::windows_core::Interface::vtable(self).Reserved10)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved11(&self) {
        (::windows_core::Interface::vtable(self).Reserved11)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved12(&self) {
        (::windows_core::Interface::vtable(self).Reserved12)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved13(&self) {
        (::windows_core::Interface::vtable(self).Reserved13)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved14(&self) {
        (::windows_core::Interface::vtable(self).Reserved14)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved15(&self) {
        (::windows_core::Interface::vtable(self).Reserved15)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved16(&self) {
        (::windows_core::Interface::vtable(self).Reserved16)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Reserved17(&self) {
        (::windows_core::Interface::vtable(self).Reserved17)(::windows_core::Interface::as_raw(self))
    }
}
impl ::core::convert::From<ITransactionProperty> for ::windows_core::IUnknown {
    fn from(value: ITransactionProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionProperty> for ::windows_core::IUnknown {
    fn from(value: &ITransactionProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionProperty {}
impl ::core::fmt::Debug for ITransactionProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionProperty {
    type Vtable = ITransactionProperty_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x788ea814_87b1_11d1_bba6_00c04fc2fa5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionProperty_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Reserved1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved6: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved9: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub GetTransactionResourcePool: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptxpool: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Reserved10: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved11: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved13: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved14: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved15: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Reserved17: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[repr(transparent)]
pub struct ITransactionProxy(::windows_core::IUnknown);
impl ITransactionProxy {
    pub unsafe fn Commit<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, guid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Commit)(::windows_core::Interface::as_raw(self), guid.into_param().abi()).ok()
    }
    pub unsafe fn Abort(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Abort)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub unsafe fn Promote(&self) -> ::windows_core::Result<super::DistributedTransactionCoordinator::ITransaction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Promote)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::DistributedTransactionCoordinator::ITransaction>(result__)
    }
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub unsafe fn CreateVoter<'a, Param0: ::windows_core::IntoParam<'a, super::DistributedTransactionCoordinator::ITransactionVoterNotifyAsync2>>(&self, ptxasync: Param0) -> ::windows_core::Result<super::DistributedTransactionCoordinator::ITransactionVoterBallotAsync2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateVoter)(::windows_core::Interface::as_raw(self), ptxasync.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::DistributedTransactionCoordinator::ITransactionVoterBallotAsync2>(result__)
    }
    pub unsafe fn GetIsolationLevel(&self, __midl__itransactionproxy0000: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetIsolationLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(__midl__itransactionproxy0000)).ok()
    }
    pub unsafe fn GetIdentifier(&self, pbstridentifier: *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetIdentifier)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstridentifier)).ok()
    }
    pub unsafe fn IsReusable(&self, pfisreusable: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsReusable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pfisreusable)).ok()
    }
}
impl ::core::convert::From<ITransactionProxy> for ::windows_core::IUnknown {
    fn from(value: ITransactionProxy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionProxy> for ::windows_core::IUnknown {
    fn from(value: &ITransactionProxy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionProxy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionProxy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionProxy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionProxy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionProxy {}
impl ::core::fmt::Debug for ITransactionProxy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionProxy").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionProxy {
    type Vtable = ITransactionProxy_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x02558374_df2e_4dae_bd6b_1d5c994f9bdc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionProxy_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub Promote: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    Promote: usize,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub CreateVoter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptxasync: ::windows_core::RawPtr, ppballot: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    CreateVoter: usize,
    pub GetIsolationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__itransactionproxy0000: *mut i32) -> ::windows_core::HRESULT,
    pub GetIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstridentifier: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub IsReusable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisreusable: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionResourcePool(::windows_core::IUnknown);
impl ITransactionResourcePool {
    pub unsafe fn PutResource<'a, Param0: ::windows_core::IntoParam<'a, IObjPool>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, ppool: Param0, punk: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PutResource)(::windows_core::Interface::as_raw(self), ppool.into_param().abi(), punk.into_param().abi()).ok()
    }
    pub unsafe fn GetResource<'a, Param0: ::windows_core::IntoParam<'a, IObjPool>>(&self, ppool: Param0) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).GetResource)(::windows_core::Interface::as_raw(self), ppool.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
impl ::core::convert::From<ITransactionResourcePool> for ::windows_core::IUnknown {
    fn from(value: ITransactionResourcePool) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionResourcePool> for ::windows_core::IUnknown {
    fn from(value: &ITransactionResourcePool) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionResourcePool {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionResourcePool {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionResourcePool {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionResourcePool {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionResourcePool {}
impl ::core::fmt::Debug for ITransactionResourcePool {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionResourcePool").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionResourcePool {
    type Vtable = ITransactionResourcePool_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc5feb7c1_346a_11d1_b1cc_00aa00ba3258);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionResourcePool_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub PutResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppool: ::windows_core::RawPtr, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppool: ::windows_core::RawPtr, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITransactionStatus(::windows_core::IUnknown);
impl ITransactionStatus {
    pub unsafe fn SetTransactionStatus(&self, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTransactionStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hrstatus)).ok()
    }
    pub unsafe fn GetTransactionStatus(&self, phrstatus: *mut ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetTransactionStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(phrstatus)).ok()
    }
}
impl ::core::convert::From<ITransactionStatus> for ::windows_core::IUnknown {
    fn from(value: ITransactionStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransactionStatus> for ::windows_core::IUnknown {
    fn from(value: &ITransactionStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITransactionStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITransactionStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransactionStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransactionStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionStatus {}
impl ::core::fmt::Debug for ITransactionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITransactionStatus {
    type Vtable = ITransactionStatus_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x61f589e8_3724_4898_a0a4_664ae9e1d1b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionStatus_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetTransactionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub GetTransactionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrstatus: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITxProxyHolder(::windows_core::IUnknown);
impl ITxProxyHolder {
    pub unsafe fn GetIdentifier(&self, pguidltx: *mut ::windows_core::GUID) {
        (::windows_core::Interface::vtable(self).GetIdentifier)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pguidltx))
    }
}
impl ::core::convert::From<ITxProxyHolder> for ::windows_core::IUnknown {
    fn from(value: ITxProxyHolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITxProxyHolder> for ::windows_core::IUnknown {
    fn from(value: &ITxProxyHolder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITxProxyHolder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITxProxyHolder {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITxProxyHolder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITxProxyHolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITxProxyHolder {}
impl ::core::fmt::Debug for ITxProxyHolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITxProxyHolder").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITxProxyHolder {
    type Vtable = ITxProxyHolder_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13d86f31_0139_41af_bcad_c7d50435fe9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITxProxyHolder_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidltx: *mut ::windows_core::GUID),
}
pub const LBEvents: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecabb0c1_7f19_11d2_978e_0000f8757e2a);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LockModes(pub i32);
pub const LockSetGet: LockModes = LockModes(0i32);
pub const LockMethod: LockModes = LockModes(1i32);
impl ::core::marker::Copy for LockModes {}
impl ::core::clone::Clone for LockModes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LockModes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LockModes {
    type Abi = Self;
}
impl ::core::fmt::Debug for LockModes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LockModes").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn MTSCreateActivity(riid: *const ::windows_core::GUID, ppobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MTSCreateActivity(riid: *const ::windows_core::GUID, ppobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        MTSCreateActivity(::core::mem::transmute(riid), ::core::mem::transmute(ppobj)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const MTXDM_E_ENLISTRESOURCEFAILED: u32 = 2147803392u32;
pub const MessageMover: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecabb0bf_7f19_11d2_978e_0000f8757e2a);
pub const MtsGrp: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4b2e958d_0393_11d1_b1ab_00aa00ba3258);
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ObjectContext(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ObjectContext {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprogid: Param0) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).CreateInstance)(::windows_core::Interface::as_raw(self), bstrprogid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn SetComplete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetComplete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetAbort(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAbort)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnableCommit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnableCommit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisableCommit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisableCommit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsInTransaction(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsInTransaction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsSecurityEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsSecurityEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsCallerInRole<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrrole: Param0) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsCallerInRole)(::windows_core::Interface::as_raw(self), bstrrole.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security(&self) -> ::windows_core::Result<SecurityProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Security)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<SecurityProperty>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ContextInfo(&self) -> ::windows_core::Result<ContextInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ContextInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ContextInfo>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ObjectContext> for ::windows_core::IUnknown {
    fn from(value: ObjectContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ObjectContext> for ::windows_core::IUnknown {
    fn from(value: &ObjectContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ObjectContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ObjectContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ObjectContext> for super::Com::IDispatch {
    fn from(value: ObjectContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ObjectContext> for super::Com::IDispatch {
    fn from(value: &ObjectContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ObjectContext {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ObjectContext {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ObjectContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ObjectContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ObjectContext {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ObjectContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ObjectContext").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ObjectContext {
    type Vtable = ObjectContext_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x74c08646_cedb_11cf_8b49_00aa00b8a790);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ObjectContext_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprogid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pobject: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateInstance: usize,
    pub SetComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnableCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DisableCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsInTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisintx: *mut i16) -> ::windows_core::HRESULT,
    pub IsSecurityEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisenabled: *mut i16) -> ::windows_core::HRESULT,
    pub IsCallerInRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrole: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pbinrole: *mut i16) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pitem: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Security: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsecurityproperty: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ContextInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontextinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ContextInfo: usize,
}
#[repr(transparent)]
pub struct ObjectControl(::windows_core::IUnknown);
impl ObjectControl {
    pub unsafe fn Activate(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Activate)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Deactivate(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Deactivate)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CanBePooled(&self, pbpoolable: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CanBePooled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbpoolable)).ok()
    }
}
impl ::core::convert::From<ObjectControl> for ::windows_core::IUnknown {
    fn from(value: ObjectControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ObjectControl> for ::windows_core::IUnknown {
    fn from(value: &ObjectControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ObjectControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ObjectControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ObjectControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ObjectControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ObjectControl {}
impl ::core::fmt::Debug for ObjectControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ObjectControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ObjectControl {
    type Vtable = ObjectControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7dc41850_0c31_11d0_8b79_00aa00b8a790);
}
#[repr(C)]
#[doc(hidden)]
pub struct ObjectControl_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CanBePooled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbpoolable: *mut i16) -> ::windows_core::HRESULT,
}
pub const PoolMgr: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecabafb5_7f19_11d2_978e_0000f8757e2a);
#[repr(C)]
pub struct RECYCLE_INFO {
    pub guidCombaseProcessIdentifier: ::windows_core::GUID,
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
impl ::core::fmt::Debug for RECYCLE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECYCLE_INFO").field("guidCombaseProcessIdentifier", &self.guidCombaseProcessIdentifier).field("ProcessStartTime", &self.ProcessStartTime).field("dwRecycleLifetimeLimit", &self.dwRecycleLifetimeLimit).field("dwRecycleMemoryLimit", &self.dwRecycleMemoryLimit).field("dwRecycleExpirationTimeout", &self.dwRecycleExpirationTimeout).finish()
    }
}
unsafe impl ::windows_core::Abi for RECYCLE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RECYCLE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RECYCLE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for RECYCLE_INFO {}
impl ::core::default::Default for RECYCLE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn RecycleSurrogate(lreasoncode: i32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RecycleSurrogate(lreasoncode: i32) -> ::windows_core::HRESULT;
        }
        RecycleSurrogate(::core::mem::transmute(lreasoncode)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ReleaseModes(pub i32);
pub const Standard: ReleaseModes = ReleaseModes(0i32);
pub const Process: ReleaseModes = ReleaseModes(1i32);
impl ::core::marker::Copy for ReleaseModes {}
impl ::core::clone::Clone for ReleaseModes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ReleaseModes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ReleaseModes {
    type Abi = Self;
}
impl ::core::fmt::Debug for ReleaseModes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReleaseModes").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn SafeRef<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(rid: *const ::windows_core::GUID, punk: Param1) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SafeRef(rid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(SafeRef(::core::mem::transmute(rid), punk.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SecurityCallContext: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecabb0a7_7f19_11d2_978e_0000f8757e2a);
pub const SecurityCallers: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecabb0a6_7f19_11d2_978e_0000f8757e2a);
pub const SecurityIdentity: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecabb0a5_7f19_11d2_978e_0000f8757e2a);
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct SecurityProperty(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl SecurityProperty {
    pub unsafe fn GetDirectCallerName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetDirectCallerName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetDirectCreatorName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetDirectCreatorName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetOriginalCallerName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetOriginalCallerName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetOriginalCreatorName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetOriginalCreatorName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<SecurityProperty> for ::windows_core::IUnknown {
    fn from(value: SecurityProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&SecurityProperty> for ::windows_core::IUnknown {
    fn from(value: &SecurityProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SecurityProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SecurityProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<SecurityProperty> for super::Com::IDispatch {
    fn from(value: SecurityProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&SecurityProperty> for super::Com::IDispatch {
    fn from(value: &SecurityProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for SecurityProperty {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a SecurityProperty {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for SecurityProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for SecurityProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for SecurityProperty {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for SecurityProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecurityProperty").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for SecurityProperty {
    type Vtable = SecurityProperty_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe74a7215_014d_11d1_a63c_00a0c911b4e0);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct SecurityProperty_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub GetDirectCallerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetDirectCreatorName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetOriginalCallerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetOriginalCreatorName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrusername: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
pub const ServicePool: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecabb0c9_7f19_11d2_978e_0000f8757e2a);
pub const ServicePoolConfig: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecabb0ca_7f19_11d2_978e_0000f8757e2a);
pub const SharedProperty: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a005c05_a5de_11cf_9e66_00aa00a3f464);
pub const SharedPropertyGroup: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a005c0b_a5de_11cf_9e66_00aa00a3f464);
pub const SharedPropertyGroupManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a005c11_a5de_11cf_9e66_00aa00a3f464);
pub const TRACKER_INIT_EVENT: &str = "Global\\COM+ Tracker Init Event";
pub const TRACKER_STARTSTOP_EVENT: &str = "Global\\COM+ Tracker Push Event";
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TRACKING_COLL_TYPE(pub i32);
pub const TRKCOLL_PROCESSES: TRACKING_COLL_TYPE = TRACKING_COLL_TYPE(0i32);
pub const TRKCOLL_APPLICATIONS: TRACKING_COLL_TYPE = TRACKING_COLL_TYPE(1i32);
pub const TRKCOLL_COMPONENTS: TRACKING_COLL_TYPE = TRACKING_COLL_TYPE(2i32);
impl ::core::marker::Copy for TRACKING_COLL_TYPE {}
impl ::core::clone::Clone for TRACKING_COLL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TRACKING_COLL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TRACKING_COLL_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TRACKING_COLL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRACKING_COLL_TYPE").field(&self.0).finish()
    }
}
pub const TrackerServer: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecabafb9_7f19_11d2_978e_0000f8757e2a);
pub const TransactionContext: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7999fc25_d3c6_11cf_acab_00a024a55aef);
pub const TransactionContextEx: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5cb66670_d3d4_11cf_acab_00a024a55aef);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TransactionVote(pub i32);
pub const TxCommit: TransactionVote = TransactionVote(0i32);
pub const TxAbort: TransactionVote = TransactionVote(1i32);
impl ::core::marker::Copy for TransactionVote {}
impl ::core::clone::Clone for TransactionVote {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TransactionVote {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TransactionVote {
    type Abi = Self;
}
impl ::core::fmt::Debug for TransactionVote {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TransactionVote").field(&self.0).finish()
    }
}
