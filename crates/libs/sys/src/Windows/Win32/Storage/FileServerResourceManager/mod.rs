pub const AdSyncTask: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 719734609, data2: 46888, data3: 19819, data4: [151, 160, 178, 218, 46, 125, 42, 59] };
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type AdrClientDisplayFlags = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientDisplayFlags_AllowEmailRequests: AdrClientDisplayFlags = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientDisplayFlags_ShowDeviceTroubleshooting: AdrClientDisplayFlags = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type AdrClientErrorType = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientErrorType_Unknown: AdrClientErrorType = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientErrorType_AccessDenied: AdrClientErrorType = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientErrorType_FileNotFound: AdrClientErrorType = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type AdrClientFlags = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientFlags_None: AdrClientFlags = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientFlags_FailForLocalPaths: AdrClientFlags = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientFlags_FailIfNotSupportedByServer: AdrClientFlags = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientFlags_FailIfNotDomainJoined: AdrClientFlags = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type AdrEmailFlags = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrEmailFlags_PutDataOwnerOnToLine: AdrEmailFlags = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrEmailFlags_PutAdminOnToLine: AdrEmailFlags = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrEmailFlags_IncludeDeviceClaims: AdrEmailFlags = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrEmailFlags_IncludeUserInfo: AdrEmailFlags = 8i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrEmailFlags_GenerateEventLog: AdrEmailFlags = 16i32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct DIFsrmClassificationEvents {
    pub base__: super::super::System::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for DIFsrmClassificationEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 647245232, data2: 55999, data3: 16856, data4: [187, 221, 177, 41, 169, 247, 4, 36] };
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_FEATURE_CLASSIFICATION: u32 = 83886080u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_FEATURE_FILESCREEN: u32 = 50331648u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_FEATURE_GENERAL: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_FEATURE_MASK: u32 = 251658240u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_FEATURE_PIPELINE: u32 = 100663296u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_FEATURE_QUOTA: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_FEATURE_REPORTS: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_INTERFACE_A_MASK: u32 = 15728640u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_INTERFACE_B_MASK: u32 = 983040u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_INTERFACE_C_MASK: u32 = 61440u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_INTERFACE_D_MASK: u32 = 3840u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_IS_PROPERTY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_METHOD_NUM_MASK: u32 = 127u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_ADR_MAX_EMAILS_SENT: ::windows_sys::core::HRESULT = -2147200130i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_ADR_NOT_DOMAIN_JOINED: ::windows_sys::core::HRESULT = -2147200110i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_ADR_PATH_IS_LOCAL: ::windows_sys::core::HRESULT = -2147200111i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_ADR_SRV_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147200112i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_ALREADY_EXISTS: ::windows_sys::core::HRESULT = -2147200253i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_AUTO_QUOTA: ::windows_sys::core::HRESULT = 283419i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CACHE_INVALID: ::windows_sys::core::HRESULT = -2147200187i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CACHE_MODULE_ALREADY_EXISTS: ::windows_sys::core::HRESULT = -2147200186i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_AGGREGATE: ::windows_sys::core::HRESULT = -2147200201i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_ALLOW_REPARSE_POINT_TAG: ::windows_sys::core::HRESULT = -2147200170i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_CHANGE_PROPERTY_TYPE: ::windows_sys::core::HRESULT = -2147200197i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_CREATE_TEMP_COPY: ::windows_sys::core::HRESULT = -2147200132i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_DELETE_SYSTEM_PROPERTY: ::windows_sys::core::HRESULT = -2147200135i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_REMOVE_READONLY: ::windows_sys::core::HRESULT = -2147200109i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_RENAME_PROPERTY: ::windows_sys::core::HRESULT = -2147200198i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_STORE_PROPERTIES: ::windows_sys::core::HRESULT = -2147200171i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_USE_DELETED_PROPERTY: ::windows_sys::core::HRESULT = -2147200143i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_USE_DEPRECATED_PROPERTY: ::windows_sys::core::HRESULT = -2147200145i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CLASSIFICATION_ALREADY_RUNNING: ::windows_sys::core::HRESULT = -2147200195i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CLASSIFICATION_CANCELED: ::windows_sys::core::HRESULT = -2147200141i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CLASSIFICATION_NOT_RUNNING: ::windows_sys::core::HRESULT = -2147200194i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CLASSIFICATION_PARTIAL_BATCH: ::windows_sys::core::HRESULT = -2147200136i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CLASSIFICATION_SCAN_FAIL: ::windows_sys::core::HRESULT = -2147200148i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CLASSIFICATION_TIMEOUT: ::windows_sys::core::HRESULT = -2147200137i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CLUSTER_NOT_RUNNING: ::windows_sys::core::HRESULT = -2147200210i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CSC_PATH_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147200106i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_DIFFERENT_CLUSTER_GROUP: ::windows_sys::core::HRESULT = -2147200207i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_DRIVER_NOT_READY: ::windows_sys::core::HRESULT = -2147200237i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_DUPLICATE_NAME: ::windows_sys::core::HRESULT = -2147200240i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_EMAIL_NOT_SENT: ::windows_sys::core::HRESULT = -2147200228i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_ENUM_PROPERTIES_FAILED: ::windows_sys::core::HRESULT = -2147200173i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_ERROR_NOT_ENABLED: ::windows_sys::core::HRESULT = -2147200133i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_EXPIRATION_PATH_NOT_WRITEABLE: ::windows_sys::core::HRESULT = -2147200105i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_EXPIRATION_PATH_TOO_LONG: ::windows_sys::core::HRESULT = -2147200104i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_EXPIRATION_VOLUME_NOT_NTFS: ::windows_sys::core::HRESULT = -2147200103i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FAIL_BATCH: ::windows_sys::core::HRESULT = -2147200247i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_ENCRYPTED: ::windows_sys::core::HRESULT = -2147200156i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_IN_USE: ::windows_sys::core::HRESULT = -2147200134i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_ACTION_GET_EXITCODE_FAILED: ::windows_sys::core::HRESULT = -2147200152i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_ACTION_TIMEOUT: ::windows_sys::core::HRESULT = -2147200153i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_EXPIRATION_DIR_IN_SCOPE: ::windows_sys::core::HRESULT = -2147200185i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_EXISTS: ::windows_sys::core::HRESULT = -2147200184i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_RUNNING: ::windows_sys::core::HRESULT = -2147200193i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_CUSTOM: ::windows_sys::core::HRESULT = -2147200191i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_DEPRECATED: ::windows_sys::core::HRESULT = -2147200102i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_EXPIRATION: ::windows_sys::core::HRESULT = -2147200192i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_INVALID_CONTINUOUS_CONFIG: ::windows_sys::core::HRESULT = -2147200108i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_MAX_FILE_CONDITIONS: ::windows_sys::core::HRESULT = -2147200146i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_NOTIFICATION: ::windows_sys::core::HRESULT = -2147200190i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_NOT_LEGACY_ACCESSIBLE: ::windows_sys::core::HRESULT = -2147200147i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_RMS: ::windows_sys::core::HRESULT = -2147200120i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_OPEN_ERROR: ::windows_sys::core::HRESULT = -2147200189i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_SYSTEM_CORRUPT: ::windows_sys::core::HRESULT = -2147200225i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INCOMPATIBLE_FORMAT: ::windows_sys::core::HRESULT = -2147200157i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INPROC_MODULE_BLOCKED: ::windows_sys::core::HRESULT = -2147200174i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INSECURE_PATH: ::windows_sys::core::HRESULT = -2147200233i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INSUFFICIENT_DISK: ::windows_sys::core::HRESULT = -2147200236i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_AD_CLAIM: ::windows_sys::core::HRESULT = -2147200142i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_COMBINATION: ::windows_sys::core::HRESULT = -2147200241i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_DATASCREEN_DEFINITION: ::windows_sys::core::HRESULT = -2147200220i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_EMAIL_ADDRESS: ::windows_sys::core::HRESULT = -2147200226i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_FILEGROUP_DEFINITION: ::windows_sys::core::HRESULT = -2147200223i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_FILENAME: ::windows_sys::core::HRESULT = -2147200214i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_FOLDER_PROPERTY_STORE: ::windows_sys::core::HRESULT = -2147200140i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_IMPORT_VERSION: ::windows_sys::core::HRESULT = -2147200245i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_LIMIT: ::windows_sys::core::HRESULT = -2147200249i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_NAME: ::windows_sys::core::HRESULT = -2147200248i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_PATH: ::windows_sys::core::HRESULT = -2147200250i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_REPORT_DESC: ::windows_sys::core::HRESULT = -2147200215i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_REPORT_FORMAT: ::windows_sys::core::HRESULT = -2147200216i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_SCHEDULER_ARGUMENT: ::windows_sys::core::HRESULT = -2147200254i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_SMTP_SERVER: ::windows_sys::core::HRESULT = -2147200232i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_TEXT: ::windows_sys::core::HRESULT = -2147200246i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_USER: ::windows_sys::core::HRESULT = -2147200251i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_LAST_ACCESS_UPDATE_DISABLED: ::windows_sys::core::HRESULT = -2147200176i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_LEGACY_SCHEDULE: ::windows_sys::core::HRESULT = -2147200107i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_LOADING_DISABLED_MODULE: ::windows_sys::core::HRESULT = -2147200202i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_LONG_CMDLINE: ::windows_sys::core::HRESULT = -2147200224i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_MAX_PROPERTY_DEFINITIONS: ::windows_sys::core::HRESULT = -2147200196i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_MESSAGE_LIMIT_EXCEEDED: ::windows_sys::core::HRESULT = -2147200200i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_MODULE_INITIALIZATION: ::windows_sys::core::HRESULT = -2147200150i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_MODULE_INVALID_PARAM: ::windows_sys::core::HRESULT = -2147200151i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_MODULE_SESSION_INITIALIZATION: ::windows_sys::core::HRESULT = -2147200149i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_MODULE_TIMEOUT: ::windows_sys::core::HRESULT = -2147200101i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_NOT_CLUSTER_VOLUME: ::windows_sys::core::HRESULT = -2147200208i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_NOT_FOUND: ::windows_sys::core::HRESULT = -2147200255i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147200239i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_NO_EMAIL_ADDRESS: ::windows_sys::core::HRESULT = -2147200131i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_NO_PROPERTY_VALUE: ::windows_sys::core::HRESULT = -2147200175i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_OBJECT_IN_USE: ::windows_sys::core::HRESULT = -2147200199i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_OUT_OF_RANGE: ::windows_sys::core::HRESULT = -2147200243i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PARTIAL_CLASSIFICATION_PROPERTY_NOT_FOUND: ::windows_sys::core::HRESULT = -2147200169i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PATH_NOT_FOUND: ::windows_sys::core::HRESULT = -2147200252i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PATH_NOT_IN_NAMESPACE: ::windows_sys::core::HRESULT = -2147200129i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PERSIST_PROPERTIES_FAILED: ::windows_sys::core::HRESULT = -2147200155i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PERSIST_PROPERTIES_FAILED_ENCRYPTED: ::windows_sys::core::HRESULT = -2147200166i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PROPERTY_DELETED: ::windows_sys::core::HRESULT = -2147200183i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PROPERTY_MUST_APPLY_TO_FILES: ::windows_sys::core::HRESULT = -2147200138i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PROPERTY_MUST_APPLY_TO_FOLDERS: ::windows_sys::core::HRESULT = -2147200124i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PROPERTY_MUST_BE_GLOBAL: ::windows_sys::core::HRESULT = -2147200122i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PROPERTY_MUST_BE_SECURE: ::windows_sys::core::HRESULT = -2147200123i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_REBUILDING_FODLER_TYPE_INDEX: ::windows_sys::core::HRESULT = -2147200139i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_REPORT_GENERATION_ERR: ::windows_sys::core::HRESULT = -2147200204i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_REPORT_JOB_ALREADY_RUNNING: ::windows_sys::core::HRESULT = -2147200205i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_REPORT_TASK_TRIGGER: ::windows_sys::core::HRESULT = -2147200203i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_REPORT_TYPE_ALREADY_EXISTS: ::windows_sys::core::HRESULT = -2147200206i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_REQD_PARAM_MISSING: ::windows_sys::core::HRESULT = -2147200242i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_RMS_NO_PROTECTORS_INSTALLED: ::windows_sys::core::HRESULT = -2147200126i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_RMS_NO_PROTECTOR_INSTALLED_FOR_FILE: ::windows_sys::core::HRESULT = -2147200125i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_RMS_TEMPLATE_NOT_FOUND: ::windows_sys::core::HRESULT = -2147200128i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_SECURE_PROPERTIES_NOT_SUPPORTED: ::windows_sys::core::HRESULT = -2147200127i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_SET_PROPERTY_FAILED: ::windows_sys::core::HRESULT = -2147200172i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_SHADOW_COPY: ::windows_sys::core::HRESULT = -2147200212i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_STORE_NOT_INSTALLED: ::windows_sys::core::HRESULT = -2147200209i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_SYNC_TASK_HAD_ERRORS: ::windows_sys::core::HRESULT = -2147200119i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_SYNC_TASK_TIMEOUT: ::windows_sys::core::HRESULT = -2147200144i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_TEXTREADER_FILENAME_TOO_LONG: ::windows_sys::core::HRESULT = -2147200158i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_TEXTREADER_IFILTER_CLSID_MALFORMED: ::windows_sys::core::HRESULT = -2147200160i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_TEXTREADER_IFILTER_NOT_FOUND: ::windows_sys::core::HRESULT = -2147200167i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_TEXTREADER_NOT_INITIALIZED: ::windows_sys::core::HRESULT = -2147200168i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_TEXTREADER_STREAM_ERROR: ::windows_sys::core::HRESULT = -2147200159i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_UNEXPECTED: ::windows_sys::core::HRESULT = -2147200234i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_UNSECURE_LINK_TO_HOSTED_MODULE: ::windows_sys::core::HRESULT = -2147200188i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_VOLUME_OFFLINE: ::windows_sys::core::HRESULT = -2147200154i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_VOLUME_UNSUPPORTED: ::windows_sys::core::HRESULT = -2147200235i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_WMI_FAILURE: ::windows_sys::core::HRESULT = -2147200121i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_XML_CORRUPTED: ::windows_sys::core::HRESULT = -2147200211i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_S_CLASSIFICATION_SCAN_FAILURES: ::windows_sys::core::HRESULT = 283398i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_S_PARTIAL_BATCH: ::windows_sys::core::HRESULT = 283396i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_S_PARTIAL_CLASSIFICATION: ::windows_sys::core::HRESULT = 283397i32;
pub const FsrmAccessDeniedRemediationClient: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 269176776, data2: 29889, data3: 18191, data4: [177, 183, 221, 123, 107, 174, 121, 189] };
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmAccountType = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmAccountType_Unknown: FsrmAccountType = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmAccountType_NetworkService: FsrmAccountType = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmAccountType_LocalService: FsrmAccountType = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmAccountType_LocalSystem: FsrmAccountType = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmAccountType_InProc: FsrmAccountType = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmAccountType_External: FsrmAccountType = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmAccountType_Automatic: FsrmAccountType = 500i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmActionType = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmActionType_Unknown: FsrmActionType = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmActionType_EventLog: FsrmActionType = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmActionType_Email: FsrmActionType = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmActionType_Command: FsrmActionType = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmActionType_Report: FsrmActionType = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmClassificationLoggingFlags = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmClassificationLoggingFlags_None: FsrmClassificationLoggingFlags = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmClassificationLoggingFlags_ClassificationsInLogFile: FsrmClassificationLoggingFlags = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmClassificationLoggingFlags_ErrorsInLogFile: FsrmClassificationLoggingFlags = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmClassificationLoggingFlags_ClassificationsInSystemLog: FsrmClassificationLoggingFlags = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmClassificationLoggingFlags_ErrorsInSystemLog: FsrmClassificationLoggingFlags = 8i32;
pub const FsrmClassificationManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2975600199, data2: 50065, data3: 17849, data4: [149, 200, 235, 89, 108, 133, 63, 58] };
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmCollectionState = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmCollectionState_Fetching: FsrmCollectionState = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmCollectionState_Committing: FsrmCollectionState = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmCollectionState_Complete: FsrmCollectionState = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmCollectionState_Cancelled: FsrmCollectionState = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmCommitOptions = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmCommitOptions_None: FsrmCommitOptions = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmCommitOptions_Asynchronous: FsrmCommitOptions = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmDaysNotSpecified: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmEnumOptions = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEnumOptions_None: FsrmEnumOptions = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEnumOptions_Asynchronous: FsrmEnumOptions = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEnumOptions_CheckRecycleBin: FsrmEnumOptions = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEnumOptions_IncludeClusterNodes: FsrmEnumOptions = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEnumOptions_IncludeDeprecatedObjects: FsrmEnumOptions = 8i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmEventType = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEventType_Unknown: FsrmEventType = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEventType_Information: FsrmEventType = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEventType_Warning: FsrmEventType = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEventType_Error: FsrmEventType = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmExecutionOption = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmExecutionOption_Unknown: FsrmExecutionOption = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmExecutionOption_EvaluateUnset: FsrmExecutionOption = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmExecutionOption_ReEvaluate_ConsiderExistingValue: FsrmExecutionOption = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmExecutionOption_ReEvaluate_IgnoreExistingValue: FsrmExecutionOption = 3i32;
pub const FsrmExportImport: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 344120375, data2: 64233, data3: 18311, data4: [144, 37, 140, 228, 224, 36, 171, 86] };
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmFileConditionType = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileConditionType_Unknown: FsrmFileConditionType = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileConditionType_Property: FsrmFileConditionType = 1i32;
pub const FsrmFileGroupManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2400412662, data2: 25967, data3: 17558, data4: [146, 38, 19, 174, 203, 215, 113, 143] };
pub const FsrmFileManagementJobManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3944282546, data2: 19514, data3: 17185, data4: [178, 3, 32, 81, 32, 207, 246, 20] };
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmFileManagementLoggingFlags = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileManagementLoggingFlags_None: FsrmFileManagementLoggingFlags = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileManagementLoggingFlags_Error: FsrmFileManagementLoggingFlags = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileManagementLoggingFlags_Information: FsrmFileManagementLoggingFlags = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileManagementLoggingFlags_Audit: FsrmFileManagementLoggingFlags = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmFileManagementType = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileManagementType_Unknown: FsrmFileManagementType = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileManagementType_Expiration: FsrmFileManagementType = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileManagementType_Custom: FsrmFileManagementType = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileManagementType_Rms: FsrmFileManagementType = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmFileScreenFlags = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileScreenFlags_Enforce: FsrmFileScreenFlags = 1i32;
pub const FsrmFileScreenManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2509508995, data2: 56147, data3: 19551, data4: [179, 123, 125, 9, 33, 207, 157, 199] };
pub const FsrmFileScreenTemplateManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 607195615, data2: 58484, data3: 18090, data4: [160, 84, 234, 163, 62, 220, 41, 42] };
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmFileStreamingInterfaceType = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileStreamingInterfaceType_Unknown: FsrmFileStreamingInterfaceType = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileStreamingInterfaceType_ILockBytes: FsrmFileStreamingInterfaceType = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileStreamingInterfaceType_IStream: FsrmFileStreamingInterfaceType = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmFileStreamingMode = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileStreamingMode_Unknown: FsrmFileStreamingMode = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileStreamingMode_Read: FsrmFileStreamingMode = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileStreamingMode_Write: FsrmFileStreamingMode = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmFileSystemPropertyId = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileSystemPropertyId_Undefined: FsrmFileSystemPropertyId = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileSystemPropertyId_FileName: FsrmFileSystemPropertyId = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileSystemPropertyId_DateCreated: FsrmFileSystemPropertyId = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileSystemPropertyId_DateLastAccessed: FsrmFileSystemPropertyId = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileSystemPropertyId_DateLastModified: FsrmFileSystemPropertyId = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileSystemPropertyId_DateNow: FsrmFileSystemPropertyId = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmGetFilePropertyOptions = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmGetFilePropertyOptions_None: FsrmGetFilePropertyOptions = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmGetFilePropertyOptions_NoRuleEvaluation: FsrmGetFilePropertyOptions = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmGetFilePropertyOptions_Persistent: FsrmGetFilePropertyOptions = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmGetFilePropertyOptions_FailOnPersistErrors: FsrmGetFilePropertyOptions = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmGetFilePropertyOptions_SkipOrphaned: FsrmGetFilePropertyOptions = 8i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmMaxExcludeFolders: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmMaxNumberPropertyDefinitions: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmMaxNumberThresholds: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmMaxThresholdValue: u32 = 250u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmMinQuotaLimit: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmMinThresholdValue: u32 = 1u32;
pub const FsrmPathMapper: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4089332413, data2: 35522, data3: 16542, data4: [187, 216, 250, 249, 182, 180, 31, 235] };
pub const FsrmPipelineModuleConnector: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3345232757, data2: 7861, data3: 17630, data4: [160, 98, 98, 53, 71, 217, 51, 188] };
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmPipelineModuleType = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPipelineModuleType_Unknown: FsrmPipelineModuleType = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPipelineModuleType_Storage: FsrmPipelineModuleType = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPipelineModuleType_Classifier: FsrmPipelineModuleType = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmPropertyBagField = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyBagField_AccessVolume: FsrmPropertyBagField = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyBagField_VolumeGuidName: FsrmPropertyBagField = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmPropertyBagFlags = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyBagFlags_UpdatedByClassifier: FsrmPropertyBagFlags = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyBagFlags_FailedLoadingProperties: FsrmPropertyBagFlags = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyBagFlags_FailedSavingProperties: FsrmPropertyBagFlags = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyBagFlags_FailedClassifyingProperties: FsrmPropertyBagFlags = 8i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmPropertyConditionType = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_Unknown: FsrmPropertyConditionType = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_Equal: FsrmPropertyConditionType = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_NotEqual: FsrmPropertyConditionType = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_GreaterThan: FsrmPropertyConditionType = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_LessThan: FsrmPropertyConditionType = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_Contain: FsrmPropertyConditionType = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_Exist: FsrmPropertyConditionType = 6i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_NotExist: FsrmPropertyConditionType = 7i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_StartWith: FsrmPropertyConditionType = 8i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_EndWith: FsrmPropertyConditionType = 9i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_ContainedIn: FsrmPropertyConditionType = 10i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_PrefixOf: FsrmPropertyConditionType = 11i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_SuffixOf: FsrmPropertyConditionType = 12i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_MatchesPattern: FsrmPropertyConditionType = 13i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmPropertyDefinitionAppliesTo = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionAppliesTo_Files: FsrmPropertyDefinitionAppliesTo = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionAppliesTo_Folders: FsrmPropertyDefinitionAppliesTo = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmPropertyDefinitionFlags = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionFlags_Global: FsrmPropertyDefinitionFlags = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionFlags_Deprecated: FsrmPropertyDefinitionFlags = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionFlags_Secure: FsrmPropertyDefinitionFlags = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmPropertyDefinitionType = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_Unknown: FsrmPropertyDefinitionType = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_OrderedList: FsrmPropertyDefinitionType = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_MultiChoiceList: FsrmPropertyDefinitionType = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_SingleChoiceList: FsrmPropertyDefinitionType = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_String: FsrmPropertyDefinitionType = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_MultiString: FsrmPropertyDefinitionType = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_Int: FsrmPropertyDefinitionType = 6i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_Bool: FsrmPropertyDefinitionType = 7i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_Date: FsrmPropertyDefinitionType = 8i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmPropertyFlags = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_None: FsrmPropertyFlags = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_Orphaned: FsrmPropertyFlags = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_RetrievedFromCache: FsrmPropertyFlags = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_RetrievedFromStorage: FsrmPropertyFlags = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_SetByClassifier: FsrmPropertyFlags = 8i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_Deleted: FsrmPropertyFlags = 16i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_Reclassified: FsrmPropertyFlags = 32i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_AggregationFailed: FsrmPropertyFlags = 64i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_Existing: FsrmPropertyFlags = 128i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_FailedLoadingProperties: FsrmPropertyFlags = 256i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_FailedClassifyingProperties: FsrmPropertyFlags = 512i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_FailedSavingProperties: FsrmPropertyFlags = 1024i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_Secure: FsrmPropertyFlags = 2048i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_PolicyDerived: FsrmPropertyFlags = 4096i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_Inherited: FsrmPropertyFlags = 8192i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_Manual: FsrmPropertyFlags = 16384i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_ExplicitValueDeleted: FsrmPropertyFlags = 32768i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_PropertyDeletedFromClear: FsrmPropertyFlags = 65536i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_PropertySourceMask: FsrmPropertyFlags = 14i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_PersistentMask: FsrmPropertyFlags = 20480i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmPropertyValueType = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyValueType_Undefined: FsrmPropertyValueType = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyValueType_Literal: FsrmPropertyValueType = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyValueType_DateOffset: FsrmPropertyValueType = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmQuotaFlags = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmQuotaFlags_Enforce: FsrmQuotaFlags = 256i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmQuotaFlags_Disable: FsrmQuotaFlags = 512i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmQuotaFlags_StatusIncomplete: FsrmQuotaFlags = 65536i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmQuotaFlags_StatusRebuilding: FsrmQuotaFlags = 131072i32;
pub const FsrmQuotaManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2430380927, data2: 13436, data3: 19452, data4: [181, 67, 84, 3, 38, 48, 95, 190] };
pub const FsrmQuotaTemplateManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2547242051, data2: 9500, data3: 17207, data4: [129, 231, 179, 46, 143, 78, 230, 94] };
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmReportFilter = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFilter_MinSize: FsrmReportFilter = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFilter_MinAgeDays: FsrmReportFilter = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFilter_MaxAgeDays: FsrmReportFilter = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFilter_MinQuotaUsage: FsrmReportFilter = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFilter_FileGroups: FsrmReportFilter = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFilter_Owners: FsrmReportFilter = 6i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFilter_NamePattern: FsrmReportFilter = 7i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFilter_Property: FsrmReportFilter = 8i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmReportFormat = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFormat_Unknown: FsrmReportFormat = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFormat_DHtml: FsrmReportFormat = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFormat_Html: FsrmReportFormat = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFormat_Txt: FsrmReportFormat = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFormat_Csv: FsrmReportFormat = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFormat_Xml: FsrmReportFormat = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmReportGenerationContext = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportGenerationContext_Undefined: FsrmReportGenerationContext = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportGenerationContext_ScheduledReport: FsrmReportGenerationContext = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportGenerationContext_InteractiveReport: FsrmReportGenerationContext = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportGenerationContext_IncidentReport: FsrmReportGenerationContext = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmReportLimit = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxFiles: FsrmReportLimit = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxFileGroups: FsrmReportLimit = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxOwners: FsrmReportLimit = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxFilesPerFileGroup: FsrmReportLimit = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxFilesPerOwner: FsrmReportLimit = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxFilesPerDuplGroup: FsrmReportLimit = 6i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxDuplicateGroups: FsrmReportLimit = 7i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxQuotas: FsrmReportLimit = 8i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxFileScreenEvents: FsrmReportLimit = 9i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxPropertyValues: FsrmReportLimit = 10i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxFilesPerPropertyValue: FsrmReportLimit = 11i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxFolders: FsrmReportLimit = 12i32;
pub const FsrmReportManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 5828407, data2: 43622, data3: 19528, data4: [189, 91, 47, 206, 67, 42, 176, 200] };
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmReportRunningStatus = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportRunningStatus_Unknown: FsrmReportRunningStatus = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportRunningStatus_NotRunning: FsrmReportRunningStatus = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportRunningStatus_Queued: FsrmReportRunningStatus = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportRunningStatus_Running: FsrmReportRunningStatus = 3i32;
pub const FsrmReportScheduler: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3928355256, data2: 7053, data3: 17040, data4: [142, 232, 225, 124, 18, 194, 254, 32] };
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmReportType = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_Unknown: FsrmReportType = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_LargeFiles: FsrmReportType = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_FilesByType: FsrmReportType = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_LeastRecentlyAccessed: FsrmReportType = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_MostRecentlyAccessed: FsrmReportType = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_QuotaUsage: FsrmReportType = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_FilesByOwner: FsrmReportType = 6i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_ExportReport: FsrmReportType = 7i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_DuplicateFiles: FsrmReportType = 8i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_FileScreenAudit: FsrmReportType = 9i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_FilesByProperty: FsrmReportType = 10i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_AutomaticClassification: FsrmReportType = 11i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_Expiration: FsrmReportType = 12i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_FoldersByProperty: FsrmReportType = 13i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmRuleFlags = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmRuleFlags_Disabled: FsrmRuleFlags = 256i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmRuleFlags_ClearAutomaticallyClassifiedProperty: FsrmRuleFlags = 1024i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmRuleFlags_ClearManuallyClassifiedProperty: FsrmRuleFlags = 2048i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmRuleFlags_Invalid: FsrmRuleFlags = 4096i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmRuleType = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmRuleType_Unknown: FsrmRuleType = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmRuleType_Classification: FsrmRuleType = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmRuleType_Generic: FsrmRuleType = 2i32;
pub const FsrmSetting: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4116109064, data2: 27981, data3: 17812, data4: [156, 97, 125, 187, 13, 174, 42, 70] };
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmStorageModuleCaps = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleCaps_Unknown: FsrmStorageModuleCaps = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleCaps_CanGet: FsrmStorageModuleCaps = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleCaps_CanSet: FsrmStorageModuleCaps = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleCaps_CanHandleDirectories: FsrmStorageModuleCaps = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleCaps_CanHandleFiles: FsrmStorageModuleCaps = 8i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmStorageModuleType = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleType_Unknown: FsrmStorageModuleType = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleType_Cache: FsrmStorageModuleType = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleType_InFile: FsrmStorageModuleType = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleType_Database: FsrmStorageModuleType = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleType_System: FsrmStorageModuleType = 100i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub type FsrmTemplateApplyOptions = i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmTemplateApplyOptions_ApplyToDerivedMatching: FsrmTemplateApplyOptions = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmTemplateApplyOptions_ApplyToDerivedAll: FsrmTemplateApplyOptions = 2i32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmAccessDeniedRemediationClient {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut *mut Self, parentwnd: usize, accesspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, errortype: AdrClientErrorType, flags: i32, windowtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, windowmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, result: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Show: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmAccessDeniedRemediationClient {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1073750804, data2: 22795, data3: 17829, data4: [142, 27, 140, 5, 218, 82, 126, 82] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmAction {
    pub base__: super::super::System::Com::IDispatch,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, id: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ActionType: unsafe extern "system" fn(this: *mut *mut Self, actiontype: *mut FsrmActionType) -> ::windows_sys::core::HRESULT,
    pub RunLimitInterval: unsafe extern "system" fn(this: *mut *mut Self, minutes: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetRunLimitInterval: unsafe extern "system" fn(this: *mut *mut Self, minutes: i32) -> ::windows_sys::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmAction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1825980554, data2: 44640, data3: 17979, data4: [158, 241, 225, 23, 83, 77, 105, 220] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmActionCommand {
    pub base__: IFsrmAction,
    #[cfg(feature = "Win32_Foundation")]
    pub ExecutablePath: unsafe extern "system" fn(this: *mut *mut Self, executablepath: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExecutablePath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetExecutablePath: unsafe extern "system" fn(this: *mut *mut Self, executablepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetExecutablePath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Arguments: unsafe extern "system" fn(this: *mut *mut Self, arguments: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Arguments: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetArguments: unsafe extern "system" fn(this: *mut *mut Self, arguments: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetArguments: usize,
    pub Account: unsafe extern "system" fn(this: *mut *mut Self, account: *mut FsrmAccountType) -> ::windows_sys::core::HRESULT,
    pub SetAccount: unsafe extern "system" fn(this: *mut *mut Self, account: FsrmAccountType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WorkingDirectory: unsafe extern "system" fn(this: *mut *mut Self, workingdirectory: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WorkingDirectory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWorkingDirectory: unsafe extern "system" fn(this: *mut *mut Self, workingdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWorkingDirectory: usize,
    pub MonitorCommand: unsafe extern "system" fn(this: *mut *mut Self, monitorcommand: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetMonitorCommand: unsafe extern "system" fn(this: *mut *mut Self, monitorcommand: i16) -> ::windows_sys::core::HRESULT,
    pub KillTimeOut: unsafe extern "system" fn(this: *mut *mut Self, minutes: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetKillTimeOut: unsafe extern "system" fn(this: *mut *mut Self, minutes: i32) -> ::windows_sys::core::HRESULT,
    pub LogResult: unsafe extern "system" fn(this: *mut *mut Self, logresults: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetLogResult: unsafe extern "system" fn(this: *mut *mut Self, logresults: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmActionCommand {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 311654281, data2: 57927, data3: 18711, data4: [156, 32, 243, 238, 156, 126, 231, 131] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmActionEmail {
    pub base__: IFsrmAction,
    #[cfg(feature = "Win32_Foundation")]
    pub MailFrom: unsafe extern "system" fn(this: *mut *mut Self, mailfrom: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MailFrom: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMailFrom: unsafe extern "system" fn(this: *mut *mut Self, mailfrom: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMailFrom: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MailReplyTo: unsafe extern "system" fn(this: *mut *mut Self, mailreplyto: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MailReplyTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMailReplyTo: unsafe extern "system" fn(this: *mut *mut Self, mailreplyto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMailReplyTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MailTo: unsafe extern "system" fn(this: *mut *mut Self, mailto: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MailTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMailTo: unsafe extern "system" fn(this: *mut *mut Self, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMailTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MailCc: unsafe extern "system" fn(this: *mut *mut Self, mailcc: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MailCc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMailCc: unsafe extern "system" fn(this: *mut *mut Self, mailcc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMailCc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MailBcc: unsafe extern "system" fn(this: *mut *mut Self, mailbcc: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MailBcc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMailBcc: unsafe extern "system" fn(this: *mut *mut Self, mailbcc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMailBcc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MailSubject: unsafe extern "system" fn(this: *mut *mut Self, mailsubject: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MailSubject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMailSubject: unsafe extern "system" fn(this: *mut *mut Self, mailsubject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMailSubject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MessageText: unsafe extern "system" fn(this: *mut *mut Self, messagetext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MessageText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMessageText: unsafe extern "system" fn(this: *mut *mut Self, messagetext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMessageText: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmActionEmail {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3594933885, data2: 9902, data3: 19626, data4: [159, 132, 78, 10, 173, 32, 127, 202] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmActionEmail2 {
    pub base__: IFsrmActionEmail,
    pub AttachmentFileListSize: unsafe extern "system" fn(this: *mut *mut Self, attachmentfilelistsize: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAttachmentFileListSize: unsafe extern "system" fn(this: *mut *mut Self, attachmentfilelistsize: i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmActionEmail2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2188800047, data2: 9522, data3: 18489, data4: [137, 191, 72, 114, 96, 154, 46, 164] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmActionEventLog {
    pub base__: IFsrmAction,
    pub EventType: unsafe extern "system" fn(this: *mut *mut Self, eventtype: *mut FsrmEventType) -> ::windows_sys::core::HRESULT,
    pub SetEventType: unsafe extern "system" fn(this: *mut *mut Self, eventtype: FsrmEventType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MessageText: unsafe extern "system" fn(this: *mut *mut Self, messagetext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MessageText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMessageText: unsafe extern "system" fn(this: *mut *mut Self, messagetext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMessageText: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmActionEventLog {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1284478659, data2: 23956, data3: 20279, data4: [164, 244, 245, 106, 180, 99, 84, 111] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmActionReport {
    pub base__: IFsrmAction,
    #[cfg(feature = "Win32_System_Com")]
    pub ReportTypes: unsafe extern "system" fn(this: *mut *mut Self, reporttypes: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReportTypes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetReportTypes: unsafe extern "system" fn(this: *mut *mut Self, reporttypes: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetReportTypes: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MailTo: unsafe extern "system" fn(this: *mut *mut Self, mailto: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MailTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMailTo: unsafe extern "system" fn(this: *mut *mut Self, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMailTo: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmActionReport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 767452100, data2: 45888, data3: 18592, data4: [165, 176, 21, 142, 7, 252, 86, 126] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmAutoApplyQuota {
    pub base__: IFsrmQuotaObject,
    #[cfg(feature = "Win32_System_Com")]
    pub ExcludeFolders: unsafe extern "system" fn(this: *mut *mut Self, folders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExcludeFolders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetExcludeFolders: unsafe extern "system" fn(this: *mut *mut Self, folders: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetExcludeFolders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CommitAndUpdateDerived: unsafe extern "system" fn(this: *mut *mut Self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommitAndUpdateDerived: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmAutoApplyQuota {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4163786537, data2: 27322, data3: 18240, data4: [191, 199, 199, 245, 143, 117, 251, 123] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmClassificationManager {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub ClassificationReportFormats: unsafe extern "system" fn(this: *mut *mut Self, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ClassificationReportFormats: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetClassificationReportFormats: unsafe extern "system" fn(this: *mut *mut Self, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetClassificationReportFormats: usize,
    pub Logging: unsafe extern "system" fn(this: *mut *mut Self, logging: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetLogging: unsafe extern "system" fn(this: *mut *mut Self, logging: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ClassificationReportMailTo: unsafe extern "system" fn(this: *mut *mut Self, mailto: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClassificationReportMailTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClassificationReportMailTo: unsafe extern "system" fn(this: *mut *mut Self, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClassificationReportMailTo: usize,
    pub ClassificationReportEnabled: unsafe extern "system" fn(this: *mut *mut Self, reportenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetClassificationReportEnabled: unsafe extern "system" fn(this: *mut *mut Self, reportenabled: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ClassificationLastReportPathWithoutExtension: unsafe extern "system" fn(this: *mut *mut Self, lastreportpath: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClassificationLastReportPathWithoutExtension: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ClassificationLastError: unsafe extern "system" fn(this: *mut *mut Self, lasterror: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClassificationLastError: usize,
    pub ClassificationRunningStatus: unsafe extern "system" fn(this: *mut *mut Self, runningstatus: *mut FsrmReportRunningStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumPropertyDefinitions: unsafe extern "system" fn(this: *mut *mut Self, options: FsrmEnumOptions, propertydefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumPropertyDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePropertyDefinition: unsafe extern "system" fn(this: *mut *mut Self, propertydefinition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePropertyDefinition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetPropertyDefinition: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertydefinition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetPropertyDefinition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumRules: unsafe extern "system" fn(this: *mut *mut Self, ruletype: FsrmRuleType, options: FsrmEnumOptions, rules: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumRules: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRule: unsafe extern "system" fn(this: *mut *mut Self, ruletype: FsrmRuleType, rule: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRule: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetRule: unsafe extern "system" fn(this: *mut *mut Self, rulename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ruletype: FsrmRuleType, rule: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetRule: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumModuleDefinitions: unsafe extern "system" fn(this: *mut *mut Self, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions, moduledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumModuleDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateModuleDefinition: unsafe extern "system" fn(this: *mut *mut Self, moduletype: FsrmPipelineModuleType, moduledefinition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateModuleDefinition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetModuleDefinition: unsafe extern "system" fn(this: *mut *mut Self, modulename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, moduletype: FsrmPipelineModuleType, moduledefinition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetModuleDefinition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RunClassification: unsafe extern "system" fn(this: *mut *mut Self, context: FsrmReportGenerationContext, reserved: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RunClassification: usize,
    pub WaitForClassificationCompletion: unsafe extern "system" fn(this: *mut *mut Self, waitseconds: i32, completed: *mut i16) -> ::windows_sys::core::HRESULT,
    pub CancelClassification: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub EnumFileProperties: unsafe extern "system" fn(this: *mut *mut Self, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmGetFilePropertyOptions, fileproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    EnumFileProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetFileProperty: unsafe extern "system" fn(this: *mut *mut Self, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmGetFilePropertyOptions, property: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetFileProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFileProperty: unsafe extern "system" fn(this: *mut *mut Self, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFileProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ClearFileProperty: unsafe extern "system" fn(this: *mut *mut Self, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, property: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClearFileProperty: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmClassificationManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3537668570, data2: 61073, data3: 18592, data4: [133, 216, 204, 114, 165, 111, 125, 4] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmClassificationManager2 {
    pub base__: IFsrmClassificationManager,
    #[cfg(feature = "Win32_System_Com")]
    pub ClassifyFiles: unsafe extern "system" fn(this: *mut *mut Self, filepaths: *const super::super::System::Com::SAFEARRAY, propertynames: *const super::super::System::Com::SAFEARRAY, propertyvalues: *const super::super::System::Com::SAFEARRAY, options: FsrmGetFilePropertyOptions) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ClassifyFiles: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmClassificationManager2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 311753, data2: 4734, data3: 18277, data4: [186, 7, 106, 49, 71, 188, 161, 18] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmClassificationRule {
    pub base__: IFsrmRule,
    pub ExecutionOption: unsafe extern "system" fn(this: *mut *mut Self, executionoption: *mut FsrmExecutionOption) -> ::windows_sys::core::HRESULT,
    pub SetExecutionOption: unsafe extern "system" fn(this: *mut *mut Self, executionoption: FsrmExecutionOption) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PropertyAffected: unsafe extern "system" fn(this: *mut *mut Self, property: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PropertyAffected: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPropertyAffected: unsafe extern "system" fn(this: *mut *mut Self, property: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPropertyAffected: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Value: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetValue: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmClassificationRule {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2948616898, data2: 21269, data3: 17835, data4: [132, 27, 198, 219, 14, 18, 1, 72] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmClassifierModuleDefinition {
    pub base__: IFsrmPipelineModuleDefinition,
    #[cfg(feature = "Win32_System_Com")]
    pub PropertiesAffected: unsafe extern "system" fn(this: *mut *mut Self, propertiesaffected: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PropertiesAffected: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPropertiesAffected: unsafe extern "system" fn(this: *mut *mut Self, propertiesaffected: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPropertiesAffected: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PropertiesUsed: unsafe extern "system" fn(this: *mut *mut Self, propertiesused: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PropertiesUsed: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPropertiesUsed: unsafe extern "system" fn(this: *mut *mut Self, propertiesused: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPropertiesUsed: usize,
    pub NeedsExplicitValue: unsafe extern "system" fn(this: *mut *mut Self, needsexplicitvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetNeedsExplicitValue: unsafe extern "system" fn(this: *mut *mut Self, needsexplicitvalue: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmClassifierModuleDefinition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3140938278, data2: 25368, data3: 19340, data4: [133, 146, 247, 45, 214, 2, 231, 165] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmClassifierModuleImplementation {
    pub base__: IFsrmPipelineModuleImplementation,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LastModified: unsafe extern "system" fn(this: *mut *mut Self, lastmodified: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LastModified: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UseRulesAndDefinitions: unsafe extern "system" fn(this: *mut *mut Self, rules: *mut ::core::ffi::c_void, propertydefinitions: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UseRulesAndDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnBeginFile: unsafe extern "system" fn(this: *mut *mut Self, propertybag: *mut ::core::ffi::c_void, arrayruleids: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnBeginFile: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DoesPropertyValueApply: unsafe extern "system" fn(this: *mut *mut Self, property: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, applyvalue: *mut i16, idrule: ::windows_sys::core::GUID, idpropdef: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoesPropertyValueApply: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPropertyValueToApply: unsafe extern "system" fn(this: *mut *mut Self, property: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::super::Foundation::BSTR, idrule: ::windows_sys::core::GUID, idpropdef: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPropertyValueToApply: usize,
    pub OnEndFile: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmClassifierModuleImplementation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1284935622, data2: 28379, data3: 16465, data4: [156, 24, 115, 183, 41, 26, 225, 6] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmCollection {
    pub base__: super::super::System::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, unknown: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, item: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, state: *mut FsrmCollectionState) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub WaitForCompletion: unsafe extern "system" fn(this: *mut *mut Self, waitseconds: i32, completed: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetById: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::GUID, entry: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetById: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4151295803, data2: 36317, data3: 19266, data4: [176, 90, 203, 28, 63, 241, 254, 232] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmCommittableCollection {
    pub base__: IFsrmMutableCollection,
    #[cfg(feature = "Win32_System_Com")]
    pub Commit: unsafe extern "system" fn(this: *mut *mut Self, options: FsrmCommitOptions, results: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Commit: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmCommittableCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2531177397, data2: 35729, data3: 18986, data4: [157, 147, 128, 163, 93, 138, 168, 71] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmDerivedObjectsResult {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub DerivedObjects: unsafe extern "system" fn(this: *mut *mut Self, derivedobjects: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DerivedObjects: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Results: unsafe extern "system" fn(this: *mut *mut Self, results: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Results: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmDerivedObjectsResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 959588909, data2: 14574, data3: 19725, data4: [128, 149, 66, 26, 128, 132, 154, 130] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmExportImport {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ExportFileGroups: unsafe extern "system" fn(this: *mut *mut Self, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ExportFileGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ImportFileGroups: unsafe extern "system" fn(this: *mut *mut Self, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroups: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ImportFileGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ExportFileScreenTemplates: unsafe extern "system" fn(this: *mut *mut Self, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ExportFileScreenTemplates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ImportFileScreenTemplates: unsafe extern "system" fn(this: *mut *mut Self, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templates: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ImportFileScreenTemplates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ExportQuotaTemplates: unsafe extern "system" fn(this: *mut *mut Self, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ExportQuotaTemplates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ImportQuotaTemplates: unsafe extern "system" fn(this: *mut *mut Self, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templates: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ImportQuotaTemplates: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmExportImport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4023061169, data2: 5828, data3: 19065, data4: [129, 44, 114, 86, 20, 195, 48, 107] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileCondition {
    pub base__: super::super::System::Com::IDispatch,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut FsrmFileConditionType) -> ::windows_sys::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmFileCondition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1885884412, data2: 26906, data3: 18970, data4: [185, 34, 151, 117, 46, 19, 140, 193] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileConditionProperty {
    pub base__: IFsrmFileCondition,
    #[cfg(feature = "Win32_Foundation")]
    pub PropertyName: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PropertyName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPropertyName: unsafe extern "system" fn(this: *mut *mut Self, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPropertyName: usize,
    pub PropertyId: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut FsrmFileSystemPropertyId) -> ::windows_sys::core::HRESULT,
    pub SetPropertyId: unsafe extern "system" fn(this: *mut *mut Self, newval: FsrmFileSystemPropertyId) -> ::windows_sys::core::HRESULT,
    pub Operator: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut FsrmPropertyConditionType) -> ::windows_sys::core::HRESULT,
    pub SetOperator: unsafe extern "system" fn(this: *mut *mut Self, newval: FsrmPropertyConditionType) -> ::windows_sys::core::HRESULT,
    pub ValueType: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut FsrmPropertyValueType) -> ::windows_sys::core::HRESULT,
    pub SetValueType: unsafe extern "system" fn(this: *mut *mut Self, newval: FsrmPropertyValueType) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, newval: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmFileConditionProperty {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2173855605, data2: 47489, data3: 17529, data4: [152, 143, 218, 23, 29, 98, 115, 96] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileGroup {
    pub base__: IFsrmObject,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Members: unsafe extern "system" fn(this: *mut *mut Self, members: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Members: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetMembers: unsafe extern "system" fn(this: *mut *mut Self, members: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetMembers: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NonMembers: unsafe extern "system" fn(this: *mut *mut Self, nonmembers: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NonMembers: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetNonMembers: unsafe extern "system" fn(this: *mut *mut Self, nonmembers: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetNonMembers: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmFileGroup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2379237641, data2: 3636, data3: 19797, data4: [175, 170, 137, 225, 241, 161, 187, 185] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileGroupImported {
    pub base__: IFsrmFileGroup,
    pub OverwriteOnCommit: unsafe extern "system" fn(this: *mut *mut Self, overwrite: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetOverwriteOnCommit: unsafe extern "system" fn(this: *mut *mut Self, overwrite: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmFileGroupImported {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2908090635, data2: 24337, data3: 19431, data4: [148, 239, 217, 238, 46, 71, 13, 237] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileGroupManager {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFileGroup: unsafe extern "system" fn(this: *mut *mut Self, filegroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFileGroup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetFileGroup: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroup: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetFileGroup: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumFileGroups: unsafe extern "system" fn(this: *mut *mut Self, options: FsrmEnumOptions, filegroups: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumFileGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ExportFileGroups: unsafe extern "system" fn(this: *mut *mut Self, filegroupnamesarray: *const super::super::System::Com::VARIANT, serializedfilegroups: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ExportFileGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ImportFileGroups: unsafe extern "system" fn(this: *mut *mut Self, serializedfilegroups: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroupnamesarray: *const super::super::System::Com::VARIANT, filegroups: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ImportFileGroups: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmFileGroupManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1114011605, data2: 396, data3: 18524, data4: [138, 81, 32, 184, 109, 0, 189, 196] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileManagementJob {
    pub base__: IFsrmObject,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NamespaceRoots: unsafe extern "system" fn(this: *mut *mut Self, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NamespaceRoots: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetNamespaceRoots: unsafe extern "system" fn(this: *mut *mut Self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetNamespaceRoots: usize,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: i16) -> ::windows_sys::core::HRESULT,
    pub OperationType: unsafe extern "system" fn(this: *mut *mut Self, operationtype: *mut FsrmFileManagementType) -> ::windows_sys::core::HRESULT,
    pub SetOperationType: unsafe extern "system" fn(this: *mut *mut Self, operationtype: FsrmFileManagementType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExpirationDirectory: unsafe extern "system" fn(this: *mut *mut Self, expirationdirectory: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExpirationDirectory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetExpirationDirectory: unsafe extern "system" fn(this: *mut *mut Self, expirationdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetExpirationDirectory: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CustomAction: unsafe extern "system" fn(this: *mut *mut Self, action: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CustomAction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Notifications: unsafe extern "system" fn(this: *mut *mut Self, notifications: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Notifications: usize,
    pub Logging: unsafe extern "system" fn(this: *mut *mut Self, loggingflags: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetLogging: unsafe extern "system" fn(this: *mut *mut Self, loggingflags: i32) -> ::windows_sys::core::HRESULT,
    pub ReportEnabled: unsafe extern "system" fn(this: *mut *mut Self, reportenabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetReportEnabled: unsafe extern "system" fn(this: *mut *mut Self, reportenabled: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Formats: unsafe extern "system" fn(this: *mut *mut Self, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Formats: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetFormats: unsafe extern "system" fn(this: *mut *mut Self, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetFormats: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MailTo: unsafe extern "system" fn(this: *mut *mut Self, mailto: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MailTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMailTo: unsafe extern "system" fn(this: *mut *mut Self, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMailTo: usize,
    pub DaysSinceFileCreated: unsafe extern "system" fn(this: *mut *mut Self, dayssincecreation: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDaysSinceFileCreated: unsafe extern "system" fn(this: *mut *mut Self, dayssincecreation: i32) -> ::windows_sys::core::HRESULT,
    pub DaysSinceFileLastAccessed: unsafe extern "system" fn(this: *mut *mut Self, dayssinceaccess: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDaysSinceFileLastAccessed: unsafe extern "system" fn(this: *mut *mut Self, dayssinceaccess: i32) -> ::windows_sys::core::HRESULT,
    pub DaysSinceFileLastModified: unsafe extern "system" fn(this: *mut *mut Self, dayssincemodify: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDaysSinceFileLastModified: unsafe extern "system" fn(this: *mut *mut Self, dayssincemodify: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PropertyConditions: unsafe extern "system" fn(this: *mut *mut Self, propertyconditions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PropertyConditions: usize,
    pub FromDate: unsafe extern "system" fn(this: *mut *mut Self, fromdate: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFromDate: unsafe extern "system" fn(this: *mut *mut Self, fromdate: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Task: unsafe extern "system" fn(this: *mut *mut Self, taskname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Task: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTask: unsafe extern "system" fn(this: *mut *mut Self, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTask: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Parameters: unsafe extern "system" fn(this: *mut *mut Self, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetParameters: unsafe extern "system" fn(this: *mut *mut Self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetParameters: usize,
    pub RunningStatus: unsafe extern "system" fn(this: *mut *mut Self, runningstatus: *mut FsrmReportRunningStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LastError: unsafe extern "system" fn(this: *mut *mut Self, lasterror: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LastError: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LastReportPathWithoutExtension: unsafe extern "system" fn(this: *mut *mut Self, path: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LastReportPathWithoutExtension: usize,
    pub LastRun: unsafe extern "system" fn(this: *mut *mut Self, lastrun: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FileNamePattern: unsafe extern "system" fn(this: *mut *mut Self, filenamepattern: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FileNamePattern: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFileNamePattern: unsafe extern "system" fn(this: *mut *mut Self, filenamepattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFileNamePattern: usize,
    pub Run: unsafe extern "system" fn(this: *mut *mut Self, context: FsrmReportGenerationContext) -> ::windows_sys::core::HRESULT,
    pub WaitForCompletion: unsafe extern "system" fn(this: *mut *mut Self, waitseconds: i32, completed: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub AddNotification: unsafe extern "system" fn(this: *mut *mut Self, days: i32) -> ::windows_sys::core::HRESULT,
    pub DeleteNotification: unsafe extern "system" fn(this: *mut *mut Self, days: i32) -> ::windows_sys::core::HRESULT,
    pub ModifyNotification: unsafe extern "system" fn(this: *mut *mut Self, days: i32, newdays: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateNotificationAction: unsafe extern "system" fn(this: *mut *mut Self, days: i32, actiontype: FsrmActionType, action: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateNotificationAction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumNotificationActions: unsafe extern "system" fn(this: *mut *mut Self, days: i32, actions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumNotificationActions: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreatePropertyCondition: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertycondition: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreatePropertyCondition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateCustomAction: unsafe extern "system" fn(this: *mut *mut Self, customaction: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateCustomAction: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmFileManagementJob {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 124807294, data2: 40758, data3: 19823, data4: [135, 120, 89, 157, 24, 132, 97, 201] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileManagementJobManager {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub ActionVariables: unsafe extern "system" fn(this: *mut *mut Self, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActionVariables: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ActionVariableDescriptions: unsafe extern "system" fn(this: *mut *mut Self, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActionVariableDescriptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumFileManagementJobs: unsafe extern "system" fn(this: *mut *mut Self, options: FsrmEnumOptions, filemanagementjobs: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumFileManagementJobs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFileManagementJob: unsafe extern "system" fn(this: *mut *mut Self, filemanagementjob: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFileManagementJob: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetFileManagementJob: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filemanagementjob: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetFileManagementJob: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmFileManagementJobManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3996262091, data2: 55646, data3: 18665, data4: [144, 124, 199, 104, 90, 1, 50, 53] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileScreen {
    pub base__: IFsrmFileScreenBase,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, path: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SourceTemplateName: unsafe extern "system" fn(this: *mut *mut Self, filescreentemplatename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SourceTemplateName: usize,
    pub MatchesSourceTemplate: unsafe extern "system" fn(this: *mut *mut Self, matches: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UserSid: unsafe extern "system" fn(this: *mut *mut Self, usersid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserSid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UserAccount: unsafe extern "system" fn(this: *mut *mut Self, useraccount: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserAccount: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ApplyTemplate: unsafe extern "system" fn(this: *mut *mut Self, filescreentemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ApplyTemplate: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmFileScreen {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1600333267, data2: 52872, data3: 18227, data4: [132, 193, 45, 106, 239, 197, 234, 7] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileScreenBase {
    pub base__: IFsrmObject,
    #[cfg(feature = "Win32_System_Com")]
    pub BlockedFileGroups: unsafe extern "system" fn(this: *mut *mut Self, blocklist: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BlockedFileGroups: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetBlockedFileGroups: unsafe extern "system" fn(this: *mut *mut Self, blocklist: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetBlockedFileGroups: usize,
    pub FileScreenFlags: unsafe extern "system" fn(this: *mut *mut Self, filescreenflags: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetFileScreenFlags: unsafe extern "system" fn(this: *mut *mut Self, filescreenflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateAction: unsafe extern "system" fn(this: *mut *mut Self, actiontype: FsrmActionType, action: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateAction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumActions: unsafe extern "system" fn(this: *mut *mut Self, actions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumActions: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmFileScreenBase {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4083383936, data2: 23330, data3: 18987, data4: [166, 55, 187, 182, 66, 180, 28, 252] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileScreenException {
    pub base__: IFsrmObject,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, path: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AllowedFileGroups: unsafe extern "system" fn(this: *mut *mut Self, allowlist: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AllowedFileGroups: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetAllowedFileGroups: unsafe extern "system" fn(this: *mut *mut Self, allowlist: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetAllowedFileGroups: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmFileScreenException {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3202862594, data2: 57207, data3: 17685, data4: [147, 137, 120, 240, 28, 90, 252, 26] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileScreenManager {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub ActionVariables: unsafe extern "system" fn(this: *mut *mut Self, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActionVariables: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ActionVariableDescriptions: unsafe extern "system" fn(this: *mut *mut Self, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActionVariableDescriptions: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateFileScreen: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreen: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateFileScreen: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetFileScreen: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreen: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetFileScreen: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub EnumFileScreens: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, filescreens: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    EnumFileScreens: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateFileScreenException: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreenexception: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateFileScreenException: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetFileScreenException: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreenexception: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetFileScreenException: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub EnumFileScreenExceptions: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, filescreenexceptions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    EnumFileScreenExceptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFileScreenCollection: unsafe extern "system" fn(this: *mut *mut Self, collection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFileScreenCollection: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmFileScreenManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4283408462, data2: 23188, data3: 19418, data4: [163, 160, 213, 180, 211, 197, 46, 186] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileScreenTemplate {
    pub base__: IFsrmFileScreenBase,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CopyTemplate: unsafe extern "system" fn(this: *mut *mut Self, filescreentemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CopyTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CommitAndUpdateDerived: unsafe extern "system" fn(this: *mut *mut Self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommitAndUpdateDerived: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmFileScreenTemplate {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 542895096, data2: 56723, data3: 17706, data4: [149, 166, 50, 181, 102, 179, 88, 40] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileScreenTemplateImported {
    pub base__: IFsrmFileScreenTemplate,
    pub OverwriteOnCommit: unsafe extern "system" fn(this: *mut *mut Self, overwrite: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetOverwriteOnCommit: unsafe extern "system" fn(this: *mut *mut Self, overwrite: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmFileScreenTemplateImported {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3774939993, data2: 15965, data3: 20173, data4: [159, 228, 239, 72, 98, 47, 223, 48] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmFileScreenTemplateManager {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateTemplate: unsafe extern "system" fn(this: *mut *mut Self, filescreentemplate: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateTemplate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetTemplate: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreentemplate: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumTemplates: unsafe extern "system" fn(this: *mut *mut Self, options: FsrmEnumOptions, filescreentemplates: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumTemplates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ExportTemplates: unsafe extern "system" fn(this: *mut *mut Self, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT, serializedfilescreentemplates: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ExportTemplates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ImportTemplates: unsafe extern "system" fn(this: *mut *mut Self, serializedfilescreentemplates: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT, filescreentemplates: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ImportTemplates: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmFileScreenTemplateManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3487788218, data2: 6473, data3: 20084, data4: [161, 79, 241, 213, 128, 206, 175, 19] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmMutableCollection {
    pub base__: IFsrmCollection,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, item: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
    pub RemoveById: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, collection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmMutableCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 464918456, data2: 14470, data3: 18908, data4: [175, 130, 166, 201, 15, 163, 93, 218] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmObject {
    pub base__: super::super::System::Com::IDispatch,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, id: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, description: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 582807443, data2: 19007, data3: 16771, data4: [137, 249, 47, 139, 138, 98, 138, 238] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmPathMapper {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetSharePathsForLocalPath: unsafe extern "system" fn(this: *mut *mut Self, localpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sharepaths: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetSharePathsForLocalPath: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmPathMapper {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1867366399, data2: 26912, data3: 18465, data4: [166, 195, 183, 233, 76, 31, 214, 12] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmPipelineModuleConnector {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub ModuleImplementation: unsafe extern "system" fn(this: *mut *mut Self, pipelinemoduleimplementation: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ModuleImplementation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ModuleName: unsafe extern "system" fn(this: *mut *mut Self, username: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ModuleName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub HostingUserAccount: unsafe extern "system" fn(this: *mut *mut Self, useraccount: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HostingUserAccount: usize,
    pub HostingProcessPid: unsafe extern "system" fn(this: *mut *mut Self, pid: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Bind: unsafe extern "system" fn(this: *mut *mut Self, moduledefinition: *mut ::core::ffi::c_void, moduleimplementation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Bind: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmPipelineModuleConnector {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3244299507, data2: 39585, data3: 18099, data4: [176, 167, 171, 20, 110, 178, 5, 242] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmPipelineModuleDefinition {
    pub base__: IFsrmObject,
    #[cfg(feature = "Win32_Foundation")]
    pub ModuleClsid: unsafe extern "system" fn(this: *mut *mut Self, moduleclsid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ModuleClsid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetModuleClsid: unsafe extern "system" fn(this: *mut *mut Self, moduleclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetModuleClsid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Company: unsafe extern "system" fn(this: *mut *mut Self, company: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Company: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCompany: unsafe extern "system" fn(this: *mut *mut Self, company: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCompany: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Version: unsafe extern "system" fn(this: *mut *mut Self, version: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Version: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetVersion: unsafe extern "system" fn(this: *mut *mut Self, version: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetVersion: usize,
    pub ModuleType: unsafe extern "system" fn(this: *mut *mut Self, moduletype: *mut FsrmPipelineModuleType) -> ::windows_sys::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, enabled: i16) -> ::windows_sys::core::HRESULT,
    pub NeedsFileContent: unsafe extern "system" fn(this: *mut *mut Self, needsfilecontent: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetNeedsFileContent: unsafe extern "system" fn(this: *mut *mut Self, needsfilecontent: i16) -> ::windows_sys::core::HRESULT,
    pub Account: unsafe extern "system" fn(this: *mut *mut Self, retrievalaccount: *mut FsrmAccountType) -> ::windows_sys::core::HRESULT,
    pub SetAccount: unsafe extern "system" fn(this: *mut *mut Self, retrievalaccount: FsrmAccountType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedExtensions: unsafe extern "system" fn(this: *mut *mut Self, supportedextensions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedExtensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSupportedExtensions: unsafe extern "system" fn(this: *mut *mut Self, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSupportedExtensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Parameters: unsafe extern "system" fn(this: *mut *mut Self, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetParameters: unsafe extern "system" fn(this: *mut *mut Self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetParameters: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmPipelineModuleDefinition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1364988535, data2: 11393, data3: 17422, data4: [143, 207, 54, 121, 33, 237, 79, 89] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmPipelineModuleImplementation {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub OnLoad: unsafe extern "system" fn(this: *mut *mut Self, moduledefinition: *mut ::core::ffi::c_void, moduleconnector: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnLoad: usize,
    pub OnUnload: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmPipelineModuleImplementation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3079698694, data2: 11010, data3: 19637, data4: [132, 169, 253, 245, 70, 19, 214, 205] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmProperty {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Value: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Sources: unsafe extern "system" fn(this: *mut *mut Self, sources: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Sources: usize,
    pub PropertyFlags: unsafe extern "system" fn(this: *mut *mut Self, flags: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmProperty {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1249115876, data2: 16642, data3: 20428, data4: [159, 251, 56, 97, 79, 158, 231, 104] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmPropertyBag {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RelativePath: unsafe extern "system" fn(this: *mut *mut Self, path: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RelativePath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub VolumeName: unsafe extern "system" fn(this: *mut *mut Self, volumename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    VolumeName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RelativeNamespaceRoot: unsafe extern "system" fn(this: *mut *mut Self, relativenamespaceroot: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RelativeNamespaceRoot: usize,
    pub VolumeIndex: unsafe extern "system" fn(this: *mut *mut Self, volumeid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub FileId: unsafe extern "system" fn(this: *mut *mut Self, fileid: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    FileId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ParentDirectoryId: unsafe extern "system" fn(this: *mut *mut Self, parentdirectoryid: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ParentDirectoryId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, size: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Size: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SizeAllocated: unsafe extern "system" fn(this: *mut *mut Self, sizeallocated: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SizeAllocated: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreationTime: unsafe extern "system" fn(this: *mut *mut Self, creationtime: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreationTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LastAccessTime: unsafe extern "system" fn(this: *mut *mut Self, lastaccesstime: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LastAccessTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LastModificationTime: unsafe extern "system" fn(this: *mut *mut Self, lastmodificationtime: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LastModificationTime: usize,
    pub Attributes: unsafe extern "system" fn(this: *mut *mut Self, attributes: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OwnerSid: unsafe extern "system" fn(this: *mut *mut Self, ownersid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OwnerSid: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub FilePropertyNames: unsafe extern "system" fn(this: *mut *mut Self, filepropertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FilePropertyNames: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Messages: unsafe extern "system" fn(this: *mut *mut Self, messages: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Messages: usize,
    pub PropertyBagFlags: unsafe extern "system" fn(this: *mut *mut Self, flags: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetFileProperty: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fileproperty: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetFileProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFileProperty: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFileProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AddMessage: unsafe extern "system" fn(this: *mut *mut Self, message: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddMessage: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetFileStreamInterface: unsafe extern "system" fn(this: *mut *mut Self, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType, pstreaminterface: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetFileStreamInterface: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmPropertyBag {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2001045969, data2: 54016, data3: 20346, data4: [154, 36, 247, 183, 102, 128, 2, 80] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmPropertyBag2 {
    pub base__: IFsrmPropertyBag,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetFieldValue: unsafe extern "system" fn(this: *mut *mut Self, field: FsrmPropertyBagField, value: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetFieldValue: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetUntrustedInFileProperties: unsafe extern "system" fn(this: *mut *mut Self, props: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetUntrustedInFileProperties: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmPropertyBag2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 239517117, data2: 9218, data3: 20461, data4: [156, 48, 146, 102, 230, 235, 44, 201] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmPropertyCondition {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, r#type: *mut FsrmPropertyConditionType) -> ::windows_sys::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut *mut Self, r#type: FsrmPropertyConditionType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, value: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Value: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetValue: usize,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmPropertyCondition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 845870703, data2: 10944, data3: 20328, data4: [191, 140, 71, 89, 240, 84, 250, 41] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmPropertyDefinition {
    pub base__: IFsrmObject,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, r#type: *mut FsrmPropertyDefinitionType) -> ::windows_sys::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut *mut Self, r#type: FsrmPropertyDefinitionType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PossibleValues: unsafe extern "system" fn(this: *mut *mut Self, possiblevalues: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PossibleValues: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPossibleValues: unsafe extern "system" fn(this: *mut *mut Self, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPossibleValues: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ValueDescriptions: unsafe extern "system" fn(this: *mut *mut Self, valuedescriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ValueDescriptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetValueDescriptions: unsafe extern "system" fn(this: *mut *mut Self, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetValueDescriptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Parameters: unsafe extern "system" fn(this: *mut *mut Self, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetParameters: unsafe extern "system" fn(this: *mut *mut Self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetParameters: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmPropertyDefinition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3990885647, data2: 59811, data3: 16796, data4: [135, 124, 1, 254, 93, 36, 197, 211] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmPropertyDefinition2 {
    pub base__: IFsrmPropertyDefinition,
    pub PropertyDefinitionFlags: unsafe extern "system" fn(this: *mut *mut Self, propertydefinitionflags: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDisplayName: usize,
    pub AppliesTo: unsafe extern "system" fn(this: *mut *mut Self, appliesto: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ValueDefinitions: unsafe extern "system" fn(this: *mut *mut Self, valuedefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ValueDefinitions: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmPropertyDefinition2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1199055186, data2: 53612, data3: 16937, data4: [180, 225, 13, 223, 227, 8, 185, 246] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmPropertyDefinitionValue {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, displayname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, description: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UniqueID: unsafe extern "system" fn(this: *mut *mut Self, uniqueid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UniqueID: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmPropertyDefinitionValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3913732424, data2: 48487, data3: 16760, data4: [142, 34, 28, 68, 146, 94, 215, 16] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmQuota {
    pub base__: IFsrmQuotaObject,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub QuotaUsed: unsafe extern "system" fn(this: *mut *mut Self, used: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    QuotaUsed: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub QuotaPeakUsage: unsafe extern "system" fn(this: *mut *mut Self, peakusage: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    QuotaPeakUsage: usize,
    pub QuotaPeakUsageTime: unsafe extern "system" fn(this: *mut *mut Self, peakusagedatetime: *mut f64) -> ::windows_sys::core::HRESULT,
    pub ResetPeakUsage: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RefreshUsageProperties: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmQuota {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 931099549, data2: 38471, data3: 19342, data4: [151, 210, 95, 252, 230, 215, 89, 205] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmQuotaBase {
    pub base__: IFsrmObject,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub QuotaLimit: unsafe extern "system" fn(this: *mut *mut Self, quotalimit: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    QuotaLimit: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetQuotaLimit: unsafe extern "system" fn(this: *mut *mut Self, quotalimit: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetQuotaLimit: usize,
    pub QuotaFlags: unsafe extern "system" fn(this: *mut *mut Self, quotaflags: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetQuotaFlags: unsafe extern "system" fn(this: *mut *mut Self, quotaflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Thresholds: unsafe extern "system" fn(this: *mut *mut Self, thresholds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Thresholds: usize,
    pub AddThreshold: unsafe extern "system" fn(this: *mut *mut Self, threshold: i32) -> ::windows_sys::core::HRESULT,
    pub DeleteThreshold: unsafe extern "system" fn(this: *mut *mut Self, threshold: i32) -> ::windows_sys::core::HRESULT,
    pub ModifyThreshold: unsafe extern "system" fn(this: *mut *mut Self, threshold: i32, newthreshold: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateThresholdAction: unsafe extern "system" fn(this: *mut *mut Self, threshold: i32, actiontype: FsrmActionType, action: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateThresholdAction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumThresholdActions: unsafe extern "system" fn(this: *mut *mut Self, threshold: i32, actions: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumThresholdActions: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmQuotaBase {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 359180181, data2: 14628, data3: 16664, data4: [183, 75, 104, 216, 240, 250, 93, 175] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmQuotaManager {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub ActionVariables: unsafe extern "system" fn(this: *mut *mut Self, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActionVariables: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ActionVariableDescriptions: unsafe extern "system" fn(this: *mut *mut Self, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActionVariableDescriptions: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateQuota: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateQuota: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateAutoApplyQuota: unsafe extern "system" fn(this: *mut *mut Self, quotatemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateAutoApplyQuota: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetQuota: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetQuota: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetAutoApplyQuota: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetAutoApplyQuota: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetRestrictiveQuota: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetRestrictiveQuota: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub EnumQuotas: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    EnumQuotas: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub EnumAutoApplyQuotas: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    EnumAutoApplyQuotas: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub EnumEffectiveQuotas: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    EnumEffectiveQuotas: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Scan: unsafe extern "system" fn(this: *mut *mut Self, strpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Scan: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateQuotaCollection: unsafe extern "system" fn(this: *mut *mut Self, collection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateQuotaCollection: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmQuotaManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2343996541, data2: 6616, data3: 20475, data4: [128, 158, 190, 79, 193, 115, 64, 20] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmQuotaManagerEx {
    pub base__: IFsrmQuotaManager,
    #[cfg(feature = "Win32_Foundation")]
    pub IsAffectedByQuota: unsafe extern "system" fn(this: *mut *mut Self, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, affected: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsAffectedByQuota: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmQuotaManagerEx {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1212599041, data2: 54320, data3: 18767, data4: [171, 180, 177, 5, 73, 153, 251, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmQuotaObject {
    pub base__: IFsrmQuotaBase,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, path: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UserSid: unsafe extern "system" fn(this: *mut *mut Self, usersid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserSid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UserAccount: unsafe extern "system" fn(this: *mut *mut Self, useraccount: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserAccount: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SourceTemplateName: unsafe extern "system" fn(this: *mut *mut Self, quotatemplatename: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SourceTemplateName: usize,
    pub MatchesSourceTemplate: unsafe extern "system" fn(this: *mut *mut Self, matches: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ApplyTemplate: unsafe extern "system" fn(this: *mut *mut Self, quotatemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ApplyTemplate: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmQuotaObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1121727761, data2: 25045, data3: 18606, data4: [182, 220, 89, 252, 0, 192, 168, 214] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmQuotaTemplate {
    pub base__: IFsrmQuotaBase,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CopyTemplate: unsafe extern "system" fn(this: *mut *mut Self, quotatemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CopyTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CommitAndUpdateDerived: unsafe extern "system" fn(this: *mut *mut Self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommitAndUpdateDerived: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmQuotaTemplate {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2733615921, data2: 10590, data3: 18107, data4: [185, 118, 232, 109, 88, 181, 46, 139] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmQuotaTemplateImported {
    pub base__: IFsrmQuotaTemplate,
    pub OverwriteOnCommit: unsafe extern "system" fn(this: *mut *mut Self, overwrite: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetOverwriteOnCommit: unsafe extern "system" fn(this: *mut *mut Self, overwrite: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmQuotaTemplateImported {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2586571027, data2: 41769, data3: 17612, data4: [128, 154, 92, 0, 252, 232, 218, 64] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmQuotaTemplateManager {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateTemplate: unsafe extern "system" fn(this: *mut *mut Self, quotatemplate: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateTemplate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetTemplate: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quotatemplate: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumTemplates: unsafe extern "system" fn(this: *mut *mut Self, options: FsrmEnumOptions, quotatemplates: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumTemplates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ExportTemplates: unsafe extern "system" fn(this: *mut *mut Self, quotatemplatenamesarray: *const super::super::System::Com::VARIANT, serializedquotatemplates: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ExportTemplates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ImportTemplates: unsafe extern "system" fn(this: *mut *mut Self, serializedquotatemplates: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quotatemplatenamesarray: *const super::super::System::Com::VARIANT, quotatemplates: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ImportTemplates: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmQuotaTemplateManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1098099777, data2: 5933, data3: 19794, data4: [150, 60, 253, 199, 228, 21, 247, 23] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmReport {
    pub base__: super::super::System::Com::IDispatch,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, reporttype: *mut FsrmReportType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, description: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LastGeneratedFileNamePrefix: unsafe extern "system" fn(this: *mut *mut Self, prefix: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LastGeneratedFileNamePrefix: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetFilter: unsafe extern "system" fn(this: *mut *mut Self, filter: FsrmReportFilter, filtervalue: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetFilter: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetFilter: unsafe extern "system" fn(this: *mut *mut Self, filter: FsrmReportFilter, filtervalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetFilter: usize,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmReport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3637281241, data2: 18104, data3: 20388, data4: [191, 165, 74, 169, 222, 201, 182, 56] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmReportJob {
    pub base__: IFsrmObject,
    #[cfg(feature = "Win32_Foundation")]
    pub Task: unsafe extern "system" fn(this: *mut *mut Self, taskname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Task: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTask: unsafe extern "system" fn(this: *mut *mut Self, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTask: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NamespaceRoots: unsafe extern "system" fn(this: *mut *mut Self, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NamespaceRoots: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetNamespaceRoots: unsafe extern "system" fn(this: *mut *mut Self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetNamespaceRoots: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Formats: unsafe extern "system" fn(this: *mut *mut Self, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Formats: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetFormats: unsafe extern "system" fn(this: *mut *mut Self, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetFormats: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MailTo: unsafe extern "system" fn(this: *mut *mut Self, mailto: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MailTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMailTo: unsafe extern "system" fn(this: *mut *mut Self, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMailTo: usize,
    pub RunningStatus: unsafe extern "system" fn(this: *mut *mut Self, runningstatus: *mut FsrmReportRunningStatus) -> ::windows_sys::core::HRESULT,
    pub LastRun: unsafe extern "system" fn(this: *mut *mut Self, lastrun: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LastError: unsafe extern "system" fn(this: *mut *mut Self, lasterror: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LastError: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LastGeneratedInDirectory: unsafe extern "system" fn(this: *mut *mut Self, path: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LastGeneratedInDirectory: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumReports: unsafe extern "system" fn(this: *mut *mut Self, reports: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumReports: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateReport: unsafe extern "system" fn(this: *mut *mut Self, reporttype: FsrmReportType, report: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateReport: usize,
    pub Run: unsafe extern "system" fn(this: *mut *mut Self, context: FsrmReportGenerationContext) -> ::windows_sys::core::HRESULT,
    pub WaitForCompletion: unsafe extern "system" fn(this: *mut *mut Self, waitseconds: i32, completed: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmReportJob {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 954757760, data2: 29020, data3: 19581, data4: [162, 128, 234, 22, 81, 161, 159, 239] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmReportManager {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumReportJobs: unsafe extern "system" fn(this: *mut *mut Self, options: FsrmEnumOptions, reportjobs: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumReportJobs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateReportJob: unsafe extern "system" fn(this: *mut *mut Self, reportjob: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateReportJob: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetReportJob: unsafe extern "system" fn(this: *mut *mut Self, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, reportjob: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetReportJob: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOutputDirectory: unsafe extern "system" fn(this: *mut *mut Self, context: FsrmReportGenerationContext, path: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOutputDirectory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOutputDirectory: unsafe extern "system" fn(this: *mut *mut Self, context: FsrmReportGenerationContext, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOutputDirectory: usize,
    pub IsFilterValidForReportType: unsafe extern "system" fn(this: *mut *mut Self, reporttype: FsrmReportType, filter: FsrmReportFilter, valid: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetDefaultFilter: unsafe extern "system" fn(this: *mut *mut Self, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetDefaultFilter: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetDefaultFilter: unsafe extern "system" fn(this: *mut *mut Self, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetDefaultFilter: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetReportSizeLimit: unsafe extern "system" fn(this: *mut *mut Self, limit: FsrmReportLimit, limitvalue: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetReportSizeLimit: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetReportSizeLimit: unsafe extern "system" fn(this: *mut *mut Self, limit: FsrmReportLimit, limitvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetReportSizeLimit: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmReportManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 666409470, data2: 28666, data3: 17537, data4: [161, 132, 211, 218, 173, 232, 160, 43] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmReportScheduler {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub VerifyNamespaces: unsafe extern "system" fn(this: *mut *mut Self, namespacessafearray: *const super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    VerifyNamespaces: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateScheduleTask: unsafe extern "system" fn(this: *mut *mut Self, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateScheduleTask: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ModifyScheduleTask: unsafe extern "system" fn(this: *mut *mut Self, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ModifyScheduleTask: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteScheduleTask: unsafe extern "system" fn(this: *mut *mut Self, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteScheduleTask: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmReportScheduler {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1752812281, data2: 26135, data3: 17540, data4: [135, 25, 113, 195, 216, 100, 95, 148] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmRule {
    pub base__: IFsrmObject,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    pub RuleType: unsafe extern "system" fn(this: *mut *mut Self, ruletype: *mut FsrmRuleType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ModuleDefinitionName: unsafe extern "system" fn(this: *mut *mut Self, moduledefinitionname: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ModuleDefinitionName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetModuleDefinitionName: unsafe extern "system" fn(this: *mut *mut Self, moduledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetModuleDefinitionName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NamespaceRoots: unsafe extern "system" fn(this: *mut *mut Self, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NamespaceRoots: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetNamespaceRoots: unsafe extern "system" fn(this: *mut *mut Self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetNamespaceRoots: usize,
    pub RuleFlags: unsafe extern "system" fn(this: *mut *mut Self, ruleflags: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetRuleFlags: unsafe extern "system" fn(this: *mut *mut Self, ruleflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Parameters: unsafe extern "system" fn(this: *mut *mut Self, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetParameters: unsafe extern "system" fn(this: *mut *mut Self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetParameters: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LastModified: unsafe extern "system" fn(this: *mut *mut Self, lastmodified: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LastModified: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmRule {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3406690656, data2: 5877, data3: 17557, data4: [144, 121, 63, 147, 96, 216, 49, 223] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmSetting {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub SmtpServer: unsafe extern "system" fn(this: *mut *mut Self, smtpserver: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SmtpServer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSmtpServer: unsafe extern "system" fn(this: *mut *mut Self, smtpserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSmtpServer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MailFrom: unsafe extern "system" fn(this: *mut *mut Self, mailfrom: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MailFrom: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMailFrom: unsafe extern "system" fn(this: *mut *mut Self, mailfrom: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMailFrom: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AdminEmail: unsafe extern "system" fn(this: *mut *mut Self, adminemail: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AdminEmail: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAdminEmail: unsafe extern "system" fn(this: *mut *mut Self, adminemail: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAdminEmail: usize,
    pub DisableCommandLine: unsafe extern "system" fn(this: *mut *mut Self, disablecommandline: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetDisableCommandLine: unsafe extern "system" fn(this: *mut *mut Self, disablecommandline: i16) -> ::windows_sys::core::HRESULT,
    pub EnableScreeningAudit: unsafe extern "system" fn(this: *mut *mut Self, enablescreeningaudit: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEnableScreeningAudit: unsafe extern "system" fn(this: *mut *mut Self, enablescreeningaudit: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EmailTest: unsafe extern "system" fn(this: *mut *mut Self, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EmailTest: usize,
    pub SetActionRunLimitInterval: unsafe extern "system" fn(this: *mut *mut Self, actiontype: FsrmActionType, delaytimeminutes: i32) -> ::windows_sys::core::HRESULT,
    pub GetActionRunLimitInterval: unsafe extern "system" fn(this: *mut *mut Self, actiontype: FsrmActionType, delaytimeminutes: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmSetting {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4094809341, data2: 5310, data3: 16992, data4: [140, 64, 3, 183, 201, 94, 96, 138] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmStorageModuleDefinition {
    pub base__: IFsrmPipelineModuleDefinition,
    pub Capabilities: unsafe extern "system" fn(this: *mut *mut Self, capabilities: *mut FsrmStorageModuleCaps) -> ::windows_sys::core::HRESULT,
    pub SetCapabilities: unsafe extern "system" fn(this: *mut *mut Self, capabilities: FsrmStorageModuleCaps) -> ::windows_sys::core::HRESULT,
    pub StorageType: unsafe extern "system" fn(this: *mut *mut Self, storagetype: *mut FsrmStorageModuleType) -> ::windows_sys::core::HRESULT,
    pub SetStorageType: unsafe extern "system" fn(this: *mut *mut Self, storagetype: FsrmStorageModuleType) -> ::windows_sys::core::HRESULT,
    pub UpdatesFileContent: unsafe extern "system" fn(this: *mut *mut Self, updatesfilecontent: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetUpdatesFileContent: unsafe extern "system" fn(this: *mut *mut Self, updatesfilecontent: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmStorageModuleDefinition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 363336528, data2: 18813, data3: 19130, data4: [128, 233, 212, 219, 204, 85, 33, 254] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFsrmStorageModuleImplementation {
    pub base__: IFsrmPipelineModuleImplementation,
    #[cfg(feature = "Win32_System_Com")]
    pub UseDefinitions: unsafe extern "system" fn(this: *mut *mut Self, propertydefinitions: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UseDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadProperties: unsafe extern "system" fn(this: *mut *mut Self, propertybag: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SaveProperties: unsafe extern "system" fn(this: *mut *mut Self, propertybag: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SaveProperties: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IFsrmStorageModuleImplementation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 183804122, data2: 35162, data3: 20048, data4: [135, 18, 169, 103, 36, 188, 236, 100] };
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const MessageSizeLimit: u32 = 4096u32;
