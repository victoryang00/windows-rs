pub const AdSyncTask: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ae64751_b728_4d6b_97a0_b2da2e7d2a3b);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AdrClientDisplayFlags(pub i32);
pub const AdrClientDisplayFlags_AllowEmailRequests: AdrClientDisplayFlags = AdrClientDisplayFlags(1i32);
pub const AdrClientDisplayFlags_ShowDeviceTroubleshooting: AdrClientDisplayFlags = AdrClientDisplayFlags(2i32);
impl ::core::marker::Copy for AdrClientDisplayFlags {}
impl ::core::clone::Clone for AdrClientDisplayFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdrClientDisplayFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AdrClientDisplayFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdrClientDisplayFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdrClientDisplayFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AdrClientErrorType(pub i32);
pub const AdrClientErrorType_Unknown: AdrClientErrorType = AdrClientErrorType(0i32);
pub const AdrClientErrorType_AccessDenied: AdrClientErrorType = AdrClientErrorType(1i32);
pub const AdrClientErrorType_FileNotFound: AdrClientErrorType = AdrClientErrorType(2i32);
impl ::core::marker::Copy for AdrClientErrorType {}
impl ::core::clone::Clone for AdrClientErrorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdrClientErrorType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AdrClientErrorType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdrClientErrorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdrClientErrorType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AdrClientFlags(pub i32);
pub const AdrClientFlags_None: AdrClientFlags = AdrClientFlags(0i32);
pub const AdrClientFlags_FailForLocalPaths: AdrClientFlags = AdrClientFlags(1i32);
pub const AdrClientFlags_FailIfNotSupportedByServer: AdrClientFlags = AdrClientFlags(2i32);
pub const AdrClientFlags_FailIfNotDomainJoined: AdrClientFlags = AdrClientFlags(4i32);
impl ::core::marker::Copy for AdrClientFlags {}
impl ::core::clone::Clone for AdrClientFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdrClientFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AdrClientFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdrClientFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdrClientFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AdrEmailFlags(pub i32);
pub const AdrEmailFlags_PutDataOwnerOnToLine: AdrEmailFlags = AdrEmailFlags(1i32);
pub const AdrEmailFlags_PutAdminOnToLine: AdrEmailFlags = AdrEmailFlags(2i32);
pub const AdrEmailFlags_IncludeDeviceClaims: AdrEmailFlags = AdrEmailFlags(4i32);
pub const AdrEmailFlags_IncludeUserInfo: AdrEmailFlags = AdrEmailFlags(8i32);
pub const AdrEmailFlags_GenerateEventLog: AdrEmailFlags = AdrEmailFlags(16i32);
impl ::core::marker::Copy for AdrEmailFlags {}
impl ::core::clone::Clone for AdrEmailFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdrEmailFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AdrEmailFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdrEmailFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdrEmailFlags").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct DIFsrmClassificationEvents(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl DIFsrmClassificationEvents {}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<DIFsrmClassificationEvents> for ::windows_core::IUnknown {
    fn from(value: DIFsrmClassificationEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&DIFsrmClassificationEvents> for ::windows_core::IUnknown {
    fn from(value: &DIFsrmClassificationEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DIFsrmClassificationEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DIFsrmClassificationEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<DIFsrmClassificationEvents> for ::win32_system::Com::IDispatch {
    fn from(value: DIFsrmClassificationEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&DIFsrmClassificationEvents> for ::win32_system::Com::IDispatch {
    fn from(value: &DIFsrmClassificationEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for DIFsrmClassificationEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a DIFsrmClassificationEvents {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for DIFsrmClassificationEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for DIFsrmClassificationEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for DIFsrmClassificationEvents {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for DIFsrmClassificationEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIFsrmClassificationEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for DIFsrmClassificationEvents {
    type Vtable = DIFsrmClassificationEvents_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x26942db0_dabf_41d8_bbdd_b129a9f70424);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct DIFsrmClassificationEvents_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
}
pub const FSRM_DISPID_FEATURE_CLASSIFICATION: u32 = 83886080u32;
pub const FSRM_DISPID_FEATURE_FILESCREEN: u32 = 50331648u32;
pub const FSRM_DISPID_FEATURE_GENERAL: u32 = 16777216u32;
pub const FSRM_DISPID_FEATURE_MASK: u32 = 251658240u32;
pub const FSRM_DISPID_FEATURE_PIPELINE: u32 = 100663296u32;
pub const FSRM_DISPID_FEATURE_QUOTA: u32 = 33554432u32;
pub const FSRM_DISPID_FEATURE_REPORTS: u32 = 67108864u32;
pub const FSRM_DISPID_INTERFACE_A_MASK: u32 = 15728640u32;
pub const FSRM_DISPID_INTERFACE_B_MASK: u32 = 983040u32;
pub const FSRM_DISPID_INTERFACE_C_MASK: u32 = 61440u32;
pub const FSRM_DISPID_INTERFACE_D_MASK: u32 = 3840u32;
pub const FSRM_DISPID_IS_PROPERTY: u32 = 128u32;
pub const FSRM_DISPID_METHOD_NUM_MASK: u32 = 127u32;
pub const FSRM_E_ADR_MAX_EMAILS_SENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200130i32);
pub const FSRM_E_ADR_NOT_DOMAIN_JOINED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200110i32);
pub const FSRM_E_ADR_PATH_IS_LOCAL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200111i32);
pub const FSRM_E_ADR_SRV_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200112i32);
pub const FSRM_E_ALREADY_EXISTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200253i32);
pub const FSRM_E_AUTO_QUOTA: ::windows_core::HRESULT = ::windows_core::HRESULT(283419i32);
pub const FSRM_E_CACHE_INVALID: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200187i32);
pub const FSRM_E_CACHE_MODULE_ALREADY_EXISTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200186i32);
pub const FSRM_E_CANNOT_AGGREGATE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200201i32);
pub const FSRM_E_CANNOT_ALLOW_REPARSE_POINT_TAG: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200170i32);
pub const FSRM_E_CANNOT_CHANGE_PROPERTY_TYPE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200197i32);
pub const FSRM_E_CANNOT_CREATE_TEMP_COPY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200132i32);
pub const FSRM_E_CANNOT_DELETE_SYSTEM_PROPERTY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200135i32);
pub const FSRM_E_CANNOT_REMOVE_READONLY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200109i32);
pub const FSRM_E_CANNOT_RENAME_PROPERTY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200198i32);
pub const FSRM_E_CANNOT_STORE_PROPERTIES: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200171i32);
pub const FSRM_E_CANNOT_USE_DELETED_PROPERTY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200143i32);
pub const FSRM_E_CANNOT_USE_DEPRECATED_PROPERTY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200145i32);
pub const FSRM_E_CLASSIFICATION_ALREADY_RUNNING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200195i32);
pub const FSRM_E_CLASSIFICATION_CANCELED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200141i32);
pub const FSRM_E_CLASSIFICATION_NOT_RUNNING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200194i32);
pub const FSRM_E_CLASSIFICATION_PARTIAL_BATCH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200136i32);
pub const FSRM_E_CLASSIFICATION_SCAN_FAIL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200148i32);
pub const FSRM_E_CLASSIFICATION_TIMEOUT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200137i32);
pub const FSRM_E_CLUSTER_NOT_RUNNING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200210i32);
pub const FSRM_E_CSC_PATH_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200106i32);
pub const FSRM_E_DIFFERENT_CLUSTER_GROUP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200207i32);
pub const FSRM_E_DRIVER_NOT_READY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200237i32);
pub const FSRM_E_DUPLICATE_NAME: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200240i32);
pub const FSRM_E_EMAIL_NOT_SENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200228i32);
pub const FSRM_E_ENUM_PROPERTIES_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200173i32);
pub const FSRM_E_ERROR_NOT_ENABLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200133i32);
pub const FSRM_E_EXPIRATION_PATH_NOT_WRITEABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200105i32);
pub const FSRM_E_EXPIRATION_PATH_TOO_LONG: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200104i32);
pub const FSRM_E_EXPIRATION_VOLUME_NOT_NTFS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200103i32);
pub const FSRM_E_FAIL_BATCH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200247i32);
pub const FSRM_E_FILE_ENCRYPTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200156i32);
pub const FSRM_E_FILE_IN_USE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200134i32);
pub const FSRM_E_FILE_MANAGEMENT_ACTION_GET_EXITCODE_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200152i32);
pub const FSRM_E_FILE_MANAGEMENT_ACTION_TIMEOUT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200153i32);
pub const FSRM_E_FILE_MANAGEMENT_EXPIRATION_DIR_IN_SCOPE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200185i32);
pub const FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_EXISTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200184i32);
pub const FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_RUNNING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200193i32);
pub const FSRM_E_FILE_MANAGEMENT_JOB_CUSTOM: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200191i32);
pub const FSRM_E_FILE_MANAGEMENT_JOB_DEPRECATED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200102i32);
pub const FSRM_E_FILE_MANAGEMENT_JOB_EXPIRATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200192i32);
pub const FSRM_E_FILE_MANAGEMENT_JOB_INVALID_CONTINUOUS_CONFIG: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200108i32);
pub const FSRM_E_FILE_MANAGEMENT_JOB_MAX_FILE_CONDITIONS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200146i32);
pub const FSRM_E_FILE_MANAGEMENT_JOB_NOTIFICATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200190i32);
pub const FSRM_E_FILE_MANAGEMENT_JOB_NOT_LEGACY_ACCESSIBLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200147i32);
pub const FSRM_E_FILE_MANAGEMENT_JOB_RMS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200120i32);
pub const FSRM_E_FILE_OPEN_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200189i32);
pub const FSRM_E_FILE_SYSTEM_CORRUPT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200225i32);
pub const FSRM_E_INCOMPATIBLE_FORMAT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200157i32);
pub const FSRM_E_INPROC_MODULE_BLOCKED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200174i32);
pub const FSRM_E_INSECURE_PATH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200233i32);
pub const FSRM_E_INSUFFICIENT_DISK: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200236i32);
pub const FSRM_E_INVALID_AD_CLAIM: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200142i32);
pub const FSRM_E_INVALID_COMBINATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200241i32);
pub const FSRM_E_INVALID_DATASCREEN_DEFINITION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200220i32);
pub const FSRM_E_INVALID_EMAIL_ADDRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200226i32);
pub const FSRM_E_INVALID_FILEGROUP_DEFINITION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200223i32);
pub const FSRM_E_INVALID_FILENAME: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200214i32);
pub const FSRM_E_INVALID_FOLDER_PROPERTY_STORE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200140i32);
pub const FSRM_E_INVALID_IMPORT_VERSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200245i32);
pub const FSRM_E_INVALID_LIMIT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200249i32);
pub const FSRM_E_INVALID_NAME: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200248i32);
pub const FSRM_E_INVALID_PATH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200250i32);
pub const FSRM_E_INVALID_REPORT_DESC: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200215i32);
pub const FSRM_E_INVALID_REPORT_FORMAT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200216i32);
pub const FSRM_E_INVALID_SCHEDULER_ARGUMENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200254i32);
pub const FSRM_E_INVALID_SMTP_SERVER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200232i32);
pub const FSRM_E_INVALID_TEXT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200246i32);
pub const FSRM_E_INVALID_USER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200251i32);
pub const FSRM_E_LAST_ACCESS_UPDATE_DISABLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200176i32);
pub const FSRM_E_LEGACY_SCHEDULE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200107i32);
pub const FSRM_E_LOADING_DISABLED_MODULE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200202i32);
pub const FSRM_E_LONG_CMDLINE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200224i32);
pub const FSRM_E_MAX_PROPERTY_DEFINITIONS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200196i32);
pub const FSRM_E_MESSAGE_LIMIT_EXCEEDED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200200i32);
pub const FSRM_E_MODULE_INITIALIZATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200150i32);
pub const FSRM_E_MODULE_INVALID_PARAM: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200151i32);
pub const FSRM_E_MODULE_SESSION_INITIALIZATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200149i32);
pub const FSRM_E_MODULE_TIMEOUT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200101i32);
pub const FSRM_E_NOT_CLUSTER_VOLUME: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200208i32);
pub const FSRM_E_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200255i32);
pub const FSRM_E_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200239i32);
pub const FSRM_E_NO_EMAIL_ADDRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200131i32);
pub const FSRM_E_NO_PROPERTY_VALUE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200175i32);
pub const FSRM_E_OBJECT_IN_USE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200199i32);
pub const FSRM_E_OUT_OF_RANGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200243i32);
pub const FSRM_E_PARTIAL_CLASSIFICATION_PROPERTY_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200169i32);
pub const FSRM_E_PATH_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200252i32);
pub const FSRM_E_PATH_NOT_IN_NAMESPACE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200129i32);
pub const FSRM_E_PERSIST_PROPERTIES_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200155i32);
pub const FSRM_E_PERSIST_PROPERTIES_FAILED_ENCRYPTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200166i32);
pub const FSRM_E_PROPERTY_DELETED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200183i32);
pub const FSRM_E_PROPERTY_MUST_APPLY_TO_FILES: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200138i32);
pub const FSRM_E_PROPERTY_MUST_APPLY_TO_FOLDERS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200124i32);
pub const FSRM_E_PROPERTY_MUST_BE_GLOBAL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200122i32);
pub const FSRM_E_PROPERTY_MUST_BE_SECURE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200123i32);
pub const FSRM_E_REBUILDING_FODLER_TYPE_INDEX: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200139i32);
pub const FSRM_E_REPORT_GENERATION_ERR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200204i32);
pub const FSRM_E_REPORT_JOB_ALREADY_RUNNING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200205i32);
pub const FSRM_E_REPORT_TASK_TRIGGER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200203i32);
pub const FSRM_E_REPORT_TYPE_ALREADY_EXISTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200206i32);
pub const FSRM_E_REQD_PARAM_MISSING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200242i32);
pub const FSRM_E_RMS_NO_PROTECTORS_INSTALLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200126i32);
pub const FSRM_E_RMS_NO_PROTECTOR_INSTALLED_FOR_FILE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200125i32);
pub const FSRM_E_RMS_TEMPLATE_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200128i32);
pub const FSRM_E_SECURE_PROPERTIES_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200127i32);
pub const FSRM_E_SET_PROPERTY_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200172i32);
pub const FSRM_E_SHADOW_COPY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200212i32);
pub const FSRM_E_STORE_NOT_INSTALLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200209i32);
pub const FSRM_E_SYNC_TASK_HAD_ERRORS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200119i32);
pub const FSRM_E_SYNC_TASK_TIMEOUT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200144i32);
pub const FSRM_E_TEXTREADER_FILENAME_TOO_LONG: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200158i32);
pub const FSRM_E_TEXTREADER_IFILTER_CLSID_MALFORMED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200160i32);
pub const FSRM_E_TEXTREADER_IFILTER_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200167i32);
pub const FSRM_E_TEXTREADER_NOT_INITIALIZED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200168i32);
pub const FSRM_E_TEXTREADER_STREAM_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200159i32);
pub const FSRM_E_UNEXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200234i32);
pub const FSRM_E_UNSECURE_LINK_TO_HOSTED_MODULE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200188i32);
pub const FSRM_E_VOLUME_OFFLINE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200154i32);
pub const FSRM_E_VOLUME_UNSUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200235i32);
pub const FSRM_E_WMI_FAILURE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200121i32);
pub const FSRM_E_XML_CORRUPTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147200211i32);
pub const FSRM_S_CLASSIFICATION_SCAN_FAILURES: ::windows_core::HRESULT = ::windows_core::HRESULT(283398i32);
pub const FSRM_S_PARTIAL_BATCH: ::windows_core::HRESULT = ::windows_core::HRESULT(283396i32);
pub const FSRM_S_PARTIAL_CLASSIFICATION: ::windows_core::HRESULT = ::windows_core::HRESULT(283397i32);
pub const FsrmAccessDeniedRemediationClient: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x100b4fc8_74c1_470f_b1b7_dd7b6bae79bd);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmAccountType(pub i32);
pub const FsrmAccountType_Unknown: FsrmAccountType = FsrmAccountType(0i32);
pub const FsrmAccountType_NetworkService: FsrmAccountType = FsrmAccountType(1i32);
pub const FsrmAccountType_LocalService: FsrmAccountType = FsrmAccountType(2i32);
pub const FsrmAccountType_LocalSystem: FsrmAccountType = FsrmAccountType(3i32);
pub const FsrmAccountType_InProc: FsrmAccountType = FsrmAccountType(4i32);
pub const FsrmAccountType_External: FsrmAccountType = FsrmAccountType(5i32);
pub const FsrmAccountType_Automatic: FsrmAccountType = FsrmAccountType(500i32);
impl ::core::marker::Copy for FsrmAccountType {}
impl ::core::clone::Clone for FsrmAccountType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmAccountType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmAccountType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmAccountType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmAccountType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmActionType(pub i32);
pub const FsrmActionType_Unknown: FsrmActionType = FsrmActionType(0i32);
pub const FsrmActionType_EventLog: FsrmActionType = FsrmActionType(1i32);
pub const FsrmActionType_Email: FsrmActionType = FsrmActionType(2i32);
pub const FsrmActionType_Command: FsrmActionType = FsrmActionType(3i32);
pub const FsrmActionType_Report: FsrmActionType = FsrmActionType(4i32);
impl ::core::marker::Copy for FsrmActionType {}
impl ::core::clone::Clone for FsrmActionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmActionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmActionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmActionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmActionType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmClassificationLoggingFlags(pub i32);
pub const FsrmClassificationLoggingFlags_None: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(0i32);
pub const FsrmClassificationLoggingFlags_ClassificationsInLogFile: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(1i32);
pub const FsrmClassificationLoggingFlags_ErrorsInLogFile: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(2i32);
pub const FsrmClassificationLoggingFlags_ClassificationsInSystemLog: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(4i32);
pub const FsrmClassificationLoggingFlags_ErrorsInSystemLog: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(8i32);
impl ::core::marker::Copy for FsrmClassificationLoggingFlags {}
impl ::core::clone::Clone for FsrmClassificationLoggingFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmClassificationLoggingFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmClassificationLoggingFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmClassificationLoggingFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmClassificationLoggingFlags").field(&self.0).finish()
    }
}
pub const FsrmClassificationManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb15c0e47_c391_45b9_95c8_eb596c853f3a);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmCollectionState(pub i32);
pub const FsrmCollectionState_Fetching: FsrmCollectionState = FsrmCollectionState(1i32);
pub const FsrmCollectionState_Committing: FsrmCollectionState = FsrmCollectionState(2i32);
pub const FsrmCollectionState_Complete: FsrmCollectionState = FsrmCollectionState(3i32);
pub const FsrmCollectionState_Cancelled: FsrmCollectionState = FsrmCollectionState(4i32);
impl ::core::marker::Copy for FsrmCollectionState {}
impl ::core::clone::Clone for FsrmCollectionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmCollectionState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmCollectionState {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmCollectionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmCollectionState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmCommitOptions(pub i32);
pub const FsrmCommitOptions_None: FsrmCommitOptions = FsrmCommitOptions(0i32);
pub const FsrmCommitOptions_Asynchronous: FsrmCommitOptions = FsrmCommitOptions(1i32);
impl ::core::marker::Copy for FsrmCommitOptions {}
impl ::core::clone::Clone for FsrmCommitOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmCommitOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmCommitOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmCommitOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmCommitOptions").field(&self.0).finish()
    }
}
pub const FsrmDaysNotSpecified: i32 = -1i32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmEnumOptions(pub i32);
pub const FsrmEnumOptions_None: FsrmEnumOptions = FsrmEnumOptions(0i32);
pub const FsrmEnumOptions_Asynchronous: FsrmEnumOptions = FsrmEnumOptions(1i32);
pub const FsrmEnumOptions_CheckRecycleBin: FsrmEnumOptions = FsrmEnumOptions(2i32);
pub const FsrmEnumOptions_IncludeClusterNodes: FsrmEnumOptions = FsrmEnumOptions(4i32);
pub const FsrmEnumOptions_IncludeDeprecatedObjects: FsrmEnumOptions = FsrmEnumOptions(8i32);
impl ::core::marker::Copy for FsrmEnumOptions {}
impl ::core::clone::Clone for FsrmEnumOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmEnumOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmEnumOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmEnumOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmEnumOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmEventType(pub i32);
pub const FsrmEventType_Unknown: FsrmEventType = FsrmEventType(0i32);
pub const FsrmEventType_Information: FsrmEventType = FsrmEventType(1i32);
pub const FsrmEventType_Warning: FsrmEventType = FsrmEventType(2i32);
pub const FsrmEventType_Error: FsrmEventType = FsrmEventType(3i32);
impl ::core::marker::Copy for FsrmEventType {}
impl ::core::clone::Clone for FsrmEventType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmEventType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmEventType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmEventType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmEventType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmExecutionOption(pub i32);
pub const FsrmExecutionOption_Unknown: FsrmExecutionOption = FsrmExecutionOption(0i32);
pub const FsrmExecutionOption_EvaluateUnset: FsrmExecutionOption = FsrmExecutionOption(1i32);
pub const FsrmExecutionOption_ReEvaluate_ConsiderExistingValue: FsrmExecutionOption = FsrmExecutionOption(2i32);
pub const FsrmExecutionOption_ReEvaluate_IgnoreExistingValue: FsrmExecutionOption = FsrmExecutionOption(3i32);
impl ::core::marker::Copy for FsrmExecutionOption {}
impl ::core::clone::Clone for FsrmExecutionOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmExecutionOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmExecutionOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmExecutionOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmExecutionOption").field(&self.0).finish()
    }
}
pub const FsrmExportImport: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1482dc37_fae9_4787_9025_8ce4e024ab56);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmFileConditionType(pub i32);
pub const FsrmFileConditionType_Unknown: FsrmFileConditionType = FsrmFileConditionType(0i32);
pub const FsrmFileConditionType_Property: FsrmFileConditionType = FsrmFileConditionType(1i32);
impl ::core::marker::Copy for FsrmFileConditionType {}
impl ::core::clone::Clone for FsrmFileConditionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmFileConditionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmFileConditionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmFileConditionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmFileConditionType").field(&self.0).finish()
    }
}
pub const FsrmFileGroupManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8f1363f6_656f_4496_9226_13aecbd7718f);
pub const FsrmFileManagementJobManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb18f9b2_4c3a_4321_b203_205120cff614);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmFileManagementLoggingFlags(pub i32);
pub const FsrmFileManagementLoggingFlags_None: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(0i32);
pub const FsrmFileManagementLoggingFlags_Error: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(1i32);
pub const FsrmFileManagementLoggingFlags_Information: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(2i32);
pub const FsrmFileManagementLoggingFlags_Audit: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(4i32);
impl ::core::marker::Copy for FsrmFileManagementLoggingFlags {}
impl ::core::clone::Clone for FsrmFileManagementLoggingFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmFileManagementLoggingFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmFileManagementLoggingFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmFileManagementLoggingFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmFileManagementLoggingFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmFileManagementType(pub i32);
pub const FsrmFileManagementType_Unknown: FsrmFileManagementType = FsrmFileManagementType(0i32);
pub const FsrmFileManagementType_Expiration: FsrmFileManagementType = FsrmFileManagementType(1i32);
pub const FsrmFileManagementType_Custom: FsrmFileManagementType = FsrmFileManagementType(2i32);
pub const FsrmFileManagementType_Rms: FsrmFileManagementType = FsrmFileManagementType(3i32);
impl ::core::marker::Copy for FsrmFileManagementType {}
impl ::core::clone::Clone for FsrmFileManagementType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmFileManagementType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmFileManagementType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmFileManagementType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmFileManagementType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmFileScreenFlags(pub i32);
pub const FsrmFileScreenFlags_Enforce: FsrmFileScreenFlags = FsrmFileScreenFlags(1i32);
impl ::core::marker::Copy for FsrmFileScreenFlags {}
impl ::core::clone::Clone for FsrmFileScreenFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmFileScreenFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmFileScreenFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmFileScreenFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmFileScreenFlags").field(&self.0).finish()
    }
}
pub const FsrmFileScreenManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x95941183_db53_4c5f_b37b_7d0921cf9dc7);
pub const FsrmFileScreenTemplateManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x243111df_e474_46aa_a054_eaa33edc292a);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmFileStreamingInterfaceType(pub i32);
pub const FsrmFileStreamingInterfaceType_Unknown: FsrmFileStreamingInterfaceType = FsrmFileStreamingInterfaceType(0i32);
pub const FsrmFileStreamingInterfaceType_ILockBytes: FsrmFileStreamingInterfaceType = FsrmFileStreamingInterfaceType(1i32);
pub const FsrmFileStreamingInterfaceType_IStream: FsrmFileStreamingInterfaceType = FsrmFileStreamingInterfaceType(2i32);
impl ::core::marker::Copy for FsrmFileStreamingInterfaceType {}
impl ::core::clone::Clone for FsrmFileStreamingInterfaceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmFileStreamingInterfaceType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmFileStreamingInterfaceType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmFileStreamingInterfaceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmFileStreamingInterfaceType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmFileStreamingMode(pub i32);
pub const FsrmFileStreamingMode_Unknown: FsrmFileStreamingMode = FsrmFileStreamingMode(0i32);
pub const FsrmFileStreamingMode_Read: FsrmFileStreamingMode = FsrmFileStreamingMode(1i32);
pub const FsrmFileStreamingMode_Write: FsrmFileStreamingMode = FsrmFileStreamingMode(2i32);
impl ::core::marker::Copy for FsrmFileStreamingMode {}
impl ::core::clone::Clone for FsrmFileStreamingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmFileStreamingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmFileStreamingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmFileStreamingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmFileStreamingMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmFileSystemPropertyId(pub i32);
pub const FsrmFileSystemPropertyId_Undefined: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(0i32);
pub const FsrmFileSystemPropertyId_FileName: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(1i32);
pub const FsrmFileSystemPropertyId_DateCreated: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(2i32);
pub const FsrmFileSystemPropertyId_DateLastAccessed: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(3i32);
pub const FsrmFileSystemPropertyId_DateLastModified: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(4i32);
pub const FsrmFileSystemPropertyId_DateNow: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(5i32);
impl ::core::marker::Copy for FsrmFileSystemPropertyId {}
impl ::core::clone::Clone for FsrmFileSystemPropertyId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmFileSystemPropertyId {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmFileSystemPropertyId {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmFileSystemPropertyId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmFileSystemPropertyId").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmGetFilePropertyOptions(pub i32);
pub const FsrmGetFilePropertyOptions_None: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(0i32);
pub const FsrmGetFilePropertyOptions_NoRuleEvaluation: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(1i32);
pub const FsrmGetFilePropertyOptions_Persistent: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(2i32);
pub const FsrmGetFilePropertyOptions_FailOnPersistErrors: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(4i32);
pub const FsrmGetFilePropertyOptions_SkipOrphaned: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(8i32);
impl ::core::marker::Copy for FsrmGetFilePropertyOptions {}
impl ::core::clone::Clone for FsrmGetFilePropertyOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmGetFilePropertyOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmGetFilePropertyOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmGetFilePropertyOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmGetFilePropertyOptions").field(&self.0).finish()
    }
}
pub const FsrmMaxExcludeFolders: u32 = 32u32;
pub const FsrmMaxNumberPropertyDefinitions: u32 = 100u32;
pub const FsrmMaxNumberThresholds: u32 = 16u32;
pub const FsrmMaxThresholdValue: u32 = 250u32;
pub const FsrmMinQuotaLimit: u32 = 1024u32;
pub const FsrmMinThresholdValue: u32 = 1u32;
pub const FsrmPathMapper: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf3be42bd_8ac2_409e_bbd8_faf9b6b41feb);
pub const FsrmPipelineModuleConnector: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc7643375_1eb5_44de_a062_623547d933bc);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmPipelineModuleType(pub i32);
pub const FsrmPipelineModuleType_Unknown: FsrmPipelineModuleType = FsrmPipelineModuleType(0i32);
pub const FsrmPipelineModuleType_Storage: FsrmPipelineModuleType = FsrmPipelineModuleType(1i32);
pub const FsrmPipelineModuleType_Classifier: FsrmPipelineModuleType = FsrmPipelineModuleType(2i32);
impl ::core::marker::Copy for FsrmPipelineModuleType {}
impl ::core::clone::Clone for FsrmPipelineModuleType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPipelineModuleType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmPipelineModuleType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPipelineModuleType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPipelineModuleType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmPropertyBagField(pub i32);
pub const FsrmPropertyBagField_AccessVolume: FsrmPropertyBagField = FsrmPropertyBagField(0i32);
pub const FsrmPropertyBagField_VolumeGuidName: FsrmPropertyBagField = FsrmPropertyBagField(1i32);
impl ::core::marker::Copy for FsrmPropertyBagField {}
impl ::core::clone::Clone for FsrmPropertyBagField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPropertyBagField {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmPropertyBagField {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPropertyBagField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPropertyBagField").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmPropertyBagFlags(pub i32);
pub const FsrmPropertyBagFlags_UpdatedByClassifier: FsrmPropertyBagFlags = FsrmPropertyBagFlags(1i32);
pub const FsrmPropertyBagFlags_FailedLoadingProperties: FsrmPropertyBagFlags = FsrmPropertyBagFlags(2i32);
pub const FsrmPropertyBagFlags_FailedSavingProperties: FsrmPropertyBagFlags = FsrmPropertyBagFlags(4i32);
pub const FsrmPropertyBagFlags_FailedClassifyingProperties: FsrmPropertyBagFlags = FsrmPropertyBagFlags(8i32);
impl ::core::marker::Copy for FsrmPropertyBagFlags {}
impl ::core::clone::Clone for FsrmPropertyBagFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPropertyBagFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmPropertyBagFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPropertyBagFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPropertyBagFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmPropertyConditionType(pub i32);
pub const FsrmPropertyConditionType_Unknown: FsrmPropertyConditionType = FsrmPropertyConditionType(0i32);
pub const FsrmPropertyConditionType_Equal: FsrmPropertyConditionType = FsrmPropertyConditionType(1i32);
pub const FsrmPropertyConditionType_NotEqual: FsrmPropertyConditionType = FsrmPropertyConditionType(2i32);
pub const FsrmPropertyConditionType_GreaterThan: FsrmPropertyConditionType = FsrmPropertyConditionType(3i32);
pub const FsrmPropertyConditionType_LessThan: FsrmPropertyConditionType = FsrmPropertyConditionType(4i32);
pub const FsrmPropertyConditionType_Contain: FsrmPropertyConditionType = FsrmPropertyConditionType(5i32);
pub const FsrmPropertyConditionType_Exist: FsrmPropertyConditionType = FsrmPropertyConditionType(6i32);
pub const FsrmPropertyConditionType_NotExist: FsrmPropertyConditionType = FsrmPropertyConditionType(7i32);
pub const FsrmPropertyConditionType_StartWith: FsrmPropertyConditionType = FsrmPropertyConditionType(8i32);
pub const FsrmPropertyConditionType_EndWith: FsrmPropertyConditionType = FsrmPropertyConditionType(9i32);
pub const FsrmPropertyConditionType_ContainedIn: FsrmPropertyConditionType = FsrmPropertyConditionType(10i32);
pub const FsrmPropertyConditionType_PrefixOf: FsrmPropertyConditionType = FsrmPropertyConditionType(11i32);
pub const FsrmPropertyConditionType_SuffixOf: FsrmPropertyConditionType = FsrmPropertyConditionType(12i32);
pub const FsrmPropertyConditionType_MatchesPattern: FsrmPropertyConditionType = FsrmPropertyConditionType(13i32);
impl ::core::marker::Copy for FsrmPropertyConditionType {}
impl ::core::clone::Clone for FsrmPropertyConditionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPropertyConditionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmPropertyConditionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPropertyConditionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPropertyConditionType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmPropertyDefinitionAppliesTo(pub i32);
pub const FsrmPropertyDefinitionAppliesTo_Files: FsrmPropertyDefinitionAppliesTo = FsrmPropertyDefinitionAppliesTo(1i32);
pub const FsrmPropertyDefinitionAppliesTo_Folders: FsrmPropertyDefinitionAppliesTo = FsrmPropertyDefinitionAppliesTo(2i32);
impl ::core::marker::Copy for FsrmPropertyDefinitionAppliesTo {}
impl ::core::clone::Clone for FsrmPropertyDefinitionAppliesTo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPropertyDefinitionAppliesTo {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmPropertyDefinitionAppliesTo {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPropertyDefinitionAppliesTo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPropertyDefinitionAppliesTo").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmPropertyDefinitionFlags(pub i32);
pub const FsrmPropertyDefinitionFlags_Global: FsrmPropertyDefinitionFlags = FsrmPropertyDefinitionFlags(1i32);
pub const FsrmPropertyDefinitionFlags_Deprecated: FsrmPropertyDefinitionFlags = FsrmPropertyDefinitionFlags(2i32);
pub const FsrmPropertyDefinitionFlags_Secure: FsrmPropertyDefinitionFlags = FsrmPropertyDefinitionFlags(4i32);
impl ::core::marker::Copy for FsrmPropertyDefinitionFlags {}
impl ::core::clone::Clone for FsrmPropertyDefinitionFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPropertyDefinitionFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmPropertyDefinitionFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPropertyDefinitionFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPropertyDefinitionFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmPropertyDefinitionType(pub i32);
pub const FsrmPropertyDefinitionType_Unknown: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(0i32);
pub const FsrmPropertyDefinitionType_OrderedList: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(1i32);
pub const FsrmPropertyDefinitionType_MultiChoiceList: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(2i32);
pub const FsrmPropertyDefinitionType_SingleChoiceList: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(3i32);
pub const FsrmPropertyDefinitionType_String: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(4i32);
pub const FsrmPropertyDefinitionType_MultiString: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(5i32);
pub const FsrmPropertyDefinitionType_Int: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(6i32);
pub const FsrmPropertyDefinitionType_Bool: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(7i32);
pub const FsrmPropertyDefinitionType_Date: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(8i32);
impl ::core::marker::Copy for FsrmPropertyDefinitionType {}
impl ::core::clone::Clone for FsrmPropertyDefinitionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPropertyDefinitionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmPropertyDefinitionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPropertyDefinitionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPropertyDefinitionType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmPropertyFlags(pub i32);
pub const FsrmPropertyFlags_None: FsrmPropertyFlags = FsrmPropertyFlags(0i32);
pub const FsrmPropertyFlags_Orphaned: FsrmPropertyFlags = FsrmPropertyFlags(1i32);
pub const FsrmPropertyFlags_RetrievedFromCache: FsrmPropertyFlags = FsrmPropertyFlags(2i32);
pub const FsrmPropertyFlags_RetrievedFromStorage: FsrmPropertyFlags = FsrmPropertyFlags(4i32);
pub const FsrmPropertyFlags_SetByClassifier: FsrmPropertyFlags = FsrmPropertyFlags(8i32);
pub const FsrmPropertyFlags_Deleted: FsrmPropertyFlags = FsrmPropertyFlags(16i32);
pub const FsrmPropertyFlags_Reclassified: FsrmPropertyFlags = FsrmPropertyFlags(32i32);
pub const FsrmPropertyFlags_AggregationFailed: FsrmPropertyFlags = FsrmPropertyFlags(64i32);
pub const FsrmPropertyFlags_Existing: FsrmPropertyFlags = FsrmPropertyFlags(128i32);
pub const FsrmPropertyFlags_FailedLoadingProperties: FsrmPropertyFlags = FsrmPropertyFlags(256i32);
pub const FsrmPropertyFlags_FailedClassifyingProperties: FsrmPropertyFlags = FsrmPropertyFlags(512i32);
pub const FsrmPropertyFlags_FailedSavingProperties: FsrmPropertyFlags = FsrmPropertyFlags(1024i32);
pub const FsrmPropertyFlags_Secure: FsrmPropertyFlags = FsrmPropertyFlags(2048i32);
pub const FsrmPropertyFlags_PolicyDerived: FsrmPropertyFlags = FsrmPropertyFlags(4096i32);
pub const FsrmPropertyFlags_Inherited: FsrmPropertyFlags = FsrmPropertyFlags(8192i32);
pub const FsrmPropertyFlags_Manual: FsrmPropertyFlags = FsrmPropertyFlags(16384i32);
pub const FsrmPropertyFlags_ExplicitValueDeleted: FsrmPropertyFlags = FsrmPropertyFlags(32768i32);
pub const FsrmPropertyFlags_PropertyDeletedFromClear: FsrmPropertyFlags = FsrmPropertyFlags(65536i32);
pub const FsrmPropertyFlags_PropertySourceMask: FsrmPropertyFlags = FsrmPropertyFlags(14i32);
pub const FsrmPropertyFlags_PersistentMask: FsrmPropertyFlags = FsrmPropertyFlags(20480i32);
impl ::core::marker::Copy for FsrmPropertyFlags {}
impl ::core::clone::Clone for FsrmPropertyFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPropertyFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmPropertyFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPropertyFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPropertyFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmPropertyValueType(pub i32);
pub const FsrmPropertyValueType_Undefined: FsrmPropertyValueType = FsrmPropertyValueType(0i32);
pub const FsrmPropertyValueType_Literal: FsrmPropertyValueType = FsrmPropertyValueType(1i32);
pub const FsrmPropertyValueType_DateOffset: FsrmPropertyValueType = FsrmPropertyValueType(2i32);
impl ::core::marker::Copy for FsrmPropertyValueType {}
impl ::core::clone::Clone for FsrmPropertyValueType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPropertyValueType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmPropertyValueType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPropertyValueType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPropertyValueType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmQuotaFlags(pub i32);
pub const FsrmQuotaFlags_Enforce: FsrmQuotaFlags = FsrmQuotaFlags(256i32);
pub const FsrmQuotaFlags_Disable: FsrmQuotaFlags = FsrmQuotaFlags(512i32);
pub const FsrmQuotaFlags_StatusIncomplete: FsrmQuotaFlags = FsrmQuotaFlags(65536i32);
pub const FsrmQuotaFlags_StatusRebuilding: FsrmQuotaFlags = FsrmQuotaFlags(131072i32);
impl ::core::marker::Copy for FsrmQuotaFlags {}
impl ::core::clone::Clone for FsrmQuotaFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmQuotaFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmQuotaFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmQuotaFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmQuotaFlags").field(&self.0).finish()
    }
}
pub const FsrmQuotaManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x90dcab7f_347c_4bfc_b543_540326305fbe);
pub const FsrmQuotaTemplateManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x97d3d443_251c_4337_81e7_b32e8f4ee65e);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmReportFilter(pub i32);
pub const FsrmReportFilter_MinSize: FsrmReportFilter = FsrmReportFilter(1i32);
pub const FsrmReportFilter_MinAgeDays: FsrmReportFilter = FsrmReportFilter(2i32);
pub const FsrmReportFilter_MaxAgeDays: FsrmReportFilter = FsrmReportFilter(3i32);
pub const FsrmReportFilter_MinQuotaUsage: FsrmReportFilter = FsrmReportFilter(4i32);
pub const FsrmReportFilter_FileGroups: FsrmReportFilter = FsrmReportFilter(5i32);
pub const FsrmReportFilter_Owners: FsrmReportFilter = FsrmReportFilter(6i32);
pub const FsrmReportFilter_NamePattern: FsrmReportFilter = FsrmReportFilter(7i32);
pub const FsrmReportFilter_Property: FsrmReportFilter = FsrmReportFilter(8i32);
impl ::core::marker::Copy for FsrmReportFilter {}
impl ::core::clone::Clone for FsrmReportFilter {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmReportFilter {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmReportFilter {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmReportFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmReportFilter").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmReportFormat(pub i32);
pub const FsrmReportFormat_Unknown: FsrmReportFormat = FsrmReportFormat(0i32);
pub const FsrmReportFormat_DHtml: FsrmReportFormat = FsrmReportFormat(1i32);
pub const FsrmReportFormat_Html: FsrmReportFormat = FsrmReportFormat(2i32);
pub const FsrmReportFormat_Txt: FsrmReportFormat = FsrmReportFormat(3i32);
pub const FsrmReportFormat_Csv: FsrmReportFormat = FsrmReportFormat(4i32);
pub const FsrmReportFormat_Xml: FsrmReportFormat = FsrmReportFormat(5i32);
impl ::core::marker::Copy for FsrmReportFormat {}
impl ::core::clone::Clone for FsrmReportFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmReportFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmReportFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmReportFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmReportFormat").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmReportGenerationContext(pub i32);
pub const FsrmReportGenerationContext_Undefined: FsrmReportGenerationContext = FsrmReportGenerationContext(1i32);
pub const FsrmReportGenerationContext_ScheduledReport: FsrmReportGenerationContext = FsrmReportGenerationContext(2i32);
pub const FsrmReportGenerationContext_InteractiveReport: FsrmReportGenerationContext = FsrmReportGenerationContext(3i32);
pub const FsrmReportGenerationContext_IncidentReport: FsrmReportGenerationContext = FsrmReportGenerationContext(4i32);
impl ::core::marker::Copy for FsrmReportGenerationContext {}
impl ::core::clone::Clone for FsrmReportGenerationContext {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmReportGenerationContext {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmReportGenerationContext {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmReportGenerationContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmReportGenerationContext").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmReportLimit(pub i32);
pub const FsrmReportLimit_MaxFiles: FsrmReportLimit = FsrmReportLimit(1i32);
pub const FsrmReportLimit_MaxFileGroups: FsrmReportLimit = FsrmReportLimit(2i32);
pub const FsrmReportLimit_MaxOwners: FsrmReportLimit = FsrmReportLimit(3i32);
pub const FsrmReportLimit_MaxFilesPerFileGroup: FsrmReportLimit = FsrmReportLimit(4i32);
pub const FsrmReportLimit_MaxFilesPerOwner: FsrmReportLimit = FsrmReportLimit(5i32);
pub const FsrmReportLimit_MaxFilesPerDuplGroup: FsrmReportLimit = FsrmReportLimit(6i32);
pub const FsrmReportLimit_MaxDuplicateGroups: FsrmReportLimit = FsrmReportLimit(7i32);
pub const FsrmReportLimit_MaxQuotas: FsrmReportLimit = FsrmReportLimit(8i32);
pub const FsrmReportLimit_MaxFileScreenEvents: FsrmReportLimit = FsrmReportLimit(9i32);
pub const FsrmReportLimit_MaxPropertyValues: FsrmReportLimit = FsrmReportLimit(10i32);
pub const FsrmReportLimit_MaxFilesPerPropertyValue: FsrmReportLimit = FsrmReportLimit(11i32);
pub const FsrmReportLimit_MaxFolders: FsrmReportLimit = FsrmReportLimit(12i32);
impl ::core::marker::Copy for FsrmReportLimit {}
impl ::core::clone::Clone for FsrmReportLimit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmReportLimit {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmReportLimit {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmReportLimit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmReportLimit").field(&self.0).finish()
    }
}
pub const FsrmReportManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0058ef37_aa66_4c48_bd5b_2fce432ab0c8);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmReportRunningStatus(pub i32);
pub const FsrmReportRunningStatus_Unknown: FsrmReportRunningStatus = FsrmReportRunningStatus(0i32);
pub const FsrmReportRunningStatus_NotRunning: FsrmReportRunningStatus = FsrmReportRunningStatus(1i32);
pub const FsrmReportRunningStatus_Queued: FsrmReportRunningStatus = FsrmReportRunningStatus(2i32);
pub const FsrmReportRunningStatus_Running: FsrmReportRunningStatus = FsrmReportRunningStatus(3i32);
impl ::core::marker::Copy for FsrmReportRunningStatus {}
impl ::core::clone::Clone for FsrmReportRunningStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmReportRunningStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmReportRunningStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmReportRunningStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmReportRunningStatus").field(&self.0).finish()
    }
}
pub const FsrmReportScheduler: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea25f1b8_1b8d_4290_8ee8_e17c12c2fe20);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmReportType(pub i32);
pub const FsrmReportType_Unknown: FsrmReportType = FsrmReportType(0i32);
pub const FsrmReportType_LargeFiles: FsrmReportType = FsrmReportType(1i32);
pub const FsrmReportType_FilesByType: FsrmReportType = FsrmReportType(2i32);
pub const FsrmReportType_LeastRecentlyAccessed: FsrmReportType = FsrmReportType(3i32);
pub const FsrmReportType_MostRecentlyAccessed: FsrmReportType = FsrmReportType(4i32);
pub const FsrmReportType_QuotaUsage: FsrmReportType = FsrmReportType(5i32);
pub const FsrmReportType_FilesByOwner: FsrmReportType = FsrmReportType(6i32);
pub const FsrmReportType_ExportReport: FsrmReportType = FsrmReportType(7i32);
pub const FsrmReportType_DuplicateFiles: FsrmReportType = FsrmReportType(8i32);
pub const FsrmReportType_FileScreenAudit: FsrmReportType = FsrmReportType(9i32);
pub const FsrmReportType_FilesByProperty: FsrmReportType = FsrmReportType(10i32);
pub const FsrmReportType_AutomaticClassification: FsrmReportType = FsrmReportType(11i32);
pub const FsrmReportType_Expiration: FsrmReportType = FsrmReportType(12i32);
pub const FsrmReportType_FoldersByProperty: FsrmReportType = FsrmReportType(13i32);
impl ::core::marker::Copy for FsrmReportType {}
impl ::core::clone::Clone for FsrmReportType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmReportType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmReportType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmReportType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmReportType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmRuleFlags(pub i32);
pub const FsrmRuleFlags_Disabled: FsrmRuleFlags = FsrmRuleFlags(256i32);
pub const FsrmRuleFlags_ClearAutomaticallyClassifiedProperty: FsrmRuleFlags = FsrmRuleFlags(1024i32);
pub const FsrmRuleFlags_ClearManuallyClassifiedProperty: FsrmRuleFlags = FsrmRuleFlags(2048i32);
pub const FsrmRuleFlags_Invalid: FsrmRuleFlags = FsrmRuleFlags(4096i32);
impl ::core::marker::Copy for FsrmRuleFlags {}
impl ::core::clone::Clone for FsrmRuleFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmRuleFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmRuleFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmRuleFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmRuleFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmRuleType(pub i32);
pub const FsrmRuleType_Unknown: FsrmRuleType = FsrmRuleType(0i32);
pub const FsrmRuleType_Classification: FsrmRuleType = FsrmRuleType(1i32);
pub const FsrmRuleType_Generic: FsrmRuleType = FsrmRuleType(2i32);
impl ::core::marker::Copy for FsrmRuleType {}
impl ::core::clone::Clone for FsrmRuleType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmRuleType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmRuleType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmRuleType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmRuleType").field(&self.0).finish()
    }
}
pub const FsrmSetting: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf556d708_6d4d_4594_9c61_7dbb0dae2a46);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmStorageModuleCaps(pub i32);
pub const FsrmStorageModuleCaps_Unknown: FsrmStorageModuleCaps = FsrmStorageModuleCaps(0i32);
pub const FsrmStorageModuleCaps_CanGet: FsrmStorageModuleCaps = FsrmStorageModuleCaps(1i32);
pub const FsrmStorageModuleCaps_CanSet: FsrmStorageModuleCaps = FsrmStorageModuleCaps(2i32);
pub const FsrmStorageModuleCaps_CanHandleDirectories: FsrmStorageModuleCaps = FsrmStorageModuleCaps(4i32);
pub const FsrmStorageModuleCaps_CanHandleFiles: FsrmStorageModuleCaps = FsrmStorageModuleCaps(8i32);
impl ::core::marker::Copy for FsrmStorageModuleCaps {}
impl ::core::clone::Clone for FsrmStorageModuleCaps {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmStorageModuleCaps {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmStorageModuleCaps {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmStorageModuleCaps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmStorageModuleCaps").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmStorageModuleType(pub i32);
pub const FsrmStorageModuleType_Unknown: FsrmStorageModuleType = FsrmStorageModuleType(0i32);
pub const FsrmStorageModuleType_Cache: FsrmStorageModuleType = FsrmStorageModuleType(1i32);
pub const FsrmStorageModuleType_InFile: FsrmStorageModuleType = FsrmStorageModuleType(2i32);
pub const FsrmStorageModuleType_Database: FsrmStorageModuleType = FsrmStorageModuleType(3i32);
pub const FsrmStorageModuleType_System: FsrmStorageModuleType = FsrmStorageModuleType(100i32);
impl ::core::marker::Copy for FsrmStorageModuleType {}
impl ::core::clone::Clone for FsrmStorageModuleType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmStorageModuleType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmStorageModuleType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmStorageModuleType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmStorageModuleType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmTemplateApplyOptions(pub i32);
pub const FsrmTemplateApplyOptions_ApplyToDerivedMatching: FsrmTemplateApplyOptions = FsrmTemplateApplyOptions(1i32);
pub const FsrmTemplateApplyOptions_ApplyToDerivedAll: FsrmTemplateApplyOptions = FsrmTemplateApplyOptions(2i32);
impl ::core::marker::Copy for FsrmTemplateApplyOptions {}
impl ::core::clone::Clone for FsrmTemplateApplyOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmTemplateApplyOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FsrmTemplateApplyOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmTemplateApplyOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmTemplateApplyOptions").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmAccessDeniedRemediationClient(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmAccessDeniedRemediationClient {
    pub unsafe fn Show<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, parentwnd: usize, accesspath: Param1, errortype: AdrClientErrorType, flags: i32, windowtitle: Param4, windowmessage: Param5) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Show)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(parentwnd), accesspath.into_param().abi(), ::core::mem::transmute(errortype), ::core::mem::transmute(flags), windowtitle.into_param().abi(), windowmessage.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmAccessDeniedRemediationClient> for ::windows_core::IUnknown {
    fn from(value: IFsrmAccessDeniedRemediationClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmAccessDeniedRemediationClient> for ::windows_core::IUnknown {
    fn from(value: &IFsrmAccessDeniedRemediationClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmAccessDeniedRemediationClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmAccessDeniedRemediationClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmAccessDeniedRemediationClient> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmAccessDeniedRemediationClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmAccessDeniedRemediationClient> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmAccessDeniedRemediationClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmAccessDeniedRemediationClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmAccessDeniedRemediationClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmAccessDeniedRemediationClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmAccessDeniedRemediationClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmAccessDeniedRemediationClient {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmAccessDeniedRemediationClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmAccessDeniedRemediationClient").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmAccessDeniedRemediationClient {
    type Vtable = IFsrmAccessDeniedRemediationClient_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x40002314_590b_45a5_8e1b_8c05da527e52);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmAccessDeniedRemediationClient_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentwnd: usize, accesspath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, errortype: AdrClientErrorType, flags: i32, windowtitle: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, windowmessage: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, result: *mut i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmAction(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmAction {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn ActionType(&self) -> ::windows_core::Result<FsrmActionType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmActionType>::zeroed();
        (::windows_core::Interface::vtable(self).ActionType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmActionType>(result__)
    }
    pub unsafe fn RunLimitInterval(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).RunLimitInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRunLimitInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(minutes)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmAction> for ::windows_core::IUnknown {
    fn from(value: IFsrmAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmAction> for ::windows_core::IUnknown {
    fn from(value: &IFsrmAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmAction> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmAction> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmAction {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmAction {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmAction").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmAction {
    type Vtable = IFsrmAction_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6cd6408a_ae60_463b_9ef1_e117534d69dc);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmAction_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub ActionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: *mut FsrmActionType) -> ::windows_core::HRESULT,
    pub RunLimitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows_core::HRESULT,
    pub SetRunLimitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmActionCommand(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmActionCommand {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn ActionType(&self) -> ::windows_core::Result<FsrmActionType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmActionType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ActionType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmActionType>(result__)
    }
    pub unsafe fn RunLimitInterval(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RunLimitInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRunLimitInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(minutes)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExecutablePath(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ExecutablePath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetExecutablePath<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, executablepath: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetExecutablePath)(::windows_core::Interface::as_raw(self), executablepath.into_param().abi()).ok()
    }
    pub unsafe fn Arguments(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Arguments)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetArguments<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, arguments: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetArguments)(::windows_core::Interface::as_raw(self), arguments.into_param().abi()).ok()
    }
    pub unsafe fn Account(&self) -> ::windows_core::Result<FsrmAccountType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmAccountType>::zeroed();
        (::windows_core::Interface::vtable(self).Account)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmAccountType>(result__)
    }
    pub unsafe fn SetAccount(&self, account: FsrmAccountType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAccount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(account)).ok()
    }
    pub unsafe fn WorkingDirectory(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).WorkingDirectory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetWorkingDirectory<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, workingdirectory: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWorkingDirectory)(::windows_core::Interface::as_raw(self), workingdirectory.into_param().abi()).ok()
    }
    pub unsafe fn MonitorCommand(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).MonitorCommand)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetMonitorCommand(&self, monitorcommand: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMonitorCommand)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(monitorcommand)).ok()
    }
    pub unsafe fn KillTimeOut(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).KillTimeOut)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetKillTimeOut(&self, minutes: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetKillTimeOut)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(minutes)).ok()
    }
    pub unsafe fn LogResult(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).LogResult)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLogResult(&self, logresults: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLogResult)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(logresults)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmActionCommand> for ::windows_core::IUnknown {
    fn from(value: IFsrmActionCommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmActionCommand> for ::windows_core::IUnknown {
    fn from(value: &IFsrmActionCommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmActionCommand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmActionCommand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmActionCommand> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmActionCommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmActionCommand> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmActionCommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmActionCommand {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmActionCommand {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmActionCommand> for IFsrmAction {
    fn from(value: IFsrmActionCommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmActionCommand> for IFsrmAction {
    fn from(value: &IFsrmActionCommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmAction> for IFsrmActionCommand {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmAction> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmAction> for &'a IFsrmActionCommand {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmAction> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmActionCommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmActionCommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmActionCommand {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmActionCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmActionCommand").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmActionCommand {
    type Vtable = IFsrmActionCommand_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x12937789_e247_4917_9c20_f3ee9c7ee783);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionCommand_Vtbl {
    pub base__: IFsrmAction_Vtbl,
    pub ExecutablePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executablepath: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetExecutablePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executablepath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arguments: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arguments: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Account: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, account: *mut FsrmAccountType) -> ::windows_core::HRESULT,
    pub SetAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, account: FsrmAccountType) -> ::windows_core::HRESULT,
    pub WorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, workingdirectory: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetWorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, workingdirectory: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub MonitorCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, monitorcommand: *mut i16) -> ::windows_core::HRESULT,
    pub SetMonitorCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, monitorcommand: i16) -> ::windows_core::HRESULT,
    pub KillTimeOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows_core::HRESULT,
    pub SetKillTimeOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows_core::HRESULT,
    pub LogResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logresults: *mut i16) -> ::windows_core::HRESULT,
    pub SetLogResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logresults: i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmActionEmail(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmActionEmail {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn ActionType(&self) -> ::windows_core::Result<FsrmActionType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmActionType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ActionType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmActionType>(result__)
    }
    pub unsafe fn RunLimitInterval(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RunLimitInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRunLimitInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(minutes)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn MailFrom(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).MailFrom)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetMailFrom<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, mailfrom: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMailFrom)(::windows_core::Interface::as_raw(self), mailfrom.into_param().abi()).ok()
    }
    pub unsafe fn MailReplyTo(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).MailReplyTo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetMailReplyTo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, mailreplyto: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMailReplyTo)(::windows_core::Interface::as_raw(self), mailreplyto.into_param().abi()).ok()
    }
    pub unsafe fn MailTo(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).MailTo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetMailTo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, mailto: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMailTo)(::windows_core::Interface::as_raw(self), mailto.into_param().abi()).ok()
    }
    pub unsafe fn MailCc(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).MailCc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetMailCc<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, mailcc: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMailCc)(::windows_core::Interface::as_raw(self), mailcc.into_param().abi()).ok()
    }
    pub unsafe fn MailBcc(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).MailBcc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetMailBcc<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, mailbcc: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMailBcc)(::windows_core::Interface::as_raw(self), mailbcc.into_param().abi()).ok()
    }
    pub unsafe fn MailSubject(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).MailSubject)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetMailSubject<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, mailsubject: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMailSubject)(::windows_core::Interface::as_raw(self), mailsubject.into_param().abi()).ok()
    }
    pub unsafe fn MessageText(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).MessageText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetMessageText<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, messagetext: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMessageText)(::windows_core::Interface::as_raw(self), messagetext.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmActionEmail> for ::windows_core::IUnknown {
    fn from(value: IFsrmActionEmail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmActionEmail> for ::windows_core::IUnknown {
    fn from(value: &IFsrmActionEmail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmActionEmail {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmActionEmail {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmActionEmail> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmActionEmail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmActionEmail> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmActionEmail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmActionEmail {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmActionEmail {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmActionEmail> for IFsrmAction {
    fn from(value: IFsrmActionEmail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmActionEmail> for IFsrmAction {
    fn from(value: &IFsrmActionEmail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmAction> for IFsrmActionEmail {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmAction> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmAction> for &'a IFsrmActionEmail {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmAction> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmActionEmail {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmActionEmail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmActionEmail {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmActionEmail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmActionEmail").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmActionEmail {
    type Vtable = IFsrmActionEmail_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd646567d_26ae_4caa_9f84_4e0aad207fca);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionEmail_Vtbl {
    pub base__: IFsrmAction_Vtbl,
    pub MailFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailfrom: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetMailFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailfrom: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub MailReplyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailreplyto: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetMailReplyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailreplyto: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub MailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetMailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub MailCc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailcc: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetMailCc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailcc: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub MailBcc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailbcc: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetMailBcc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailbcc: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub MailSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailsubject: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetMailSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailsubject: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub MessageText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetext: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetMessageText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetext: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmActionEmail2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmActionEmail2 {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn ActionType(&self) -> ::windows_core::Result<FsrmActionType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmActionType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ActionType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmActionType>(result__)
    }
    pub unsafe fn RunLimitInterval(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.RunLimitInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetRunLimitInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(minutes)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn MailFrom(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MailFrom)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetMailFrom<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, mailfrom: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMailFrom)(::windows_core::Interface::as_raw(self), mailfrom.into_param().abi()).ok()
    }
    pub unsafe fn MailReplyTo(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MailReplyTo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetMailReplyTo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, mailreplyto: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMailReplyTo)(::windows_core::Interface::as_raw(self), mailreplyto.into_param().abi()).ok()
    }
    pub unsafe fn MailTo(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MailTo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetMailTo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, mailto: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMailTo)(::windows_core::Interface::as_raw(self), mailto.into_param().abi()).ok()
    }
    pub unsafe fn MailCc(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MailCc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetMailCc<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, mailcc: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMailCc)(::windows_core::Interface::as_raw(self), mailcc.into_param().abi()).ok()
    }
    pub unsafe fn MailBcc(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MailBcc)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetMailBcc<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, mailbcc: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMailBcc)(::windows_core::Interface::as_raw(self), mailbcc.into_param().abi()).ok()
    }
    pub unsafe fn MailSubject(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MailSubject)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetMailSubject<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, mailsubject: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMailSubject)(::windows_core::Interface::as_raw(self), mailsubject.into_param().abi()).ok()
    }
    pub unsafe fn MessageText(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MessageText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetMessageText<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, messagetext: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMessageText)(::windows_core::Interface::as_raw(self), messagetext.into_param().abi()).ok()
    }
    pub unsafe fn AttachmentFileListSize(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).AttachmentFileListSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAttachmentFileListSize(&self, attachmentfilelistsize: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAttachmentFileListSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(attachmentfilelistsize)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmActionEmail2> for ::windows_core::IUnknown {
    fn from(value: IFsrmActionEmail2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmActionEmail2> for ::windows_core::IUnknown {
    fn from(value: &IFsrmActionEmail2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmActionEmail2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmActionEmail2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmActionEmail2> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmActionEmail2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmActionEmail2> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmActionEmail2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmActionEmail2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmActionEmail2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmActionEmail2> for IFsrmAction {
    fn from(value: IFsrmActionEmail2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmActionEmail2> for IFsrmAction {
    fn from(value: &IFsrmActionEmail2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmAction> for IFsrmActionEmail2 {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmAction> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmAction> for &'a IFsrmActionEmail2 {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmAction> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmActionEmail2> for IFsrmActionEmail {
    fn from(value: IFsrmActionEmail2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmActionEmail2> for IFsrmActionEmail {
    fn from(value: &IFsrmActionEmail2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmActionEmail> for IFsrmActionEmail2 {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmActionEmail> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmActionEmail> for &'a IFsrmActionEmail2 {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmActionEmail> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmActionEmail2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmActionEmail2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmActionEmail2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmActionEmail2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmActionEmail2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmActionEmail2 {
    type Vtable = IFsrmActionEmail2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8276702f_2532_4839_89bf_4872609a2ea4);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionEmail2_Vtbl {
    pub base__: IFsrmActionEmail_Vtbl,
    pub AttachmentFileListSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attachmentfilelistsize: *mut i32) -> ::windows_core::HRESULT,
    pub SetAttachmentFileListSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attachmentfilelistsize: i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmActionEventLog(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmActionEventLog {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn ActionType(&self) -> ::windows_core::Result<FsrmActionType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmActionType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ActionType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmActionType>(result__)
    }
    pub unsafe fn RunLimitInterval(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RunLimitInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRunLimitInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(minutes)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EventType(&self) -> ::windows_core::Result<FsrmEventType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmEventType>::zeroed();
        (::windows_core::Interface::vtable(self).EventType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmEventType>(result__)
    }
    pub unsafe fn SetEventType(&self, eventtype: FsrmEventType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEventType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(eventtype)).ok()
    }
    pub unsafe fn MessageText(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).MessageText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetMessageText<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, messagetext: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMessageText)(::windows_core::Interface::as_raw(self), messagetext.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmActionEventLog> for ::windows_core::IUnknown {
    fn from(value: IFsrmActionEventLog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmActionEventLog> for ::windows_core::IUnknown {
    fn from(value: &IFsrmActionEventLog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmActionEventLog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmActionEventLog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmActionEventLog> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmActionEventLog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmActionEventLog> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmActionEventLog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmActionEventLog {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmActionEventLog {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmActionEventLog> for IFsrmAction {
    fn from(value: IFsrmActionEventLog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmActionEventLog> for IFsrmAction {
    fn from(value: &IFsrmActionEventLog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmAction> for IFsrmActionEventLog {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmAction> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmAction> for &'a IFsrmActionEventLog {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmAction> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmActionEventLog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmActionEventLog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmActionEventLog {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmActionEventLog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmActionEventLog").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmActionEventLog {
    type Vtable = IFsrmActionEventLog_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c8f96c3_5d94_4f37_a4f4_f56ab463546f);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionEventLog_Vtbl {
    pub base__: IFsrmAction_Vtbl,
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventtype: *mut FsrmEventType) -> ::windows_core::HRESULT,
    pub SetEventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventtype: FsrmEventType) -> ::windows_core::HRESULT,
    pub MessageText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetext: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetMessageText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetext: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmActionReport(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmActionReport {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn ActionType(&self) -> ::windows_core::Result<FsrmActionType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmActionType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ActionType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmActionType>(result__)
    }
    pub unsafe fn RunLimitInterval(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RunLimitInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRunLimitInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(minutes)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ReportTypes(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).ReportTypes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetReportTypes(&self, reporttypes: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetReportTypes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(reporttypes)).ok()
    }
    pub unsafe fn MailTo(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).MailTo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetMailTo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, mailto: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMailTo)(::windows_core::Interface::as_raw(self), mailto.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmActionReport> for ::windows_core::IUnknown {
    fn from(value: IFsrmActionReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmActionReport> for ::windows_core::IUnknown {
    fn from(value: &IFsrmActionReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmActionReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmActionReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmActionReport> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmActionReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmActionReport> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmActionReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmActionReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmActionReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmActionReport> for IFsrmAction {
    fn from(value: IFsrmActionReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmActionReport> for IFsrmAction {
    fn from(value: &IFsrmActionReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmAction> for IFsrmActionReport {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmAction> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmAction> for &'a IFsrmActionReport {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmAction> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmActionReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmActionReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmActionReport {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmActionReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmActionReport").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmActionReport {
    type Vtable = IFsrmActionReport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2dbe63c4_b340_48a0_a5b0_158e07fc567e);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionReport_Vtbl {
    pub base__: IFsrmAction_Vtbl,
    #[cfg(feature = "win32-system")]
    pub ReportTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttypes: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ReportTypes: usize,
    #[cfg(feature = "win32-system")]
    pub SetReportTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttypes: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetReportTypes: usize,
    pub MailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetMailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmAutoApplyQuota(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmAutoApplyQuota {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, description: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.QuotaLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetQuotaLimit<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, quotalimit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetQuotaLimit)(::windows_core::Interface::as_raw(self), quotalimit.into_param().abi()).ok()
    }
    pub unsafe fn QuotaFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.QuotaFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetQuotaFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(quotaflags)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Thresholds(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Thresholds)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddThreshold)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold)).ok()
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DeleteThreshold)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold)).ok()
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.ModifyThreshold)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold), ::core::mem::transmute(newthreshold)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows_core::Result<IFsrmAction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateThresholdAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold), ::core::mem::transmute(actiontype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmAction>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EnumThresholdActions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Path)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserSid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UserSid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserAccount(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UserAccount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SourceTemplateName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SourceTemplateName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn MatchesSourceTemplate(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MatchesSourceTemplate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn ApplyTemplate<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, quotatemplatename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ApplyTemplate)(::windows_core::Interface::as_raw(self), quotatemplatename.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ExcludeFolders(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).ExcludeFolders)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetExcludeFolders(&self, folders: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetExcludeFolders)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(folders)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows_core::Result<IFsrmDerivedObjectsResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CommitAndUpdateDerived)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(commitoptions), ::core::mem::transmute(applyoptions), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmDerivedObjectsResult>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmAutoApplyQuota> for ::windows_core::IUnknown {
    fn from(value: IFsrmAutoApplyQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmAutoApplyQuota> for ::windows_core::IUnknown {
    fn from(value: &IFsrmAutoApplyQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmAutoApplyQuota> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmAutoApplyQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmAutoApplyQuota> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmAutoApplyQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmAutoApplyQuota> for IFsrmObject {
    fn from(value: IFsrmAutoApplyQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmAutoApplyQuota> for IFsrmObject {
    fn from(value: &IFsrmAutoApplyQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for &'a IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmAutoApplyQuota> for IFsrmQuotaBase {
    fn from(value: IFsrmAutoApplyQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmAutoApplyQuota> for IFsrmQuotaBase {
    fn from(value: &IFsrmAutoApplyQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmQuotaBase> for IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmQuotaBase> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmQuotaBase> for &'a IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmQuotaBase> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmAutoApplyQuota> for IFsrmQuotaObject {
    fn from(value: IFsrmAutoApplyQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmAutoApplyQuota> for IFsrmQuotaObject {
    fn from(value: &IFsrmAutoApplyQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmQuotaObject> for IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmQuotaObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmQuotaObject> for &'a IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmQuotaObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmAutoApplyQuota {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmAutoApplyQuota {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmAutoApplyQuota {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmAutoApplyQuota {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmAutoApplyQuota").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmAutoApplyQuota {
    type Vtable = IFsrmAutoApplyQuota_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf82e5729_6aba_4740_bfc7_c7f58f75fb7b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmAutoApplyQuota_Vtbl {
    pub base__: IFsrmQuotaObject_Vtbl,
    #[cfg(feature = "win32-system")]
    pub ExcludeFolders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folders: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ExcludeFolders: usize,
    #[cfg(feature = "win32-system")]
    pub SetExcludeFolders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folders: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetExcludeFolders: usize,
    #[cfg(feature = "win32-system")]
    pub CommitAndUpdateDerived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CommitAndUpdateDerived: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmClassificationManager(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmClassificationManager {
    #[cfg(feature = "win32-system")]
    pub unsafe fn ClassificationReportFormats(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).ClassificationReportFormats)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetClassificationReportFormats(&self, formats: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClassificationReportFormats)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(formats)).ok()
    }
    pub unsafe fn Logging(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Logging)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetLogging(&self, logging: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLogging)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(logging)).ok()
    }
    pub unsafe fn ClassificationReportMailTo(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ClassificationReportMailTo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetClassificationReportMailTo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, mailto: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClassificationReportMailTo)(::windows_core::Interface::as_raw(self), mailto.into_param().abi()).ok()
    }
    pub unsafe fn ClassificationReportEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).ClassificationReportEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetClassificationReportEnabled(&self, reportenabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClassificationReportEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(reportenabled)).ok()
    }
    pub unsafe fn ClassificationLastReportPathWithoutExtension(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ClassificationLastReportPathWithoutExtension)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ClassificationLastError(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ClassificationLastError)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ClassificationRunningStatus(&self) -> ::windows_core::Result<FsrmReportRunningStatus> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmReportRunningStatus>::zeroed();
        (::windows_core::Interface::vtable(self).ClassificationRunningStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmReportRunningStatus>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumPropertyDefinitions(&self, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumPropertyDefinitions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreatePropertyDefinition(&self) -> ::windows_core::Result<IFsrmPropertyDefinition> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreatePropertyDefinition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmPropertyDefinition>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetPropertyDefinition<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, propertyname: Param0) -> ::windows_core::Result<IFsrmPropertyDefinition> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetPropertyDefinition)(::windows_core::Interface::as_raw(self), propertyname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmPropertyDefinition>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumRules(&self, ruletype: FsrmRuleType, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumRules)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ruletype), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateRule(&self, ruletype: FsrmRuleType) -> ::windows_core::Result<IFsrmRule> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateRule)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ruletype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmRule>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetRule<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, rulename: Param0, ruletype: FsrmRuleType) -> ::windows_core::Result<IFsrmRule> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetRule)(::windows_core::Interface::as_raw(self), rulename.into_param().abi(), ::core::mem::transmute(ruletype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmRule>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumModuleDefinitions(&self, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumModuleDefinitions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(moduletype), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateModuleDefinition(&self, moduletype: FsrmPipelineModuleType) -> ::windows_core::Result<IFsrmPipelineModuleDefinition> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateModuleDefinition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(moduletype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmPipelineModuleDefinition>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetModuleDefinition<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, modulename: Param0, moduletype: FsrmPipelineModuleType) -> ::windows_core::Result<IFsrmPipelineModuleDefinition> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetModuleDefinition)(::windows_core::Interface::as_raw(self), modulename.into_param().abi(), ::core::mem::transmute(moduletype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmPipelineModuleDefinition>(result__)
    }
    pub unsafe fn RunClassification<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, context: FsrmReportGenerationContext, reserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RunClassification)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(context), reserved.into_param().abi()).ok()
    }
    pub unsafe fn WaitForClassificationCompletion(&self, waitseconds: i32) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).WaitForClassificationCompletion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(waitseconds), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn CancelClassification(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelClassification)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumFileProperties<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, filepath: Param0, options: FsrmGetFilePropertyOptions) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumFileProperties)(::windows_core::Interface::as_raw(self), filepath.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetFileProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, filepath: Param0, propertyname: Param1, options: FsrmGetFilePropertyOptions) -> ::windows_core::Result<IFsrmProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFileProperty)(::windows_core::Interface::as_raw(self), filepath.into_param().abi(), propertyname.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmProperty>(result__)
    }
    pub unsafe fn SetFileProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, filepath: Param0, propertyname: Param1, propertyvalue: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFileProperty)(::windows_core::Interface::as_raw(self), filepath.into_param().abi(), propertyname.into_param().abi(), propertyvalue.into_param().abi()).ok()
    }
    pub unsafe fn ClearFileProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, filepath: Param0, property: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ClearFileProperty)(::windows_core::Interface::as_raw(self), filepath.into_param().abi(), property.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmClassificationManager> for ::windows_core::IUnknown {
    fn from(value: IFsrmClassificationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmClassificationManager> for ::windows_core::IUnknown {
    fn from(value: &IFsrmClassificationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmClassificationManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmClassificationManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmClassificationManager> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmClassificationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmClassificationManager> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmClassificationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmClassificationManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmClassificationManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmClassificationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmClassificationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmClassificationManager {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmClassificationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmClassificationManager").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmClassificationManager {
    type Vtable = IFsrmClassificationManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd2dc89da_ee91_48a0_85d8_cc72a56f7d04);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassificationManager_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub ClassificationReportFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ClassificationReportFormats: usize,
    #[cfg(feature = "win32-system")]
    pub SetClassificationReportFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetClassificationReportFormats: usize,
    pub Logging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logging: *mut i32) -> ::windows_core::HRESULT,
    pub SetLogging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logging: i32) -> ::windows_core::HRESULT,
    pub ClassificationReportMailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetClassificationReportMailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ClassificationReportEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportenabled: *mut i16) -> ::windows_core::HRESULT,
    pub SetClassificationReportEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportenabled: i16) -> ::windows_core::HRESULT,
    pub ClassificationLastReportPathWithoutExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastreportpath: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ClassificationLastError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lasterror: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ClassificationRunningStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub EnumPropertyDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, propertydefinitions: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    EnumPropertyDefinitions: usize,
    #[cfg(feature = "win32-system")]
    pub CreatePropertyDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertydefinition: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreatePropertyDefinition: usize,
    #[cfg(feature = "win32-system")]
    pub GetPropertyDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, propertydefinition: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetPropertyDefinition: usize,
    #[cfg(feature = "win32-system")]
    pub EnumRules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruletype: FsrmRuleType, options: FsrmEnumOptions, rules: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    EnumRules: usize,
    #[cfg(feature = "win32-system")]
    pub CreateRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruletype: FsrmRuleType, rule: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateRule: usize,
    #[cfg(feature = "win32-system")]
    pub GetRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rulename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ruletype: FsrmRuleType, rule: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetRule: usize,
    #[cfg(feature = "win32-system")]
    pub EnumModuleDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions, moduledefinitions: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    EnumModuleDefinitions: usize,
    #[cfg(feature = "win32-system")]
    pub CreateModuleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduletype: FsrmPipelineModuleType, moduledefinition: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateModuleDefinition: usize,
    #[cfg(feature = "win32-system")]
    pub GetModuleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modulename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, moduletype: FsrmPipelineModuleType, moduledefinition: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetModuleDefinition: usize,
    pub RunClassification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext, reserved: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub WaitForClassificationCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut i16) -> ::windows_core::HRESULT,
    pub CancelClassification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub EnumFileProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, options: FsrmGetFilePropertyOptions, fileproperties: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    EnumFileProperties: usize,
    #[cfg(feature = "win32-system")]
    pub GetFileProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, options: FsrmGetFilePropertyOptions, property: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetFileProperty: usize,
    pub SetFileProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, propertyvalue: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ClearFileProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, property: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmClassificationManager2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmClassificationManager2 {
    #[cfg(feature = "win32-system")]
    pub unsafe fn ClassificationReportFormats(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ClassificationReportFormats)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetClassificationReportFormats(&self, formats: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetClassificationReportFormats)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(formats)).ok()
    }
    pub unsafe fn Logging(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Logging)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetLogging(&self, logging: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLogging)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(logging)).ok()
    }
    pub unsafe fn ClassificationReportMailTo(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ClassificationReportMailTo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetClassificationReportMailTo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, mailto: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetClassificationReportMailTo)(::windows_core::Interface::as_raw(self), mailto.into_param().abi()).ok()
    }
    pub unsafe fn ClassificationReportEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ClassificationReportEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetClassificationReportEnabled(&self, reportenabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetClassificationReportEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(reportenabled)).ok()
    }
    pub unsafe fn ClassificationLastReportPathWithoutExtension(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ClassificationLastReportPathWithoutExtension)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ClassificationLastError(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ClassificationLastError)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ClassificationRunningStatus(&self) -> ::windows_core::Result<FsrmReportRunningStatus> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmReportRunningStatus>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ClassificationRunningStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmReportRunningStatus>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumPropertyDefinitions(&self, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumPropertyDefinitions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreatePropertyDefinition(&self) -> ::windows_core::Result<IFsrmPropertyDefinition> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreatePropertyDefinition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmPropertyDefinition>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetPropertyDefinition<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, propertyname: Param0) -> ::windows_core::Result<IFsrmPropertyDefinition> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPropertyDefinition)(::windows_core::Interface::as_raw(self), propertyname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmPropertyDefinition>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumRules(&self, ruletype: FsrmRuleType, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumRules)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ruletype), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateRule(&self, ruletype: FsrmRuleType) -> ::windows_core::Result<IFsrmRule> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateRule)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ruletype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmRule>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetRule<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, rulename: Param0, ruletype: FsrmRuleType) -> ::windows_core::Result<IFsrmRule> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRule)(::windows_core::Interface::as_raw(self), rulename.into_param().abi(), ::core::mem::transmute(ruletype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmRule>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumModuleDefinitions(&self, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumModuleDefinitions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(moduletype), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateModuleDefinition(&self, moduletype: FsrmPipelineModuleType) -> ::windows_core::Result<IFsrmPipelineModuleDefinition> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateModuleDefinition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(moduletype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmPipelineModuleDefinition>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetModuleDefinition<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, modulename: Param0, moduletype: FsrmPipelineModuleType) -> ::windows_core::Result<IFsrmPipelineModuleDefinition> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetModuleDefinition)(::windows_core::Interface::as_raw(self), modulename.into_param().abi(), ::core::mem::transmute(moduletype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmPipelineModuleDefinition>(result__)
    }
    pub unsafe fn RunClassification<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, context: FsrmReportGenerationContext, reserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RunClassification)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(context), reserved.into_param().abi()).ok()
    }
    pub unsafe fn WaitForClassificationCompletion(&self, waitseconds: i32) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.WaitForClassificationCompletion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(waitseconds), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn CancelClassification(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CancelClassification)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumFileProperties<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, filepath: Param0, options: FsrmGetFilePropertyOptions) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumFileProperties)(::windows_core::Interface::as_raw(self), filepath.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetFileProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, filepath: Param0, propertyname: Param1, options: FsrmGetFilePropertyOptions) -> ::windows_core::Result<IFsrmProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetFileProperty)(::windows_core::Interface::as_raw(self), filepath.into_param().abi(), propertyname.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmProperty>(result__)
    }
    pub unsafe fn SetFileProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, filepath: Param0, propertyname: Param1, propertyvalue: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetFileProperty)(::windows_core::Interface::as_raw(self), filepath.into_param().abi(), propertyname.into_param().abi(), propertyvalue.into_param().abi()).ok()
    }
    pub unsafe fn ClearFileProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, filepath: Param0, property: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ClearFileProperty)(::windows_core::Interface::as_raw(self), filepath.into_param().abi(), property.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ClassifyFiles(&self, filepaths: *const ::win32_system::Com::SAFEARRAY, propertynames: *const ::win32_system::Com::SAFEARRAY, propertyvalues: *const ::win32_system::Com::SAFEARRAY, options: FsrmGetFilePropertyOptions) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ClassifyFiles)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(filepaths), ::core::mem::transmute(propertynames), ::core::mem::transmute(propertyvalues), ::core::mem::transmute(options)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmClassificationManager2> for ::windows_core::IUnknown {
    fn from(value: IFsrmClassificationManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmClassificationManager2> for ::windows_core::IUnknown {
    fn from(value: &IFsrmClassificationManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmClassificationManager2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmClassificationManager2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmClassificationManager2> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmClassificationManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmClassificationManager2> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmClassificationManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmClassificationManager2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmClassificationManager2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmClassificationManager2> for IFsrmClassificationManager {
    fn from(value: IFsrmClassificationManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmClassificationManager2> for IFsrmClassificationManager {
    fn from(value: &IFsrmClassificationManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmClassificationManager> for IFsrmClassificationManager2 {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmClassificationManager> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmClassificationManager> for &'a IFsrmClassificationManager2 {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmClassificationManager> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmClassificationManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmClassificationManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmClassificationManager2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmClassificationManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmClassificationManager2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmClassificationManager2 {
    type Vtable = IFsrmClassificationManager2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0004c1c9_127e_4765_ba07_6a3147bca112);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassificationManager2_Vtbl {
    pub base__: IFsrmClassificationManager_Vtbl,
    #[cfg(feature = "win32-system")]
    pub ClassifyFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepaths: *const ::win32_system::Com::SAFEARRAY, propertynames: *const ::win32_system::Com::SAFEARRAY, propertyvalues: *const ::win32_system::Com::SAFEARRAY, options: FsrmGetFilePropertyOptions) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ClassifyFiles: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmClassificationRule(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmClassificationRule {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, description: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn RuleType(&self) -> ::windows_core::Result<FsrmRuleType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmRuleType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RuleType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmRuleType>(result__)
    }
    pub unsafe fn ModuleDefinitionName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ModuleDefinitionName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetModuleDefinitionName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, moduledefinitionname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetModuleDefinitionName)(::windows_core::Interface::as_raw(self), moduledefinitionname.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn NamespaceRoots(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.NamespaceRoots)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetNamespaceRoots)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(namespaceroots)).ok()
    }
    pub unsafe fn RuleFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RuleFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRuleFlags(&self, ruleflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRuleFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ruleflags)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Parameters(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetParameters(&self, parameters: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetParameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(parameters)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn LastModified(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastModified)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn ExecutionOption(&self) -> ::windows_core::Result<FsrmExecutionOption> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmExecutionOption>::zeroed();
        (::windows_core::Interface::vtable(self).ExecutionOption)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmExecutionOption>(result__)
    }
    pub unsafe fn SetExecutionOption(&self, executionoption: FsrmExecutionOption) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetExecutionOption)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(executionoption)).ok()
    }
    pub unsafe fn PropertyAffected(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PropertyAffected)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetPropertyAffected<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, property: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPropertyAffected)(::windows_core::Interface::as_raw(self), property.into_param().abi()).ok()
    }
    pub unsafe fn Value(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Value)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetValue<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValue)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmClassificationRule> for ::windows_core::IUnknown {
    fn from(value: IFsrmClassificationRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmClassificationRule> for ::windows_core::IUnknown {
    fn from(value: &IFsrmClassificationRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmClassificationRule {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmClassificationRule {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmClassificationRule> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmClassificationRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmClassificationRule> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmClassificationRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmClassificationRule {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmClassificationRule {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmClassificationRule> for IFsrmObject {
    fn from(value: IFsrmClassificationRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmClassificationRule> for IFsrmObject {
    fn from(value: &IFsrmClassificationRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for IFsrmClassificationRule {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for &'a IFsrmClassificationRule {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmClassificationRule> for IFsrmRule {
    fn from(value: IFsrmClassificationRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmClassificationRule> for IFsrmRule {
    fn from(value: &IFsrmClassificationRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmRule> for IFsrmClassificationRule {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmRule> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmRule> for &'a IFsrmClassificationRule {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmRule> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmClassificationRule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmClassificationRule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmClassificationRule {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmClassificationRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmClassificationRule").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmClassificationRule {
    type Vtable = IFsrmClassificationRule_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xafc052c2_5315_45ab_841b_c6db0e120148);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassificationRule_Vtbl {
    pub base__: IFsrmRule_Vtbl,
    pub ExecutionOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executionoption: *mut FsrmExecutionOption) -> ::windows_core::HRESULT,
    pub SetExecutionOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executionoption: FsrmExecutionOption) -> ::windows_core::HRESULT,
    pub PropertyAffected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetPropertyAffected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmClassifierModuleDefinition(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmClassifierModuleDefinition {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, description: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ModuleClsid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ModuleClsid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetModuleClsid<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, moduleclsid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetModuleClsid)(::windows_core::Interface::as_raw(self), moduleclsid.into_param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Company(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Company)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetCompany<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, company: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetCompany)(::windows_core::Interface::as_raw(self), company.into_param().abi()).ok()
    }
    pub unsafe fn Version(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Version)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetVersion<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, version: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetVersion)(::windows_core::Interface::as_raw(self), version.into_param().abi()).ok()
    }
    pub unsafe fn ModuleType(&self) -> ::windows_core::Result<FsrmPipelineModuleType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmPipelineModuleType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ModuleType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmPipelineModuleType>(result__)
    }
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn NeedsFileContent(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.NeedsFileContent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetNeedsFileContent(&self, needsfilecontent: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetNeedsFileContent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(needsfilecontent)).ok()
    }
    pub unsafe fn Account(&self) -> ::windows_core::Result<FsrmAccountType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmAccountType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Account)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmAccountType>(result__)
    }
    pub unsafe fn SetAccount(&self, retrievalaccount: FsrmAccountType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetAccount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(retrievalaccount)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SupportedExtensions(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SupportedExtensions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetSupportedExtensions(&self, supportedextensions: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSupportedExtensions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(supportedextensions)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Parameters(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetParameters(&self, parameters: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetParameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(parameters)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn PropertiesAffected(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).PropertiesAffected)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetPropertiesAffected(&self, propertiesaffected: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPropertiesAffected)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propertiesaffected)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn PropertiesUsed(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).PropertiesUsed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetPropertiesUsed(&self, propertiesused: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPropertiesUsed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propertiesused)).ok()
    }
    pub unsafe fn NeedsExplicitValue(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).NeedsExplicitValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetNeedsExplicitValue(&self, needsexplicitvalue: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNeedsExplicitValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(needsexplicitvalue)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmClassifierModuleDefinition> for ::windows_core::IUnknown {
    fn from(value: IFsrmClassifierModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmClassifierModuleDefinition> for ::windows_core::IUnknown {
    fn from(value: &IFsrmClassifierModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmClassifierModuleDefinition> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmClassifierModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmClassifierModuleDefinition> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmClassifierModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmClassifierModuleDefinition> for IFsrmObject {
    fn from(value: IFsrmClassifierModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmClassifierModuleDefinition> for IFsrmObject {
    fn from(value: &IFsrmClassifierModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for &'a IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmClassifierModuleDefinition> for IFsrmPipelineModuleDefinition {
    fn from(value: IFsrmClassifierModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmClassifierModuleDefinition> for IFsrmPipelineModuleDefinition {
    fn from(value: &IFsrmClassifierModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmPipelineModuleDefinition> for IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmPipelineModuleDefinition> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmPipelineModuleDefinition> for &'a IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmPipelineModuleDefinition> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmClassifierModuleDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmClassifierModuleDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmClassifierModuleDefinition {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmClassifierModuleDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmClassifierModuleDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmClassifierModuleDefinition {
    type Vtable = IFsrmClassifierModuleDefinition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb36ea26_6318_4b8c_8592_f72dd602e7a5);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassifierModuleDefinition_Vtbl {
    pub base__: IFsrmPipelineModuleDefinition_Vtbl,
    #[cfg(feature = "win32-system")]
    pub PropertiesAffected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiesaffected: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    PropertiesAffected: usize,
    #[cfg(feature = "win32-system")]
    pub SetPropertiesAffected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiesaffected: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetPropertiesAffected: usize,
    #[cfg(feature = "win32-system")]
    pub PropertiesUsed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiesused: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    PropertiesUsed: usize,
    #[cfg(feature = "win32-system")]
    pub SetPropertiesUsed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiesused: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetPropertiesUsed: usize,
    pub NeedsExplicitValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, needsexplicitvalue: *mut i16) -> ::windows_core::HRESULT,
    pub SetNeedsExplicitValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, needsexplicitvalue: i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmClassifierModuleImplementation(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmClassifierModuleImplementation {
    #[cfg(feature = "win32-system")]
    pub unsafe fn OnLoad<'a, Param0: ::windows_core::IntoParam<'a, IFsrmPipelineModuleDefinition>>(&self, moduledefinition: Param0) -> ::windows_core::Result<IFsrmPipelineModuleConnector> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.OnLoad)(::windows_core::Interface::as_raw(self), moduledefinition.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmPipelineModuleConnector>(result__)
    }
    pub unsafe fn OnUnload(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnUnload)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn LastModified(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).LastModified)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn UseRulesAndDefinitions<'a, Param0: ::windows_core::IntoParam<'a, IFsrmCollection>, Param1: ::windows_core::IntoParam<'a, IFsrmCollection>>(&self, rules: Param0, propertydefinitions: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UseRulesAndDefinitions)(::windows_core::Interface::as_raw(self), rules.into_param().abi(), propertydefinitions.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn OnBeginFile<'a, Param0: ::windows_core::IntoParam<'a, IFsrmPropertyBag>>(&self, propertybag: Param0, arrayruleids: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnBeginFile)(::windows_core::Interface::as_raw(self), propertybag.into_param().abi(), ::core::mem::transmute(arrayruleids)).ok()
    }
    pub unsafe fn DoesPropertyValueApply<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param4: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, property: Param0, value: Param1, applyvalue: *mut i16, idrule: Param3, idpropdef: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DoesPropertyValueApply)(::windows_core::Interface::as_raw(self), property.into_param().abi(), value.into_param().abi(), ::core::mem::transmute(applyvalue), idrule.into_param().abi(), idpropdef.into_param().abi()).ok()
    }
    pub unsafe fn GetPropertyValueToApply<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param3: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, property: Param0, value: *mut ::win32_foundation::BSTR, idrule: Param2, idpropdef: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyValueToApply)(::windows_core::Interface::as_raw(self), property.into_param().abi(), ::core::mem::transmute(value), idrule.into_param().abi(), idpropdef.into_param().abi()).ok()
    }
    pub unsafe fn OnEndFile(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnEndFile)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmClassifierModuleImplementation> for ::windows_core::IUnknown {
    fn from(value: IFsrmClassifierModuleImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmClassifierModuleImplementation> for ::windows_core::IUnknown {
    fn from(value: &IFsrmClassifierModuleImplementation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmClassifierModuleImplementation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmClassifierModuleImplementation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmClassifierModuleImplementation> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmClassifierModuleImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmClassifierModuleImplementation> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmClassifierModuleImplementation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmClassifierModuleImplementation {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmClassifierModuleImplementation {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmClassifierModuleImplementation> for IFsrmPipelineModuleImplementation {
    fn from(value: IFsrmClassifierModuleImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmClassifierModuleImplementation> for IFsrmPipelineModuleImplementation {
    fn from(value: &IFsrmClassifierModuleImplementation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmPipelineModuleImplementation> for IFsrmClassifierModuleImplementation {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmPipelineModuleImplementation> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmPipelineModuleImplementation> for &'a IFsrmClassifierModuleImplementation {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmPipelineModuleImplementation> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmClassifierModuleImplementation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmClassifierModuleImplementation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmClassifierModuleImplementation {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmClassifierModuleImplementation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmClassifierModuleImplementation").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmClassifierModuleImplementation {
    type Vtable = IFsrmClassifierModuleImplementation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c968fc6_6edb_4051_9c18_73b7291ae106);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassifierModuleImplementation_Vtbl {
    pub base__: IFsrmPipelineModuleImplementation_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub LastModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastmodified: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    LastModified: usize,
    #[cfg(feature = "win32-system")]
    pub UseRulesAndDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rules: ::windows_core::RawPtr, propertydefinitions: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    UseRulesAndDefinitions: usize,
    #[cfg(feature = "win32-system")]
    pub OnBeginFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertybag: ::windows_core::RawPtr, arrayruleids: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    OnBeginFile: usize,
    pub DoesPropertyValueApply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, value: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, applyvalue: *mut i16, idrule: ::windows_core::GUID, idpropdef: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetPropertyValueToApply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, value: *mut ::win32_foundation::BSTR, idrule: ::windows_core::GUID, idpropdef: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OnEndFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn State(&self) -> ::windows_core::Result<FsrmCollectionState> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmCollectionState>::zeroed();
        (::windows_core::Interface::vtable(self).State)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmCollectionState>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).WaitForCompletion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(waitseconds), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GetById<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, id: Param0) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetById)(::windows_core::Interface::as_raw(self), id.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmCollection> for ::windows_core::IUnknown {
    fn from(value: IFsrmCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmCollection> for ::windows_core::IUnknown {
    fn from(value: &IFsrmCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmCollection> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmCollection> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmCollection {
    type Vtable = IFsrmCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf76fbf3b_8ddd_4b42_b05a_cb1c3ff1fee8);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmCollection_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unknown: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, item: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: *mut FsrmCollectionState) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub WaitForCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub GetById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::windows_core::GUID, entry: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    GetById: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmCommittableCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmCommittableCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn State(&self) -> ::windows_core::Result<FsrmCollectionState> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmCollectionState>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.State)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmCollectionState>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.WaitForCompletion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(waitseconds), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GetById<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, id: Param0) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetById)(::windows_core::Interface::as_raw(self), id.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Add<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, item: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Add)(::windows_core::Interface::as_raw(self), item.into_param().abi()).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Remove)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index)).ok()
    }
    pub unsafe fn RemoveById<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, id: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RemoveById)(::windows_core::Interface::as_raw(self), id.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IFsrmMutableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmMutableCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Commit(&self, options: FsrmCommitOptions) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Commit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmCommittableCollection> for ::windows_core::IUnknown {
    fn from(value: IFsrmCommittableCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmCommittableCollection> for ::windows_core::IUnknown {
    fn from(value: &IFsrmCommittableCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmCommittableCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmCommittableCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmCommittableCollection> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmCommittableCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmCommittableCollection> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmCommittableCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmCommittableCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmCommittableCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmCommittableCollection> for IFsrmCollection {
    fn from(value: IFsrmCommittableCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmCommittableCollection> for IFsrmCollection {
    fn from(value: &IFsrmCommittableCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmCollection> for IFsrmCommittableCollection {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmCollection> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmCollection> for &'a IFsrmCommittableCollection {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmCollection> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmCommittableCollection> for IFsrmMutableCollection {
    fn from(value: IFsrmCommittableCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmCommittableCollection> for IFsrmMutableCollection {
    fn from(value: &IFsrmCommittableCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmMutableCollection> for IFsrmCommittableCollection {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmMutableCollection> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmMutableCollection> for &'a IFsrmCommittableCollection {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmMutableCollection> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmCommittableCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmCommittableCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmCommittableCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmCommittableCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmCommittableCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmCommittableCollection {
    type Vtable = IFsrmCommittableCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96deb3b5_8b91_4a2a_9d93_80a35d8aa847);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmCommittableCollection_Vtbl {
    pub base__: IFsrmMutableCollection_Vtbl,
    #[cfg(feature = "win32-system")]
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmCommitOptions, results: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Commit: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmDerivedObjectsResult(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmDerivedObjectsResult {
    #[cfg(feature = "win32-system")]
    pub unsafe fn DerivedObjects(&self) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).DerivedObjects)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Results(&self) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Results)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmDerivedObjectsResult> for ::windows_core::IUnknown {
    fn from(value: IFsrmDerivedObjectsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmDerivedObjectsResult> for ::windows_core::IUnknown {
    fn from(value: &IFsrmDerivedObjectsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmDerivedObjectsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmDerivedObjectsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmDerivedObjectsResult> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmDerivedObjectsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmDerivedObjectsResult> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmDerivedObjectsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmDerivedObjectsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmDerivedObjectsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmDerivedObjectsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmDerivedObjectsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmDerivedObjectsResult {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmDerivedObjectsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmDerivedObjectsResult").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmDerivedObjectsResult {
    type Vtable = IFsrmDerivedObjectsResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x39322a2d_38ee_4d0d_8095_421a80849a82);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmDerivedObjectsResult_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub DerivedObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, derivedobjects: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    DerivedObjects: usize,
    #[cfg(feature = "win32-system")]
    pub Results: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, results: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Results: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmExportImport(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmExportImport {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ExportFileGroups<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, filepath: Param0, filegroupnamessafearray: *const ::win32_system::Com::VARIANT, remotehost: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExportFileGroups)(::windows_core::Interface::as_raw(self), filepath.into_param().abi(), ::core::mem::transmute(filegroupnamessafearray), remotehost.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ImportFileGroups<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, filepath: Param0, filegroupnamessafearray: *const ::win32_system::Com::VARIANT, remotehost: Param2) -> ::windows_core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ImportFileGroups)(::windows_core::Interface::as_raw(self), filepath.into_param().abi(), ::core::mem::transmute(filegroupnamessafearray), remotehost.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ExportFileScreenTemplates<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, filepath: Param0, templatenamessafearray: *const ::win32_system::Com::VARIANT, remotehost: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExportFileScreenTemplates)(::windows_core::Interface::as_raw(self), filepath.into_param().abi(), ::core::mem::transmute(templatenamessafearray), remotehost.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ImportFileScreenTemplates<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, filepath: Param0, templatenamessafearray: *const ::win32_system::Com::VARIANT, remotehost: Param2) -> ::windows_core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ImportFileScreenTemplates)(::windows_core::Interface::as_raw(self), filepath.into_param().abi(), ::core::mem::transmute(templatenamessafearray), remotehost.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ExportQuotaTemplates<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, filepath: Param0, templatenamessafearray: *const ::win32_system::Com::VARIANT, remotehost: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExportQuotaTemplates)(::windows_core::Interface::as_raw(self), filepath.into_param().abi(), ::core::mem::transmute(templatenamessafearray), remotehost.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ImportQuotaTemplates<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, filepath: Param0, templatenamessafearray: *const ::win32_system::Com::VARIANT, remotehost: Param2) -> ::windows_core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ImportQuotaTemplates)(::windows_core::Interface::as_raw(self), filepath.into_param().abi(), ::core::mem::transmute(templatenamessafearray), remotehost.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCommittableCollection>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmExportImport> for ::windows_core::IUnknown {
    fn from(value: IFsrmExportImport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmExportImport> for ::windows_core::IUnknown {
    fn from(value: &IFsrmExportImport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmExportImport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmExportImport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmExportImport> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmExportImport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmExportImport> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmExportImport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmExportImport {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmExportImport {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmExportImport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmExportImport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmExportImport {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmExportImport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmExportImport").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmExportImport {
    type Vtable = IFsrmExportImport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xefcb0ab1_16c4_4a79_812c_725614c3306b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmExportImport_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ExportFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, filegroupnamessafearray: *const ::win32_system::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ExportFileGroups: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ImportFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, filegroupnamessafearray: *const ::win32_system::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, filegroups: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ImportFileGroups: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ExportFileScreenTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, templatenamessafearray: *const ::win32_system::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ExportFileScreenTemplates: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ImportFileScreenTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, templatenamessafearray: *const ::win32_system::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, templates: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ImportFileScreenTemplates: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ExportQuotaTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, templatenamessafearray: *const ::win32_system::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ExportQuotaTemplates: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ImportQuotaTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, templatenamessafearray: *const ::win32_system::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, templates: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ImportQuotaTemplates: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmFileCondition(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmFileCondition {
    pub unsafe fn Type(&self) -> ::windows_core::Result<FsrmFileConditionType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmFileConditionType>::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmFileConditionType>(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileCondition> for ::windows_core::IUnknown {
    fn from(value: IFsrmFileCondition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileCondition> for ::windows_core::IUnknown {
    fn from(value: &IFsrmFileCondition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmFileCondition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmFileCondition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileCondition> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmFileCondition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileCondition> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmFileCondition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmFileCondition {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmFileCondition {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmFileCondition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmFileCondition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmFileCondition {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmFileCondition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileCondition").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmFileCondition {
    type Vtable = IFsrmFileCondition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70684ffc_691a_4a1a_b922_97752e138cc1);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileCondition_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut FsrmFileConditionType) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmFileConditionProperty(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmFileConditionProperty {
    pub unsafe fn Type(&self) -> ::windows_core::Result<FsrmFileConditionType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmFileConditionType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmFileConditionType>(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PropertyName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PropertyName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetPropertyName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, newval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPropertyName)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn PropertyId(&self) -> ::windows_core::Result<FsrmFileSystemPropertyId> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmFileSystemPropertyId>::zeroed();
        (::windows_core::Interface::vtable(self).PropertyId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmFileSystemPropertyId>(result__)
    }
    pub unsafe fn SetPropertyId(&self, newval: FsrmFileSystemPropertyId) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPropertyId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newval)).ok()
    }
    pub unsafe fn Operator(&self) -> ::windows_core::Result<FsrmPropertyConditionType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmPropertyConditionType>::zeroed();
        (::windows_core::Interface::vtable(self).Operator)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmPropertyConditionType>(result__)
    }
    pub unsafe fn SetOperator(&self, newval: FsrmPropertyConditionType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOperator)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newval)).ok()
    }
    pub unsafe fn ValueType(&self) -> ::windows_core::Result<FsrmPropertyValueType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmPropertyValueType>::zeroed();
        (::windows_core::Interface::vtable(self).ValueType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmPropertyValueType>(result__)
    }
    pub unsafe fn SetValueType(&self, newval: FsrmPropertyValueType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValueType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newval)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Value(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Value)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetValue<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, newval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValue)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileConditionProperty> for ::windows_core::IUnknown {
    fn from(value: IFsrmFileConditionProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileConditionProperty> for ::windows_core::IUnknown {
    fn from(value: &IFsrmFileConditionProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmFileConditionProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmFileConditionProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileConditionProperty> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmFileConditionProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileConditionProperty> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmFileConditionProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmFileConditionProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmFileConditionProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileConditionProperty> for IFsrmFileCondition {
    fn from(value: IFsrmFileConditionProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileConditionProperty> for IFsrmFileCondition {
    fn from(value: &IFsrmFileConditionProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmFileCondition> for IFsrmFileConditionProperty {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmFileCondition> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmFileCondition> for &'a IFsrmFileConditionProperty {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmFileCondition> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmFileConditionProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmFileConditionProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmFileConditionProperty {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmFileConditionProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileConditionProperty").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmFileConditionProperty {
    type Vtable = IFsrmFileConditionProperty_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x81926775_b981_4479_988f_da171d627360);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileConditionProperty_Vtbl {
    pub base__: IFsrmFileCondition_Vtbl,
    pub PropertyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetPropertyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub PropertyId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut FsrmFileSystemPropertyId) -> ::windows_core::HRESULT,
    pub SetPropertyId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: FsrmFileSystemPropertyId) -> ::windows_core::HRESULT,
    pub Operator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut FsrmPropertyConditionType) -> ::windows_core::HRESULT,
    pub SetOperator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: FsrmPropertyConditionType) -> ::windows_core::HRESULT,
    pub ValueType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut FsrmPropertyValueType) -> ::windows_core::HRESULT,
    pub SetValueType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: FsrmPropertyValueType) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Value: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetValue: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmFileGroup(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmFileGroup {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, description: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Members(&self) -> ::windows_core::Result<IFsrmMutableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Members)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmMutableCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetMembers<'a, Param0: ::windows_core::IntoParam<'a, IFsrmMutableCollection>>(&self, members: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMembers)(::windows_core::Interface::as_raw(self), members.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn NonMembers(&self) -> ::windows_core::Result<IFsrmMutableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).NonMembers)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmMutableCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetNonMembers<'a, Param0: ::windows_core::IntoParam<'a, IFsrmMutableCollection>>(&self, nonmembers: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNonMembers)(::windows_core::Interface::as_raw(self), nonmembers.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileGroup> for ::windows_core::IUnknown {
    fn from(value: IFsrmFileGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileGroup> for ::windows_core::IUnknown {
    fn from(value: &IFsrmFileGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmFileGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmFileGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileGroup> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmFileGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileGroup> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmFileGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmFileGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmFileGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileGroup> for IFsrmObject {
    fn from(value: IFsrmFileGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileGroup> for IFsrmObject {
    fn from(value: &IFsrmFileGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for IFsrmFileGroup {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for &'a IFsrmFileGroup {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmFileGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmFileGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmFileGroup {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmFileGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileGroup").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmFileGroup {
    type Vtable = IFsrmFileGroup_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8dd04909_0e34_4d55_afaa_89e1f1a1bbb9);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileGroup_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Members: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, members: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Members: usize,
    #[cfg(feature = "win32-system")]
    pub SetMembers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, members: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetMembers: usize,
    #[cfg(feature = "win32-system")]
    pub NonMembers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nonmembers: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    NonMembers: usize,
    #[cfg(feature = "win32-system")]
    pub SetNonMembers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nonmembers: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetNonMembers: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmFileGroupImported(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmFileGroupImported {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, description: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Members(&self) -> ::windows_core::Result<IFsrmMutableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Members)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmMutableCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetMembers<'a, Param0: ::windows_core::IntoParam<'a, IFsrmMutableCollection>>(&self, members: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMembers)(::windows_core::Interface::as_raw(self), members.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn NonMembers(&self) -> ::windows_core::Result<IFsrmMutableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.NonMembers)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmMutableCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetNonMembers<'a, Param0: ::windows_core::IntoParam<'a, IFsrmMutableCollection>>(&self, nonmembers: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetNonMembers)(::windows_core::Interface::as_raw(self), nonmembers.into_param().abi()).ok()
    }
    pub unsafe fn OverwriteOnCommit(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).OverwriteOnCommit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetOverwriteOnCommit(&self, overwrite: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOverwriteOnCommit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(overwrite)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileGroupImported> for ::windows_core::IUnknown {
    fn from(value: IFsrmFileGroupImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileGroupImported> for ::windows_core::IUnknown {
    fn from(value: &IFsrmFileGroupImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmFileGroupImported {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmFileGroupImported {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileGroupImported> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmFileGroupImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileGroupImported> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmFileGroupImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmFileGroupImported {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmFileGroupImported {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileGroupImported> for IFsrmObject {
    fn from(value: IFsrmFileGroupImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileGroupImported> for IFsrmObject {
    fn from(value: &IFsrmFileGroupImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for IFsrmFileGroupImported {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for &'a IFsrmFileGroupImported {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileGroupImported> for IFsrmFileGroup {
    fn from(value: IFsrmFileGroupImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileGroupImported> for IFsrmFileGroup {
    fn from(value: &IFsrmFileGroupImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmFileGroup> for IFsrmFileGroupImported {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmFileGroup> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmFileGroup> for &'a IFsrmFileGroupImported {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmFileGroup> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmFileGroupImported {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmFileGroupImported {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmFileGroupImported {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmFileGroupImported {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileGroupImported").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmFileGroupImported {
    type Vtable = IFsrmFileGroupImported_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xad55f10b_5f11_4be7_94ef_d9ee2e470ded);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileGroupImported_Vtbl {
    pub base__: IFsrmFileGroup_Vtbl,
    pub OverwriteOnCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: *mut i16) -> ::windows_core::HRESULT,
    pub SetOverwriteOnCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmFileGroupManager(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmFileGroupManager {
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateFileGroup(&self) -> ::windows_core::Result<IFsrmFileGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateFileGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmFileGroup>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetFileGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<IFsrmFileGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFileGroup)(::windows_core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmFileGroup>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumFileGroups(&self, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumFileGroups)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ExportFileGroups(&self, filegroupnamesarray: *const ::win32_system::Com::VARIANT) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ExportFileGroups)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(filegroupnamesarray), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ImportFileGroups<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, serializedfilegroups: Param0, filegroupnamesarray: *const ::win32_system::Com::VARIANT) -> ::windows_core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ImportFileGroups)(::windows_core::Interface::as_raw(self), serializedfilegroups.into_param().abi(), ::core::mem::transmute(filegroupnamesarray), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCommittableCollection>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileGroupManager> for ::windows_core::IUnknown {
    fn from(value: IFsrmFileGroupManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileGroupManager> for ::windows_core::IUnknown {
    fn from(value: &IFsrmFileGroupManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmFileGroupManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmFileGroupManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileGroupManager> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmFileGroupManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileGroupManager> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmFileGroupManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmFileGroupManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmFileGroupManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmFileGroupManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmFileGroupManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmFileGroupManager {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmFileGroupManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileGroupManager").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmFileGroupManager {
    type Vtable = IFsrmFileGroupManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x426677d5_018c_485c_8a51_20b86d00bdc4);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileGroupManager_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub CreateFileGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filegroup: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateFileGroup: usize,
    #[cfg(feature = "win32-system")]
    pub GetFileGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, filegroup: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetFileGroup: usize,
    #[cfg(feature = "win32-system")]
    pub EnumFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filegroups: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    EnumFileGroups: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ExportFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filegroupnamesarray: *const ::win32_system::Com::VARIANT, serializedfilegroups: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ExportFileGroups: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ImportFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serializedfilegroups: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, filegroupnamesarray: *const ::win32_system::Com::VARIANT, filegroups: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ImportFileGroups: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmFileManagementJob(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmFileManagementJob {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, description: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn NamespaceRoots(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).NamespaceRoots)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNamespaceRoots)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(namespaceroots)).ok()
    }
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn OperationType(&self) -> ::windows_core::Result<FsrmFileManagementType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmFileManagementType>::zeroed();
        (::windows_core::Interface::vtable(self).OperationType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmFileManagementType>(result__)
    }
    pub unsafe fn SetOperationType(&self, operationtype: FsrmFileManagementType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOperationType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(operationtype)).ok()
    }
    pub unsafe fn ExpirationDirectory(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ExpirationDirectory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetExpirationDirectory<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, expirationdirectory: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetExpirationDirectory)(::windows_core::Interface::as_raw(self), expirationdirectory.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CustomAction(&self) -> ::windows_core::Result<IFsrmActionCommand> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CustomAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmActionCommand>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Notifications(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).Notifications)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn Logging(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Logging)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetLogging(&self, loggingflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLogging)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(loggingflags)).ok()
    }
    pub unsafe fn ReportEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).ReportEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetReportEnabled(&self, reportenabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetReportEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(reportenabled)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Formats(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).Formats)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetFormats(&self, formats: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFormats)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(formats)).ok()
    }
    pub unsafe fn MailTo(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).MailTo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetMailTo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, mailto: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMailTo)(::windows_core::Interface::as_raw(self), mailto.into_param().abi()).ok()
    }
    pub unsafe fn DaysSinceFileCreated(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).DaysSinceFileCreated)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDaysSinceFileCreated(&self, dayssincecreation: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDaysSinceFileCreated)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dayssincecreation)).ok()
    }
    pub unsafe fn DaysSinceFileLastAccessed(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).DaysSinceFileLastAccessed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDaysSinceFileLastAccessed(&self, dayssinceaccess: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDaysSinceFileLastAccessed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dayssinceaccess)).ok()
    }
    pub unsafe fn DaysSinceFileLastModified(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).DaysSinceFileLastModified)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDaysSinceFileLastModified(&self, dayssincemodify: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDaysSinceFileLastModified)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dayssincemodify)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn PropertyConditions(&self) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).PropertyConditions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn FromDate(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).FromDate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn SetFromDate(&self, fromdate: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFromDate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(fromdate)).ok()
    }
    pub unsafe fn Task(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Task)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, taskname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTask)(::windows_core::Interface::as_raw(self), taskname.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Parameters(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).Parameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetParameters(&self, parameters: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetParameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(parameters)).ok()
    }
    pub unsafe fn RunningStatus(&self) -> ::windows_core::Result<FsrmReportRunningStatus> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmReportRunningStatus>::zeroed();
        (::windows_core::Interface::vtable(self).RunningStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmReportRunningStatus>(result__)
    }
    pub unsafe fn LastError(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).LastError)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn LastReportPathWithoutExtension(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).LastReportPathWithoutExtension)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn LastRun(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).LastRun)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn FileNamePattern(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).FileNamePattern)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetFileNamePattern<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, filenamepattern: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFileNamePattern)(::windows_core::Interface::as_raw(self), filenamepattern.into_param().abi()).ok()
    }
    pub unsafe fn Run(&self, context: FsrmReportGenerationContext) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Run)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(context)).ok()
    }
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).WaitForCompletion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(waitseconds), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddNotification(&self, days: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddNotification)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(days)).ok()
    }
    pub unsafe fn DeleteNotification(&self, days: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteNotification)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(days)).ok()
    }
    pub unsafe fn ModifyNotification(&self, days: i32, newdays: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ModifyNotification)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(days), ::core::mem::transmute(newdays)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateNotificationAction(&self, days: i32, actiontype: FsrmActionType) -> ::windows_core::Result<IFsrmAction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateNotificationAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(days), ::core::mem::transmute(actiontype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmAction>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumNotificationActions(&self, days: i32) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumNotificationActions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(days), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreatePropertyCondition<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<IFsrmPropertyCondition> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreatePropertyCondition)(::windows_core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmPropertyCondition>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateCustomAction(&self) -> ::windows_core::Result<IFsrmActionCommand> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateCustomAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmActionCommand>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileManagementJob> for ::windows_core::IUnknown {
    fn from(value: IFsrmFileManagementJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileManagementJob> for ::windows_core::IUnknown {
    fn from(value: &IFsrmFileManagementJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmFileManagementJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmFileManagementJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileManagementJob> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmFileManagementJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileManagementJob> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmFileManagementJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmFileManagementJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmFileManagementJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileManagementJob> for IFsrmObject {
    fn from(value: IFsrmFileManagementJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileManagementJob> for IFsrmObject {
    fn from(value: &IFsrmFileManagementJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for IFsrmFileManagementJob {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for &'a IFsrmFileManagementJob {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmFileManagementJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmFileManagementJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmFileManagementJob {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmFileManagementJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileManagementJob").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmFileManagementJob {
    type Vtable = IFsrmFileManagementJob_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0770687e_9f36_4d6f_8778_599d188461c9);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileManagementJob_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub NamespaceRoots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    NamespaceRoots: usize,
    #[cfg(feature = "win32-system")]
    pub SetNamespaceRoots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetNamespaceRoots: usize,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows_core::HRESULT,
    pub OperationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operationtype: *mut FsrmFileManagementType) -> ::windows_core::HRESULT,
    pub SetOperationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operationtype: FsrmFileManagementType) -> ::windows_core::HRESULT,
    pub ExpirationDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, expirationdirectory: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetExpirationDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, expirationdirectory: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub CustomAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CustomAction: usize,
    #[cfg(feature = "win32-system")]
    pub Notifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notifications: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Notifications: usize,
    pub Logging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingflags: *mut i32) -> ::windows_core::HRESULT,
    pub SetLogging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingflags: i32) -> ::windows_core::HRESULT,
    pub ReportEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportenabled: *mut i16) -> ::windows_core::HRESULT,
    pub SetReportEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportenabled: i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Formats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Formats: usize,
    #[cfg(feature = "win32-system")]
    pub SetFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetFormats: usize,
    pub MailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetMailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub DaysSinceFileCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dayssincecreation: *mut i32) -> ::windows_core::HRESULT,
    pub SetDaysSinceFileCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dayssincecreation: i32) -> ::windows_core::HRESULT,
    pub DaysSinceFileLastAccessed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dayssinceaccess: *mut i32) -> ::windows_core::HRESULT,
    pub SetDaysSinceFileLastAccessed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dayssinceaccess: i32) -> ::windows_core::HRESULT,
    pub DaysSinceFileLastModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dayssincemodify: *mut i32) -> ::windows_core::HRESULT,
    pub SetDaysSinceFileLastModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dayssincemodify: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub PropertyConditions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyconditions: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    PropertyConditions: usize,
    pub FromDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fromdate: *mut f64) -> ::windows_core::HRESULT,
    pub SetFromDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fromdate: f64) -> ::windows_core::HRESULT,
    pub Task: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Parameters: usize,
    #[cfg(feature = "win32-system")]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetParameters: usize,
    pub RunningStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> ::windows_core::HRESULT,
    pub LastError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lasterror: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub LastReportPathWithoutExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub LastRun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastrun: *mut f64) -> ::windows_core::HRESULT,
    pub FileNamePattern: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filenamepattern: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetFileNamePattern: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filenamepattern: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Run: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext) -> ::windows_core::HRESULT,
    pub WaitForCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut i16) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i32) -> ::windows_core::HRESULT,
    pub DeleteNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i32) -> ::windows_core::HRESULT,
    pub ModifyNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i32, newdays: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub CreateNotificationAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i32, actiontype: FsrmActionType, action: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateNotificationAction: usize,
    #[cfg(feature = "win32-system")]
    pub EnumNotificationActions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i32, actions: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    EnumNotificationActions: usize,
    #[cfg(feature = "win32-system")]
    pub CreatePropertyCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, propertycondition: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreatePropertyCondition: usize,
    #[cfg(feature = "win32-system")]
    pub CreateCustomAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customaction: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateCustomAction: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmFileManagementJobManager(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmFileManagementJobManager {
    #[cfg(feature = "win32-system")]
    pub unsafe fn ActionVariables(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).ActionVariables)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ActionVariableDescriptions(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).ActionVariableDescriptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumFileManagementJobs(&self, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumFileManagementJobs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateFileManagementJob(&self) -> ::windows_core::Result<IFsrmFileManagementJob> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateFileManagementJob)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmFileManagementJob>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetFileManagementJob<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<IFsrmFileManagementJob> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFileManagementJob)(::windows_core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmFileManagementJob>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileManagementJobManager> for ::windows_core::IUnknown {
    fn from(value: IFsrmFileManagementJobManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileManagementJobManager> for ::windows_core::IUnknown {
    fn from(value: &IFsrmFileManagementJobManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmFileManagementJobManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmFileManagementJobManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileManagementJobManager> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmFileManagementJobManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileManagementJobManager> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmFileManagementJobManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmFileManagementJobManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmFileManagementJobManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmFileManagementJobManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmFileManagementJobManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmFileManagementJobManager {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmFileManagementJobManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileManagementJobManager").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmFileManagementJobManager {
    type Vtable = IFsrmFileManagementJobManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee321ecb_d95e_48e9_907c_c7685a013235);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileManagementJobManager_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub ActionVariables: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variables: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ActionVariables: usize,
    #[cfg(feature = "win32-system")]
    pub ActionVariableDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptions: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ActionVariableDescriptions: usize,
    #[cfg(feature = "win32-system")]
    pub EnumFileManagementJobs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filemanagementjobs: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    EnumFileManagementJobs: usize,
    #[cfg(feature = "win32-system")]
    pub CreateFileManagementJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filemanagementjob: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateFileManagementJob: usize,
    #[cfg(feature = "win32-system")]
    pub GetFileManagementJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, filemanagementjob: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetFileManagementJob: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmFileScreen(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmFileScreen {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, description: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn BlockedFileGroups(&self) -> ::windows_core::Result<IFsrmMutableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.BlockedFileGroups)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmMutableCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetBlockedFileGroups<'a, Param0: ::windows_core::IntoParam<'a, IFsrmMutableCollection>>(&self, blocklist: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetBlockedFileGroups)(::windows_core::Interface::as_raw(self), blocklist.into_param().abi()).ok()
    }
    pub unsafe fn FileScreenFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.FileScreenFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetFileScreenFlags(&self, filescreenflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetFileScreenFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(filescreenflags)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateAction(&self, actiontype: FsrmActionType) -> ::windows_core::Result<IFsrmAction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(actiontype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmAction>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumActions(&self) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumActions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Path)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SourceTemplateName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).SourceTemplateName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn MatchesSourceTemplate(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).MatchesSourceTemplate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn UserSid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).UserSid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserAccount(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).UserAccount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ApplyTemplate<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, filescreentemplatename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ApplyTemplate)(::windows_core::Interface::as_raw(self), filescreentemplatename.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileScreen> for ::windows_core::IUnknown {
    fn from(value: IFsrmFileScreen) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileScreen> for ::windows_core::IUnknown {
    fn from(value: &IFsrmFileScreen) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmFileScreen {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmFileScreen {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileScreen> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmFileScreen) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileScreen> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmFileScreen) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmFileScreen {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmFileScreen {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileScreen> for IFsrmObject {
    fn from(value: IFsrmFileScreen) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileScreen> for IFsrmObject {
    fn from(value: &IFsrmFileScreen) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for IFsrmFileScreen {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for &'a IFsrmFileScreen {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileScreen> for IFsrmFileScreenBase {
    fn from(value: IFsrmFileScreen) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileScreen> for IFsrmFileScreenBase {
    fn from(value: &IFsrmFileScreen) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmFileScreenBase> for IFsrmFileScreen {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmFileScreenBase> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmFileScreenBase> for &'a IFsrmFileScreen {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmFileScreenBase> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmFileScreen {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmFileScreen {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmFileScreen {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmFileScreen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileScreen").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmFileScreen {
    type Vtable = IFsrmFileScreen_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f6325d3_ce88_4733_84c1_2d6aefc5ea07);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreen_Vtbl {
    pub base__: IFsrmFileScreenBase_Vtbl,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SourceTemplateName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreentemplatename: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub MatchesSourceTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matches: *mut i16) -> ::windows_core::HRESULT,
    pub UserSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub UserAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccount: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ApplyTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreentemplatename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmFileScreenBase(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmFileScreenBase {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, description: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn BlockedFileGroups(&self) -> ::windows_core::Result<IFsrmMutableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).BlockedFileGroups)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmMutableCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetBlockedFileGroups<'a, Param0: ::windows_core::IntoParam<'a, IFsrmMutableCollection>>(&self, blocklist: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBlockedFileGroups)(::windows_core::Interface::as_raw(self), blocklist.into_param().abi()).ok()
    }
    pub unsafe fn FileScreenFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).FileScreenFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetFileScreenFlags(&self, filescreenflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFileScreenFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(filescreenflags)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateAction(&self, actiontype: FsrmActionType) -> ::windows_core::Result<IFsrmAction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(actiontype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmAction>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumActions(&self) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumActions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileScreenBase> for ::windows_core::IUnknown {
    fn from(value: IFsrmFileScreenBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileScreenBase> for ::windows_core::IUnknown {
    fn from(value: &IFsrmFileScreenBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmFileScreenBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmFileScreenBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileScreenBase> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmFileScreenBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileScreenBase> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmFileScreenBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmFileScreenBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmFileScreenBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileScreenBase> for IFsrmObject {
    fn from(value: IFsrmFileScreenBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileScreenBase> for IFsrmObject {
    fn from(value: &IFsrmFileScreenBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for IFsrmFileScreenBase {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for &'a IFsrmFileScreenBase {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmFileScreenBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmFileScreenBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmFileScreenBase {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmFileScreenBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileScreenBase").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmFileScreenBase {
    type Vtable = IFsrmFileScreenBase_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf3637e80_5b22_4a2b_a637_bbb642b41cfc);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenBase_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    #[cfg(feature = "win32-system")]
    pub BlockedFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blocklist: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    BlockedFileGroups: usize,
    #[cfg(feature = "win32-system")]
    pub SetBlockedFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blocklist: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetBlockedFileGroups: usize,
    pub FileScreenFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreenflags: *mut i32) -> ::windows_core::HRESULT,
    pub SetFileScreenFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreenflags: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub CreateAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, action: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateAction: usize,
    #[cfg(feature = "win32-system")]
    pub EnumActions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actions: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    EnumActions: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmFileScreenException(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmFileScreenException {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, description: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Path)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn AllowedFileGroups(&self) -> ::windows_core::Result<IFsrmMutableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AllowedFileGroups)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmMutableCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetAllowedFileGroups<'a, Param0: ::windows_core::IntoParam<'a, IFsrmMutableCollection>>(&self, allowlist: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllowedFileGroups)(::windows_core::Interface::as_raw(self), allowlist.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileScreenException> for ::windows_core::IUnknown {
    fn from(value: IFsrmFileScreenException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileScreenException> for ::windows_core::IUnknown {
    fn from(value: &IFsrmFileScreenException) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmFileScreenException {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmFileScreenException {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileScreenException> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmFileScreenException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileScreenException> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmFileScreenException) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmFileScreenException {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmFileScreenException {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileScreenException> for IFsrmObject {
    fn from(value: IFsrmFileScreenException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileScreenException> for IFsrmObject {
    fn from(value: &IFsrmFileScreenException) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for IFsrmFileScreenException {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for &'a IFsrmFileScreenException {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmFileScreenException {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmFileScreenException {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmFileScreenException {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmFileScreenException {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileScreenException").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmFileScreenException {
    type Vtable = IFsrmFileScreenException_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbee7ce02_df77_4515_9389_78f01c5afc1a);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenException_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub AllowedFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allowlist: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    AllowedFileGroups: usize,
    #[cfg(feature = "win32-system")]
    pub SetAllowedFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allowlist: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetAllowedFileGroups: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmFileScreenManager(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmFileScreenManager {
    #[cfg(feature = "win32-system")]
    pub unsafe fn ActionVariables(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).ActionVariables)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ActionVariableDescriptions(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).ActionVariableDescriptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateFileScreen<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0) -> ::windows_core::Result<IFsrmFileScreen> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateFileScreen)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmFileScreen>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetFileScreen<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0) -> ::windows_core::Result<IFsrmFileScreen> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFileScreen)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmFileScreen>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumFileScreens<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumFileScreens)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateFileScreenException<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0) -> ::windows_core::Result<IFsrmFileScreenException> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateFileScreenException)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmFileScreenException>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetFileScreenException<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0) -> ::windows_core::Result<IFsrmFileScreenException> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFileScreenException)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmFileScreenException>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumFileScreenExceptions<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumFileScreenExceptions)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateFileScreenCollection(&self) -> ::windows_core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateFileScreenCollection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCommittableCollection>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileScreenManager> for ::windows_core::IUnknown {
    fn from(value: IFsrmFileScreenManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileScreenManager> for ::windows_core::IUnknown {
    fn from(value: &IFsrmFileScreenManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmFileScreenManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmFileScreenManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileScreenManager> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmFileScreenManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileScreenManager> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmFileScreenManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmFileScreenManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmFileScreenManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmFileScreenManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmFileScreenManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmFileScreenManager {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmFileScreenManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileScreenManager").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmFileScreenManager {
    type Vtable = IFsrmFileScreenManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xff4fa04e_5a94_4bda_a3a0_d5b4d3c52eba);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenManager_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub ActionVariables: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variables: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ActionVariables: usize,
    #[cfg(feature = "win32-system")]
    pub ActionVariableDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptions: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ActionVariableDescriptions: usize,
    #[cfg(feature = "win32-system")]
    pub CreateFileScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, filescreen: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateFileScreen: usize,
    #[cfg(feature = "win32-system")]
    pub GetFileScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, filescreen: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetFileScreen: usize,
    #[cfg(feature = "win32-system")]
    pub EnumFileScreens: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, options: FsrmEnumOptions, filescreens: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    EnumFileScreens: usize,
    #[cfg(feature = "win32-system")]
    pub CreateFileScreenException: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, filescreenexception: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateFileScreenException: usize,
    #[cfg(feature = "win32-system")]
    pub GetFileScreenException: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, filescreenexception: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetFileScreenException: usize,
    #[cfg(feature = "win32-system")]
    pub EnumFileScreenExceptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, options: FsrmEnumOptions, filescreenexceptions: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    EnumFileScreenExceptions: usize,
    #[cfg(feature = "win32-system")]
    pub CreateFileScreenCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateFileScreenCollection: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmFileScreenTemplate(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmFileScreenTemplate {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, description: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn BlockedFileGroups(&self) -> ::windows_core::Result<IFsrmMutableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.BlockedFileGroups)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmMutableCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetBlockedFileGroups<'a, Param0: ::windows_core::IntoParam<'a, IFsrmMutableCollection>>(&self, blocklist: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetBlockedFileGroups)(::windows_core::Interface::as_raw(self), blocklist.into_param().abi()).ok()
    }
    pub unsafe fn FileScreenFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.FileScreenFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetFileScreenFlags(&self, filescreenflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetFileScreenFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(filescreenflags)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateAction(&self, actiontype: FsrmActionType) -> ::windows_core::Result<IFsrmAction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(actiontype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmAction>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumActions(&self) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumActions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn CopyTemplate<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, filescreentemplatename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CopyTemplate)(::windows_core::Interface::as_raw(self), filescreentemplatename.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows_core::Result<IFsrmDerivedObjectsResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CommitAndUpdateDerived)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(commitoptions), ::core::mem::transmute(applyoptions), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmDerivedObjectsResult>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileScreenTemplate> for ::windows_core::IUnknown {
    fn from(value: IFsrmFileScreenTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileScreenTemplate> for ::windows_core::IUnknown {
    fn from(value: &IFsrmFileScreenTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileScreenTemplate> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmFileScreenTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileScreenTemplate> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmFileScreenTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileScreenTemplate> for IFsrmObject {
    fn from(value: IFsrmFileScreenTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileScreenTemplate> for IFsrmObject {
    fn from(value: &IFsrmFileScreenTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for &'a IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileScreenTemplate> for IFsrmFileScreenBase {
    fn from(value: IFsrmFileScreenTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileScreenTemplate> for IFsrmFileScreenBase {
    fn from(value: &IFsrmFileScreenTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmFileScreenBase> for IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmFileScreenBase> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmFileScreenBase> for &'a IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmFileScreenBase> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmFileScreenTemplate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmFileScreenTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmFileScreenTemplate {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmFileScreenTemplate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileScreenTemplate").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmFileScreenTemplate {
    type Vtable = IFsrmFileScreenTemplate_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x205bebf8_dd93_452a_95a6_32b566b35828);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenTemplate_Vtbl {
    pub base__: IFsrmFileScreenBase_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub CopyTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreentemplatename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub CommitAndUpdateDerived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CommitAndUpdateDerived: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmFileScreenTemplateImported(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmFileScreenTemplateImported {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, description: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn BlockedFileGroups(&self) -> ::windows_core::Result<IFsrmMutableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.BlockedFileGroups)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmMutableCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetBlockedFileGroups<'a, Param0: ::windows_core::IntoParam<'a, IFsrmMutableCollection>>(&self, blocklist: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetBlockedFileGroups)(::windows_core::Interface::as_raw(self), blocklist.into_param().abi()).ok()
    }
    pub unsafe fn FileScreenFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.FileScreenFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetFileScreenFlags(&self, filescreenflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetFileScreenFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(filescreenflags)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateAction(&self, actiontype: FsrmActionType) -> ::windows_core::Result<IFsrmAction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(actiontype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmAction>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumActions(&self) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EnumActions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn CopyTemplate<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, filescreentemplatename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CopyTemplate)(::windows_core::Interface::as_raw(self), filescreentemplatename.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows_core::Result<IFsrmDerivedObjectsResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CommitAndUpdateDerived)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(commitoptions), ::core::mem::transmute(applyoptions), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmDerivedObjectsResult>(result__)
    }
    pub unsafe fn OverwriteOnCommit(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).OverwriteOnCommit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetOverwriteOnCommit(&self, overwrite: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOverwriteOnCommit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(overwrite)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileScreenTemplateImported> for ::windows_core::IUnknown {
    fn from(value: IFsrmFileScreenTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileScreenTemplateImported> for ::windows_core::IUnknown {
    fn from(value: &IFsrmFileScreenTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileScreenTemplateImported> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmFileScreenTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileScreenTemplateImported> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmFileScreenTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileScreenTemplateImported> for IFsrmObject {
    fn from(value: IFsrmFileScreenTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileScreenTemplateImported> for IFsrmObject {
    fn from(value: &IFsrmFileScreenTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for &'a IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileScreenTemplateImported> for IFsrmFileScreenBase {
    fn from(value: IFsrmFileScreenTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileScreenTemplateImported> for IFsrmFileScreenBase {
    fn from(value: &IFsrmFileScreenTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmFileScreenBase> for IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmFileScreenBase> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmFileScreenBase> for &'a IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmFileScreenBase> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileScreenTemplateImported> for IFsrmFileScreenTemplate {
    fn from(value: IFsrmFileScreenTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileScreenTemplateImported> for IFsrmFileScreenTemplate {
    fn from(value: &IFsrmFileScreenTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmFileScreenTemplate> for IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmFileScreenTemplate> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmFileScreenTemplate> for &'a IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmFileScreenTemplate> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmFileScreenTemplateImported {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmFileScreenTemplateImported {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmFileScreenTemplateImported {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmFileScreenTemplateImported {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileScreenTemplateImported").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmFileScreenTemplateImported {
    type Vtable = IFsrmFileScreenTemplateImported_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe1010359_3e5d_4ecd_9fe4_ef48622fdf30);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenTemplateImported_Vtbl {
    pub base__: IFsrmFileScreenTemplate_Vtbl,
    pub OverwriteOnCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: *mut i16) -> ::windows_core::HRESULT,
    pub SetOverwriteOnCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmFileScreenTemplateManager(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmFileScreenTemplateManager {
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateTemplate(&self) -> ::windows_core::Result<IFsrmFileScreenTemplate> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateTemplate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmFileScreenTemplate>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetTemplate<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<IFsrmFileScreenTemplate> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetTemplate)(::windows_core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmFileScreenTemplate>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumTemplates(&self, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumTemplates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ExportTemplates(&self, filescreentemplatenamesarray: *const ::win32_system::Com::VARIANT) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ExportTemplates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(filescreentemplatenamesarray), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ImportTemplates<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, serializedfilescreentemplates: Param0, filescreentemplatenamesarray: *const ::win32_system::Com::VARIANT) -> ::windows_core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ImportTemplates)(::windows_core::Interface::as_raw(self), serializedfilescreentemplates.into_param().abi(), ::core::mem::transmute(filescreentemplatenamesarray), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCommittableCollection>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileScreenTemplateManager> for ::windows_core::IUnknown {
    fn from(value: IFsrmFileScreenTemplateManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileScreenTemplateManager> for ::windows_core::IUnknown {
    fn from(value: &IFsrmFileScreenTemplateManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmFileScreenTemplateManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmFileScreenTemplateManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmFileScreenTemplateManager> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmFileScreenTemplateManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmFileScreenTemplateManager> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmFileScreenTemplateManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmFileScreenTemplateManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmFileScreenTemplateManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmFileScreenTemplateManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmFileScreenTemplateManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmFileScreenTemplateManager {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmFileScreenTemplateManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileScreenTemplateManager").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmFileScreenTemplateManager {
    type Vtable = IFsrmFileScreenTemplateManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcfe36cba_1949_4e74_a14f_f1d580ceaf13);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenTemplateManager_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub CreateTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreentemplate: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateTemplate: usize,
    #[cfg(feature = "win32-system")]
    pub GetTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, filescreentemplate: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetTemplate: usize,
    #[cfg(feature = "win32-system")]
    pub EnumTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filescreentemplates: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    EnumTemplates: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ExportTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreentemplatenamesarray: *const ::win32_system::Com::VARIANT, serializedfilescreentemplates: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ExportTemplates: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ImportTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serializedfilescreentemplates: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, filescreentemplatenamesarray: *const ::win32_system::Com::VARIANT, filescreentemplates: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ImportTemplates: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmMutableCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmMutableCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).base__._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn State(&self) -> ::windows_core::Result<FsrmCollectionState> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmCollectionState>::zeroed();
        (::windows_core::Interface::vtable(self).base__.State)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmCollectionState>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.WaitForCompletion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(waitseconds), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GetById<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, id: Param0) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetById)(::windows_core::Interface::as_raw(self), id.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Add<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, item: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), item.into_param().abi()).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index)).ok()
    }
    pub unsafe fn RemoveById<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, id: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveById)(::windows_core::Interface::as_raw(self), id.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IFsrmMutableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmMutableCollection>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmMutableCollection> for ::windows_core::IUnknown {
    fn from(value: IFsrmMutableCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmMutableCollection> for ::windows_core::IUnknown {
    fn from(value: &IFsrmMutableCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmMutableCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmMutableCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmMutableCollection> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmMutableCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmMutableCollection> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmMutableCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmMutableCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmMutableCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmMutableCollection> for IFsrmCollection {
    fn from(value: IFsrmMutableCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmMutableCollection> for IFsrmCollection {
    fn from(value: &IFsrmMutableCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmCollection> for IFsrmMutableCollection {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmCollection> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmCollection> for &'a IFsrmMutableCollection {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmCollection> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmMutableCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmMutableCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmMutableCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmMutableCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmMutableCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmMutableCollection {
    type Vtable = IFsrmMutableCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1bb617b8_3886_49dc_af82_a6c90fa35dda);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmMutableCollection_Vtbl {
    pub base__: IFsrmCollection_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32) -> ::windows_core::HRESULT,
    pub RemoveById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Clone: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmObject(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmObject {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, description: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmObject> for ::windows_core::IUnknown {
    fn from(value: IFsrmObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmObject> for ::windows_core::IUnknown {
    fn from(value: &IFsrmObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmObject> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmObject> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmObject {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmObject").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmObject {
    type Vtable = IFsrmObject_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x22bcef93_4a3f_4183_89f9_2f8b8a628aee);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmObject_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmPathMapper(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmPathMapper {
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetSharePathsForLocalPath<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, localpath: Param0) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).GetSharePathsForLocalPath)(::windows_core::Interface::as_raw(self), localpath.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmPathMapper> for ::windows_core::IUnknown {
    fn from(value: IFsrmPathMapper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmPathMapper> for ::windows_core::IUnknown {
    fn from(value: &IFsrmPathMapper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmPathMapper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmPathMapper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmPathMapper> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmPathMapper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmPathMapper> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmPathMapper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmPathMapper {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmPathMapper {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmPathMapper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmPathMapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmPathMapper {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmPathMapper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPathMapper").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmPathMapper {
    type Vtable = IFsrmPathMapper_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f4dbfff_6920_4821_a6c3_b7e94c1fd60c);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPathMapper_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub GetSharePathsForLocalPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, sharepaths: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetSharePathsForLocalPath: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmPipelineModuleConnector(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmPipelineModuleConnector {
    #[cfg(feature = "win32-system")]
    pub unsafe fn ModuleImplementation(&self) -> ::windows_core::Result<IFsrmPipelineModuleImplementation> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ModuleImplementation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmPipelineModuleImplementation>(result__)
    }
    pub unsafe fn ModuleName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ModuleName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn HostingUserAccount(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).HostingUserAccount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn HostingProcessPid(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).HostingProcessPid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Bind<'a, Param0: ::windows_core::IntoParam<'a, IFsrmPipelineModuleDefinition>, Param1: ::windows_core::IntoParam<'a, IFsrmPipelineModuleImplementation>>(&self, moduledefinition: Param0, moduleimplementation: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Bind)(::windows_core::Interface::as_raw(self), moduledefinition.into_param().abi(), moduleimplementation.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmPipelineModuleConnector> for ::windows_core::IUnknown {
    fn from(value: IFsrmPipelineModuleConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmPipelineModuleConnector> for ::windows_core::IUnknown {
    fn from(value: &IFsrmPipelineModuleConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmPipelineModuleConnector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmPipelineModuleConnector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmPipelineModuleConnector> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmPipelineModuleConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmPipelineModuleConnector> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmPipelineModuleConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmPipelineModuleConnector {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmPipelineModuleConnector {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmPipelineModuleConnector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmPipelineModuleConnector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmPipelineModuleConnector {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmPipelineModuleConnector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPipelineModuleConnector").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmPipelineModuleConnector {
    type Vtable = IFsrmPipelineModuleConnector_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc16014f3_9aa1_46b3_b0a7_ab146eb205f2);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPipelineModuleConnector_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub ModuleImplementation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipelinemoduleimplementation: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ModuleImplementation: usize,
    pub ModuleName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub HostingUserAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccount: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub HostingProcessPid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Bind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduledefinition: ::windows_core::RawPtr, moduleimplementation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Bind: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmPipelineModuleDefinition(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmPipelineModuleDefinition {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, description: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ModuleClsid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ModuleClsid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetModuleClsid<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, moduleclsid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetModuleClsid)(::windows_core::Interface::as_raw(self), moduleclsid.into_param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Company(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Company)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetCompany<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, company: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCompany)(::windows_core::Interface::as_raw(self), company.into_param().abi()).ok()
    }
    pub unsafe fn Version(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Version)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetVersion<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, version: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetVersion)(::windows_core::Interface::as_raw(self), version.into_param().abi()).ok()
    }
    pub unsafe fn ModuleType(&self) -> ::windows_core::Result<FsrmPipelineModuleType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmPipelineModuleType>::zeroed();
        (::windows_core::Interface::vtable(self).ModuleType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmPipelineModuleType>(result__)
    }
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn NeedsFileContent(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).NeedsFileContent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetNeedsFileContent(&self, needsfilecontent: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNeedsFileContent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(needsfilecontent)).ok()
    }
    pub unsafe fn Account(&self) -> ::windows_core::Result<FsrmAccountType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmAccountType>::zeroed();
        (::windows_core::Interface::vtable(self).Account)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmAccountType>(result__)
    }
    pub unsafe fn SetAccount(&self, retrievalaccount: FsrmAccountType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAccount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(retrievalaccount)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SupportedExtensions(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).SupportedExtensions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetSupportedExtensions(&self, supportedextensions: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSupportedExtensions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(supportedextensions)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Parameters(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).Parameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetParameters(&self, parameters: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetParameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(parameters)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmPipelineModuleDefinition> for ::windows_core::IUnknown {
    fn from(value: IFsrmPipelineModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmPipelineModuleDefinition> for ::windows_core::IUnknown {
    fn from(value: &IFsrmPipelineModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmPipelineModuleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmPipelineModuleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmPipelineModuleDefinition> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmPipelineModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmPipelineModuleDefinition> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmPipelineModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmPipelineModuleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmPipelineModuleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmPipelineModuleDefinition> for IFsrmObject {
    fn from(value: IFsrmPipelineModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmPipelineModuleDefinition> for IFsrmObject {
    fn from(value: &IFsrmPipelineModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for IFsrmPipelineModuleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for &'a IFsrmPipelineModuleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmPipelineModuleDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmPipelineModuleDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmPipelineModuleDefinition {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmPipelineModuleDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPipelineModuleDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmPipelineModuleDefinition {
    type Vtable = IFsrmPipelineModuleDefinition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x515c1277_2c81_440e_8fcf_367921ed4f59);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPipelineModuleDefinition_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub ModuleClsid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleclsid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetModuleClsid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleclsid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Company: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, company: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetCompany: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, company: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ModuleType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduletype: *mut FsrmPipelineModuleType) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows_core::HRESULT,
    pub NeedsFileContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, needsfilecontent: *mut i16) -> ::windows_core::HRESULT,
    pub SetNeedsFileContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, needsfilecontent: i16) -> ::windows_core::HRESULT,
    pub Account: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retrievalaccount: *mut FsrmAccountType) -> ::windows_core::HRESULT,
    pub SetAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retrievalaccount: FsrmAccountType) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub SupportedExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedextensions: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SupportedExtensions: usize,
    #[cfg(feature = "win32-system")]
    pub SetSupportedExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedextensions: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetSupportedExtensions: usize,
    #[cfg(feature = "win32-system")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Parameters: usize,
    #[cfg(feature = "win32-system")]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetParameters: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmPipelineModuleImplementation(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmPipelineModuleImplementation {
    #[cfg(feature = "win32-system")]
    pub unsafe fn OnLoad<'a, Param0: ::windows_core::IntoParam<'a, IFsrmPipelineModuleDefinition>>(&self, moduledefinition: Param0) -> ::windows_core::Result<IFsrmPipelineModuleConnector> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).OnLoad)(::windows_core::Interface::as_raw(self), moduledefinition.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmPipelineModuleConnector>(result__)
    }
    pub unsafe fn OnUnload(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnUnload)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmPipelineModuleImplementation> for ::windows_core::IUnknown {
    fn from(value: IFsrmPipelineModuleImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmPipelineModuleImplementation> for ::windows_core::IUnknown {
    fn from(value: &IFsrmPipelineModuleImplementation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmPipelineModuleImplementation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmPipelineModuleImplementation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmPipelineModuleImplementation> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmPipelineModuleImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmPipelineModuleImplementation> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmPipelineModuleImplementation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmPipelineModuleImplementation {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmPipelineModuleImplementation {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmPipelineModuleImplementation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmPipelineModuleImplementation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmPipelineModuleImplementation {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmPipelineModuleImplementation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPipelineModuleImplementation").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmPipelineModuleImplementation {
    type Vtable = IFsrmPipelineModuleImplementation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7907906_2b02_4cb5_84a9_fdf54613d6cd);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPipelineModuleImplementation_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub OnLoad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduledefinition: ::windows_core::RawPtr, moduleconnector: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    OnLoad: usize,
    pub OnUnload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmProperty(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmProperty {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Value(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Value)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Sources(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).Sources)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn PropertyFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).PropertyFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmProperty> for ::windows_core::IUnknown {
    fn from(value: IFsrmProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmProperty> for ::windows_core::IUnknown {
    fn from(value: &IFsrmProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmProperty> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmProperty> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmProperty {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmProperty").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmProperty {
    type Vtable = IFsrmProperty_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a73fee4_4102_4fcc_9ffb_38614f9ee768);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmProperty_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Sources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sources: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Sources: usize,
    pub PropertyFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmPropertyBag(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmPropertyBag {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn RelativePath(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).RelativePath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn VolumeName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).VolumeName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn RelativeNamespaceRoot(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).RelativeNamespaceRoot)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn VolumeIndex(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).VolumeIndex)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn FileId(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).FileId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ParentDirectoryId(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).ParentDirectoryId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Size(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Size)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SizeAllocated(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).SizeAllocated)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn CreationTime(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).CreationTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn LastAccessTime(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).LastAccessTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn LastModificationTime(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).LastModificationTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn Attributes(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).Attributes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn OwnerSid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).OwnerSid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn FilePropertyNames(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).FilePropertyNames)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Messages(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).Messages)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn PropertyBagFlags(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).PropertyBagFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetFileProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<IFsrmProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetFileProperty)(::windows_core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmProperty>(result__)
    }
    pub unsafe fn SetFileProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0, value: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFileProperty)(::windows_core::Interface::as_raw(self), name.into_param().abi(), value.into_param().abi()).ok()
    }
    pub unsafe fn AddMessage<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, message: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddMessage)(::windows_core::Interface::as_raw(self), message.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GetFileStreamInterface(&self, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetFileStreamInterface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(accessmode), ::core::mem::transmute(interfacetype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmPropertyBag> for ::windows_core::IUnknown {
    fn from(value: IFsrmPropertyBag) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmPropertyBag> for ::windows_core::IUnknown {
    fn from(value: &IFsrmPropertyBag) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmPropertyBag {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmPropertyBag {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmPropertyBag> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmPropertyBag) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmPropertyBag> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmPropertyBag) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmPropertyBag {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmPropertyBag {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmPropertyBag {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmPropertyBag {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmPropertyBag {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmPropertyBag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPropertyBag").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmPropertyBag {
    type Vtable = IFsrmPropertyBag_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x774589d1_d300_4f7a_9a24_f7b766800250);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyBag_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub RelativePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub VolumeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volumename: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub RelativeNamespaceRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativenamespaceroot: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub VolumeIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volumeid: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub FileId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fileid: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    FileId: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ParentDirectoryId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentdirectoryid: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ParentDirectoryId: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Size: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SizeAllocated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sizeallocated: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SizeAllocated: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub CreationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, creationtime: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    CreationTime: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub LastAccessTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastaccesstime: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    LastAccessTime: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub LastModificationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastmodificationtime: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    LastModificationTime: usize,
    pub Attributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: *mut u32) -> ::windows_core::HRESULT,
    pub OwnerSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ownersid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub FilePropertyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepropertynames: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    FilePropertyNames: usize,
    #[cfg(feature = "win32-system")]
    pub Messages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messages: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Messages: usize,
    pub PropertyBagFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub GetFileProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, fileproperty: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetFileProperty: usize,
    pub SetFileProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, value: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub AddMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub GetFileStreamInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType, pstreaminterface: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    GetFileStreamInterface: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmPropertyBag2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmPropertyBag2 {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn RelativePath(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RelativePath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn VolumeName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.VolumeName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn RelativeNamespaceRoot(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RelativeNamespaceRoot)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn VolumeIndex(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.VolumeIndex)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn FileId(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.FileId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ParentDirectoryId(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ParentDirectoryId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Size(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Size)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SizeAllocated(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SizeAllocated)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn CreationTime(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreationTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn LastAccessTime(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastAccessTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn LastModificationTime(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastModificationTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn Attributes(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Attributes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn OwnerSid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.OwnerSid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn FilePropertyNames(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.FilePropertyNames)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Messages(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Messages)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn PropertyBagFlags(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PropertyBagFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetFileProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<IFsrmProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetFileProperty)(::windows_core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmProperty>(result__)
    }
    pub unsafe fn SetFileProperty<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0, value: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetFileProperty)(::windows_core::Interface::as_raw(self), name.into_param().abi(), value.into_param().abi()).ok()
    }
    pub unsafe fn AddMessage<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, message: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddMessage)(::windows_core::Interface::as_raw(self), message.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GetFileStreamInterface(&self, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetFileStreamInterface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(accessmode), ::core::mem::transmute(interfacetype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GetFieldValue(&self, field: FsrmPropertyBagField) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetFieldValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(field), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetUntrustedInFileProperties(&self) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetUntrustedInFileProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmPropertyBag2> for ::windows_core::IUnknown {
    fn from(value: IFsrmPropertyBag2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmPropertyBag2> for ::windows_core::IUnknown {
    fn from(value: &IFsrmPropertyBag2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmPropertyBag2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmPropertyBag2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmPropertyBag2> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmPropertyBag2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmPropertyBag2> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmPropertyBag2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmPropertyBag2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmPropertyBag2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmPropertyBag2> for IFsrmPropertyBag {
    fn from(value: IFsrmPropertyBag2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmPropertyBag2> for IFsrmPropertyBag {
    fn from(value: &IFsrmPropertyBag2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmPropertyBag> for IFsrmPropertyBag2 {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmPropertyBag> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmPropertyBag> for &'a IFsrmPropertyBag2 {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmPropertyBag> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmPropertyBag2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmPropertyBag2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmPropertyBag2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmPropertyBag2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPropertyBag2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmPropertyBag2 {
    type Vtable = IFsrmPropertyBag2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e46bdbd_2402_4fed_9c30_9266e6eb2cc9);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyBag2_Vtbl {
    pub base__: IFsrmPropertyBag_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub GetFieldValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, field: FsrmPropertyBagField, value: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    GetFieldValue: usize,
    #[cfg(feature = "win32-system")]
    pub GetUntrustedInFileProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, props: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetUntrustedInFileProperties: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmPropertyCondition(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmPropertyCondition {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Type(&self) -> ::windows_core::Result<FsrmPropertyConditionType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmPropertyConditionType>::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmPropertyConditionType>(result__)
    }
    pub unsafe fn SetType(&self, r#type: FsrmPropertyConditionType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(r#type)).ok()
    }
    pub unsafe fn Value(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Value)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetValue<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValue)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmPropertyCondition> for ::windows_core::IUnknown {
    fn from(value: IFsrmPropertyCondition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmPropertyCondition> for ::windows_core::IUnknown {
    fn from(value: &IFsrmPropertyCondition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmPropertyCondition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmPropertyCondition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmPropertyCondition> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmPropertyCondition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmPropertyCondition> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmPropertyCondition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmPropertyCondition {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmPropertyCondition {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmPropertyCondition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmPropertyCondition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmPropertyCondition {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmPropertyCondition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPropertyCondition").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmPropertyCondition {
    type Vtable = IFsrmPropertyCondition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x326af66f_2ac0_4f68_bf8c_4759f054fa29);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyCondition_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut FsrmPropertyConditionType) -> ::windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: FsrmPropertyConditionType) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmPropertyDefinition(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmPropertyDefinition {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, description: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Type(&self) -> ::windows_core::Result<FsrmPropertyDefinitionType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmPropertyDefinitionType>::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmPropertyDefinitionType>(result__)
    }
    pub unsafe fn SetType(&self, r#type: FsrmPropertyDefinitionType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(r#type)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn PossibleValues(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).PossibleValues)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetPossibleValues(&self, possiblevalues: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPossibleValues)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(possiblevalues)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ValueDescriptions(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).ValueDescriptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetValueDescriptions(&self, valuedescriptions: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValueDescriptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(valuedescriptions)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Parameters(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).Parameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetParameters(&self, parameters: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetParameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(parameters)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmPropertyDefinition> for ::windows_core::IUnknown {
    fn from(value: IFsrmPropertyDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmPropertyDefinition> for ::windows_core::IUnknown {
    fn from(value: &IFsrmPropertyDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmPropertyDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmPropertyDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmPropertyDefinition> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmPropertyDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmPropertyDefinition> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmPropertyDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmPropertyDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmPropertyDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmPropertyDefinition> for IFsrmObject {
    fn from(value: IFsrmPropertyDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmPropertyDefinition> for IFsrmObject {
    fn from(value: &IFsrmPropertyDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for IFsrmPropertyDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for &'a IFsrmPropertyDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmPropertyDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmPropertyDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmPropertyDefinition {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmPropertyDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPropertyDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmPropertyDefinition {
    type Vtable = IFsrmPropertyDefinition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xede0150f_e9a3_419c_877c_01fe5d24c5d3);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyDefinition_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut FsrmPropertyDefinitionType) -> ::windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: FsrmPropertyDefinitionType) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub PossibleValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, possiblevalues: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    PossibleValues: usize,
    #[cfg(feature = "win32-system")]
    pub SetPossibleValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, possiblevalues: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetPossibleValues: usize,
    #[cfg(feature = "win32-system")]
    pub ValueDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, valuedescriptions: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ValueDescriptions: usize,
    #[cfg(feature = "win32-system")]
    pub SetValueDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, valuedescriptions: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetValueDescriptions: usize,
    #[cfg(feature = "win32-system")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Parameters: usize,
    #[cfg(feature = "win32-system")]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetParameters: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmPropertyDefinition2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmPropertyDefinition2 {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, description: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Type(&self) -> ::windows_core::Result<FsrmPropertyDefinitionType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmPropertyDefinitionType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmPropertyDefinitionType>(result__)
    }
    pub unsafe fn SetType(&self, r#type: FsrmPropertyDefinitionType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(r#type)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn PossibleValues(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PossibleValues)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetPossibleValues(&self, possiblevalues: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPossibleValues)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(possiblevalues)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ValueDescriptions(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ValueDescriptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetValueDescriptions(&self, valuedescriptions: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetValueDescriptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(valuedescriptions)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Parameters(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetParameters(&self, parameters: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetParameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(parameters)).ok()
    }
    pub unsafe fn PropertyDefinitionFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).PropertyDefinitionFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn DisplayName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDisplayName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn AppliesTo(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).AppliesTo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ValueDefinitions(&self) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ValueDefinitions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmPropertyDefinition2> for ::windows_core::IUnknown {
    fn from(value: IFsrmPropertyDefinition2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmPropertyDefinition2> for ::windows_core::IUnknown {
    fn from(value: &IFsrmPropertyDefinition2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmPropertyDefinition2> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmPropertyDefinition2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmPropertyDefinition2> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmPropertyDefinition2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmPropertyDefinition2> for IFsrmObject {
    fn from(value: IFsrmPropertyDefinition2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmPropertyDefinition2> for IFsrmObject {
    fn from(value: &IFsrmPropertyDefinition2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for &'a IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmPropertyDefinition2> for IFsrmPropertyDefinition {
    fn from(value: IFsrmPropertyDefinition2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmPropertyDefinition2> for IFsrmPropertyDefinition {
    fn from(value: &IFsrmPropertyDefinition2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmPropertyDefinition> for IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmPropertyDefinition> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmPropertyDefinition> for &'a IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmPropertyDefinition> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmPropertyDefinition2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmPropertyDefinition2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmPropertyDefinition2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmPropertyDefinition2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPropertyDefinition2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmPropertyDefinition2 {
    type Vtable = IFsrmPropertyDefinition2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x47782152_d16c_4229_b4e1_0ddfe308b9f6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyDefinition2_Vtbl {
    pub base__: IFsrmPropertyDefinition_Vtbl,
    pub PropertyDefinitionFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertydefinitionflags: *mut i32) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub AppliesTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appliesto: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub ValueDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, valuedefinitions: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ValueDefinitions: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmPropertyDefinitionValue(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmPropertyDefinitionValue {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn DisplayName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UniqueID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).UniqueID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmPropertyDefinitionValue> for ::windows_core::IUnknown {
    fn from(value: IFsrmPropertyDefinitionValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmPropertyDefinitionValue> for ::windows_core::IUnknown {
    fn from(value: &IFsrmPropertyDefinitionValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmPropertyDefinitionValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmPropertyDefinitionValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmPropertyDefinitionValue> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmPropertyDefinitionValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmPropertyDefinitionValue> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmPropertyDefinitionValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmPropertyDefinitionValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmPropertyDefinitionValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmPropertyDefinitionValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmPropertyDefinitionValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmPropertyDefinitionValue {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmPropertyDefinitionValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPropertyDefinitionValue").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmPropertyDefinitionValue {
    type Vtable = IFsrmPropertyDefinitionValue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe946d148_bd67_4178_8e22_1c44925ed710);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyDefinitionValue_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub UniqueID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uniqueid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmQuota(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmQuota {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, description: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.QuotaLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetQuotaLimit<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, quotalimit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetQuotaLimit)(::windows_core::Interface::as_raw(self), quotalimit.into_param().abi()).ok()
    }
    pub unsafe fn QuotaFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.QuotaFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetQuotaFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(quotaflags)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Thresholds(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Thresholds)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddThreshold)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold)).ok()
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DeleteThreshold)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold)).ok()
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.ModifyThreshold)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold), ::core::mem::transmute(newthreshold)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows_core::Result<IFsrmAction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateThresholdAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold), ::core::mem::transmute(actiontype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmAction>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EnumThresholdActions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Path)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserSid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UserSid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserAccount(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UserAccount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SourceTemplateName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SourceTemplateName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn MatchesSourceTemplate(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MatchesSourceTemplate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn ApplyTemplate<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, quotatemplatename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ApplyTemplate)(::windows_core::Interface::as_raw(self), quotatemplatename.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn QuotaUsed(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).QuotaUsed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn QuotaPeakUsage(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).QuotaPeakUsage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn QuotaPeakUsageTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).QuotaPeakUsageTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn ResetPeakUsage(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ResetPeakUsage)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RefreshUsageProperties(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RefreshUsageProperties)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuota> for ::windows_core::IUnknown {
    fn from(value: IFsrmQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuota> for ::windows_core::IUnknown {
    fn from(value: &IFsrmQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmQuota {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmQuota {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuota> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuota> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmQuota {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmQuota {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuota> for IFsrmObject {
    fn from(value: IFsrmQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuota> for IFsrmObject {
    fn from(value: &IFsrmQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for IFsrmQuota {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for &'a IFsrmQuota {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuota> for IFsrmQuotaBase {
    fn from(value: IFsrmQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuota> for IFsrmQuotaBase {
    fn from(value: &IFsrmQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmQuotaBase> for IFsrmQuota {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmQuotaBase> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmQuotaBase> for &'a IFsrmQuota {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmQuotaBase> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuota> for IFsrmQuotaObject {
    fn from(value: IFsrmQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuota> for IFsrmQuotaObject {
    fn from(value: &IFsrmQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmQuotaObject> for IFsrmQuota {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmQuotaObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmQuotaObject> for &'a IFsrmQuota {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmQuotaObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmQuota {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmQuota {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmQuota {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmQuota {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmQuota").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmQuota {
    type Vtable = IFsrmQuota_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x377f739d_9647_4b8e_97d2_5ffce6d759cd);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuota_Vtbl {
    pub base__: IFsrmQuotaObject_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub QuotaUsed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, used: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    QuotaUsed: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub QuotaPeakUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peakusage: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    QuotaPeakUsage: usize,
    pub QuotaPeakUsageTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peakusagedatetime: *mut f64) -> ::windows_core::HRESULT,
    pub ResetPeakUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RefreshUsageProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmQuotaBase(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmQuotaBase {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, description: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).QuotaLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetQuotaLimit<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, quotalimit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetQuotaLimit)(::windows_core::Interface::as_raw(self), quotalimit.into_param().abi()).ok()
    }
    pub unsafe fn QuotaFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).QuotaFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetQuotaFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(quotaflags)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Thresholds(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).Thresholds)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddThreshold)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold)).ok()
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteThreshold)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold)).ok()
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ModifyThreshold)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold), ::core::mem::transmute(newthreshold)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows_core::Result<IFsrmAction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateThresholdAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold), ::core::mem::transmute(actiontype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmAction>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumThresholdActions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuotaBase> for ::windows_core::IUnknown {
    fn from(value: IFsrmQuotaBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuotaBase> for ::windows_core::IUnknown {
    fn from(value: &IFsrmQuotaBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmQuotaBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmQuotaBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuotaBase> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmQuotaBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuotaBase> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmQuotaBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmQuotaBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmQuotaBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuotaBase> for IFsrmObject {
    fn from(value: IFsrmQuotaBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuotaBase> for IFsrmObject {
    fn from(value: &IFsrmQuotaBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for IFsrmQuotaBase {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for &'a IFsrmQuotaBase {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmQuotaBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmQuotaBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmQuotaBase {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmQuotaBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmQuotaBase").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmQuotaBase {
    type Vtable = IFsrmQuotaBase_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1568a795_3924_4118_b74b_68d8f0fa5daf);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaBase_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub QuotaLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotalimit: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    QuotaLimit: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetQuotaLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotalimit: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetQuotaLimit: usize,
    pub QuotaFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotaflags: *mut i32) -> ::windows_core::HRESULT,
    pub SetQuotaFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotaflags: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Thresholds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, thresholds: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Thresholds: usize,
    pub AddThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows_core::HRESULT,
    pub DeleteThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows_core::HRESULT,
    pub ModifyThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, newthreshold: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub CreateThresholdAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, actiontype: FsrmActionType, action: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateThresholdAction: usize,
    #[cfg(feature = "win32-system")]
    pub EnumThresholdActions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, actions: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    EnumThresholdActions: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmQuotaManager(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmQuotaManager {
    #[cfg(feature = "win32-system")]
    pub unsafe fn ActionVariables(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).ActionVariables)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ActionVariableDescriptions(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).ActionVariableDescriptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateQuota<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0) -> ::windows_core::Result<IFsrmQuota> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateQuota)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmQuota>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateAutoApplyQuota<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, quotatemplatename: Param0, path: Param1) -> ::windows_core::Result<IFsrmAutoApplyQuota> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateAutoApplyQuota)(::windows_core::Interface::as_raw(self), quotatemplatename.into_param().abi(), path.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmAutoApplyQuota>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetQuota<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0) -> ::windows_core::Result<IFsrmQuota> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetQuota)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmQuota>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetAutoApplyQuota<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0) -> ::windows_core::Result<IFsrmAutoApplyQuota> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetAutoApplyQuota)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmAutoApplyQuota>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetRestrictiveQuota<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0) -> ::windows_core::Result<IFsrmQuota> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetRestrictiveQuota)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmQuota>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumQuotas<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumQuotas)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumAutoApplyQuotas<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumAutoApplyQuotas)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumEffectiveQuotas<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumEffectiveQuotas)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCommittableCollection>(result__)
    }
    pub unsafe fn Scan<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, strpath: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Scan)(::windows_core::Interface::as_raw(self), strpath.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateQuotaCollection(&self) -> ::windows_core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateQuotaCollection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCommittableCollection>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuotaManager> for ::windows_core::IUnknown {
    fn from(value: IFsrmQuotaManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuotaManager> for ::windows_core::IUnknown {
    fn from(value: &IFsrmQuotaManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmQuotaManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmQuotaManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuotaManager> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmQuotaManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuotaManager> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmQuotaManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmQuotaManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmQuotaManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmQuotaManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmQuotaManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmQuotaManager {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmQuotaManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmQuotaManager").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmQuotaManager {
    type Vtable = IFsrmQuotaManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8bb68c7d_19d8_4ffb_809e_be4fc1734014);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaManager_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub ActionVariables: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variables: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ActionVariables: usize,
    #[cfg(feature = "win32-system")]
    pub ActionVariableDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptions: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    ActionVariableDescriptions: usize,
    #[cfg(feature = "win32-system")]
    pub CreateQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, quota: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateQuota: usize,
    #[cfg(feature = "win32-system")]
    pub CreateAutoApplyQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, path: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, quota: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateAutoApplyQuota: usize,
    #[cfg(feature = "win32-system")]
    pub GetQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, quota: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetQuota: usize,
    #[cfg(feature = "win32-system")]
    pub GetAutoApplyQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, quota: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetAutoApplyQuota: usize,
    #[cfg(feature = "win32-system")]
    pub GetRestrictiveQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, quota: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetRestrictiveQuota: usize,
    #[cfg(feature = "win32-system")]
    pub EnumQuotas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    EnumQuotas: usize,
    #[cfg(feature = "win32-system")]
    pub EnumAutoApplyQuotas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    EnumAutoApplyQuotas: usize,
    #[cfg(feature = "win32-system")]
    pub EnumEffectiveQuotas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    EnumEffectiveQuotas: usize,
    pub Scan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub CreateQuotaCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateQuotaCollection: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmQuotaManagerEx(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmQuotaManagerEx {
    #[cfg(feature = "win32-system")]
    pub unsafe fn ActionVariables(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ActionVariables)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn ActionVariableDescriptions(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ActionVariableDescriptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateQuota<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0) -> ::windows_core::Result<IFsrmQuota> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateQuota)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmQuota>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateAutoApplyQuota<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, quotatemplatename: Param0, path: Param1) -> ::windows_core::Result<IFsrmAutoApplyQuota> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateAutoApplyQuota)(::windows_core::Interface::as_raw(self), quotatemplatename.into_param().abi(), path.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmAutoApplyQuota>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetQuota<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0) -> ::windows_core::Result<IFsrmQuota> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetQuota)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmQuota>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetAutoApplyQuota<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0) -> ::windows_core::Result<IFsrmAutoApplyQuota> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAutoApplyQuota)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmAutoApplyQuota>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetRestrictiveQuota<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0) -> ::windows_core::Result<IFsrmQuota> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRestrictiveQuota)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmQuota>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumQuotas<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumQuotas)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumAutoApplyQuotas<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumAutoApplyQuotas)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumEffectiveQuotas<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumEffectiveQuotas)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCommittableCollection>(result__)
    }
    pub unsafe fn Scan<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, strpath: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Scan)(::windows_core::Interface::as_raw(self), strpath.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateQuotaCollection(&self) -> ::windows_core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateQuotaCollection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCommittableCollection>(result__)
    }
    pub unsafe fn IsAffectedByQuota<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsAffectedByQuota)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuotaManagerEx> for ::windows_core::IUnknown {
    fn from(value: IFsrmQuotaManagerEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuotaManagerEx> for ::windows_core::IUnknown {
    fn from(value: &IFsrmQuotaManagerEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmQuotaManagerEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmQuotaManagerEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuotaManagerEx> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmQuotaManagerEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuotaManagerEx> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmQuotaManagerEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmQuotaManagerEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmQuotaManagerEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuotaManagerEx> for IFsrmQuotaManager {
    fn from(value: IFsrmQuotaManagerEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuotaManagerEx> for IFsrmQuotaManager {
    fn from(value: &IFsrmQuotaManagerEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmQuotaManager> for IFsrmQuotaManagerEx {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmQuotaManager> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmQuotaManager> for &'a IFsrmQuotaManagerEx {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmQuotaManager> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmQuotaManagerEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmQuotaManagerEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmQuotaManagerEx {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmQuotaManagerEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmQuotaManagerEx").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmQuotaManagerEx {
    type Vtable = IFsrmQuotaManagerEx_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4846cb01_d430_494f_abb4_b1054999fb09);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaManagerEx_Vtbl {
    pub base__: IFsrmQuotaManager_Vtbl,
    pub IsAffectedByQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, options: FsrmEnumOptions, affected: *mut i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmQuotaObject(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmQuotaObject {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, description: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.QuotaLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetQuotaLimit<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, quotalimit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetQuotaLimit)(::windows_core::Interface::as_raw(self), quotalimit.into_param().abi()).ok()
    }
    pub unsafe fn QuotaFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.QuotaFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetQuotaFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(quotaflags)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Thresholds(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Thresholds)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddThreshold)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold)).ok()
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteThreshold)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold)).ok()
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ModifyThreshold)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold), ::core::mem::transmute(newthreshold)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows_core::Result<IFsrmAction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateThresholdAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold), ::core::mem::transmute(actiontype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmAction>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumThresholdActions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Path)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserSid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).UserSid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserAccount(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).UserAccount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SourceTemplateName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).SourceTemplateName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn MatchesSourceTemplate(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).MatchesSourceTemplate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn ApplyTemplate<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, quotatemplatename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ApplyTemplate)(::windows_core::Interface::as_raw(self), quotatemplatename.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuotaObject> for ::windows_core::IUnknown {
    fn from(value: IFsrmQuotaObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuotaObject> for ::windows_core::IUnknown {
    fn from(value: &IFsrmQuotaObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmQuotaObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmQuotaObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuotaObject> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmQuotaObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuotaObject> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmQuotaObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmQuotaObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmQuotaObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuotaObject> for IFsrmObject {
    fn from(value: IFsrmQuotaObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuotaObject> for IFsrmObject {
    fn from(value: &IFsrmQuotaObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for IFsrmQuotaObject {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for &'a IFsrmQuotaObject {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuotaObject> for IFsrmQuotaBase {
    fn from(value: IFsrmQuotaObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuotaObject> for IFsrmQuotaBase {
    fn from(value: &IFsrmQuotaObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmQuotaBase> for IFsrmQuotaObject {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmQuotaBase> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmQuotaBase> for &'a IFsrmQuotaObject {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmQuotaBase> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmQuotaObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmQuotaObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmQuotaObject {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmQuotaObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmQuotaObject").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmQuotaObject {
    type Vtable = IFsrmQuotaObject_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x42dc3511_61d5_48ae_b6dc_59fc00c0a8d6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaObject_Vtbl {
    pub base__: IFsrmQuotaBase_Vtbl,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub UserSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub UserAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccount: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SourceTemplateName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatename: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub MatchesSourceTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matches: *mut i16) -> ::windows_core::HRESULT,
    pub ApplyTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmQuotaTemplate(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmQuotaTemplate {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, description: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.QuotaLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetQuotaLimit<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, quotalimit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetQuotaLimit)(::windows_core::Interface::as_raw(self), quotalimit.into_param().abi()).ok()
    }
    pub unsafe fn QuotaFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.QuotaFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetQuotaFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(quotaflags)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Thresholds(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Thresholds)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddThreshold)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold)).ok()
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteThreshold)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold)).ok()
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ModifyThreshold)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold), ::core::mem::transmute(newthreshold)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows_core::Result<IFsrmAction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateThresholdAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold), ::core::mem::transmute(actiontype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmAction>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumThresholdActions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn CopyTemplate<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, quotatemplatename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CopyTemplate)(::windows_core::Interface::as_raw(self), quotatemplatename.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows_core::Result<IFsrmDerivedObjectsResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CommitAndUpdateDerived)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(commitoptions), ::core::mem::transmute(applyoptions), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmDerivedObjectsResult>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuotaTemplate> for ::windows_core::IUnknown {
    fn from(value: IFsrmQuotaTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuotaTemplate> for ::windows_core::IUnknown {
    fn from(value: &IFsrmQuotaTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuotaTemplate> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmQuotaTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuotaTemplate> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmQuotaTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuotaTemplate> for IFsrmObject {
    fn from(value: IFsrmQuotaTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuotaTemplate> for IFsrmObject {
    fn from(value: &IFsrmQuotaTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for &'a IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuotaTemplate> for IFsrmQuotaBase {
    fn from(value: IFsrmQuotaTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuotaTemplate> for IFsrmQuotaBase {
    fn from(value: &IFsrmQuotaTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmQuotaBase> for IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmQuotaBase> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmQuotaBase> for &'a IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmQuotaBase> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmQuotaTemplate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmQuotaTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmQuotaTemplate {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmQuotaTemplate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmQuotaTemplate").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmQuotaTemplate {
    type Vtable = IFsrmQuotaTemplate_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa2efab31_295e_46bb_b976_e86d58b52e8b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaTemplate_Vtbl {
    pub base__: IFsrmQuotaBase_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub CopyTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub CommitAndUpdateDerived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CommitAndUpdateDerived: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmQuotaTemplateImported(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmQuotaTemplateImported {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, description: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.QuotaLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetQuotaLimit<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, quotalimit: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetQuotaLimit)(::windows_core::Interface::as_raw(self), quotalimit.into_param().abi()).ok()
    }
    pub unsafe fn QuotaFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.QuotaFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetQuotaFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(quotaflags)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Thresholds(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Thresholds)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddThreshold)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold)).ok()
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DeleteThreshold)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold)).ok()
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.ModifyThreshold)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold), ::core::mem::transmute(newthreshold)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows_core::Result<IFsrmAction> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateThresholdAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold), ::core::mem::transmute(actiontype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmAction>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EnumThresholdActions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threshold), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn CopyTemplate<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, quotatemplatename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CopyTemplate)(::windows_core::Interface::as_raw(self), quotatemplatename.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows_core::Result<IFsrmDerivedObjectsResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CommitAndUpdateDerived)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(commitoptions), ::core::mem::transmute(applyoptions), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmDerivedObjectsResult>(result__)
    }
    pub unsafe fn OverwriteOnCommit(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).OverwriteOnCommit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetOverwriteOnCommit(&self, overwrite: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOverwriteOnCommit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(overwrite)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuotaTemplateImported> for ::windows_core::IUnknown {
    fn from(value: IFsrmQuotaTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuotaTemplateImported> for ::windows_core::IUnknown {
    fn from(value: &IFsrmQuotaTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuotaTemplateImported> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmQuotaTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuotaTemplateImported> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmQuotaTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuotaTemplateImported> for IFsrmObject {
    fn from(value: IFsrmQuotaTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuotaTemplateImported> for IFsrmObject {
    fn from(value: &IFsrmQuotaTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for &'a IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuotaTemplateImported> for IFsrmQuotaBase {
    fn from(value: IFsrmQuotaTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuotaTemplateImported> for IFsrmQuotaBase {
    fn from(value: &IFsrmQuotaTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmQuotaBase> for IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmQuotaBase> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmQuotaBase> for &'a IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmQuotaBase> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuotaTemplateImported> for IFsrmQuotaTemplate {
    fn from(value: IFsrmQuotaTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuotaTemplateImported> for IFsrmQuotaTemplate {
    fn from(value: &IFsrmQuotaTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmQuotaTemplate> for IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmQuotaTemplate> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmQuotaTemplate> for &'a IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmQuotaTemplate> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmQuotaTemplateImported {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmQuotaTemplateImported {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmQuotaTemplateImported {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmQuotaTemplateImported {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmQuotaTemplateImported").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmQuotaTemplateImported {
    type Vtable = IFsrmQuotaTemplateImported_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a2bf113_a329_44cc_809a_5c00fce8da40);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaTemplateImported_Vtbl {
    pub base__: IFsrmQuotaTemplate_Vtbl,
    pub OverwriteOnCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: *mut i16) -> ::windows_core::HRESULT,
    pub SetOverwriteOnCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmQuotaTemplateManager(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmQuotaTemplateManager {
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateTemplate(&self) -> ::windows_core::Result<IFsrmQuotaTemplate> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateTemplate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmQuotaTemplate>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetTemplate<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<IFsrmQuotaTemplate> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetTemplate)(::windows_core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmQuotaTemplate>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumTemplates(&self, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumTemplates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ExportTemplates(&self, quotatemplatenamesarray: *const ::win32_system::Com::VARIANT) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ExportTemplates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(quotatemplatenamesarray), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ImportTemplates<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, serializedquotatemplates: Param0, quotatemplatenamesarray: *const ::win32_system::Com::VARIANT) -> ::windows_core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ImportTemplates)(::windows_core::Interface::as_raw(self), serializedquotatemplates.into_param().abi(), ::core::mem::transmute(quotatemplatenamesarray), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCommittableCollection>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuotaTemplateManager> for ::windows_core::IUnknown {
    fn from(value: IFsrmQuotaTemplateManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuotaTemplateManager> for ::windows_core::IUnknown {
    fn from(value: &IFsrmQuotaTemplateManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmQuotaTemplateManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmQuotaTemplateManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmQuotaTemplateManager> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmQuotaTemplateManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmQuotaTemplateManager> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmQuotaTemplateManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmQuotaTemplateManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmQuotaTemplateManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmQuotaTemplateManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmQuotaTemplateManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmQuotaTemplateManager {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmQuotaTemplateManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmQuotaTemplateManager").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmQuotaTemplateManager {
    type Vtable = IFsrmQuotaTemplateManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4173ac41_172d_4d52_963c_fdc7e415f717);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaTemplateManager_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub CreateTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplate: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateTemplate: usize,
    #[cfg(feature = "win32-system")]
    pub GetTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, quotatemplate: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetTemplate: usize,
    #[cfg(feature = "win32-system")]
    pub EnumTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, quotatemplates: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    EnumTemplates: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ExportTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatenamesarray: *const ::win32_system::Com::VARIANT, serializedquotatemplates: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ExportTemplates: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ImportTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serializedquotatemplates: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, quotatemplatenamesarray: *const ::win32_system::Com::VARIANT, quotatemplates: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ImportTemplates: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmReport(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmReport {
    pub unsafe fn Type(&self) -> ::windows_core::Result<FsrmReportType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmReportType>::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmReportType>(result__)
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, description: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn LastGeneratedFileNamePrefix(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).LastGeneratedFileNamePrefix)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GetFilter(&self, filter: FsrmReportFilter) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(filter), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetFilter<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, filter: FsrmReportFilter, filtervalue: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(filter), filtervalue.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmReport> for ::windows_core::IUnknown {
    fn from(value: IFsrmReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmReport> for ::windows_core::IUnknown {
    fn from(value: &IFsrmReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmReport> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmReport> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmReport {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmReport").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmReport {
    type Vtable = IFsrmReport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8cc81d9_46b8_4fa4_bfa5_4aa9dec9b638);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmReport_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *mut FsrmReportType) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub LastGeneratedFileNamePrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prefix: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub GetFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filter: FsrmReportFilter, filtervalue: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    GetFilter: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filter: FsrmReportFilter, filtervalue: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetFilter: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmReportJob(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmReportJob {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, description: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Task(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Task)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, taskname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTask)(::windows_core::Interface::as_raw(self), taskname.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn NamespaceRoots(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).NamespaceRoots)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNamespaceRoots)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(namespaceroots)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Formats(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).Formats)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetFormats(&self, formats: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFormats)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(formats)).ok()
    }
    pub unsafe fn MailTo(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).MailTo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetMailTo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, mailto: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMailTo)(::windows_core::Interface::as_raw(self), mailto.into_param().abi()).ok()
    }
    pub unsafe fn RunningStatus(&self) -> ::windows_core::Result<FsrmReportRunningStatus> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmReportRunningStatus>::zeroed();
        (::windows_core::Interface::vtable(self).RunningStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmReportRunningStatus>(result__)
    }
    pub unsafe fn LastRun(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).LastRun)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn LastError(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).LastError)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn LastGeneratedInDirectory(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).LastGeneratedInDirectory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumReports(&self) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumReports)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateReport(&self, reporttype: FsrmReportType) -> ::windows_core::Result<IFsrmReport> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateReport)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(reporttype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmReport>(result__)
    }
    pub unsafe fn Run(&self, context: FsrmReportGenerationContext) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Run)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(context)).ok()
    }
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).WaitForCompletion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(waitseconds), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmReportJob> for ::windows_core::IUnknown {
    fn from(value: IFsrmReportJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmReportJob> for ::windows_core::IUnknown {
    fn from(value: &IFsrmReportJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmReportJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmReportJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmReportJob> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmReportJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmReportJob> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmReportJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmReportJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmReportJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmReportJob> for IFsrmObject {
    fn from(value: IFsrmReportJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmReportJob> for IFsrmObject {
    fn from(value: &IFsrmReportJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for IFsrmReportJob {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for &'a IFsrmReportJob {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmReportJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmReportJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmReportJob {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmReportJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmReportJob").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmReportJob {
    type Vtable = IFsrmReportJob_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38e87280_715c_4c7d_a280_ea1651a19fef);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmReportJob_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub Task: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub NamespaceRoots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    NamespaceRoots: usize,
    #[cfg(feature = "win32-system")]
    pub SetNamespaceRoots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetNamespaceRoots: usize,
    #[cfg(feature = "win32-system")]
    pub Formats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Formats: usize,
    #[cfg(feature = "win32-system")]
    pub SetFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetFormats: usize,
    pub MailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetMailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub RunningStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> ::windows_core::HRESULT,
    pub LastRun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastrun: *mut f64) -> ::windows_core::HRESULT,
    pub LastError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lasterror: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub LastGeneratedInDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub EnumReports: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reports: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    EnumReports: usize,
    #[cfg(feature = "win32-system")]
    pub CreateReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, report: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateReport: usize,
    pub Run: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext) -> ::windows_core::HRESULT,
    pub WaitForCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut i16) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmReportManager(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmReportManager {
    #[cfg(feature = "win32-system")]
    pub unsafe fn EnumReportJobs(&self, options: FsrmEnumOptions) -> ::windows_core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EnumReportJobs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateReportJob(&self) -> ::windows_core::Result<IFsrmReportJob> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateReportJob)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmReportJob>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetReportJob<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, taskname: Param0) -> ::windows_core::Result<IFsrmReportJob> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetReportJob)(::windows_core::Interface::as_raw(self), taskname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmReportJob>(result__)
    }
    pub unsafe fn GetOutputDirectory(&self, context: FsrmReportGenerationContext) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetOutputDirectory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(context), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetOutputDirectory<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, context: FsrmReportGenerationContext, path: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOutputDirectory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(context), path.into_param().abi()).ok()
    }
    pub unsafe fn IsFilterValidForReportType(&self, reporttype: FsrmReportType, filter: FsrmReportFilter) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsFilterValidForReportType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(reporttype), ::core::mem::transmute(filter), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GetDefaultFilter(&self, reporttype: FsrmReportType, filter: FsrmReportFilter) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetDefaultFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(reporttype), ::core::mem::transmute(filter), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetDefaultFilter<'a, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDefaultFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(reporttype), ::core::mem::transmute(filter), filtervalue.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GetReportSizeLimit(&self, limit: FsrmReportLimit) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetReportSizeLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(limit), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetReportSizeLimit<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, limit: FsrmReportLimit, limitvalue: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetReportSizeLimit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(limit), limitvalue.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmReportManager> for ::windows_core::IUnknown {
    fn from(value: IFsrmReportManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmReportManager> for ::windows_core::IUnknown {
    fn from(value: &IFsrmReportManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmReportManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmReportManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmReportManager> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmReportManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmReportManager> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmReportManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmReportManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmReportManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmReportManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmReportManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmReportManager {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmReportManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmReportManager").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmReportManager {
    type Vtable = IFsrmReportManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27b899fe_6ffa_4481_a184_d3daade8a02b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmReportManager_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub EnumReportJobs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, reportjobs: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    EnumReportJobs: usize,
    #[cfg(feature = "win32-system")]
    pub CreateReportJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportjob: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateReportJob: usize,
    #[cfg(feature = "win32-system")]
    pub GetReportJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, reportjob: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetReportJob: usize,
    pub GetOutputDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext, path: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetOutputDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext, path: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub IsFilterValidForReportType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, valid: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub GetDefaultFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    GetDefaultFilter: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetDefaultFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetDefaultFilter: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub GetReportSizeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, limit: FsrmReportLimit, limitvalue: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    GetReportSizeLimit: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetReportSizeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, limit: FsrmReportLimit, limitvalue: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetReportSizeLimit: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmReportScheduler(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmReportScheduler {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn VerifyNamespaces(&self, namespacessafearray: *const ::win32_system::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).VerifyNamespaces)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(namespacessafearray)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn CreateScheduleTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, taskname: Param0, namespacessafearray: *const ::win32_system::Com::VARIANT, serializedtask: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateScheduleTask)(::windows_core::Interface::as_raw(self), taskname.into_param().abi(), ::core::mem::transmute(namespacessafearray), serializedtask.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn ModifyScheduleTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, taskname: Param0, namespacessafearray: *const ::win32_system::Com::VARIANT, serializedtask: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ModifyScheduleTask)(::windows_core::Interface::as_raw(self), taskname.into_param().abi(), ::core::mem::transmute(namespacessafearray), serializedtask.into_param().abi()).ok()
    }
    pub unsafe fn DeleteScheduleTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, taskname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteScheduleTask)(::windows_core::Interface::as_raw(self), taskname.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmReportScheduler> for ::windows_core::IUnknown {
    fn from(value: IFsrmReportScheduler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmReportScheduler> for ::windows_core::IUnknown {
    fn from(value: &IFsrmReportScheduler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmReportScheduler {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmReportScheduler {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmReportScheduler> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmReportScheduler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmReportScheduler> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmReportScheduler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmReportScheduler {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmReportScheduler {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmReportScheduler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmReportScheduler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmReportScheduler {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmReportScheduler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmReportScheduler").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmReportScheduler {
    type Vtable = IFsrmReportScheduler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6879caf9_6617_4484_8719_71c3d8645f94);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmReportScheduler_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub VerifyNamespaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespacessafearray: *const ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    VerifyNamespaces: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub CreateScheduleTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, namespacessafearray: *const ::win32_system::Com::VARIANT, serializedtask: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    CreateScheduleTask: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub ModifyScheduleTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, namespacessafearray: *const ::win32_system::Com::VARIANT, serializedtask: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    ModifyScheduleTask: usize,
    pub DeleteScheduleTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmRule(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmRule {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, description: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn RuleType(&self) -> ::windows_core::Result<FsrmRuleType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmRuleType>::zeroed();
        (::windows_core::Interface::vtable(self).RuleType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmRuleType>(result__)
    }
    pub unsafe fn ModuleDefinitionName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ModuleDefinitionName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetModuleDefinitionName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, moduledefinitionname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetModuleDefinitionName)(::windows_core::Interface::as_raw(self), moduledefinitionname.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn NamespaceRoots(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).NamespaceRoots)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNamespaceRoots)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(namespaceroots)).ok()
    }
    pub unsafe fn RuleFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).RuleFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRuleFlags(&self, ruleflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRuleFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ruleflags)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Parameters(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).Parameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetParameters(&self, parameters: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetParameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(parameters)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn LastModified(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).LastModified)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmRule> for ::windows_core::IUnknown {
    fn from(value: IFsrmRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmRule> for ::windows_core::IUnknown {
    fn from(value: &IFsrmRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmRule {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmRule {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmRule> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmRule> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmRule {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmRule {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmRule> for IFsrmObject {
    fn from(value: IFsrmRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmRule> for IFsrmObject {
    fn from(value: &IFsrmRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for IFsrmRule {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for &'a IFsrmRule {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmRule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmRule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmRule {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmRule").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmRule {
    type Vtable = IFsrmRule_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb0df960_16f5_4495_9079_3f9360d831df);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmRule_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub RuleType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruletype: *mut FsrmRuleType) -> ::windows_core::HRESULT,
    pub ModuleDefinitionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduledefinitionname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetModuleDefinitionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduledefinitionname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub NamespaceRoots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    NamespaceRoots: usize,
    #[cfg(feature = "win32-system")]
    pub SetNamespaceRoots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetNamespaceRoots: usize,
    pub RuleFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruleflags: *mut i32) -> ::windows_core::HRESULT,
    pub SetRuleFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruleflags: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *mut *mut ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Parameters: usize,
    #[cfg(feature = "win32-system")]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetParameters: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub LastModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastmodified: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    LastModified: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmSetting(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmSetting {
    pub unsafe fn SmtpServer(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).SmtpServer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetSmtpServer<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, smtpserver: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSmtpServer)(::windows_core::Interface::as_raw(self), smtpserver.into_param().abi()).ok()
    }
    pub unsafe fn MailFrom(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).MailFrom)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetMailFrom<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, mailfrom: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMailFrom)(::windows_core::Interface::as_raw(self), mailfrom.into_param().abi()).ok()
    }
    pub unsafe fn AdminEmail(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).AdminEmail)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetAdminEmail<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, adminemail: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAdminEmail)(::windows_core::Interface::as_raw(self), adminemail.into_param().abi()).ok()
    }
    pub unsafe fn DisableCommandLine(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).DisableCommandLine)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetDisableCommandLine(&self, disablecommandline: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDisableCommandLine)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(disablecommandline)).ok()
    }
    pub unsafe fn EnableScreeningAudit(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).EnableScreeningAudit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnableScreeningAudit(&self, enablescreeningaudit: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnableScreeningAudit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enablescreeningaudit)).ok()
    }
    pub unsafe fn EmailTest<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, mailto: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EmailTest)(::windows_core::Interface::as_raw(self), mailto.into_param().abi()).ok()
    }
    pub unsafe fn SetActionRunLimitInterval(&self, actiontype: FsrmActionType, delaytimeminutes: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetActionRunLimitInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(actiontype), ::core::mem::transmute(delaytimeminutes)).ok()
    }
    pub unsafe fn GetActionRunLimitInterval(&self, actiontype: FsrmActionType) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).GetActionRunLimitInterval)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(actiontype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmSetting> for ::windows_core::IUnknown {
    fn from(value: IFsrmSetting) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmSetting> for ::windows_core::IUnknown {
    fn from(value: &IFsrmSetting) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmSetting {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmSetting {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmSetting> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmSetting) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmSetting> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmSetting) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmSetting {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmSetting {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmSetting {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmSetting {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmSetting {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmSetting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmSetting").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmSetting {
    type Vtable = IFsrmSetting_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf411d4fd_14be_4260_8c40_03b7c95e608a);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmSetting_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub SmtpServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smtpserver: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetSmtpServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smtpserver: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub MailFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailfrom: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetMailFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailfrom: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub AdminEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adminemail: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetAdminEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adminemail: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub DisableCommandLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disablecommandline: *mut i16) -> ::windows_core::HRESULT,
    pub SetDisableCommandLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disablecommandline: i16) -> ::windows_core::HRESULT,
    pub EnableScreeningAudit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enablescreeningaudit: *mut i16) -> ::windows_core::HRESULT,
    pub SetEnableScreeningAudit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enablescreeningaudit: i16) -> ::windows_core::HRESULT,
    pub EmailTest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub SetActionRunLimitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, delaytimeminutes: i32) -> ::windows_core::HRESULT,
    pub GetActionRunLimitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, delaytimeminutes: *mut i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmStorageModuleDefinition(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmStorageModuleDefinition {
    pub unsafe fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Id)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::GUID>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, description: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ModuleClsid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ModuleClsid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetModuleClsid<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, moduleclsid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetModuleClsid)(::windows_core::Interface::as_raw(self), moduleclsid.into_param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, name: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Company(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Company)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetCompany<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, company: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetCompany)(::windows_core::Interface::as_raw(self), company.into_param().abi()).ok()
    }
    pub unsafe fn Version(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Version)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetVersion<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, version: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetVersion)(::windows_core::Interface::as_raw(self), version.into_param().abi()).ok()
    }
    pub unsafe fn ModuleType(&self) -> ::windows_core::Result<FsrmPipelineModuleType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmPipelineModuleType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ModuleType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmPipelineModuleType>(result__)
    }
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn NeedsFileContent(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.NeedsFileContent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetNeedsFileContent(&self, needsfilecontent: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetNeedsFileContent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(needsfilecontent)).ok()
    }
    pub unsafe fn Account(&self) -> ::windows_core::Result<FsrmAccountType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmAccountType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Account)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmAccountType>(result__)
    }
    pub unsafe fn SetAccount(&self, retrievalaccount: FsrmAccountType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetAccount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(retrievalaccount)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SupportedExtensions(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SupportedExtensions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetSupportedExtensions(&self, supportedextensions: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSupportedExtensions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(supportedextensions)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Parameters(&self) -> ::windows_core::Result<*mut ::win32_system::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::win32_system::Com::SAFEARRAY>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::win32_system::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetParameters(&self, parameters: *const ::win32_system::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetParameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(parameters)).ok()
    }
    pub unsafe fn Capabilities(&self) -> ::windows_core::Result<FsrmStorageModuleCaps> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmStorageModuleCaps>::zeroed();
        (::windows_core::Interface::vtable(self).Capabilities)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmStorageModuleCaps>(result__)
    }
    pub unsafe fn SetCapabilities(&self, capabilities: FsrmStorageModuleCaps) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCapabilities)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(capabilities)).ok()
    }
    pub unsafe fn StorageType(&self) -> ::windows_core::Result<FsrmStorageModuleType> {
        let mut result__ = ::core::mem::MaybeUninit::<FsrmStorageModuleType>::zeroed();
        (::windows_core::Interface::vtable(self).StorageType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsrmStorageModuleType>(result__)
    }
    pub unsafe fn SetStorageType(&self, storagetype: FsrmStorageModuleType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStorageType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(storagetype)).ok()
    }
    pub unsafe fn UpdatesFileContent(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).UpdatesFileContent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetUpdatesFileContent(&self, updatesfilecontent: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUpdatesFileContent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(updatesfilecontent)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmStorageModuleDefinition> for ::windows_core::IUnknown {
    fn from(value: IFsrmStorageModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmStorageModuleDefinition> for ::windows_core::IUnknown {
    fn from(value: &IFsrmStorageModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmStorageModuleDefinition> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmStorageModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmStorageModuleDefinition> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmStorageModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmStorageModuleDefinition> for IFsrmObject {
    fn from(value: IFsrmStorageModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmStorageModuleDefinition> for IFsrmObject {
    fn from(value: &IFsrmStorageModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmObject> for &'a IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmObject> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmStorageModuleDefinition> for IFsrmPipelineModuleDefinition {
    fn from(value: IFsrmStorageModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmStorageModuleDefinition> for IFsrmPipelineModuleDefinition {
    fn from(value: &IFsrmStorageModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmPipelineModuleDefinition> for IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmPipelineModuleDefinition> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmPipelineModuleDefinition> for &'a IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmPipelineModuleDefinition> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmStorageModuleDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmStorageModuleDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmStorageModuleDefinition {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmStorageModuleDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmStorageModuleDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmStorageModuleDefinition {
    type Vtable = IFsrmStorageModuleDefinition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x15a81350_497d_4aba_80e9_d4dbcc5521fe);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmStorageModuleDefinition_Vtbl {
    pub base__: IFsrmPipelineModuleDefinition_Vtbl,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilities: *mut FsrmStorageModuleCaps) -> ::windows_core::HRESULT,
    pub SetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilities: FsrmStorageModuleCaps) -> ::windows_core::HRESULT,
    pub StorageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storagetype: *mut FsrmStorageModuleType) -> ::windows_core::HRESULT,
    pub SetStorageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storagetype: FsrmStorageModuleType) -> ::windows_core::HRESULT,
    pub UpdatesFileContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updatesfilecontent: *mut i16) -> ::windows_core::HRESULT,
    pub SetUpdatesFileContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updatesfilecontent: i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IFsrmStorageModuleImplementation(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IFsrmStorageModuleImplementation {
    #[cfg(feature = "win32-system")]
    pub unsafe fn OnLoad<'a, Param0: ::windows_core::IntoParam<'a, IFsrmPipelineModuleDefinition>>(&self, moduledefinition: Param0) -> ::windows_core::Result<IFsrmPipelineModuleConnector> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.OnLoad)(::windows_core::Interface::as_raw(self), moduledefinition.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsrmPipelineModuleConnector>(result__)
    }
    pub unsafe fn OnUnload(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnUnload)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn UseDefinitions<'a, Param0: ::windows_core::IntoParam<'a, IFsrmCollection>>(&self, propertydefinitions: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UseDefinitions)(::windows_core::Interface::as_raw(self), propertydefinitions.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn LoadProperties<'a, Param0: ::windows_core::IntoParam<'a, IFsrmPropertyBag>>(&self, propertybag: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LoadProperties)(::windows_core::Interface::as_raw(self), propertybag.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SaveProperties<'a, Param0: ::windows_core::IntoParam<'a, IFsrmPropertyBag>>(&self, propertybag: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SaveProperties)(::windows_core::Interface::as_raw(self), propertybag.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmStorageModuleImplementation> for ::windows_core::IUnknown {
    fn from(value: IFsrmStorageModuleImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmStorageModuleImplementation> for ::windows_core::IUnknown {
    fn from(value: &IFsrmStorageModuleImplementation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFsrmStorageModuleImplementation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFsrmStorageModuleImplementation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmStorageModuleImplementation> for ::win32_system::Com::IDispatch {
    fn from(value: IFsrmStorageModuleImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmStorageModuleImplementation> for ::win32_system::Com::IDispatch {
    fn from(value: &IFsrmStorageModuleImplementation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IFsrmStorageModuleImplementation {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IFsrmStorageModuleImplementation {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IFsrmStorageModuleImplementation> for IFsrmPipelineModuleImplementation {
    fn from(value: IFsrmStorageModuleImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IFsrmStorageModuleImplementation> for IFsrmPipelineModuleImplementation {
    fn from(value: &IFsrmStorageModuleImplementation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmPipelineModuleImplementation> for IFsrmStorageModuleImplementation {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmPipelineModuleImplementation> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IFsrmPipelineModuleImplementation> for &'a IFsrmStorageModuleImplementation {
    fn into_param(self) -> ::windows_core::Param<'a, IFsrmPipelineModuleImplementation> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IFsrmStorageModuleImplementation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IFsrmStorageModuleImplementation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IFsrmStorageModuleImplementation {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IFsrmStorageModuleImplementation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmStorageModuleImplementation").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IFsrmStorageModuleImplementation {
    type Vtable = IFsrmStorageModuleImplementation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0af4a0da_895a_4e50_8712_a96724bcec64);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmStorageModuleImplementation_Vtbl {
    pub base__: IFsrmPipelineModuleImplementation_Vtbl,
    #[cfg(feature = "win32-system")]
    pub UseDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertydefinitions: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    UseDefinitions: usize,
    #[cfg(feature = "win32-system")]
    pub LoadProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertybag: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    LoadProperties: usize,
    #[cfg(feature = "win32-system")]
    pub SaveProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertybag: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SaveProperties: usize,
}
pub const MessageSizeLimit: u32 = 4096u32;
