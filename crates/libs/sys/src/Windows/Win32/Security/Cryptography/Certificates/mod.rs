#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
    pub fn CertSrvBackupClose(hbc: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
    pub fn CertSrvBackupEnd(hbc: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
    pub fn CertSrvBackupFree(pv: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
    pub fn CertSrvBackupGetBackupLogsW(hbc: *const ::core::ffi::c_void, ppwszzbackuplogfiles: *mut ::windows_sys::core::PWSTR, pcbsize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
    pub fn CertSrvBackupGetDatabaseNamesW(hbc: *const ::core::ffi::c_void, ppwszzattachmentinformation: *mut ::windows_sys::core::PWSTR, pcbsize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
    pub fn CertSrvBackupGetDynamicFileListW(hbc: *const ::core::ffi::c_void, ppwszzfilelist: *mut ::windows_sys::core::PWSTR, pcbsize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
    pub fn CertSrvBackupOpenFileW(hbc: *mut ::core::ffi::c_void, pwszattachmentname: ::windows_sys::core::PCWSTR, cbreadhintsize: u32, plifilesize: *mut i64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
    pub fn CertSrvBackupPrepareW(pwszservername: ::windows_sys::core::PCWSTR, grbitjet: u32, dwbackupflags: CSBACKUP_TYPE, phbc: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
    pub fn CertSrvBackupRead(hbc: *mut ::core::ffi::c_void, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, pcbread: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
    pub fn CertSrvBackupTruncateLogs(hbc: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvIsServerOnlineW(pwszservername: ::windows_sys::core::PCWSTR, pfserveronline: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
    pub fn CertSrvRestoreEnd(hbc: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
    pub fn CertSrvRestoreGetDatabaseLocationsW(hbc: *const ::core::ffi::c_void, ppwszzdatabaselocationlist: *mut ::windows_sys::core::PWSTR, pcbsize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
    pub fn CertSrvRestorePrepareW(pwszservername: ::windows_sys::core::PCWSTR, dwrestoreflags: u32, phbc: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
    pub fn CertSrvRestoreRegisterComplete(hbc: *mut ::core::ffi::c_void, hrrestorestate: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
    pub fn CertSrvRestoreRegisterThroughFile(hbc: *mut ::core::ffi::c_void, pwszcheckpointfilepath: ::windows_sys::core::PCWSTR, pwszlogpath: ::windows_sys::core::PCWSTR, rgrstmap: *mut CSEDB_RSTMAPW, crstmap: i32, pwszbackuplogpath: ::windows_sys::core::PCWSTR, genlow: u32, genhigh: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
    pub fn CertSrvRestoreRegisterW(hbc: *mut ::core::ffi::c_void, pwszcheckpointfilepath: ::windows_sys::core::PCWSTR, pwszlogpath: ::windows_sys::core::PCWSTR, rgrstmap: *mut CSEDB_RSTMAPW, crstmap: i32, pwszbackuplogpath: ::windows_sys::core::PCWSTR, genlow: u32, genhigh: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
    pub fn CertSrvServerControlW(pwszservername: ::windows_sys::core::PCWSTR, dwcontrolflags: u32, pcbout: *mut u32, ppbout: *mut *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PstAcquirePrivateKey(pcert: *const super::CERT_CONTEXT) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Authentication_Identity\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
    pub fn PstGetCertificateChain(pcert: *const super::CERT_CONTEXT, ptrustedissuers: *const super::super::Authentication::Identity::SecPkgContext_IssuerListInfoEx, ppcertchaincontext: *mut *mut super::CERT_CHAIN_CONTEXT) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PstGetCertificates(ptargetname: *const super::super::super::Foundation::UNICODE_STRING, ccriteria: u32, rgpcriteria: *const super::CERT_SELECT_CRITERIA, bisclient: super::super::super::Foundation::BOOL, pdwcertchaincontextcount: *mut u32, ppcertchaincontexts: *mut *mut *mut super::CERT_CHAIN_CONTEXT) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Authentication_Identity\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
    pub fn PstGetTrustAnchors(ptargetname: *const super::super::super::Foundation::UNICODE_STRING, ccriteria: u32, rgpcriteria: *const super::CERT_SELECT_CRITERIA, pptrustedissuers: *mut *mut super::super::Authentication::Identity::SecPkgContext_IssuerListInfoEx) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Authentication_Identity\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
    pub fn PstGetTrustAnchorsEx(ptargetname: *const super::super::super::Foundation::UNICODE_STRING, ccriteria: u32, rgpcriteria: *const super::CERT_SELECT_CRITERIA, pcertcontext: *const super::CERT_CONTEXT, pptrustedissuers: *mut *mut super::super::Authentication::Identity::SecPkgContext_IssuerListInfoEx) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PstGetUserNameForCertificate(pcertcontext: *const super::CERT_CONTEXT, username: *mut super::super::super::Foundation::UNICODE_STRING) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Authentication_Identity\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
    pub fn PstMapCertificate(pcert: *const super::CERT_CONTEXT, ptokeninformationtype: *mut super::super::Authentication::Identity::LSA_TOKEN_INFORMATION_TYPE, pptokeninformation: *mut *mut ::core::ffi::c_void) -> super::super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PstValidate(ptargetname: *const super::super::super::Foundation::UNICODE_STRING, bisclient: super::super::super::Foundation::BOOL, prequestedissuancepolicy: *const super::CERT_USAGE_MATCH, phadditionalcertstore: *const super::HCERTSTORE, pcert: *const super::CERT_CONTEXT, pprovguid: *mut ::windows_sys::core::GUID) -> super::super::super::Foundation::NTSTATUS;
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type ADDED_CERT_TYPE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XECT_EXTENSION_V1: ADDED_CERT_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XECT_EXTENSION_V2: ADDED_CERT_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type AlgorithmFlags = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const AlgorithmFlagsNone: AlgorithmFlags = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const AlgorithmFlagsWrap: AlgorithmFlags = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type AlgorithmOperationFlags = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_NO_OPERATION: AlgorithmOperationFlags = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_CIPHER_OPERATION: AlgorithmOperationFlags = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_HASH_OPERATION: AlgorithmOperationFlags = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_ASYMMETRIC_ENCRYPTION_OPERATION: AlgorithmOperationFlags = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_SECRET_AGREEMENT_OPERATION: AlgorithmOperationFlags = 8i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_SIGNATURE_OPERATION: AlgorithmOperationFlags = 16i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_RNG_OPERATION: AlgorithmOperationFlags = 32i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_KEY_DERIVATION_OPERATION: AlgorithmOperationFlags = 64i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_ANY_ASYMMETRIC_OPERATION: AlgorithmOperationFlags = 28i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_PREFER_SIGNATURE_ONLY_OPERATION: AlgorithmOperationFlags = 2097152i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_PREFER_NON_SIGNATURE_OPERATION: AlgorithmOperationFlags = 4194304i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_EXACT_MATCH_OPERATION: AlgorithmOperationFlags = 8388608i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_PREFERENCE_MASK_OPERATION: AlgorithmOperationFlags = 14680064i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type AlgorithmType = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_BCRYPT_UNKNOWN_INTERFACE: AlgorithmType = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_BCRYPT_CIPHER_INTERFACE: AlgorithmType = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_BCRYPT_HASH_INTERFACE: AlgorithmType = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_BCRYPT_ASYMMETRIC_ENCRYPTION_INTERFACE: AlgorithmType = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_BCRYPT_SIGNATURE_INTERFACE: AlgorithmType = 5i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_BCRYPT_SECRET_AGREEMENT_INTERFACE: AlgorithmType = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_BCRYPT_RNG_INTERFACE: AlgorithmType = 6i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_BCRYPT_KEY_DERIVATION_INTERFACE: AlgorithmType = 7i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type AlternativeNameType = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ALT_NAME_UNKNOWN: AlternativeNameType = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ALT_NAME_OTHER_NAME: AlternativeNameType = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ALT_NAME_RFC822_NAME: AlternativeNameType = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ALT_NAME_DNS_NAME: AlternativeNameType = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ALT_NAME_X400_ADDRESS: AlternativeNameType = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ALT_NAME_DIRECTORY_NAME: AlternativeNameType = 5i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ALT_NAME_EDI_PARTY_NAME: AlternativeNameType = 6i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ALT_NAME_URL: AlternativeNameType = 7i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ALT_NAME_IP_ADDRESS: AlternativeNameType = 8i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ALT_NAME_REGISTERED_ID: AlternativeNameType = 9i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ALT_NAME_GUID: AlternativeNameType = 10i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ALT_NAME_USER_PRINCIPLE_NAME: AlternativeNameType = 11i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CAIF_DSENTRY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CAIF_LOCAL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CAIF_REGISTRY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CAIF_REGISTRYPARENT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CAIF_SHAREDFOLDERENTRY: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub struct CAINFO {
    pub cbSize: u32,
    pub CAType: ENUM_CATYPES,
    pub cCASignatureCerts: u32,
    pub cCAExchangeCerts: u32,
    pub cExitModules: u32,
    pub lPropIdMax: i32,
    pub lRoleSeparationEnabled: i32,
    pub cKRACertUsedCount: u32,
    pub cKRACertCount: u32,
    pub fAdvancedServer: u32,
}
impl ::core::marker::Copy for CAINFO {}
impl ::core::clone::Clone for CAINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CAPATHLENGTH_INFINITE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CA_ACCESS_MASKROLES: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CA_CRL_BASE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CA_CRL_DELTA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CA_CRL_REPUBLISH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CA_DISP_ERROR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CA_DISP_INCOMPLETE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CA_DISP_INVALID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CA_DISP_REVOKED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CA_DISP_UNDER_SUBMISSION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CA_DISP_VALID: u32 = 3u32;
pub const CAlternativeName: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821395, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CAlternativeNames: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821396, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CBinaryConverter: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821378, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CCLOCKSKEWMINUTESDEFAULT: u32 = 10u32;
pub const CCertAdmin: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 938130160, data2: 32694, data3: 4560, data4: [136, 23, 0, 160, 201, 3, 184, 60] };
pub const CCertConfig: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 925879864, data2: 17188, data3: 4560, data4: [136, 16, 0, 160, 201, 3, 184, 60] };
pub const CCertEncodeAltName: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 486296794, data2: 4721, data3: 4561, data4: [155, 212, 0, 192, 79, 182, 131, 250] };
pub const CCertEncodeBitString: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1835744472, data2: 4728, data3: 4561, data4: [155, 212, 0, 192, 79, 182, 131, 250] };
pub const CCertEncodeCRLDistInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 33185952, data2: 48127, data3: 4560, data4: [136, 37, 0, 160, 201, 3, 184, 60] };
pub const CCertEncodeDateArray: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 807368624, data2: 42096, data3: 4560, data4: [136, 33, 0, 160, 201, 3, 184, 60] };
pub const CCertEncodeLongArray: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1309048992, data2: 41122, data3: 4560, data4: [136, 33, 0, 160, 201, 3, 184, 60] };
pub const CCertEncodeStringArray: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 430403552, data2: 29844, data3: 4560, data4: [136, 22, 0, 160, 201, 3, 184, 60] };
pub const CCertGetConfig: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3335276976, data2: 52759, data3: 4560, data4: [136, 51, 0, 160, 201, 3, 184, 60] };
pub const CCertProperties: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821423, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertProperty: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821422, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyArchived: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821431, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyArchivedKeyHash: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821435, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyAutoEnroll: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821426, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyBackedUp: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821432, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyDescription: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821425, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyEnrollment: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821433, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyEnrollmentPolicyServer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821452, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyFriendlyName: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821424, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyKeyProvInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821430, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyRenewal: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821434, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertyRequestOriginator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821427, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertPropertySHA1Hash: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821428, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertRequest: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2561668080, data2: 21796, data3: 4560, data4: [136, 18, 0, 160, 201, 3, 184, 60] };
pub const CCertServerExit: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1279942208, data2: 29484, data3: 4560, data4: [136, 22, 0, 160, 201, 3, 184, 60] };
pub const CCertServerPolicy: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2852129062, data2: 65470, data3: 4559, data4: [136, 0, 0, 160, 201, 3, 184, 60] };
pub const CCertView: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2704084858, data2: 7812, data3: 4561, data4: [155, 214, 0, 192, 79, 182, 131, 250] };
pub const CCertificateAttestationChallenge: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 325234081, data2: 60256, data3: 17770, data4: [182, 225, 17, 128, 80, 219, 116, 27] };
pub const CCertificatePolicies: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821407, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCertificatePolicy: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821406, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCryptAttribute: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821420, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCryptAttributes: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821421, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCspInformation: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821383, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCspInformations: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821384, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CCspStatus: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821385, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type CERTADMIN_GET_ROLES_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CA_ACCESS_ADMIN: CERTADMIN_GET_ROLES_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CA_ACCESS_AUDITOR: CERTADMIN_GET_ROLES_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CA_ACCESS_ENROLL: CERTADMIN_GET_ROLES_FLAGS = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CA_ACCESS_OFFICER: CERTADMIN_GET_ROLES_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CA_ACCESS_OPERATOR: CERTADMIN_GET_ROLES_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CA_ACCESS_READ: CERTADMIN_GET_ROLES_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CERTENROLL_INDEX_BASE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type CERTENROLL_OBJECTID = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NONE: CERTENROLL_OBJECTID = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA: CERTENROLL_OBJECTID = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS: CERTENROLL_OBJECTID = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_HASH: CERTENROLL_OBJECTID = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_ENCRYPT: CERTENROLL_OBJECTID = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_1: CERTENROLL_OBJECTID = 5i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_2: CERTENROLL_OBJECTID = 6i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_3: CERTENROLL_OBJECTID = 7i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_4: CERTENROLL_OBJECTID = 8i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_5: CERTENROLL_OBJECTID = 9i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_6: CERTENROLL_OBJECTID = 10i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_7: CERTENROLL_OBJECTID = 11i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_8: CERTENROLL_OBJECTID = 12i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_9: CERTENROLL_OBJECTID = 13i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_10: CERTENROLL_OBJECTID = 14i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_12: CERTENROLL_OBJECTID = 15i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_RSA: CERTENROLL_OBJECTID = 16i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_MD2RSA: CERTENROLL_OBJECTID = 17i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_MD4RSA: CERTENROLL_OBJECTID = 18i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_MD5RSA: CERTENROLL_OBJECTID = 19i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_SHA1RSA: CERTENROLL_OBJECTID = 20i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_SETOAEP_RSA: CERTENROLL_OBJECTID = 21i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_DH: CERTENROLL_OBJECTID = 22i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_data: CERTENROLL_OBJECTID = 23i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_signedData: CERTENROLL_OBJECTID = 24i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_envelopedData: CERTENROLL_OBJECTID = 25i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_signEnvData: CERTENROLL_OBJECTID = 26i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_digestedData: CERTENROLL_OBJECTID = 27i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_hashedData: CERTENROLL_OBJECTID = 28i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_encryptedData: CERTENROLL_OBJECTID = 29i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_emailAddr: CERTENROLL_OBJECTID = 30i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_unstructName: CERTENROLL_OBJECTID = 31i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_contentType: CERTENROLL_OBJECTID = 32i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_messageDigest: CERTENROLL_OBJECTID = 33i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_signingTime: CERTENROLL_OBJECTID = 34i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_counterSign: CERTENROLL_OBJECTID = 35i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_challengePwd: CERTENROLL_OBJECTID = 36i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_unstructAddr: CERTENROLL_OBJECTID = 37i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_extCertAttrs: CERTENROLL_OBJECTID = 38i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_certExtensions: CERTENROLL_OBJECTID = 39i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_SMIMECapabilities: CERTENROLL_OBJECTID = 40i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_preferSignedData: CERTENROLL_OBJECTID = 41i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_SMIMEalg: CERTENROLL_OBJECTID = 42i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_SMIMEalgESDH: CERTENROLL_OBJECTID = 43i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_SMIMEalgCMS3DESwrap: CERTENROLL_OBJECTID = 44i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_SMIMEalgCMSRC2wrap: CERTENROLL_OBJECTID = 45i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_MD2: CERTENROLL_OBJECTID = 46i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_MD4: CERTENROLL_OBJECTID = 47i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_MD5: CERTENROLL_OBJECTID = 48i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_RC2CBC: CERTENROLL_OBJECTID = 49i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_RC4: CERTENROLL_OBJECTID = 50i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_DES_EDE3_CBC: CERTENROLL_OBJECTID = 51i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_RC5_CBCPad: CERTENROLL_OBJECTID = 52i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ANSI_X942: CERTENROLL_OBJECTID = 53i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ANSI_X942_DH: CERTENROLL_OBJECTID = 54i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_X957: CERTENROLL_OBJECTID = 55i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_X957_DSA: CERTENROLL_OBJECTID = 56i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_X957_SHA1DSA: CERTENROLL_OBJECTID = 57i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_DS: CERTENROLL_OBJECTID = 58i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_DSALG: CERTENROLL_OBJECTID = 59i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_DSALG_CRPT: CERTENROLL_OBJECTID = 60i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_DSALG_HASH: CERTENROLL_OBJECTID = 61i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_DSALG_SIGN: CERTENROLL_OBJECTID = 62i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_DSALG_RSA: CERTENROLL_OBJECTID = 63i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIW: CERTENROLL_OBJECTID = 64i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC: CERTENROLL_OBJECTID = 65i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_md4RSA: CERTENROLL_OBJECTID = 66i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_md5RSA: CERTENROLL_OBJECTID = 67i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_md4RSA2: CERTENROLL_OBJECTID = 68i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_desECB: CERTENROLL_OBJECTID = 69i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_desCBC: CERTENROLL_OBJECTID = 70i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_desOFB: CERTENROLL_OBJECTID = 71i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_desCFB: CERTENROLL_OBJECTID = 72i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_desMAC: CERTENROLL_OBJECTID = 73i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_rsaSign: CERTENROLL_OBJECTID = 74i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_dsa: CERTENROLL_OBJECTID = 75i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_shaDSA: CERTENROLL_OBJECTID = 76i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_mdc2RSA: CERTENROLL_OBJECTID = 77i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_shaRSA: CERTENROLL_OBJECTID = 78i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_dhCommMod: CERTENROLL_OBJECTID = 79i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_desEDE: CERTENROLL_OBJECTID = 80i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_sha: CERTENROLL_OBJECTID = 81i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_mdc2: CERTENROLL_OBJECTID = 82i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_dsaComm: CERTENROLL_OBJECTID = 83i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_dsaCommSHA: CERTENROLL_OBJECTID = 84i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_rsaXchg: CERTENROLL_OBJECTID = 85i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_keyHashSeal: CERTENROLL_OBJECTID = 86i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_md2RSASign: CERTENROLL_OBJECTID = 87i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_md5RSASign: CERTENROLL_OBJECTID = 88i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_sha1: CERTENROLL_OBJECTID = 89i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_dsaSHA1: CERTENROLL_OBJECTID = 90i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_dsaCommSHA1: CERTENROLL_OBJECTID = 91i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWSEC_sha1RSASign: CERTENROLL_OBJECTID = 92i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWDIR: CERTENROLL_OBJECTID = 93i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWDIR_CRPT: CERTENROLL_OBJECTID = 94i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWDIR_HASH: CERTENROLL_OBJECTID = 95i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWDIR_SIGN: CERTENROLL_OBJECTID = 96i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWDIR_md2: CERTENROLL_OBJECTID = 97i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OIWDIR_md2RSA: CERTENROLL_OBJECTID = 98i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INFOSEC: CERTENROLL_OBJECTID = 99i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INFOSEC_sdnsSignature: CERTENROLL_OBJECTID = 100i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INFOSEC_mosaicSignature: CERTENROLL_OBJECTID = 101i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INFOSEC_sdnsConfidentiality: CERTENROLL_OBJECTID = 102i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INFOSEC_mosaicConfidentiality: CERTENROLL_OBJECTID = 103i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INFOSEC_sdnsIntegrity: CERTENROLL_OBJECTID = 104i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INFOSEC_mosaicIntegrity: CERTENROLL_OBJECTID = 105i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INFOSEC_sdnsTokenProtection: CERTENROLL_OBJECTID = 106i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INFOSEC_mosaicTokenProtection: CERTENROLL_OBJECTID = 107i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INFOSEC_sdnsKeyManagement: CERTENROLL_OBJECTID = 108i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INFOSEC_mosaicKeyManagement: CERTENROLL_OBJECTID = 109i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INFOSEC_sdnsKMandSig: CERTENROLL_OBJECTID = 110i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INFOSEC_mosaicKMandSig: CERTENROLL_OBJECTID = 111i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INFOSEC_SuiteASignature: CERTENROLL_OBJECTID = 112i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INFOSEC_SuiteAConfidentiality: CERTENROLL_OBJECTID = 113i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INFOSEC_SuiteAIntegrity: CERTENROLL_OBJECTID = 114i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INFOSEC_SuiteATokenProtection: CERTENROLL_OBJECTID = 115i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INFOSEC_SuiteAKeyManagement: CERTENROLL_OBJECTID = 116i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INFOSEC_SuiteAKMandSig: CERTENROLL_OBJECTID = 117i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INFOSEC_mosaicUpdatedSig: CERTENROLL_OBJECTID = 118i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INFOSEC_mosaicKMandUpdSig: CERTENROLL_OBJECTID = 119i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INFOSEC_mosaicUpdatedInteg: CERTENROLL_OBJECTID = 120i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_COMMON_NAME: CERTENROLL_OBJECTID = 121i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_SUR_NAME: CERTENROLL_OBJECTID = 122i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_DEVICE_SERIAL_NUMBER: CERTENROLL_OBJECTID = 123i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_COUNTRY_NAME: CERTENROLL_OBJECTID = 124i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_LOCALITY_NAME: CERTENROLL_OBJECTID = 125i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_STATE_OR_PROVINCE_NAME: CERTENROLL_OBJECTID = 126i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_STREET_ADDRESS: CERTENROLL_OBJECTID = 127i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ORGANIZATION_NAME: CERTENROLL_OBJECTID = 128i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ORGANIZATIONAL_UNIT_NAME: CERTENROLL_OBJECTID = 129i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_TITLE: CERTENROLL_OBJECTID = 130i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_DESCRIPTION: CERTENROLL_OBJECTID = 131i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_SEARCH_GUIDE: CERTENROLL_OBJECTID = 132i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_BUSINESS_CATEGORY: CERTENROLL_OBJECTID = 133i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_POSTAL_ADDRESS: CERTENROLL_OBJECTID = 134i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_POSTAL_CODE: CERTENROLL_OBJECTID = 135i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_POST_OFFICE_BOX: CERTENROLL_OBJECTID = 136i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PHYSICAL_DELIVERY_OFFICE_NAME: CERTENROLL_OBJECTID = 137i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_TELEPHONE_NUMBER: CERTENROLL_OBJECTID = 138i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_TELEX_NUMBER: CERTENROLL_OBJECTID = 139i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_TELETEXT_TERMINAL_IDENTIFIER: CERTENROLL_OBJECTID = 140i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_FACSIMILE_TELEPHONE_NUMBER: CERTENROLL_OBJECTID = 141i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_X21_ADDRESS: CERTENROLL_OBJECTID = 142i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INTERNATIONAL_ISDN_NUMBER: CERTENROLL_OBJECTID = 143i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_REGISTERED_ADDRESS: CERTENROLL_OBJECTID = 144i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_DESTINATION_INDICATOR: CERTENROLL_OBJECTID = 145i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PREFERRED_DELIVERY_METHOD: CERTENROLL_OBJECTID = 146i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PRESENTATION_ADDRESS: CERTENROLL_OBJECTID = 147i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_SUPPORTED_APPLICATION_CONTEXT: CERTENROLL_OBJECTID = 148i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_MEMBER: CERTENROLL_OBJECTID = 149i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OWNER: CERTENROLL_OBJECTID = 150i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ROLE_OCCUPANT: CERTENROLL_OBJECTID = 151i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_SEE_ALSO: CERTENROLL_OBJECTID = 152i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_USER_PASSWORD: CERTENROLL_OBJECTID = 153i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_USER_CERTIFICATE: CERTENROLL_OBJECTID = 154i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CA_CERTIFICATE: CERTENROLL_OBJECTID = 155i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_AUTHORITY_REVOCATION_LIST: CERTENROLL_OBJECTID = 156i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CERTIFICATE_REVOCATION_LIST: CERTENROLL_OBJECTID = 157i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CROSS_CERTIFICATE_PAIR: CERTENROLL_OBJECTID = 158i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_GIVEN_NAME: CERTENROLL_OBJECTID = 159i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INITIALS: CERTENROLL_OBJECTID = 160i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_DN_QUALIFIER: CERTENROLL_OBJECTID = 161i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_DOMAIN_COMPONENT: CERTENROLL_OBJECTID = 162i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_12_FRIENDLY_NAME_ATTR: CERTENROLL_OBJECTID = 163i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_12_LOCAL_KEY_ID: CERTENROLL_OBJECTID = 164i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_12_KEY_PROVIDER_NAME_ATTR: CERTENROLL_OBJECTID = 165i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_LOCAL_MACHINE_KEYSET: CERTENROLL_OBJECTID = 166i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_12_EXTENDED_ATTRIBUTES: CERTENROLL_OBJECTID = 167i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_KEYID_RDN: CERTENROLL_OBJECTID = 168i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_AUTHORITY_KEY_IDENTIFIER: CERTENROLL_OBJECTID = 169i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_KEY_ATTRIBUTES: CERTENROLL_OBJECTID = 170i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CERT_POLICIES_95: CERTENROLL_OBJECTID = 171i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_KEY_USAGE_RESTRICTION: CERTENROLL_OBJECTID = 172i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_SUBJECT_ALT_NAME: CERTENROLL_OBJECTID = 173i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ISSUER_ALT_NAME: CERTENROLL_OBJECTID = 174i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_BASIC_CONSTRAINTS: CERTENROLL_OBJECTID = 175i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_KEY_USAGE: CERTENROLL_OBJECTID = 176i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PRIVATEKEY_USAGE_PERIOD: CERTENROLL_OBJECTID = 177i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_BASIC_CONSTRAINTS2: CERTENROLL_OBJECTID = 178i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CERT_POLICIES: CERTENROLL_OBJECTID = 179i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ANY_CERT_POLICY: CERTENROLL_OBJECTID = 180i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_AUTHORITY_KEY_IDENTIFIER2: CERTENROLL_OBJECTID = 181i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_SUBJECT_KEY_IDENTIFIER: CERTENROLL_OBJECTID = 182i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_SUBJECT_ALT_NAME2: CERTENROLL_OBJECTID = 183i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ISSUER_ALT_NAME2: CERTENROLL_OBJECTID = 184i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CRL_REASON_CODE: CERTENROLL_OBJECTID = 185i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_REASON_CODE_HOLD: CERTENROLL_OBJECTID = 186i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CRL_DIST_POINTS: CERTENROLL_OBJECTID = 187i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ENHANCED_KEY_USAGE: CERTENROLL_OBJECTID = 188i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CRL_NUMBER: CERTENROLL_OBJECTID = 189i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_DELTA_CRL_INDICATOR: CERTENROLL_OBJECTID = 190i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ISSUING_DIST_POINT: CERTENROLL_OBJECTID = 191i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_FRESHEST_CRL: CERTENROLL_OBJECTID = 192i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NAME_CONSTRAINTS: CERTENROLL_OBJECTID = 193i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_POLICY_MAPPINGS: CERTENROLL_OBJECTID = 194i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_LEGACY_POLICY_MAPPINGS: CERTENROLL_OBJECTID = 195i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_POLICY_CONSTRAINTS: CERTENROLL_OBJECTID = 196i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RENEWAL_CERTIFICATE: CERTENROLL_OBJECTID = 197i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ENROLLMENT_NAME_VALUE_PAIR: CERTENROLL_OBJECTID = 198i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ENROLLMENT_CSP_PROVIDER: CERTENROLL_OBJECTID = 199i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OS_VERSION: CERTENROLL_OBJECTID = 200i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ENROLLMENT_AGENT: CERTENROLL_OBJECTID = 201i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKIX: CERTENROLL_OBJECTID = 202i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKIX_PE: CERTENROLL_OBJECTID = 203i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_AUTHORITY_INFO_ACCESS: CERTENROLL_OBJECTID = 204i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_BIOMETRIC_EXT: CERTENROLL_OBJECTID = 205i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_LOGOTYPE_EXT: CERTENROLL_OBJECTID = 206i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CERT_EXTENSIONS: CERTENROLL_OBJECTID = 207i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NEXT_UPDATE_LOCATION: CERTENROLL_OBJECTID = 208i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_REMOVE_CERTIFICATE: CERTENROLL_OBJECTID = 209i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CROSS_CERT_DIST_POINTS: CERTENROLL_OBJECTID = 210i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CTL: CERTENROLL_OBJECTID = 211i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_SORTED_CTL: CERTENROLL_OBJECTID = 212i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_SERIALIZED: CERTENROLL_OBJECTID = 213i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NT_PRINCIPAL_NAME: CERTENROLL_OBJECTID = 214i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PRODUCT_UPDATE: CERTENROLL_OBJECTID = 215i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ANY_APPLICATION_POLICY: CERTENROLL_OBJECTID = 216i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_AUTO_ENROLL_CTL_USAGE: CERTENROLL_OBJECTID = 217i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ENROLL_CERTTYPE_EXTENSION: CERTENROLL_OBJECTID = 218i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CERT_MANIFOLD: CERTENROLL_OBJECTID = 219i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CERTSRV_CA_VERSION: CERTENROLL_OBJECTID = 220i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CERTSRV_PREVIOUS_CERT_HASH: CERTENROLL_OBJECTID = 221i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CRL_VIRTUAL_BASE: CERTENROLL_OBJECTID = 222i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CRL_NEXT_PUBLISH: CERTENROLL_OBJECTID = 223i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_KP_CA_EXCHANGE: CERTENROLL_OBJECTID = 224i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_KP_KEY_RECOVERY_AGENT: CERTENROLL_OBJECTID = 225i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CERTIFICATE_TEMPLATE: CERTENROLL_OBJECTID = 226i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ENTERPRISE_OID_ROOT: CERTENROLL_OBJECTID = 227i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RDN_DUMMY_SIGNER: CERTENROLL_OBJECTID = 228i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_APPLICATION_CERT_POLICIES: CERTENROLL_OBJECTID = 229i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_APPLICATION_POLICY_MAPPINGS: CERTENROLL_OBJECTID = 230i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_APPLICATION_POLICY_CONSTRAINTS: CERTENROLL_OBJECTID = 231i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ARCHIVED_KEY_ATTR: CERTENROLL_OBJECTID = 232i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CRL_SELF_CDP: CERTENROLL_OBJECTID = 233i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_REQUIRE_CERT_CHAIN_POLICY: CERTENROLL_OBJECTID = 234i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ARCHIVED_KEY_CERT_HASH: CERTENROLL_OBJECTID = 235i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ISSUED_CERT_HASH: CERTENROLL_OBJECTID = 236i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_DS_EMAIL_REPLICATION: CERTENROLL_OBJECTID = 237i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_REQUEST_CLIENT_INFO: CERTENROLL_OBJECTID = 238i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ENCRYPTED_KEY_HASH: CERTENROLL_OBJECTID = 239i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CERTSRV_CROSSCA_VERSION: CERTENROLL_OBJECTID = 240i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NTDS_REPLICATION: CERTENROLL_OBJECTID = 241i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_SUBJECT_DIR_ATTRS: CERTENROLL_OBJECTID = 242i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKIX_KP: CERTENROLL_OBJECTID = 243i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKIX_KP_SERVER_AUTH: CERTENROLL_OBJECTID = 244i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKIX_KP_CLIENT_AUTH: CERTENROLL_OBJECTID = 245i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKIX_KP_CODE_SIGNING: CERTENROLL_OBJECTID = 246i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKIX_KP_EMAIL_PROTECTION: CERTENROLL_OBJECTID = 247i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKIX_KP_IPSEC_END_SYSTEM: CERTENROLL_OBJECTID = 248i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKIX_KP_IPSEC_TUNNEL: CERTENROLL_OBJECTID = 249i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKIX_KP_IPSEC_USER: CERTENROLL_OBJECTID = 250i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKIX_KP_TIMESTAMP_SIGNING: CERTENROLL_OBJECTID = 251i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKIX_KP_OCSP_SIGNING: CERTENROLL_OBJECTID = 252i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKIX_OCSP_NOCHECK: CERTENROLL_OBJECTID = 253i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_IPSEC_KP_IKE_INTERMEDIATE: CERTENROLL_OBJECTID = 254i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_KP_CTL_USAGE_SIGNING: CERTENROLL_OBJECTID = 255i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_KP_TIME_STAMP_SIGNING: CERTENROLL_OBJECTID = 256i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_SERVER_GATED_CRYPTO: CERTENROLL_OBJECTID = 257i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_SGC_NETSCAPE: CERTENROLL_OBJECTID = 258i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_KP_EFS: CERTENROLL_OBJECTID = 259i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_EFS_RECOVERY: CERTENROLL_OBJECTID = 260i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_WHQL_CRYPTO: CERTENROLL_OBJECTID = 261i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NT5_CRYPTO: CERTENROLL_OBJECTID = 262i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_OEM_WHQL_CRYPTO: CERTENROLL_OBJECTID = 263i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_EMBEDDED_NT_CRYPTO: CERTENROLL_OBJECTID = 264i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ROOT_LIST_SIGNER: CERTENROLL_OBJECTID = 265i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_KP_QUALIFIED_SUBORDINATION: CERTENROLL_OBJECTID = 266i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_KP_KEY_RECOVERY: CERTENROLL_OBJECTID = 267i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_KP_DOCUMENT_SIGNING: CERTENROLL_OBJECTID = 268i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_KP_LIFETIME_SIGNING: CERTENROLL_OBJECTID = 269i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_KP_MOBILE_DEVICE_SOFTWARE: CERTENROLL_OBJECTID = 270i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_KP_SMART_DISPLAY: CERTENROLL_OBJECTID = 271i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_KP_CSP_SIGNATURE: CERTENROLL_OBJECTID = 272i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_DRM: CERTENROLL_OBJECTID = 273i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_DRM_INDIVIDUALIZATION: CERTENROLL_OBJECTID = 274i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_LICENSES: CERTENROLL_OBJECTID = 275i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_LICENSE_SERVER: CERTENROLL_OBJECTID = 276i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_KP_SMARTCARD_LOGON: CERTENROLL_OBJECTID = 277i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_YESNO_TRUST_ATTR: CERTENROLL_OBJECTID = 278i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKIX_POLICY_QUALIFIER_CPS: CERTENROLL_OBJECTID = 279i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKIX_POLICY_QUALIFIER_USERNOTICE: CERTENROLL_OBJECTID = 280i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CERT_POLICIES_95_QUALIFIER1: CERTENROLL_OBJECTID = 281i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKIX_ACC_DESCR: CERTENROLL_OBJECTID = 282i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKIX_OCSP: CERTENROLL_OBJECTID = 283i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKIX_CA_ISSUERS: CERTENROLL_OBJECTID = 284i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_VERISIGN_PRIVATE_6_9: CERTENROLL_OBJECTID = 285i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_VERISIGN_ONSITE_JURISDICTION_HASH: CERTENROLL_OBJECTID = 286i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_VERISIGN_BITSTRING_6_13: CERTENROLL_OBJECTID = 287i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_VERISIGN_ISS_STRONG_CRYPTO: CERTENROLL_OBJECTID = 288i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NETSCAPE: CERTENROLL_OBJECTID = 289i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NETSCAPE_CERT_EXTENSION: CERTENROLL_OBJECTID = 290i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NETSCAPE_CERT_TYPE: CERTENROLL_OBJECTID = 291i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NETSCAPE_BASE_URL: CERTENROLL_OBJECTID = 292i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NETSCAPE_REVOCATION_URL: CERTENROLL_OBJECTID = 293i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NETSCAPE_CA_REVOCATION_URL: CERTENROLL_OBJECTID = 294i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NETSCAPE_CERT_RENEWAL_URL: CERTENROLL_OBJECTID = 295i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NETSCAPE_CA_POLICY_URL: CERTENROLL_OBJECTID = 296i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NETSCAPE_SSL_SERVER_NAME: CERTENROLL_OBJECTID = 297i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NETSCAPE_COMMENT: CERTENROLL_OBJECTID = 298i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NETSCAPE_DATA_TYPE: CERTENROLL_OBJECTID = 299i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NETSCAPE_CERT_SEQUENCE: CERTENROLL_OBJECTID = 300i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CT_PKI_DATA: CERTENROLL_OBJECTID = 301i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CT_PKI_RESPONSE: CERTENROLL_OBJECTID = 302i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKIX_NO_SIGNATURE: CERTENROLL_OBJECTID = 303i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CMC: CERTENROLL_OBJECTID = 304i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CMC_STATUS_INFO: CERTENROLL_OBJECTID = 305i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CMC_IDENTIFICATION: CERTENROLL_OBJECTID = 306i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CMC_IDENTITY_PROOF: CERTENROLL_OBJECTID = 307i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CMC_DATA_RETURN: CERTENROLL_OBJECTID = 308i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CMC_TRANSACTION_ID: CERTENROLL_OBJECTID = 309i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CMC_SENDER_NONCE: CERTENROLL_OBJECTID = 310i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CMC_RECIPIENT_NONCE: CERTENROLL_OBJECTID = 311i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CMC_ADD_EXTENSIONS: CERTENROLL_OBJECTID = 312i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CMC_ENCRYPTED_POP: CERTENROLL_OBJECTID = 313i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CMC_DECRYPTED_POP: CERTENROLL_OBJECTID = 314i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CMC_LRA_POP_WITNESS: CERTENROLL_OBJECTID = 315i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CMC_GET_CERT: CERTENROLL_OBJECTID = 316i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CMC_GET_CRL: CERTENROLL_OBJECTID = 317i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CMC_REVOKE_REQUEST: CERTENROLL_OBJECTID = 318i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CMC_REG_INFO: CERTENROLL_OBJECTID = 319i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CMC_RESPONSE_INFO: CERTENROLL_OBJECTID = 320i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CMC_QUERY_PENDING: CERTENROLL_OBJECTID = 321i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CMC_ID_POP_LINK_RANDOM: CERTENROLL_OBJECTID = 322i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CMC_ID_POP_LINK_WITNESS: CERTENROLL_OBJECTID = 323i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CMC_ID_CONFIRM_CERT_ACCEPTANCE: CERTENROLL_OBJECTID = 324i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CMC_ADD_ATTRIBUTES: CERTENROLL_OBJECTID = 325i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_LOYALTY_OTHER_LOGOTYPE: CERTENROLL_OBJECTID = 326i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_BACKGROUND_OTHER_LOGOTYPE: CERTENROLL_OBJECTID = 327i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKIX_OCSP_BASIC_SIGNED_RESPONSE: CERTENROLL_OBJECTID = 328i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_7_DATA: CERTENROLL_OBJECTID = 329i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_7_SIGNED: CERTENROLL_OBJECTID = 330i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_7_ENVELOPED: CERTENROLL_OBJECTID = 331i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_7_SIGNEDANDENVELOPED: CERTENROLL_OBJECTID = 332i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_7_DIGESTED: CERTENROLL_OBJECTID = 333i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_7_ENCRYPTED: CERTENROLL_OBJECTID = 334i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_9_CONTENT_TYPE: CERTENROLL_OBJECTID = 335i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_9_MESSAGE_DIGEST: CERTENROLL_OBJECTID = 336i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CERT_PROP_ID_PREFIX: CERTENROLL_OBJECTID = 337i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CERT_KEY_IDENTIFIER_PROP_ID: CERTENROLL_OBJECTID = 338i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CERT_ISSUER_SERIAL_NUMBER_MD5_HASH_PROP_ID: CERTENROLL_OBJECTID = 339i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CERT_SUBJECT_NAME_MD5_HASH_PROP_ID: CERTENROLL_OBJECTID = 340i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CERT_MD5_HASH_PROP_ID: CERTENROLL_OBJECTID = 341i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_SHA256RSA: CERTENROLL_OBJECTID = 342i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_SHA384RSA: CERTENROLL_OBJECTID = 343i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_SHA512RSA: CERTENROLL_OBJECTID = 344i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NIST_sha256: CERTENROLL_OBJECTID = 345i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NIST_sha384: CERTENROLL_OBJECTID = 346i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NIST_sha512: CERTENROLL_OBJECTID = 347i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_MGF1: CERTENROLL_OBJECTID = 348i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ECC_PUBLIC_KEY: CERTENROLL_OBJECTID = 349i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ECDSA_SHA1: CERTENROLL_OBJECTID = 350i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ECDSA_SPECIFIED: CERTENROLL_OBJECTID = 351i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ANY_ENHANCED_KEY_USAGE: CERTENROLL_OBJECTID = 352i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_SSA_PSS: CERTENROLL_OBJECTID = 353i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ATTR_SUPPORTED_ALGORITHMS: CERTENROLL_OBJECTID = 355i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ATTR_TPM_SECURITY_ASSERTIONS: CERTENROLL_OBJECTID = 356i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ATTR_TPM_SPECIFICATION: CERTENROLL_OBJECTID = 357i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CERT_DISALLOWED_FILETIME_PROP_ID: CERTENROLL_OBJECTID = 358i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CERT_SIGNATURE_HASH_PROP_ID: CERTENROLL_OBJECTID = 359i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CERT_STRONG_KEY_OS_1: CERTENROLL_OBJECTID = 360i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CERT_STRONG_KEY_OS_CURRENT: CERTENROLL_OBJECTID = 361i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CERT_STRONG_KEY_OS_PREFIX: CERTENROLL_OBJECTID = 362i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CERT_STRONG_SIGN_OS_1: CERTENROLL_OBJECTID = 363i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CERT_STRONG_SIGN_OS_CURRENT: CERTENROLL_OBJECTID = 364i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_CERT_STRONG_SIGN_OS_PREFIX: CERTENROLL_OBJECTID = 365i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_DH_SINGLE_PASS_STDDH_SHA1_KDF: CERTENROLL_OBJECTID = 366i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_DH_SINGLE_PASS_STDDH_SHA256_KDF: CERTENROLL_OBJECTID = 367i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_DH_SINGLE_PASS_STDDH_SHA384_KDF: CERTENROLL_OBJECTID = 368i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_DISALLOWED_HASH: CERTENROLL_OBJECTID = 369i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_DISALLOWED_LIST: CERTENROLL_OBJECTID = 370i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ECC_CURVE_P256: CERTENROLL_OBJECTID = 371i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ECC_CURVE_P384: CERTENROLL_OBJECTID = 372i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ECC_CURVE_P521: CERTENROLL_OBJECTID = 373i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ECDSA_SHA256: CERTENROLL_OBJECTID = 374i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ECDSA_SHA384: CERTENROLL_OBJECTID = 375i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ECDSA_SHA512: CERTENROLL_OBJECTID = 376i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ENROLL_CAXCHGCERT_HASH: CERTENROLL_OBJECTID = 377i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ENROLL_EK_INFO: CERTENROLL_OBJECTID = 378i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ENROLL_EKPUB_CHALLENGE: CERTENROLL_OBJECTID = 379i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ENROLL_EKVERIFYCERT: CERTENROLL_OBJECTID = 380i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ENROLL_EKVERIFYCREDS: CERTENROLL_OBJECTID = 381i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ENROLL_EKVERIFYKEY: CERTENROLL_OBJECTID = 382i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_EV_RDN_COUNTRY: CERTENROLL_OBJECTID = 383i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_EV_RDN_LOCALE: CERTENROLL_OBJECTID = 384i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_EV_RDN_STATE_OR_PROVINCE: CERTENROLL_OBJECTID = 385i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INHIBIT_ANY_POLICY: CERTENROLL_OBJECTID = 386i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_INTERNATIONALIZED_EMAIL_ADDRESS: CERTENROLL_OBJECTID = 387i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_KP_KERNEL_MODE_CODE_SIGNING: CERTENROLL_OBJECTID = 388i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_KP_KERNEL_MODE_HAL_EXTENSION_SIGNING: CERTENROLL_OBJECTID = 389i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_KP_KERNEL_MODE_TRUSTED_BOOT_SIGNING: CERTENROLL_OBJECTID = 390i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_KP_TPM_AIK_CERTIFICATE: CERTENROLL_OBJECTID = 391i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_KP_TPM_EK_CERTIFICATE: CERTENROLL_OBJECTID = 392i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_KP_TPM_PLATFORM_CERTIFICATE: CERTENROLL_OBJECTID = 393i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NIST_AES128_CBC: CERTENROLL_OBJECTID = 394i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NIST_AES128_WRAP: CERTENROLL_OBJECTID = 395i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NIST_AES192_CBC: CERTENROLL_OBJECTID = 396i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NIST_AES192_WRAP: CERTENROLL_OBJECTID = 397i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NIST_AES256_CBC: CERTENROLL_OBJECTID = 398i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_NIST_AES256_WRAP: CERTENROLL_OBJECTID = 399i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_12_PbeIds: CERTENROLL_OBJECTID = 400i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_12_pbeWithSHA1And128BitRC2: CERTENROLL_OBJECTID = 401i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_12_pbeWithSHA1And128BitRC4: CERTENROLL_OBJECTID = 402i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_12_pbeWithSHA1And2KeyTripleDES: CERTENROLL_OBJECTID = 403i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_12_pbeWithSHA1And3KeyTripleDES: CERTENROLL_OBJECTID = 404i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_12_pbeWithSHA1And40BitRC2: CERTENROLL_OBJECTID = 405i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_12_pbeWithSHA1And40BitRC4: CERTENROLL_OBJECTID = 406i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKCS_12_PROTECTED_PASSWORD_SECRET_BAG_TYPE_ID: CERTENROLL_OBJECTID = 407i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKINIT_KP_KDC: CERTENROLL_OBJECTID = 408i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKIX_CA_REPOSITORY: CERTENROLL_OBJECTID = 409i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKIX_OCSP_NONCE: CERTENROLL_OBJECTID = 410i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_PKIX_TIME_STAMPING: CERTENROLL_OBJECTID = 411i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_QC_EU_COMPLIANCE: CERTENROLL_OBJECTID = 412i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_QC_SSCD: CERTENROLL_OBJECTID = 413i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_QC_STATEMENTS_EXT: CERTENROLL_OBJECTID = 414i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RDN_TPM_MANUFACTURER: CERTENROLL_OBJECTID = 415i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RDN_TPM_MODEL: CERTENROLL_OBJECTID = 416i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RDN_TPM_VERSION: CERTENROLL_OBJECTID = 417i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_REVOKED_LIST_SIGNER: CERTENROLL_OBJECTID = 418i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RFC3161_counterSign: CERTENROLL_OBJECTID = 419i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ROOT_PROGRAM_AUTO_UPDATE_CA_REVOCATION: CERTENROLL_OBJECTID = 420i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ROOT_PROGRAM_AUTO_UPDATE_END_REVOCATION: CERTENROLL_OBJECTID = 421i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ROOT_PROGRAM_FLAGS: CERTENROLL_OBJECTID = 422i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ROOT_PROGRAM_NO_OCSP_FAILOVER_TO_CRL: CERTENROLL_OBJECTID = 423i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSA_PSPECIFIED: CERTENROLL_OBJECTID = 424i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_RSAES_OAEP: CERTENROLL_OBJECTID = 425i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_SUBJECT_INFO_ACCESS: CERTENROLL_OBJECTID = 426i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_TIMESTAMP_TOKEN: CERTENROLL_OBJECTID = 427i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ENROLL_SCEP_ERROR: CERTENROLL_OBJECTID = 428i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OIDVerisign_MessageType: CERTENROLL_OBJECTID = 429i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OIDVerisign_PkiStatus: CERTENROLL_OBJECTID = 430i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OIDVerisign_FailInfo: CERTENROLL_OBJECTID = 431i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OIDVerisign_SenderNonce: CERTENROLL_OBJECTID = 432i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OIDVerisign_RecipientNonce: CERTENROLL_OBJECTID = 433i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OIDVerisign_TransactionID: CERTENROLL_OBJECTID = 434i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ENROLL_ATTESTATION_CHALLENGE: CERTENROLL_OBJECTID = 435i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ENROLL_ATTESTATION_STATEMENT: CERTENROLL_OBJECTID = 436i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ENROLL_ENCRYPTION_ALGORITHM: CERTENROLL_OBJECTID = 437i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_OID_ENROLL_KSP_NAME: CERTENROLL_OBJECTID = 438i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type CERTENROLL_PROPERTYID = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_PROPERTYID_NONE: CERTENROLL_PROPERTYID = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_KEY_PROV_HANDLE_PROP_ID: CERTENROLL_PROPERTYID = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_KEY_PROV_INFO_PROP_ID: CERTENROLL_PROPERTYID = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SHA1_HASH_PROP_ID: CERTENROLL_PROPERTYID = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_MD5_HASH_PROP_ID: CERTENROLL_PROPERTYID = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_HASH_PROP_ID: CERTENROLL_PROPERTYID = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_KEY_CONTEXT_PROP_ID: CERTENROLL_PROPERTYID = 5i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_KEY_SPEC_PROP_ID: CERTENROLL_PROPERTYID = 6i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_IE30_RESERVED_PROP_ID: CERTENROLL_PROPERTYID = 7i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_PUBKEY_HASH_RESERVED_PROP_ID: CERTENROLL_PROPERTYID = 8i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ENHKEY_USAGE_PROP_ID: CERTENROLL_PROPERTYID = 9i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_CTL_USAGE_PROP_ID: CERTENROLL_PROPERTYID = 9i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NEXT_UPDATE_LOCATION_PROP_ID: CERTENROLL_PROPERTYID = 10i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_FRIENDLY_NAME_PROP_ID: CERTENROLL_PROPERTYID = 11i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_PVK_FILE_PROP_ID: CERTENROLL_PROPERTYID = 12i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_DESCRIPTION_PROP_ID: CERTENROLL_PROPERTYID = 13i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ACCESS_STATE_PROP_ID: CERTENROLL_PROPERTYID = 14i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SIGNATURE_HASH_PROP_ID: CERTENROLL_PROPERTYID = 15i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SMART_CARD_DATA_PROP_ID: CERTENROLL_PROPERTYID = 16i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_EFS_PROP_ID: CERTENROLL_PROPERTYID = 17i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_FORTEZZA_DATA_PROP_ID: CERTENROLL_PROPERTYID = 18i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ARCHIVED_PROP_ID: CERTENROLL_PROPERTYID = 19i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_KEY_IDENTIFIER_PROP_ID: CERTENROLL_PROPERTYID = 20i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_AUTO_ENROLL_PROP_ID: CERTENROLL_PROPERTYID = 21i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_PUBKEY_ALG_PARA_PROP_ID: CERTENROLL_PROPERTYID = 22i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_CROSS_CERT_DIST_POINTS_PROP_ID: CERTENROLL_PROPERTYID = 23i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ISSUER_PUBLIC_KEY_MD5_HASH_PROP_ID: CERTENROLL_PROPERTYID = 24i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SUBJECT_PUBLIC_KEY_MD5_HASH_PROP_ID: CERTENROLL_PROPERTYID = 25i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ENROLLMENT_PROP_ID: CERTENROLL_PROPERTYID = 26i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_DATE_STAMP_PROP_ID: CERTENROLL_PROPERTYID = 27i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ISSUER_SERIAL_NUMBER_MD5_HASH_PROP_ID: CERTENROLL_PROPERTYID = 28i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SUBJECT_NAME_MD5_HASH_PROP_ID: CERTENROLL_PROPERTYID = 29i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_EXTENDED_ERROR_INFO_PROP_ID: CERTENROLL_PROPERTYID = 30i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_RENEWAL_PROP_ID: CERTENROLL_PROPERTYID = 64i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ARCHIVED_KEY_HASH_PROP_ID: CERTENROLL_PROPERTYID = 65i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_AUTO_ENROLL_RETRY_PROP_ID: CERTENROLL_PROPERTYID = 66i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_AIA_URL_RETRIEVED_PROP_ID: CERTENROLL_PROPERTYID = 67i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_AUTHORITY_INFO_ACCESS_PROP_ID: CERTENROLL_PROPERTYID = 68i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_BACKED_UP_PROP_ID: CERTENROLL_PROPERTYID = 69i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_OCSP_RESPONSE_PROP_ID: CERTENROLL_PROPERTYID = 70i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_REQUEST_ORIGINATOR_PROP_ID: CERTENROLL_PROPERTYID = 71i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SOURCE_LOCATION_PROP_ID: CERTENROLL_PROPERTYID = 72i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SOURCE_URL_PROP_ID: CERTENROLL_PROPERTYID = 73i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NEW_KEY_PROP_ID: CERTENROLL_PROPERTYID = 74i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_OCSP_CACHE_PREFIX_PROP_ID: CERTENROLL_PROPERTYID = 75i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SMART_CARD_ROOT_INFO_PROP_ID: CERTENROLL_PROPERTYID = 76i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NO_AUTO_EXPIRE_CHECK_PROP_ID: CERTENROLL_PROPERTYID = 77i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NCRYPT_KEY_HANDLE_PROP_ID: CERTENROLL_PROPERTYID = 78i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_HCRYPTPROV_OR_NCRYPT_KEY_HANDLE_PROP_ID: CERTENROLL_PROPERTYID = 79i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SUBJECT_INFO_ACCESS_PROP_ID: CERTENROLL_PROPERTYID = 80i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_CA_OCSP_AUTHORITY_INFO_ACCESS_PROP_ID: CERTENROLL_PROPERTYID = 81i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_CA_DISABLE_CRL_PROP_ID: CERTENROLL_PROPERTYID = 82i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ROOT_PROGRAM_CERT_POLICIES_PROP_ID: CERTENROLL_PROPERTYID = 83i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ROOT_PROGRAM_NAME_CONSTRAINTS_PROP_ID: CERTENROLL_PROPERTYID = 84i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SUBJECT_OCSP_AUTHORITY_INFO_ACCESS_PROP_ID: CERTENROLL_PROPERTYID = 85i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SUBJECT_DISABLE_CRL_PROP_ID: CERTENROLL_PROPERTYID = 86i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_CEP_PROP_ID: CERTENROLL_PROPERTYID = 87i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SIGN_HASH_CNG_ALG_PROP_ID: CERTENROLL_PROPERTYID = 89i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SCARD_PIN_ID_PROP_ID: CERTENROLL_PROPERTYID = 90i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SCARD_PIN_INFO_PROP_ID: CERTENROLL_PROPERTYID = 91i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SUBJECT_PUB_KEY_BIT_LENGTH_PROP_ID: CERTENROLL_PROPERTYID = 92i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_PUB_KEY_CNG_ALG_BIT_LENGTH_PROP_ID: CERTENROLL_PROPERTYID = 93i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ISSUER_PUB_KEY_BIT_LENGTH_PROP_ID: CERTENROLL_PROPERTYID = 94i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ISSUER_CHAIN_SIGN_HASH_CNG_ALG_PROP_ID: CERTENROLL_PROPERTYID = 95i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ISSUER_CHAIN_PUB_KEY_CNG_ALG_BIT_LENGTH_PROP_ID: CERTENROLL_PROPERTYID = 96i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NO_EXPIRE_NOTIFICATION_PROP_ID: CERTENROLL_PROPERTYID = 97i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_AUTH_ROOT_SHA256_HASH_PROP_ID: CERTENROLL_PROPERTYID = 98i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NCRYPT_KEY_HANDLE_TRANSFER_PROP_ID: CERTENROLL_PROPERTYID = 99i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_HCRYPTPROV_TRANSFER_PROP_ID: CERTENROLL_PROPERTYID = 100i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SMART_CARD_READER_PROP_ID: CERTENROLL_PROPERTYID = 101i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SEND_AS_TRUSTED_ISSUER_PROP_ID: CERTENROLL_PROPERTYID = 102i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_KEY_REPAIR_ATTEMPTED_PROP_ID: CERTENROLL_PROPERTYID = 103i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_DISALLOWED_FILETIME_PROP_ID: CERTENROLL_PROPERTYID = 104i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ROOT_PROGRAM_CHAIN_POLICIES_PROP_ID: CERTENROLL_PROPERTYID = 105i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SMART_CARD_READER_NON_REMOVABLE_PROP_ID: CERTENROLL_PROPERTYID = 106i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SHA256_HASH_PROP_ID: CERTENROLL_PROPERTYID = 107i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SCEP_SERVER_CERTS_PROP_ID: CERTENROLL_PROPERTYID = 108i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SCEP_RA_SIGNATURE_CERT_PROP_ID: CERTENROLL_PROPERTYID = 109i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SCEP_RA_ENCRYPTION_CERT_PROP_ID: CERTENROLL_PROPERTYID = 110i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SCEP_CA_CERT_PROP_ID: CERTENROLL_PROPERTYID = 111i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SCEP_SIGNER_CERT_PROP_ID: CERTENROLL_PROPERTYID = 112i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SCEP_NONCE_PROP_ID: CERTENROLL_PROPERTYID = 113i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SCEP_ENCRYPT_HASH_CNG_ALG_PROP_ID: CERTENROLL_PROPERTYID = 114i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SCEP_FLAGS_PROP_ID: CERTENROLL_PROPERTYID = 115i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SCEP_GUID_PROP_ID: CERTENROLL_PROPERTYID = 116i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SERIALIZABLE_KEY_CONTEXT_PROP_ID: CERTENROLL_PROPERTYID = 117i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ISOLATED_KEY_PROP_ID: CERTENROLL_PROPERTYID = 118i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SERIAL_CHAIN_PROP_ID: CERTENROLL_PROPERTYID = 119i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_KEY_CLASSIFICATION_PROP_ID: CERTENROLL_PROPERTYID = 120i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_DISALLOWED_ENHKEY_USAGE_PROP_ID: CERTENROLL_PROPERTYID = 122i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NONCOMPLIANT_ROOT_URL_PROP_ID: CERTENROLL_PROPERTYID = 123i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_PIN_SHA256_HASH_PROP_ID: CERTENROLL_PROPERTYID = 124i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_CLR_DELETE_KEY_PROP_ID: CERTENROLL_PROPERTYID = 125i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NOT_BEFORE_FILETIME_PROP_ID: CERTENROLL_PROPERTYID = 126i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_CERT_NOT_BEFORE_ENHKEY_USAGE_PROP_ID: CERTENROLL_PROPERTYID = 127i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_FIRST_RESERVED_PROP_ID: CERTENROLL_PROPERTYID = 128i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_LAST_RESERVED_PROP_ID: CERTENROLL_PROPERTYID = 32767i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_FIRST_USER_PROP_ID: CERTENROLL_PROPERTYID = 32768i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_LAST_USER_PROP_ID: CERTENROLL_PROPERTYID = 65535i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_STORE_LOCALIZED_NAME_PROP_ID: CERTENROLL_PROPERTYID = 4096i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub struct CERTTRANSBLOB {
    pub cb: u32,
    pub pb: *mut u8,
}
impl ::core::marker::Copy for CERTTRANSBLOB {}
impl ::core::clone::Clone for CERTTRANSBLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub struct CERTVIEWRESTRICTION {
    pub ColumnIndex: u32,
    pub SeekOperator: i32,
    pub SortOrder: i32,
    pub pbValue: *mut u8,
    pub cbValue: u32,
}
impl ::core::marker::Copy for CERTVIEWRESTRICTION {}
impl ::core::clone::Clone for CERTVIEWRESTRICTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type CERT_ALT_NAME = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CERT_ALT_NAME_RFC822_NAME: CERT_ALT_NAME = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CERT_ALT_NAME_DNS_NAME: CERT_ALT_NAME = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CERT_ALT_NAME_URL: CERT_ALT_NAME = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CERT_ALT_NAME_REGISTERED_ID: CERT_ALT_NAME = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CERT_ALT_NAME_DIRECTORY_NAME: CERT_ALT_NAME = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CERT_ALT_NAME_IP_ADDRESS: CERT_ALT_NAME = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CERT_ALT_NAME_OTHER_NAME: CERT_ALT_NAME = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type CERT_CREATE_REQUEST_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XECR_CMC: CERT_CREATE_REQUEST_FLAGS = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XECR_PKCS10_V1_5: CERT_CREATE_REQUEST_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XECR_PKCS10_V2_0: CERT_CREATE_REQUEST_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XECR_PKCS7: CERT_CREATE_REQUEST_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type CERT_DELETE_ROW_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CDR_EXPIRED: CERT_DELETE_ROW_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CDR_REQUEST_LAST_CHANGED: CERT_DELETE_ROW_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type CERT_EXIT_EVENT_MASK = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXITEVENT_CERTDENIED: CERT_EXIT_EVENT_MASK = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXITEVENT_CERTISSUED: CERT_EXIT_EVENT_MASK = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXITEVENT_CERTPENDING: CERT_EXIT_EVENT_MASK = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXITEVENT_CERTRETRIEVEPENDING: CERT_EXIT_EVENT_MASK = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXITEVENT_CERTREVOKED: CERT_EXIT_EVENT_MASK = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXITEVENT_CRLISSUED: CERT_EXIT_EVENT_MASK = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXITEVENT_SHUTDOWN: CERT_EXIT_EVENT_MASK = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type CERT_GET_CONFIG_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CC_DEFAULTCONFIG: CERT_GET_CONFIG_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CC_FIRSTCONFIG: CERT_GET_CONFIG_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CC_LOCALACTIVECONFIG: CERT_GET_CONFIG_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CC_LOCALCONFIG: CERT_GET_CONFIG_FLAGS = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CC_UIPICKCONFIG: CERT_GET_CONFIG_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CC_UIPICKCONFIGSKIPLOCALCA: CERT_GET_CONFIG_FLAGS = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type CERT_IMPORT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_IN_BASE64HEADER: CERT_IMPORT_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_IN_BASE64: CERT_IMPORT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_IN_BINARY: CERT_IMPORT_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type CERT_PROPERTY_TYPE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PROPTYPE_BINARY: CERT_PROPERTY_TYPE = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PROPTYPE_DATE: CERT_PROPERTY_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PROPTYPE_LONG: CERT_PROPERTY_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PROPTYPE_STRING: CERT_PROPERTY_TYPE = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type CERT_REQUEST_OUT_TYPE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_OUT_BASE64HEADER: CERT_REQUEST_OUT_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_OUT_BASE64: CERT_REQUEST_OUT_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_OUT_BINARY: CERT_REQUEST_OUT_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type CERT_VIEW_COLUMN_INDEX = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CV_COLUMN_LOG_DEFAULT: CERT_VIEW_COLUMN_INDEX = -2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CV_COLUMN_LOG_FAILED_DEFAULT: CERT_VIEW_COLUMN_INDEX = -3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CV_COLUMN_QUEUE_DEFAULT: CERT_VIEW_COLUMN_INDEX = -1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type CERT_VIEW_SEEK_OPERATOR_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CVR_SEEK_EQ: CERT_VIEW_SEEK_OPERATOR_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CVR_SEEK_LE: CERT_VIEW_SEEK_OPERATOR_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CVR_SEEK_LT: CERT_VIEW_SEEK_OPERATOR_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CVR_SEEK_GE: CERT_VIEW_SEEK_OPERATOR_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CVR_SEEK_GT: CERT_VIEW_SEEK_OPERATOR_FLAGS = 16u32;
pub const CEnroll: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1140388489, data2: 31264, data3: 4560, data4: [143, 6, 0, 192, 79, 194, 149, 225] };
pub const CEnroll2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 309762276, data2: 59184, data3: 20060, data4: [162, 177, 33, 73, 10, 112, 200, 161] };
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CMM_READONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CMM_REFRESHONLY: u32 = 1u32;
pub const CObjectId: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821376, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CObjectIds: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821377, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CPF_BADURL_ERROR: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CPF_BASE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CPF_CASTORE_ERROR: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CPF_COMPLETE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CPF_DELTA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CPF_FILE_ERROR: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CPF_FTP_ERROR: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CPF_HTTP_ERROR: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CPF_LDAP_ERROR: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CPF_MANUAL: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CPF_POSTPONED_BASE_FILE_ERROR: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CPF_POSTPONED_BASE_LDAP_ERROR: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CPF_SHADOW: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CPF_SIGNATURE_ERROR: u32 = 128u32;
pub const CPolicyQualifier: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821404, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CPolicyQualifiers: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821405, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CRLF_ALLOW_REQUEST_ATTRIBUTE_SUBJECT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CRLF_BUILD_ROOTCA_CRLENTRIES_BASEDONKEY: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CRLF_CRLNUMBER_CRITICAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CRLF_DELETE_EXPIRED_CRLS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CRLF_DELTA_USE_OLDEST_UNEXPIRED_BASE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CRLF_DISABLE_CHAIN_VERIFICATION: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CRLF_DISABLE_RDN_REORDER: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CRLF_DISABLE_ROOT_CROSS_CERTS: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CRLF_ENFORCE_ENROLLMENT_AGENT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CRLF_IGNORE_CROSS_CERT_TRUST_ERROR: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CRLF_IGNORE_INVALID_POLICIES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CRLF_IGNORE_UNKNOWN_CMC_ATTRIBUTES: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CRLF_LOG_FULL_RESPONSE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CRLF_PRESERVE_EXPIRED_CA_CERTS: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CRLF_PRESERVE_REVOKED_CA_CERTS: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CRLF_PUBLISH_EXPIRED_CERT_CRLS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CRLF_REBUILD_MODIFIED_SUBJECT_ONLY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CRLF_REVCHECK_IGNORE_NOREVCHECK: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CRLF_REVCHECK_IGNORE_OFFLINE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CRLF_SAVE_FAILED_CERTS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CRLF_USE_CROSS_CERT_TEMPLATE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CRLF_USE_XCHG_CERT_TEMPLATE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type CRLRevocationReason = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRL_REASON_UNSPECIFIED: CRLRevocationReason = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRL_REASON_KEY_COMPROMISE: CRLRevocationReason = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRL_REASON_CA_COMPROMISE: CRLRevocationReason = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRL_REASON_AFFILIATION_CHANGED: CRLRevocationReason = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRL_REASON_SUPERSEDED: CRLRevocationReason = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRL_REASON_CESSATION_OF_OPERATION: CRLRevocationReason = 5i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRL_REASON_CERTIFICATE_HOLD: CRLRevocationReason = 6i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRL_REASON_REMOVE_FROM_CRL: CRLRevocationReason = 8i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRL_REASON_PRIVILEGE_WITHDRAWN: CRLRevocationReason = 9i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRL_REASON_AA_COMPROMISE: CRLRevocationReason = 10i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CRYPT_ENUM_ALL_PROVIDERS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type CR_DISP = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_DISP_DENIED: CR_DISP = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_DISP_ERROR: CR_DISP = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_DISP_INCOMPLETE: CR_DISP = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_DISP_ISSUED: CR_DISP = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_DISP_ISSUED_OUT_OF_BAND: CR_DISP = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_DISP_UNDER_SUBMISSION: CR_DISP = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_DISP_REVOKED: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_FLG_CACROSSCERT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_FLG_CAXCHGCERT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_FLG_CHALLENGEPENDING: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_FLG_CHALLENGESATISFIED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_FLG_DEFINEDCACERT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_FLG_ENFORCEUTF8: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_FLG_ENROLLONBEHALFOF: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_FLG_FORCETELETEX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_FLG_FORCEUTF8: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_FLG_PUBLISHERROR: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_FLG_RENEWAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_FLG_SUBJECTUNMODIFIED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_FLG_TRUSTEKCERT: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_FLG_TRUSTEKKEY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_FLG_TRUSTONUSE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_FLG_VALIDENCRYPTEDKEYHASH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_GEMT_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_GEMT_HRESULT_STRING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_GEMT_HTTP_ERROR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_IN_CERTIFICATETRANSPARENCY: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_IN_CHALLENGERESPONSE: u32 = 1280u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_IN_CLIENTIDNONE: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_IN_CMC: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_IN_CONNECTONLY: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_IN_CRLS: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_IN_ENCODEANY: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_IN_ENCODEMASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_IN_FORMATANY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_IN_FORMATMASK: u32 = 65280u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_IN_FULLRESPONSE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_IN_HTTP: u32 = 196608u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_IN_KEYGEN: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_IN_MACHINE: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_IN_PKCS10: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_IN_PKCS7: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_IN_RETURNCHALLENGE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_IN_ROBO: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_IN_RPC: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_IN_SCEP: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_IN_SCEPPOST: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_IN_SIGNEDCERTIFICATETIMESTAMPLIST: u32 = 1536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_OUT_BASE64REQUESTHEADER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_OUT_BASE64X509CRLHEADER: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_OUT_CHAIN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_OUT_CRLS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_OUT_ENCODEMASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_OUT_HEX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_OUT_HEXADDR: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_OUT_HEXASCII: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_OUT_HEXASCIIADDR: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_OUT_HEXRAW: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_OUT_NOCR: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_OUT_NOCRLF: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_ADVANCEDSERVER: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_BASECRL: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_BASECRLPUBLISHSTATUS: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_CABACKWARDCROSSCERT: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_CABACKWARDCROSSCERTSTATE: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_CACERTSTATE: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_CACERTSTATUSCODE: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_CACERTVERSION: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_CAFORWARDCROSSCERT: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_CAFORWARDCROSSCERTSTATE: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_CANAME: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_CAPROPIDMAX: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_CASIGCERT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_CASIGCERTCHAIN: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_CASIGCERTCOUNT: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_CASIGCERTCRLCHAIN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_CATYPE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_CAXCHGCERT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_CAXCHGCERTCHAIN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_CAXCHGCERTCOUNT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_CAXCHGCERTCRLCHAIN: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_CERTAIAOCSPURLS: u32 = 43u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_CERTAIAURLS: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_CERTCDPURLS: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_CRLSTATE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_DELTACRL: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_DELTACRLPUBLISHSTATUS: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_DNSNAME: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_EXITCOUNT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_EXITDESCRIPTION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_FILEVERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_KRACERT: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_KRACERTCOUNT: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_KRACERTSTATE: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_KRACERTUSEDCOUNT: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_LOCALENAME: u32 = 44u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_PARENTCA: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_POLICYDESCRIPTION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_PRODUCTVERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_ROLESEPARATIONENABLED: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_SANITIZEDCANAME: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_SANITIZEDCASHORTNAME: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_SCEPMAX: u32 = 1002u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_SCEPMIN: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_SCEPSERVERCAPABILITIES: u32 = 1001u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_SCEPSERVERCERTS: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_SCEPSERVERCERTSCHAIN: u32 = 1002u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_SHAREDFOLDER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_SUBJECTTEMPLATE_OIDS: u32 = 45u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CR_PROP_TEMPLATES: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSBACKUP_DISABLE_INCREMENTAL: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type CSBACKUP_TYPE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSBACKUP_TYPE_FULL: CSBACKUP_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSBACKUP_TYPE_LOGS_ONLY: CSBACKUP_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSBACKUP_TYPE_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSBFT_DATABASE_DIRECTORY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSBFT_DIRECTORY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSBFT_LOG_DIRECTORY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSCONTROL_RESTART: u64 = 3u64;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSCONTROL_SHUTDOWN: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSCONTROL_SUSPEND: u64 = 2u64;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub struct CSEDB_RSTMAPW {
    pub pwszDatabaseName: ::windows_sys::core::PWSTR,
    pub pwszNewDatabaseName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for CSEDB_RSTMAPW {}
impl ::core::clone::Clone for CSEDB_RSTMAPW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSRESTORE_TYPE_CATCHUP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSRESTORE_TYPE_FULL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSRESTORE_TYPE_MASK: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSRESTORE_TYPE_ONLINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSURL_ADDTOCERTCDP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSURL_ADDTOCERTOCSP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSURL_ADDTOCRLCDP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSURL_ADDTOFRESHESTCRL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSURL_ADDTOIDP: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSURL_PUBLISHRETRY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSURL_SERVERPUBLISH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSURL_SERVERPUBLISHDELTA: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSVER_MAJOR: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSVER_MAJOR_LONGHORN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSVER_MAJOR_THRESHOLD: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSVER_MAJOR_WHISTLER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSVER_MAJOR_WIN2K: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSVER_MAJOR_WIN7: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSVER_MAJOR_WIN8: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSVER_MAJOR_WINBLUE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSVER_MINOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSVER_MINOR_LONGHORN_BETA1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSVER_MINOR_THRESHOLD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSVER_MINOR_WHISTLER_BETA2: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSVER_MINOR_WHISTLER_BETA3: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSVER_MINOR_WIN2K: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSVER_MINOR_WIN7: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSVER_MINOR_WIN8: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CSVER_MINOR_WINBLUE: u32 = 1u32;
pub const CSignerCertificate: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821437, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CSmimeCapabilities: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821402, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CSmimeCapability: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821401, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CVIEWAGEMINUTESDEFAULT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type CVRC_COLUMN = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CVRC_COLUMN_SCHEMA: CVRC_COLUMN = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CVRC_COLUMN_RESULT: CVRC_COLUMN = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CVRC_COLUMN_VALUE: CVRC_COLUMN = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CVRC_COLUMN_MASK: CVRC_COLUMN = 4095u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type CVRC_TABLE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CVRC_TABLE_ATTRIBUTES: CVRC_TABLE = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CVRC_TABLE_CRL: CVRC_TABLE = 20480u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CVRC_TABLE_EXTENSIONS: CVRC_TABLE = 12288u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CVRC_TABLE_REQCERT: CVRC_TABLE = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CVRC_TABLE_MASK: u32 = 61440u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CVRC_TABLE_SHIFT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CVR_SEEK_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CVR_SEEK_NODELTA: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CVR_SEEK_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CVR_SORT_ASCEND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CVR_SORT_DESCEND: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CVR_SORT_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CV_COLUMN_ATTRIBUTE_DEFAULT: i32 = -5i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CV_COLUMN_CRL_DEFAULT: i32 = -6i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CV_COLUMN_EXTENSION_DEFAULT: i32 = -4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CV_COLUMN_LOG_REVOKED_DEFAULT: i32 = -7i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CV_OUT_ENCODEMASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CV_OUT_HEXRAW: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CV_OUT_NOCR: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CV_OUT_NOCRLF: u32 = 1073741824u32;
pub const CX500DistinguishedName: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821379, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509Attribute: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821410, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509AttributeArchiveKey: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821415, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509AttributeArchiveKeyHash: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821416, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509AttributeClientId: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821413, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509AttributeCspProvider: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821419, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509AttributeExtensions: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821412, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509AttributeOSVersion: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821418, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509AttributeRenewalCertificate: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821414, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509Attributes: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821411, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509CertificateRequestCertificate: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821443, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509CertificateRequestCmc: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821445, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509CertificateRequestPkcs10: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821442, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509CertificateRequestPkcs7: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821444, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509CertificateRevocationList: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821472, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509CertificateRevocationListEntries: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821471, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509CertificateRevocationListEntry: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821470, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509CertificateTemplateADWritable: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2201412387, data2: 11882, data3: 18948, data4: [147, 124, 84, 143, 104, 24, 57, 179] };
pub const CX509EndorsementKey: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 295852573, data2: 47523, data3: 20189, data4: [175, 131, 59, 89, 173, 190, 211, 97] };
pub const CX509Enrollment: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821446, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509EnrollmentHelper: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821456, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509EnrollmentPolicyActiveDirectory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2448658471, data2: 8575, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509EnrollmentPolicyWebService: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2448658472, data2: 8575, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509EnrollmentWebClassFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821449, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509Extension: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821389, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionAlternativeNames: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821397, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionAuthorityKeyIdentifier: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821400, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionBasicConstraints: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821398, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionCertificatePolicies: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821408, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionEnhancedKeyUsage: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821392, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionKeyUsage: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821391, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionMSApplicationPolicies: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821409, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionSmimeCapabilities: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821403, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionSubjectKeyIdentifier: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821399, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionTemplate: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821394, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509ExtensionTemplateName: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821393, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509Extensions: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821390, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509MachineEnrollmentFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821457, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509NameValuePair: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821439, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509PolicyServerListManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2448658473, data2: 8575, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509PolicyServerUrl: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2448658474, data2: 8575, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509PrivateKey: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821388, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509PublicKey: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821387, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509SCEPEnrollment: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821473, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
pub const CX509SCEPEnrollmentHelper: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821474, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type CommitTemplateFlags = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CommitFlagSaveTemplateGenerateOID: CommitTemplateFlags = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CommitFlagSaveTemplateUseCurrentOID: CommitTemplateFlags = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CommitFlagSaveTemplateOverwrite: CommitTemplateFlags = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CommitFlagDeleteTemplate: CommitTemplateFlags = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DBFLAGS_CHECKPOINTDEPTH60MB: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DBFLAGS_CIRCULARLOGGING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DBFLAGS_CREATEIFNEEDED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DBFLAGS_DISABLESNAPSHOTBACKUP: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DBFLAGS_ENABLEVOLATILEREQUESTS: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DBFLAGS_LAZYFLUSH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DBFLAGS_LOGBUFFERSHUGE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DBFLAGS_LOGBUFFERSLARGE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DBFLAGS_LOGFILESIZE16MB: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DBFLAGS_MAXCACHESIZEX100: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DBFLAGS_MULTITHREADTRANSACTIONS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DBFLAGS_READONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DBG_CERTSRV: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DBSESSIONCOUNTDEFAULT: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DB_DISP_ACTIVE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DB_DISP_CA_CERT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DB_DISP_CA_CERT_CHAIN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DB_DISP_DENIED: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DB_DISP_ERROR: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DB_DISP_FOREIGN: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DB_DISP_ISSUED: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DB_DISP_KRA_CERT: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DB_DISP_LOG_FAILED_MIN: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DB_DISP_LOG_MIN: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DB_DISP_PENDING: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DB_DISP_QUEUE_MAX: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DB_DISP_REVOKED: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type DelayRetryAction = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DelayRetryUnknown: DelayRetryAction = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DelayRetryNone: DelayRetryAction = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DelayRetryShort: DelayRetryAction = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DelayRetryLong: DelayRetryAction = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DelayRetrySuccess: DelayRetryAction = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DelayRetryPastSuccess: DelayRetryAction = 5i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EANR_SUPPRESS_IA5CONVERSION: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EAN_NAMEOBJECTID: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_ADDOLDCERTTYPE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_ADDOLDKEYUSAGE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_ATTRIBUTECA: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_ATTRIBUTEEKU: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_ATTRIBUTEENDDATE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_ATTRIBUTESUBJECTALTNAME2: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_AUDITCERTTEMPLATELOAD: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_BASICCONSTRAINTSCA: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_BASICCONSTRAINTSCRITICAL: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_DISABLEEXTENSIONLIST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_DISABLELDAPPACKAGELIST: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_DISABLEOLDOSCNUPN: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_EMAILOPTIONAL: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_ENABLEAKICRITICAL: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_ENABLEAKIISSUERNAME: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_ENABLEAKIISSUERSERIAL: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_ENABLEAKIKEYID: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_ENABLECHASECLIENTDC: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_ENABLEDEFAULTSMIME: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_ENABLEKEYENCIPHERMENTCACERT: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_ENABLELDAPREFERRALS: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_ENABLEOCSPREVNOCHECK: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_ENABLERENEWONBEHALFOF: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_ENABLEREQUESTEXTENSIONS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_ENABLEUPNMAP: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_IGNOREREQUESTERGROUP: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_REQUESTEXTENSIONLIST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EDITF_SERVERUPGRADED: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ENUMEXT_OBJECTID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type ENUM_CATYPES = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ENUM_ENTERPRISE_ROOTCA: ENUM_CATYPES = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ENUM_ENTERPRISE_SUBCA: ENUM_CATYPES = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ENUM_STANDALONE_ROOTCA: ENUM_CATYPES = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ENUM_STANDALONE_SUBCA: ENUM_CATYPES = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ENUM_UNKNOWN_CA: ENUM_CATYPES = 5i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type ENUM_CERT_COLUMN_VALUE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CV_OUT_BASE64: ENUM_CERT_COLUMN_VALUE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CV_OUT_BASE64HEADER: ENUM_CERT_COLUMN_VALUE_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CV_OUT_BASE64REQUESTHEADER: ENUM_CERT_COLUMN_VALUE_FLAGS = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CV_OUT_BASE64X509CRLHEADER: ENUM_CERT_COLUMN_VALUE_FLAGS = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CV_OUT_BINARY: ENUM_CERT_COLUMN_VALUE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CV_OUT_HEX: ENUM_CERT_COLUMN_VALUE_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CV_OUT_HEXADDR: ENUM_CERT_COLUMN_VALUE_FLAGS = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CV_OUT_HEXASCII: ENUM_CERT_COLUMN_VALUE_FLAGS = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CV_OUT_HEXASCIIADDR: ENUM_CERT_COLUMN_VALUE_FLAGS = 11u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXITEVENT_CERTIMPORTED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXITEVENT_INVALID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXITEVENT_STARTUP: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXITPUB_ACTIVEDIRECTORY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXITPUB_DEFAULT_ENTERPRISE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXITPUB_DEFAULT_STANDALONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXITPUB_FILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXITPUB_REMOVEOLDCERTS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXTENSION_CRITICAL_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXTENSION_DELETE_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXTENSION_DISABLE_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXTENSION_ORIGIN_ADMIN: u32 = 196608u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXTENSION_ORIGIN_CACERT: u32 = 589824u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXTENSION_ORIGIN_CMC: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXTENSION_ORIGIN_IMPORTEDCERT: u32 = 393216u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXTENSION_ORIGIN_MASK: u32 = 983040u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXTENSION_ORIGIN_PKCS7: u32 = 458752u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXTENSION_ORIGIN_POLICY: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXTENSION_ORIGIN_RENEWALCERT: u32 = 327680u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXTENSION_ORIGIN_REQUEST: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXTENSION_ORIGIN_SERVER: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EXTENSION_POLICY_MASK: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type EncodingType = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_STRING_BASE64HEADER: EncodingType = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_STRING_BASE64: EncodingType = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_STRING_BINARY: EncodingType = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_STRING_BASE64REQUESTHEADER: EncodingType = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_STRING_HEX: EncodingType = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_STRING_HEXASCII: EncodingType = 5i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_STRING_BASE64_ANY: EncodingType = 6i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_STRING_ANY: EncodingType = 7i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_STRING_HEX_ANY: EncodingType = 8i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_STRING_BASE64X509CRLHEADER: EncodingType = 9i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_STRING_HEXADDR: EncodingType = 10i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_STRING_HEXASCIIADDR: EncodingType = 11i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_STRING_HEXRAW: EncodingType = 12i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_STRING_BASE64URI: EncodingType = 13i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_STRING_ENCODEMASK: EncodingType = 255i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_STRING_CHAIN: EncodingType = 256i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_STRING_TEXT: EncodingType = 512i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_STRING_PERCENTESCAPE: EncodingType = 134217728i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_STRING_HASHDATA: EncodingType = 268435456i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_STRING_STRICT: EncodingType = 536870912i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_STRING_NOCRLF: EncodingType = 1073741824i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_STRING_NOCR: EncodingType = -2147483648i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type EnrollmentCAProperty = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CAPropCommonName: EnrollmentCAProperty = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CAPropDistinguishedName: EnrollmentCAProperty = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CAPropSanitizedName: EnrollmentCAProperty = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CAPropSanitizedShortName: EnrollmentCAProperty = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CAPropDNSName: EnrollmentCAProperty = 5i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CAPropCertificateTypes: EnrollmentCAProperty = 6i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CAPropCertificate: EnrollmentCAProperty = 7i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CAPropDescription: EnrollmentCAProperty = 8i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CAPropWebServers: EnrollmentCAProperty = 9i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CAPropSiteName: EnrollmentCAProperty = 10i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CAPropSecurity: EnrollmentCAProperty = 11i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const CAPropRenewalOnly: EnrollmentCAProperty = 12i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type EnrollmentDisplayStatus = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DisplayNo: EnrollmentDisplayStatus = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DisplayYes: EnrollmentDisplayStatus = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type EnrollmentEnrollStatus = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const Enrolled: EnrollmentEnrollStatus = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollPended: EnrollmentEnrollStatus = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollUIDeferredEnrollmentRequired: EnrollmentEnrollStatus = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollError: EnrollmentEnrollStatus = 16i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollUnknown: EnrollmentEnrollStatus = 32i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollSkipped: EnrollmentEnrollStatus = 64i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollDenied: EnrollmentEnrollStatus = 256i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type EnrollmentPolicyFlags = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DisableGroupPolicyList: EnrollmentPolicyFlags = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DisableUserServerList: EnrollmentPolicyFlags = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type EnrollmentPolicyServerPropertyFlags = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DefaultNone: EnrollmentPolicyServerPropertyFlags = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const DefaultPolicyServer: EnrollmentPolicyServerPropertyFlags = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type EnrollmentSelectionStatus = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SelectedNo: EnrollmentSelectionStatus = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SelectedYes: EnrollmentSelectionStatus = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type EnrollmentTemplateProperty = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropCommonName: EnrollmentTemplateProperty = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropFriendlyName: EnrollmentTemplateProperty = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropEKUs: EnrollmentTemplateProperty = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropCryptoProviders: EnrollmentTemplateProperty = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropMajorRevision: EnrollmentTemplateProperty = 5i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropDescription: EnrollmentTemplateProperty = 6i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropKeySpec: EnrollmentTemplateProperty = 7i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropSchemaVersion: EnrollmentTemplateProperty = 8i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropMinorRevision: EnrollmentTemplateProperty = 9i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropRASignatureCount: EnrollmentTemplateProperty = 10i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropMinimumKeySize: EnrollmentTemplateProperty = 11i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropOID: EnrollmentTemplateProperty = 12i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropSupersede: EnrollmentTemplateProperty = 13i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropRACertificatePolicies: EnrollmentTemplateProperty = 14i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropRAEKUs: EnrollmentTemplateProperty = 15i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropCertificatePolicies: EnrollmentTemplateProperty = 16i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropV1ApplicationPolicy: EnrollmentTemplateProperty = 17i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropAsymmetricAlgorithm: EnrollmentTemplateProperty = 18i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropKeySecurityDescriptor: EnrollmentTemplateProperty = 19i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropSymmetricAlgorithm: EnrollmentTemplateProperty = 20i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropSymmetricKeyLength: EnrollmentTemplateProperty = 21i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropHashAlgorithm: EnrollmentTemplateProperty = 22i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropKeyUsage: EnrollmentTemplateProperty = 23i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropEnrollmentFlags: EnrollmentTemplateProperty = 24i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropSubjectNameFlags: EnrollmentTemplateProperty = 25i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropPrivateKeyFlags: EnrollmentTemplateProperty = 26i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropGeneralFlags: EnrollmentTemplateProperty = 27i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropSecurityDescriptor: EnrollmentTemplateProperty = 28i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropExtensions: EnrollmentTemplateProperty = 29i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropValidityPeriod: EnrollmentTemplateProperty = 30i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TemplatePropRenewalPeriod: EnrollmentTemplateProperty = 31i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type FNCERTSRVBACKUPCLOSE = ::core::option::Option<unsafe extern "system" fn(hbc: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type FNCERTSRVBACKUPEND = ::core::option::Option<unsafe extern "system" fn(hbc: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type FNCERTSRVBACKUPFREE = ::core::option::Option<unsafe extern "system" fn(pv: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type FNCERTSRVBACKUPGETBACKUPLOGSW = ::core::option::Option<unsafe extern "system" fn(hbc: *mut ::core::ffi::c_void, ppwszzbackuplogfiles: *mut *mut u16, pcbsize: *mut u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type FNCERTSRVBACKUPGETDATABASENAMESW = ::core::option::Option<unsafe extern "system" fn(hbc: *mut ::core::ffi::c_void, ppwszzattachmentinformation: *mut *mut u16, pcbsize: *mut u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type FNCERTSRVBACKUPGETDYNAMICFILELISTW = ::core::option::Option<unsafe extern "system" fn(hbc: *mut ::core::ffi::c_void, ppwszzfilelist: *mut *mut u16, pcbsize: *mut u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type FNCERTSRVBACKUPOPENFILEW = ::core::option::Option<unsafe extern "system" fn(hbc: *mut ::core::ffi::c_void, pwszattachmentname: ::windows_sys::core::PCWSTR, cbreadhintsize: u32, plifilesize: *mut i64) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type FNCERTSRVBACKUPPREPAREW = ::core::option::Option<unsafe extern "system" fn(pwszservername: ::windows_sys::core::PCWSTR, grbitjet: u32, dwbackupflags: u32, phbc: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type FNCERTSRVBACKUPREAD = ::core::option::Option<unsafe extern "system" fn(hbc: *mut ::core::ffi::c_void, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, pcbread: *mut u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type FNCERTSRVBACKUPTRUNCATELOGS = ::core::option::Option<unsafe extern "system" fn(hbc: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type FNCERTSRVISSERVERONLINEW = ::core::option::Option<unsafe extern "system" fn(pwszservername: ::windows_sys::core::PCWSTR, pfserveronline: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type FNCERTSRVRESTOREEND = ::core::option::Option<unsafe extern "system" fn(hbc: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type FNCERTSRVRESTOREGETDATABASELOCATIONSW = ::core::option::Option<unsafe extern "system" fn(hbc: *mut ::core::ffi::c_void, ppwszzdatabaselocationlist: *mut *mut u16, pcbsize: *mut u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type FNCERTSRVRESTOREPREPAREW = ::core::option::Option<unsafe extern "system" fn(pwszservername: ::windows_sys::core::PCWSTR, dwrestoreflags: u32, phbc: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type FNCERTSRVRESTOREREGISTERCOMPLETE = ::core::option::Option<unsafe extern "system" fn(hbc: *mut ::core::ffi::c_void, hrrestorestate: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type FNCERTSRVRESTOREREGISTERW = ::core::option::Option<unsafe extern "system" fn(hbc: *mut ::core::ffi::c_void, pwszcheckpointfilepath: ::windows_sys::core::PCWSTR, pwszlogpath: ::windows_sys::core::PCWSTR, rgrstmap: *mut CSEDB_RSTMAPW, crstmap: i32, pwszbackuplogpath: ::windows_sys::core::PCWSTR, genlow: u32, genhigh: u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type FNCERTSRVSERVERCONTROLW = ::core::option::Option<unsafe extern "system" fn(pwszservername: ::windows_sys::core::PCWSTR, dwcontrolflags: u32, pcbout: *mut u32, ppbout: *mut *mut u8) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type FNIMPORTPFXTOPROVIDER = ::core::option::Option<unsafe extern "system" fn(hwndparent: super::super::super::Foundation::HWND, pbpfx: *const u8, cbpfx: u32, importflags: ImportPFXFlags, pwszpassword: ::windows_sys::core::PCWSTR, pwszprovidername: ::windows_sys::core::PCWSTR, pwszreadername: ::windows_sys::core::PCWSTR, pwszcontainernameprefix: ::windows_sys::core::PCWSTR, pwszpin: ::windows_sys::core::PCWSTR, pwszfriendlyname: ::windows_sys::core::PCWSTR, pccertout: *mut u32, prgpcertout: *mut *mut *mut super::CERT_CONTEXT) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type FNIMPORTPFXTOPROVIDERFREEDATA = ::core::option::Option<unsafe extern "system" fn(ccert: u32, rgpcert: *const *const super::CERT_CONTEXT)>;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const FR_PROP_CLAIMCHALLENGE: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type FULL_RESPONSE_PROPERTY_ID = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const FR_PROP_NONE: FULL_RESPONSE_PROPERTY_ID = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const FR_PROP_FULLRESPONSE: FULL_RESPONSE_PROPERTY_ID = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const FR_PROP_STATUSINFOCOUNT: FULL_RESPONSE_PROPERTY_ID = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const FR_PROP_BODYPARTSTRING: FULL_RESPONSE_PROPERTY_ID = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const FR_PROP_STATUS: FULL_RESPONSE_PROPERTY_ID = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const FR_PROP_STATUSSTRING: FULL_RESPONSE_PROPERTY_ID = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const FR_PROP_OTHERINFOCHOICE: FULL_RESPONSE_PROPERTY_ID = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const FR_PROP_FAILINFO: FULL_RESPONSE_PROPERTY_ID = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const FR_PROP_PENDINFOTOKEN: FULL_RESPONSE_PROPERTY_ID = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const FR_PROP_PENDINFOTIME: FULL_RESPONSE_PROPERTY_ID = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const FR_PROP_ISSUEDCERTIFICATEHASH: FULL_RESPONSE_PROPERTY_ID = 10u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const FR_PROP_ISSUEDCERTIFICATE: FULL_RESPONSE_PROPERTY_ID = 11u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const FR_PROP_ISSUEDCERTIFICATECHAIN: FULL_RESPONSE_PROPERTY_ID = 12u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const FR_PROP_ISSUEDCERTIFICATECRLCHAIN: FULL_RESPONSE_PROPERTY_ID = 13u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const FR_PROP_ENCRYPTEDKEYHASH: FULL_RESPONSE_PROPERTY_ID = 14u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const FR_PROP_FULLRESPONSENOPKCS7: FULL_RESPONSE_PROPERTY_ID = 15u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const FR_PROP_CAEXCHANGECERTIFICATEHASH: FULL_RESPONSE_PROPERTY_ID = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const FR_PROP_CAEXCHANGECERTIFICATE: FULL_RESPONSE_PROPERTY_ID = 17u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const FR_PROP_CAEXCHANGECERTIFICATECHAIN: FULL_RESPONSE_PROPERTY_ID = 18u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const FR_PROP_CAEXCHANGECERTIFICATECRLCHAIN: FULL_RESPONSE_PROPERTY_ID = 19u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const FR_PROP_ATTESTATIONCHALLENGE: FULL_RESPONSE_PROPERTY_ID = 20u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const FR_PROP_ATTESTATIONPROVIDERNAME: FULL_RESPONSE_PROPERTY_ID = 21u32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAlternativeName {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeFromString: unsafe extern "system" fn(this: *mut *mut Self, r#type: AlternativeNameType, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeFromString: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeFromRawData: unsafe extern "system" fn(this: *mut *mut Self, r#type: AlternativeNameType, encoding: EncodingType, strrawdata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeFromRawData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub InitializeFromOtherName: unsafe extern "system" fn(this: *mut *mut Self, pobjectid: *mut ::core::ffi::c_void, encoding: EncodingType, strrawdata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, tobewrapped: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    InitializeFromOtherName: usize,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut AlternativeNameType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub StrValue: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StrValue: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ObjectId: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ObjectId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_RawData: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_RawData: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAlternativeName {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692435, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAlternativeNames {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IAlternativeNames {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692436, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IBinaryConverter {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub StringToString: unsafe extern "system" fn(this: *mut *mut Self, strencodedin: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encodingin: EncodingType, encoding: EncodingType, pstrencoded: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StringToString: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub VariantByteArrayToString: unsafe extern "system" fn(this: *mut *mut Self, pvarbytearray: *const super::super::super::System::Com::VARIANT, encoding: EncodingType, pstrencoded: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    VariantByteArrayToString: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub StringToVariantByteArray: unsafe extern "system" fn(this: *mut *mut Self, strencoded: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, pvarbytearray: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    StringToVariantByteArray: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IBinaryConverter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692418, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IBinaryConverter2 {
    pub base__: IBinaryConverter,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub StringArrayToVariantArray: unsafe extern "system" fn(this: *mut *mut Self, pvarstringarray: *const super::super::super::System::Com::VARIANT, pvarvariantarray: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    StringArrayToVariantArray: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub VariantArrayToStringArray: unsafe extern "system" fn(this: *mut *mut Self, pvarvariantarray: *const super::super::super::System::Com::VARIANT, pvarstringarray: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    VariantArrayToStringArray: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IBinaryConverter2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2373527732, data2: 19991, data3: 17037, data4: [154, 23, 114, 141, 240, 13, 27, 43] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICEnroll {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub createFilePKCS10: unsafe extern "system" fn(this: *mut *mut Self, dnname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, usage: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, wszpkcs10filename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    createFilePKCS10: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub acceptFilePKCS7: unsafe extern "system" fn(this: *mut *mut Self, wszpkcs7filename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    acceptFilePKCS7: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub createPKCS10: unsafe extern "system" fn(this: *mut *mut Self, dnname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, usage: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppkcs10: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    createPKCS10: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub acceptPKCS7: unsafe extern "system" fn(this: *mut *mut Self, pkcs7: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    acceptPKCS7: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub getCertFromPKCS7: unsafe extern "system" fn(this: *mut *mut Self, wszpkcs7: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrcert: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    getCertFromPKCS7: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub enumProviders: unsafe extern "system" fn(this: *mut *mut Self, dwindex: i32, dwflags: i32, pbstrprovname: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    enumProviders: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub enumContainers: unsafe extern "system" fn(this: *mut *mut Self, dwindex: i32, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    enumContainers: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub freeRequestInfo: unsafe extern "system" fn(this: *mut *mut Self, pkcs7orpkcs10: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    freeRequestInfo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MyStoreName: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MyStoreName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMyStoreName: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMyStoreName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MyStoreType: unsafe extern "system" fn(this: *mut *mut Self, pbstrtype: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MyStoreType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMyStoreType: unsafe extern "system" fn(this: *mut *mut Self, bstrtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMyStoreType: usize,
    pub MyStoreFlags: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMyStoreFlags: unsafe extern "system" fn(this: *mut *mut Self, dwflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CAStoreName: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CAStoreName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCAStoreName: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCAStoreName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CAStoreType: unsafe extern "system" fn(this: *mut *mut Self, pbstrtype: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CAStoreType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCAStoreType: unsafe extern "system" fn(this: *mut *mut Self, bstrtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCAStoreType: usize,
    pub CAStoreFlags: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetCAStoreFlags: unsafe extern "system" fn(this: *mut *mut Self, dwflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RootStoreName: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RootStoreName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRootStoreName: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRootStoreName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RootStoreType: unsafe extern "system" fn(this: *mut *mut Self, pbstrtype: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RootStoreType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRootStoreType: unsafe extern "system" fn(this: *mut *mut Self, bstrtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRootStoreType: usize,
    pub RootStoreFlags: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetRootStoreFlags: unsafe extern "system" fn(this: *mut *mut Self, dwflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestStoreName: unsafe extern "system" fn(this: *mut *mut Self, pbstrname: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestStoreName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRequestStoreName: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRequestStoreName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestStoreType: unsafe extern "system" fn(this: *mut *mut Self, pbstrtype: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestStoreType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRequestStoreType: unsafe extern "system" fn(this: *mut *mut Self, bstrtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRequestStoreType: usize,
    pub RequestStoreFlags: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetRequestStoreFlags: unsafe extern "system" fn(this: *mut *mut Self, dwflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ContainerName: unsafe extern "system" fn(this: *mut *mut Self, pbstrcontainer: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ContainerName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetContainerName: unsafe extern "system" fn(this: *mut *mut Self, bstrcontainer: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetContainerName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ProviderName: unsafe extern "system" fn(this: *mut *mut Self, pbstrprovider: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProviderName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetProviderName: unsafe extern "system" fn(this: *mut *mut Self, bstrprovider: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetProviderName: usize,
    pub ProviderType: unsafe extern "system" fn(this: *mut *mut Self, pdwtype: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetProviderType: unsafe extern "system" fn(this: *mut *mut Self, dwtype: i32) -> ::windows_sys::core::HRESULT,
    pub KeySpec: unsafe extern "system" fn(this: *mut *mut Self, pdw: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetKeySpec: unsafe extern "system" fn(this: *mut *mut Self, dw: i32) -> ::windows_sys::core::HRESULT,
    pub ProviderFlags: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetProviderFlags: unsafe extern "system" fn(this: *mut *mut Self, dwflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UseExistingKeySet: unsafe extern "system" fn(this: *mut *mut Self, fuseexistingkeys: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UseExistingKeySet: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUseExistingKeySet: unsafe extern "system" fn(this: *mut *mut Self, fuseexistingkeys: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUseExistingKeySet: usize,
    pub GenKeyFlags: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetGenKeyFlags: unsafe extern "system" fn(this: *mut *mut Self, dwflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteRequestCert: unsafe extern "system" fn(this: *mut *mut Self, fdelete: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteRequestCert: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDeleteRequestCert: unsafe extern "system" fn(this: *mut *mut Self, fdelete: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDeleteRequestCert: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteCertToCSP: unsafe extern "system" fn(this: *mut *mut Self, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteCertToCSP: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWriteCertToCSP: unsafe extern "system" fn(this: *mut *mut Self, fbool: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWriteCertToCSP: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SPCFileName: unsafe extern "system" fn(this: *mut *mut Self, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SPCFileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSPCFileName: unsafe extern "system" fn(this: *mut *mut Self, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSPCFileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PVKFileName: unsafe extern "system" fn(this: *mut *mut Self, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PVKFileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPVKFileName: unsafe extern "system" fn(this: *mut *mut Self, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPVKFileName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub HashAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HashAlgorithm: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetHashAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetHashAlgorithm: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICEnroll {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1140388488, data2: 31264, data3: 4560, data4: [143, 6, 0, 192, 79, 194, 149, 225] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICEnroll2 {
    pub base__: ICEnroll,
    #[cfg(feature = "Win32_Foundation")]
    pub addCertTypeToRequest: unsafe extern "system" fn(this: *mut *mut Self, certtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    addCertTypeToRequest: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub addNameValuePairToSignature: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    addNameValuePairToSignature: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteCertToUserDS: unsafe extern "system" fn(this: *mut *mut Self, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteCertToUserDS: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWriteCertToUserDS: unsafe extern "system" fn(this: *mut *mut Self, fbool: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWriteCertToUserDS: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableT61DNEncoding: unsafe extern "system" fn(this: *mut *mut Self, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableT61DNEncoding: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableT61DNEncoding: unsafe extern "system" fn(this: *mut *mut Self, fbool: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableT61DNEncoding: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICEnroll2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1884071728, data2: 51467, data3: 4561, data4: [155, 236, 0, 192, 79, 194, 149, 225] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICEnroll3 {
    pub base__: ICEnroll2,
    #[cfg(feature = "Win32_Foundation")]
    pub InstallPKCS7: unsafe extern "system" fn(this: *mut *mut Self, pkcs7: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstallPKCS7: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetSupportedKeySpec: unsafe extern "system" fn(this: *mut *mut Self, pdwkeyspec: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetKeyLen: unsafe extern "system" fn(this: *mut *mut Self, fmin: super::super::super::Foundation::BOOL, fexchange: super::super::super::Foundation::BOOL, pdwkeysize: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetKeyLen: usize,
    pub EnumAlgs: unsafe extern "system" fn(this: *mut *mut Self, dwindex: i32, algclass: i32, pdwalgid: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAlgName: unsafe extern "system" fn(this: *mut *mut Self, algid: i32, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAlgName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReuseHardwareKeyIfUnableToGenNew: unsafe extern "system" fn(this: *mut *mut Self, freusehardwarekeyifunabletogennew: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReuseHardwareKeyIfUnableToGenNew: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReuseHardwareKeyIfUnableToGenNew: unsafe extern "system" fn(this: *mut *mut Self, freusehardwarekeyifunabletogennew: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReuseHardwareKeyIfUnableToGenNew: usize,
    pub SetHashAlgID: unsafe extern "system" fn(this: *mut *mut Self, hashalgid: i32) -> ::windows_sys::core::HRESULT,
    pub HashAlgID: unsafe extern "system" fn(this: *mut *mut Self, hashalgid: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLimitExchangeKeyToEncipherment: unsafe extern "system" fn(this: *mut *mut Self, flimitexchangekeytoencipherment: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLimitExchangeKeyToEncipherment: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LimitExchangeKeyToEncipherment: unsafe extern "system" fn(this: *mut *mut Self, flimitexchangekeytoencipherment: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LimitExchangeKeyToEncipherment: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableSMIMECapabilities: unsafe extern "system" fn(this: *mut *mut Self, fenablesmimecapabilities: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableSMIMECapabilities: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableSMIMECapabilities: unsafe extern "system" fn(this: *mut *mut Self, fenablesmimecapabilities: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableSMIMECapabilities: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICEnroll3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3263966613, data2: 47070, data3: 4562, data4: [164, 33, 0, 192, 79, 121, 254, 142] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICEnroll4 {
    pub base__: ICEnroll3,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPrivateKeyArchiveCertificate: unsafe extern "system" fn(this: *mut *mut Self, bstrcert: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPrivateKeyArchiveCertificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PrivateKeyArchiveCertificate: unsafe extern "system" fn(this: *mut *mut Self, pbstrcert: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PrivateKeyArchiveCertificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetThumbPrint: unsafe extern "system" fn(this: *mut *mut Self, bstrthumbprint: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetThumbPrint: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ThumbPrint: unsafe extern "system" fn(this: *mut *mut Self, pbstrthumbprint: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ThumbPrint: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub binaryToString: unsafe extern "system" fn(this: *mut *mut Self, flags: i32, strbinary: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrencoded: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    binaryToString: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub stringToBinary: unsafe extern "system" fn(this: *mut *mut Self, flags: i32, strencoded: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrbinary: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    stringToBinary: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub addExtensionToRequest: unsafe extern "system" fn(this: *mut *mut Self, flags: i32, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    addExtensionToRequest: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub addAttributeToRequest: unsafe extern "system" fn(this: *mut *mut Self, flags: i32, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    addAttributeToRequest: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub addNameValuePairToRequest: unsafe extern "system" fn(this: *mut *mut Self, flags: i32, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    addNameValuePairToRequest: usize,
    pub resetExtensions: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub resetAttributes: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub createRequest: unsafe extern "system" fn(this: *mut *mut Self, flags: CERT_CREATE_REQUEST_FLAGS, strdnname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, usage: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrrequest: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    createRequest: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub createFileRequest: unsafe extern "system" fn(this: *mut *mut Self, flags: CERT_CREATE_REQUEST_FLAGS, strdnname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strusage: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strrequestfilename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    createFileRequest: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub acceptResponse: unsafe extern "system" fn(this: *mut *mut Self, strresponse: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    acceptResponse: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub acceptFileResponse: unsafe extern "system" fn(this: *mut *mut Self, strresponsefilename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    acceptFileResponse: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub getCertFromResponse: unsafe extern "system" fn(this: *mut *mut Self, strresponse: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrcert: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    getCertFromResponse: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub getCertFromFileResponse: unsafe extern "system" fn(this: *mut *mut Self, strresponsefilename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrcert: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    getCertFromFileResponse: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub createPFX: unsafe extern "system" fn(this: *mut *mut Self, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrpfx: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    createPFX: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub createFilePFX: unsafe extern "system" fn(this: *mut *mut Self, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strpfxfilename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    createFilePFX: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub setPendingRequestInfo: unsafe extern "system" fn(this: *mut *mut Self, lrequestid: i32, strcadns: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strcaname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strfriendlyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    setPendingRequestInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub enumPendingRequest: unsafe extern "system" fn(this: *mut *mut Self, lindex: i32, ldesiredproperty: PENDING_REQUEST_DESIRED_PROPERTY, pvarproperty: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    enumPendingRequest: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub removePendingRequest: unsafe extern "system" fn(this: *mut *mut Self, strthumbprint: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    removePendingRequest: usize,
    pub GetKeyLenEx: unsafe extern "system" fn(this: *mut *mut Self, lsizespec: XEKL_KEYSIZE, lkeyspec: XEKL_KEYSPEC, pdwkeysize: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InstallPKCS7Ex: unsafe extern "system" fn(this: *mut *mut Self, pkcs7: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, plcertinstalled: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstallPKCS7Ex: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub addCertTypeToRequestEx: unsafe extern "system" fn(this: *mut *mut Self, ltype: ADDED_CERT_TYPE, bstroidorname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, lmajorversion: i32, fminorversion: super::super::super::Foundation::BOOL, lminorversion: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    addCertTypeToRequestEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub getProviderType: unsafe extern "system" fn(this: *mut *mut Self, strprovname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, plprovtype: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    getProviderType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSignerCertificate: unsafe extern "system" fn(this: *mut *mut Self, bstrcert: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSignerCertificate: usize,
    pub SetClientId: unsafe extern "system" fn(this: *mut *mut Self, lclientid: i32) -> ::windows_sys::core::HRESULT,
    pub ClientId: unsafe extern "system" fn(this: *mut *mut Self, plclientid: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub addBlobPropertyToCertificate: unsafe extern "system" fn(this: *mut *mut Self, lpropertyid: i32, lreserved: i32, bstrproperty: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    addBlobPropertyToCertificate: usize,
    pub resetBlobProperties: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIncludeSubjectKeyID: unsafe extern "system" fn(this: *mut *mut Self, finclude: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIncludeSubjectKeyID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IncludeSubjectKeyID: unsafe extern "system" fn(this: *mut *mut Self, pfinclude: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IncludeSubjectKeyID: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICEnroll4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3253803146, data2: 11957, data3: 19072, data4: [132, 27, 126, 114, 154, 53, 109, 144] };
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ICF_ALLOWFOREIGN: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ICF_EXISTINGROW: u32 = 131072u32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertAdmin {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub IsValidCertificate: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strserialnumber: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdisposition: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsValidCertificate: usize,
    pub GetRevocationReason: unsafe extern "system" fn(this: *mut *mut Self, preason: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RevokeCertificate: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strserialnumber: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, reason: i32, date: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RevokeCertificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRequestAttributes: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, requestid: i32, strattributes: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRequestAttributes: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetCertificateExtension: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, requestid: i32, strextensionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, r#type: CERT_PROPERTY_TYPE, flags: i32, pvarvalue: *const super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetCertificateExtension: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DenyRequest: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, requestid: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DenyRequest: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ResubmitRequest: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, requestid: i32, pdisposition: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ResubmitRequest: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PublishCRL: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, date: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PublishCRL: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCRL: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, flags: i32, pstrcrl: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCRL: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ImportCertificate: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, flags: CERT_IMPORT_FLAGS, prequestid: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ImportCertificate: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertAdmin {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 887056720, data2: 32694, data3: 4560, data4: [136, 23, 0, 160, 201, 3, 184, 60] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertAdmin2 {
    pub base__: ICertAdmin,
    #[cfg(feature = "Win32_Foundation")]
    pub PublishCRLs: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, date: f64, crlflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PublishCRLs: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetCAProperty: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propid: i32, propindex: i32, proptype: i32, flags: i32, pvarpropertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetCAProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetCAProperty: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propid: i32, propindex: i32, proptype: CERT_PROPERTY_TYPE, pvarpropertyvalue: *const super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetCAProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCAPropertyFlags: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propid: i32, ppropflags: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCAPropertyFlags: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCAPropertyDisplayName: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propid: i32, pstrdisplayname: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCAPropertyDisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetArchivedKey: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, requestid: i32, flags: i32, pstrarchivedkey: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetArchivedKey: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetConfigEntry: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strnodepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strentryname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvarentry: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetConfigEntry: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetConfigEntry: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strnodepath: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strentryname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvarentry: *const super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetConfigEntry: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ImportKey: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, requestid: i32, strcerthash: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, flags: CERT_IMPORT_FLAGS, strkey: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ImportKey: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMyRoles: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, proles: *mut CERTADMIN_GET_ROLES_FLAGS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMyRoles: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteRow: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, flags: CERT_DELETE_ROW_FLAGS, date: f64, table: CVRC_TABLE, rowid: i32, pcdeleted: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteRow: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertAdmin2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4156795969, data2: 47310, data3: 20404, data4: [170, 88, 61, 29, 192, 227, 107, 57] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertConfig {
    pub base__: super::super::super::System::Com::IDispatch,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, pindex: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetField: unsafe extern "system" fn(this: *mut *mut Self, strfieldname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetField: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetConfig: unsafe extern "system" fn(this: *mut *mut Self, flags: i32, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetConfig: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertConfig {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 925879860, data2: 17188, data3: 4560, data4: [136, 16, 0, 160, 201, 3, 184, 60] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertConfig2 {
    pub base__: ICertConfig,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSharedFolder: unsafe extern "system" fn(this: *mut *mut Self, strsharedfolder: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSharedFolder: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertConfig2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2048454110, data2: 32376, data3: 16739, data4: [141, 237, 120, 226, 201, 206, 233, 36] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertEncodeAltName {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Decode: unsafe extern "system" fn(this: *mut *mut Self, strbinary: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Decode: usize,
    pub GetNameCount: unsafe extern "system" fn(this: *mut *mut Self, pnamecount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetNameChoice: unsafe extern "system" fn(this: *mut *mut Self, nameindex: i32, pnamechoice: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, nameindex: i32, pstrname: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetName: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self, namecount: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNameEntry: unsafe extern "system" fn(this: *mut *mut Self, nameindex: i32, namechoice: CERT_ALT_NAME, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNameEntry: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Encode: unsafe extern "system" fn(this: *mut *mut Self, pstrbinary: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Encode: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertEncodeAltName {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 479890544, data2: 4721, data3: 4561, data4: [155, 212, 0, 192, 79, 182, 131, 250] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertEncodeAltName2 {
    pub base__: ICertEncodeAltName,
    #[cfg(feature = "Win32_Foundation")]
    pub DecodeBlob: unsafe extern "system" fn(this: *mut *mut Self, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DecodeBlob: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EncodeBlob: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pstrencodeddata: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EncodeBlob: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNameBlob: unsafe extern "system" fn(this: *mut *mut Self, nameindex: i32, encoding: EncodingType, pstrname: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNameBlob: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNameEntryBlob: unsafe extern "system" fn(this: *mut *mut Self, nameindex: i32, namechoice: i32, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNameEntryBlob: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertEncodeAltName2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4135575927, data2: 24305, data3: 17717, data4: [180, 206, 41, 223, 21, 226, 224, 195] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertEncodeBitString {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Decode: unsafe extern "system" fn(this: *mut *mut Self, strbinary: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Decode: usize,
    pub GetBitCount: unsafe extern "system" fn(this: *mut *mut Self, pbitcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBitString: unsafe extern "system" fn(this: *mut *mut Self, pstrbitstring: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBitString: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Encode: unsafe extern "system" fn(this: *mut *mut Self, bitcount: i32, strbitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrbinary: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Encode: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertEncodeBitString {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1840588222, data2: 4728, data3: 4561, data4: [155, 212, 0, 192, 79, 182, 131, 250] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertEncodeBitString2 {
    pub base__: ICertEncodeBitString,
    #[cfg(feature = "Win32_Foundation")]
    pub DecodeBlob: unsafe extern "system" fn(this: *mut *mut Self, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DecodeBlob: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EncodeBlob: unsafe extern "system" fn(this: *mut *mut Self, bitcount: i32, strbitstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encodingin: EncodingType, encoding: EncodingType, pstrencodeddata: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EncodeBlob: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBitStringBlob: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pstrbitstring: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBitStringBlob: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertEncodeBitString2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3765491431, data2: 9199, data3: 19922, data4: [130, 66, 235, 217, 201, 40, 203, 48] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertEncodeCRLDistInfo {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Decode: unsafe extern "system" fn(this: *mut *mut Self, strbinary: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Decode: usize,
    pub GetDistPointCount: unsafe extern "system" fn(this: *mut *mut Self, pdistpointcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetNameCount: unsafe extern "system" fn(this: *mut *mut Self, distpointindex: i32, pnamecount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetNameChoice: unsafe extern "system" fn(this: *mut *mut Self, distpointindex: i32, nameindex: i32, pnamechoice: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, distpointindex: i32, nameindex: i32, pstrname: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetName: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self, distpointcount: i32) -> ::windows_sys::core::HRESULT,
    pub SetNameCount: unsafe extern "system" fn(this: *mut *mut Self, distpointindex: i32, namecount: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNameEntry: unsafe extern "system" fn(this: *mut *mut Self, distpointindex: i32, nameindex: i32, namechoice: CERT_ALT_NAME, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNameEntry: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Encode: unsafe extern "system" fn(this: *mut *mut Self, pstrbinary: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Encode: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertEncodeCRLDistInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 26576448, data2: 48127, data3: 4560, data4: [136, 37, 0, 160, 201, 3, 184, 60] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertEncodeCRLDistInfo2 {
    pub base__: ICertEncodeCRLDistInfo,
    #[cfg(feature = "Win32_Foundation")]
    pub DecodeBlob: unsafe extern "system" fn(this: *mut *mut Self, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DecodeBlob: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EncodeBlob: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pstrencodeddata: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EncodeBlob: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertEncodeCRLDistInfo2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3022478667, data2: 15920, data3: 17519, data4: [173, 54, 9, 208, 49, 32, 176, 120] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertEncodeDateArray {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Decode: unsafe extern "system" fn(this: *mut *mut Self, strbinary: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Decode: usize,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, pcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pvalue: *mut f64) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self, count: i32) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, index: i32, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Encode: unsafe extern "system" fn(this: *mut *mut Self, pstrbinary: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Encode: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertEncodeDateArray {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 798255520, data2: 42096, data3: 4560, data4: [136, 33, 0, 160, 201, 3, 184, 60] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertEncodeDateArray2 {
    pub base__: ICertEncodeDateArray,
    #[cfg(feature = "Win32_Foundation")]
    pub DecodeBlob: unsafe extern "system" fn(this: *mut *mut Self, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DecodeBlob: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EncodeBlob: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pstrencodeddata: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EncodeBlob: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertEncodeDateArray2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2577722805, data2: 11150, data3: 17549, data4: [191, 149, 187, 168, 215, 120, 157, 200] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertEncodeLongArray {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Decode: unsafe extern "system" fn(this: *mut *mut Self, strbinary: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Decode: usize,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, pcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self, count: i32) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, index: i32, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Encode: unsafe extern "system" fn(this: *mut *mut Self, pstrbinary: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Encode: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertEncodeLongArray {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 367194672, data2: 41122, data3: 4560, data4: [136, 33, 0, 160, 201, 3, 184, 60] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertEncodeLongArray2 {
    pub base__: ICertEncodeLongArray,
    #[cfg(feature = "Win32_Foundation")]
    pub DecodeBlob: unsafe extern "system" fn(this: *mut *mut Self, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DecodeBlob: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EncodeBlob: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pstrencodeddata: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EncodeBlob: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertEncodeLongArray2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1325262922, data2: 48539, data3: 20418, data4: [161, 8, 195, 71, 212, 120, 132, 15] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertEncodeStringArray {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Decode: unsafe extern "system" fn(this: *mut *mut Self, strbinary: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Decode: usize,
    pub GetStringType: unsafe extern "system" fn(this: *mut *mut Self, pstringtype: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, pcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pstr: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetValue: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self, count: i32, stringtype: super::CERT_RDN_ATTR_VALUE_TYPE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, index: i32, str: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetValue: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Encode: unsafe extern "system" fn(this: *mut *mut Self, pstrbinary: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Encode: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertEncodeStringArray {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 313034784, data2: 29844, data3: 4560, data4: [136, 22, 0, 160, 201, 3, 184, 60] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertEncodeStringArray2 {
    pub base__: ICertEncodeStringArray,
    #[cfg(feature = "Win32_Foundation")]
    pub DecodeBlob: unsafe extern "system" fn(this: *mut *mut Self, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DecodeBlob: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EncodeBlob: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pstrencodeddata: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EncodeBlob: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertEncodeStringArray2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2624064915, data2: 39805, data3: 20117, data4: [144, 24, 79, 254, 16, 186, 90, 218] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertExit {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, peventmask: *mut CERT_EXIT_EVENT_MASK) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub Notify: unsafe extern "system" fn(this: *mut *mut Self, exitevent: i32, context: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDescription: unsafe extern "system" fn(this: *mut *mut Self, pstrdescription: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDescription: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertExit {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3785023904, data2: 29540, data3: 4560, data4: [136, 22, 0, 160, 201, 3, 184, 60] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertExit2 {
    pub base__: ICertExit,
    #[cfg(feature = "Win32_System_Com")]
    pub GetManageModule: unsafe extern "system" fn(this: *mut *mut Self, ppmanagemodule: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetManageModule: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertExit2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 180308043, data2: 53321, data3: 17997, data4: [167, 237, 85, 46, 117, 41, 176, 255] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertGetConfig {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub GetConfig: unsafe extern "system" fn(this: *mut *mut Self, flags: CERT_GET_CONFIG_FLAGS, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetConfig: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertGetConfig {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3354003904, data2: 52759, data3: 4560, data4: [136, 51, 0, 160, 201, 3, 184, 60] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertManageModule {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strstoragelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, flags: i32, pvarproperty: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strstoragelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, flags: i32, pvarproperty: *const super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Configure: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strstoragelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, flags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Configure: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertManageModule {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3889671490, data2: 48445, data3: 4561, data4: [154, 77, 0, 192, 79, 194, 151, 235] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPolicy {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub VerifyRequest: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, context: i32, bnewrequest: i32, flags: i32, pdisposition: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    VerifyRequest: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDescription: unsafe extern "system" fn(this: *mut *mut Self, pstrdescription: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDescription: usize,
    pub ShutDown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertPolicy {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 951802368, data2: 30262, data3: 4560, data4: [180, 19, 0, 160, 201, 27, 191, 140] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPolicy2 {
    pub base__: ICertPolicy,
    #[cfg(feature = "Win32_System_Com")]
    pub GetManageModule: unsafe extern "system" fn(this: *mut *mut Self, ppmanagemodule: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetManageModule: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertPolicy2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1035243790, data2: 32769, data3: 19441, data4: [170, 27, 244, 58, 128, 131, 23, 160] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertProperties {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeFromCertificate: unsafe extern "system" fn(this: *mut *mut Self, machinecontext: i16, encoding: EncodingType, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeFromCertificate: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692463, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertProperty {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeFromCertificate: unsafe extern "system" fn(this: *mut *mut Self, machinecontext: i16, encoding: EncodingType, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeFromCertificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeDecode: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeDecode: usize,
    pub PropertyId: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut CERTENROLL_PROPERTYID) -> ::windows_sys::core::HRESULT,
    pub SetPropertyId: unsafe extern "system" fn(this: *mut *mut Self, value: CERTENROLL_PROPERTYID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_RawData: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_RawData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveFromCertificate: unsafe extern "system" fn(this: *mut *mut Self, machinecontext: i16, encoding: EncodingType, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveFromCertificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetValueOnCertificate: unsafe extern "system" fn(this: *mut *mut Self, machinecontext: i16, encoding: EncodingType, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetValueOnCertificate: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertProperty {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692462, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPropertyArchived {
    pub base__: ICertProperty,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, archivedvalue: i16) -> ::windows_sys::core::HRESULT,
    pub Archived: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertPropertyArchived {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692471, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPropertyArchivedKeyHash {
    pub base__: ICertProperty,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strarchivedkeyhashvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_ArchivedKeyHash: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_ArchivedKeyHash: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertPropertyArchivedKeyHash {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692475, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPropertyAutoEnroll {
    pub base__: ICertProperty,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TemplateName: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TemplateName: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertPropertyAutoEnroll {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692466, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPropertyBackedUp {
    pub base__: ICertProperty,
    pub InitializeFromCurrentTime: unsafe extern "system" fn(this: *mut *mut Self, backedupvalue: i16) -> ::windows_sys::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, backedupvalue: i16, date: f64) -> ::windows_sys::core::HRESULT,
    pub BackedUpValue: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub BackedUpTime: unsafe extern "system" fn(this: *mut *mut Self, pdate: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertPropertyBackedUp {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692472, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPropertyDescription {
    pub base__: ICertProperty,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, strdescription: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertPropertyDescription {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692465, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPropertyEnrollment {
    pub base__: ICertProperty,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, requestid: i32, strcadnsname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strcaname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strfriendlyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub RequestId: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CADnsName: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CADnsName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CAName: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CAName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FriendlyName: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FriendlyName: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertPropertyEnrollment {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692473, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPropertyEnrollmentPolicyServer {
    pub base__: ICertProperty,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, propertyflags: EnrollmentPolicyServerPropertyFlags, authflags: X509EnrollmentAuthFlags, enrollmentserverauthflags: X509EnrollmentAuthFlags, urlflags: PolicyServerUrlFlags, strrequestid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strenrollmentserverurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPolicyServerUrl: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPolicyServerUrl: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPolicyServerId: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPolicyServerId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnrollmentServerUrl: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnrollmentServerUrl: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRequestIdString: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRequestIdString: usize,
    pub GetPropertyFlags: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut EnrollmentPolicyServerPropertyFlags) -> ::windows_sys::core::HRESULT,
    pub GetUrlFlags: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut PolicyServerUrlFlags) -> ::windows_sys::core::HRESULT,
    pub GetAuthentication: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut X509EnrollmentAuthFlags) -> ::windows_sys::core::HRESULT,
    pub GetEnrollmentServerAuthentication: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut X509EnrollmentAuthFlags) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertPropertyEnrollmentPolicyServer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692490, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPropertyFriendlyName {
    pub base__: ICertProperty,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, strfriendlyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FriendlyName: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FriendlyName: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertPropertyFriendlyName {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692464, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPropertyKeyProvInfo {
    pub base__: ICertProperty,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateKey: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateKey: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertPropertyKeyProvInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692470, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPropertyRenewal {
    pub base__: ICertProperty,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strrenewalvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeFromCertificateHash: unsafe extern "system" fn(this: *mut *mut Self, machinecontext: i16, encoding: EncodingType, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeFromCertificateHash: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_Renewal: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_Renewal: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertPropertyRenewal {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692474, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPropertyRequestOriginator {
    pub base__: ICertProperty,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, strrequestoriginator: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub InitializeFromLocalRequestOriginator: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestOriginator: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestOriginator: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertPropertyRequestOriginator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692467, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertPropertySHA1Hash {
    pub base__: ICertProperty,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strrenewalvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_SHA1Hash: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_SHA1Hash: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertPropertySHA1Hash {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692468, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertRequest {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Submit: unsafe extern "system" fn(this: *mut *mut Self, flags: i32, strrequest: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strattributes: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdisposition: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Submit: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RetrievePending: unsafe extern "system" fn(this: *mut *mut Self, requestid: i32, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdisposition: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RetrievePending: usize,
    pub GetLastStatus: unsafe extern "system" fn(this: *mut *mut Self, pstatus: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetRequestId: unsafe extern "system" fn(this: *mut *mut Self, prequestid: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDispositionMessage: unsafe extern "system" fn(this: *mut *mut Self, pstrdispositionmessage: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDispositionMessage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCACertificate: unsafe extern "system" fn(this: *mut *mut Self, fexchangecertificate: i32, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, flags: i32, pstrcertificate: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCACertificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCertificate: unsafe extern "system" fn(this: *mut *mut Self, flags: i32, pstrcertificate: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCertificate: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 21907520, data2: 21795, data3: 4560, data4: [136, 18, 0, 160, 201, 3, 184, 60] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertRequest2 {
    pub base__: ICertRequest,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIssuedCertificate: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, requestid: i32, strserialnumber: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdisposition: *mut CR_DISP) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIssuedCertificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetErrorMessageText: unsafe extern "system" fn(this: *mut *mut Self, hrmessage: i32, flags: i32, pstrerrormessagetext: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetErrorMessageText: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetCAProperty: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propid: i32, propindex: i32, proptype: i32, flags: i32, pvarpropertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetCAProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCAPropertyFlags: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propid: i32, ppropflags: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCAPropertyFlags: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCAPropertyDisplayName: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propid: i32, pstrdisplayname: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCAPropertyDisplayName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetFullResponseProperty: unsafe extern "system" fn(this: *mut *mut Self, propid: FULL_RESPONSE_PROPERTY_ID, propindex: i32, proptype: CERT_PROPERTY_TYPE, flags: CERT_REQUEST_OUT_TYPE, pvarpropertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetFullResponseProperty: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertRequest2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2759272840, data2: 19077, data3: 20393, data4: [130, 78, 181, 207, 92, 22, 64, 90] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertRequest3 {
    pub base__: ICertRequest2,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCredential: unsafe extern "system" fn(this: *mut *mut Self, hwnd: i32, authtype: X509EnrollmentAuthFlags, strcredential: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCredential: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRequestIdString: unsafe extern "system" fn(this: *mut *mut Self, pstrrequestid: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRequestIdString: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIssuedCertificate2: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strrequestid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strserialnumber: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pdisposition: *mut CR_DISP) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIssuedCertificate2: usize,
    pub GetRefreshPolicy: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertRequest3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2949183787, data2: 13218, data3: 18529, data4: [191, 54, 41, 51, 183, 205, 103, 179] };
}
#[repr(C)]
pub struct ICertRequestD {
    pub base__: ::windows_sys::core::IUnknown,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, dwflags: u32, pwszauthority: ::windows_sys::core::PCWSTR, pdwrequestid: *mut u32, pdwdisposition: *mut u32, pwszattributes: ::windows_sys::core::PCWSTR, pctbrequest: *const CERTTRANSBLOB, pctbcertchain: *mut CERTTRANSBLOB, pctbencodedcert: *mut CERTTRANSBLOB, pctbdispositionmessage: *mut CERTTRANSBLOB) -> ::windows_sys::core::HRESULT,
    pub GetCACert: unsafe extern "system" fn(this: *mut *mut Self, fchain: u32, pwszauthority: ::windows_sys::core::PCWSTR, pctbout: *mut CERTTRANSBLOB) -> ::windows_sys::core::HRESULT,
    pub Ping: unsafe extern "system" fn(this: *mut *mut Self, pwszauthority: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICertRequestD {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3651038832, data2: 64648, data3: 4560, data4: [180, 152, 0, 160, 201, 3, 18, 243] };
}
#[repr(C)]
pub struct ICertRequestD2 {
    pub base__: ICertRequestD,
    pub Request2: unsafe extern "system" fn(this: *mut *mut Self, pwszauthority: ::windows_sys::core::PCWSTR, dwflags: u32, pwszserialnumber: ::windows_sys::core::PCWSTR, pdwrequestid: *mut u32, pdwdisposition: *mut u32, pwszattributes: ::windows_sys::core::PCWSTR, pctbrequest: *const CERTTRANSBLOB, pctbfullresponse: *mut CERTTRANSBLOB, pctbencodedcert: *mut CERTTRANSBLOB, pctbdispositionmessage: *mut CERTTRANSBLOB) -> ::windows_sys::core::HRESULT,
    pub GetCAProperty: unsafe extern "system" fn(this: *mut *mut Self, pwszauthority: ::windows_sys::core::PCWSTR, propid: i32, propindex: i32, proptype: i32, pctbpropertyvalue: *mut CERTTRANSBLOB) -> ::windows_sys::core::HRESULT,
    pub GetCAPropertyInfo: unsafe extern "system" fn(this: *mut *mut Self, pwszauthority: ::windows_sys::core::PCWSTR, pcproperty: *mut i32, pctbpropinfo: *mut CERTTRANSBLOB) -> ::windows_sys::core::HRESULT,
    pub Ping2: unsafe extern "system" fn(this: *mut *mut Self, pwszauthority: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICertRequestD2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1411579194, data2: 54456, data3: 19695, data4: [161, 46, 232, 125, 76, 162, 46, 144] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertServerExit {
    pub base__: super::super::super::System::Com::IDispatch,
    pub SetContext: unsafe extern "system" fn(this: *mut *mut Self, context: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetRequestProperty: unsafe extern "system" fn(this: *mut *mut Self, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertytype: i32, pvarpropertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetRequestProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRequestAttribute: unsafe extern "system" fn(this: *mut *mut Self, strattributename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrattributevalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRequestAttribute: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetCertificateProperty: unsafe extern "system" fn(this: *mut *mut Self, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertytype: i32, pvarpropertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetCertificateProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetCertificateExtension: unsafe extern "system" fn(this: *mut *mut Self, strextensionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, r#type: i32, pvarvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetCertificateExtension: usize,
    pub GetCertificateExtensionFlags: unsafe extern "system" fn(this: *mut *mut Self, pextflags: *mut i32) -> ::windows_sys::core::HRESULT,
    pub EnumerateExtensionsSetup: unsafe extern "system" fn(this: *mut *mut Self, flags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumerateExtensions: unsafe extern "system" fn(this: *mut *mut Self, pstrextensionname: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumerateExtensions: usize,
    pub EnumerateExtensionsClose: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EnumerateAttributesSetup: unsafe extern "system" fn(this: *mut *mut Self, flags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumerateAttributes: unsafe extern "system" fn(this: *mut *mut Self, pstrattributename: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumerateAttributes: usize,
    pub EnumerateAttributesClose: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertServerExit {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1269427088, data2: 29484, data3: 4560, data4: [136, 22, 0, 160, 201, 3, 184, 60] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertServerPolicy {
    pub base__: super::super::super::System::Com::IDispatch,
    pub SetContext: unsafe extern "system" fn(this: *mut *mut Self, context: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetRequestProperty: unsafe extern "system" fn(this: *mut *mut Self, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertytype: i32, pvarpropertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetRequestProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRequestAttribute: unsafe extern "system" fn(this: *mut *mut Self, strattributename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstrattributevalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRequestAttribute: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetCertificateProperty: unsafe extern "system" fn(this: *mut *mut Self, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertytype: CERT_PROPERTY_TYPE, pvarpropertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetCertificateProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetCertificateProperty: unsafe extern "system" fn(this: *mut *mut Self, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertytype: i32, pvarpropertyvalue: *const super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetCertificateProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetCertificateExtension: unsafe extern "system" fn(this: *mut *mut Self, strextensionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, r#type: CERT_PROPERTY_TYPE, pvarvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetCertificateExtension: usize,
    pub GetCertificateExtensionFlags: unsafe extern "system" fn(this: *mut *mut Self, pextflags: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetCertificateExtension: unsafe extern "system" fn(this: *mut *mut Self, strextensionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, r#type: i32, extflags: i32, pvarvalue: *const super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetCertificateExtension: usize,
    pub EnumerateExtensionsSetup: unsafe extern "system" fn(this: *mut *mut Self, flags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumerateExtensions: unsafe extern "system" fn(this: *mut *mut Self, pstrextensionname: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumerateExtensions: usize,
    pub EnumerateExtensionsClose: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EnumerateAttributesSetup: unsafe extern "system" fn(this: *mut *mut Self, flags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumerateAttributes: unsafe extern "system" fn(this: *mut *mut Self, pstrattributename: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumerateAttributes: usize,
    pub EnumerateAttributesClose: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertServerPolicy {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2852129058, data2: 65470, data3: 4559, data4: [136, 0, 0, 160, 201, 3, 184, 60] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertView {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub OpenConnection: unsafe extern "system" fn(this: *mut *mut Self, strconfig: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OpenConnection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumCertViewColumn: unsafe extern "system" fn(this: *mut *mut Self, fresultcolumn: CVRC_COLUMN, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumCertViewColumn: usize,
    pub GetColumnCount: unsafe extern "system" fn(this: *mut *mut Self, fresultcolumn: CVRC_COLUMN, pccolumn: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetColumnIndex: unsafe extern "system" fn(this: *mut *mut Self, fresultcolumn: CVRC_COLUMN, strcolumnname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pcolumnindex: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetColumnIndex: usize,
    pub SetResultColumnCount: unsafe extern "system" fn(this: *mut *mut Self, cresultcolumn: i32) -> ::windows_sys::core::HRESULT,
    pub SetResultColumn: unsafe extern "system" fn(this: *mut *mut Self, columnindex: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetRestriction: unsafe extern "system" fn(this: *mut *mut Self, columnindex: CERT_VIEW_COLUMN_INDEX, seekoperator: CERT_VIEW_SEEK_OPERATOR_FLAGS, sortorder: i32, pvarvalue: *const super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetRestriction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenView: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenView: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertView {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3287991108, data2: 7812, data3: 4561, data4: [155, 214, 0, 192, 79, 182, 131, 250] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertView2 {
    pub base__: ICertView,
    pub SetTable: unsafe extern "system" fn(this: *mut *mut Self, table: CVRC_TABLE) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertView2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3583292034, data2: 34897, data3: 19297, data4: [156, 102, 62, 218, 223, 132, 136, 99] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertificateAttestationChallenge {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strpendingfullcmcresponsewithchallenge: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DecryptChallenge: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pstrenvelopedpkcs7reencryptedtoca: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DecryptChallenge: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestID: unsafe extern "system" fn(this: *mut *mut Self, pstrrequestid: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestID: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertificateAttestationChallenge {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1863801468, data2: 19002, data3: 16558, data4: [157, 186, 89, 47, 214, 187, 249, 184] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertificateAttestationChallenge2 {
    pub base__: ICertificateAttestationChallenge,
    #[cfg(feature = "Win32_Foundation")]
    pub SetKeyContainerName: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetKeyContainerName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub put_KeyBlob: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_KeyBlob: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertificateAttestationChallenge2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1177629517, data2: 57958, data3: 18390, data4: [189, 121, 190, 83, 203, 46, 39, 83] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertificatePolicies {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertificatePolicies {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692447, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertificatePolicy {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ObjectId: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ObjectId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PolicyQualifiers: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PolicyQualifiers: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertificatePolicy {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692446, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertificationAuthorities {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ComputeSiteCosts: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub get_ItemByName: unsafe extern "system" fn(this: *mut *mut Self, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    get_ItemByName: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertificationAuthorities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 330797061, data2: 8577, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICertificationAuthority {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Property: unsafe extern "system" fn(this: *mut *mut Self, property: EnrollmentCAProperty, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Property: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICertificationAuthority {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2203918177, data2: 7829, data3: 19400, data4: [180, 211, 151, 108, 66, 185, 104, 247] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICryptAttribute {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromObjectId: unsafe extern "system" fn(this: *mut *mut Self, pobjectid: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromObjectId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromValues: unsafe extern "system" fn(this: *mut *mut Self, pattributes: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromValues: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ObjectId: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ObjectId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Values: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Values: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICryptAttribute {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692460, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICryptAttributes {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_IndexByObjectId: unsafe extern "system" fn(this: *mut *mut Self, pobjectid: *mut ::core::ffi::c_void, pindex: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_IndexByObjectId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICryptAttributes {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692461, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICspAlgorithm {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAlgorithmOid: unsafe extern "system" fn(this: *mut *mut Self, length: i32, algflags: AlgorithmFlags, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAlgorithmOid: usize,
    pub DefaultLength: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub IncrementLength: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LongName: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LongName: usize,
    pub Valid: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub MaxLength: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MinLength: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut AlgorithmType) -> ::windows_sys::core::HRESULT,
    pub Operations: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut AlgorithmOperationFlags) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICspAlgorithm {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692421, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICspAlgorithms {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub get_ItemByName: unsafe extern "system" fn(this: *mut *mut Self, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    get_ItemByName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_IndexByObjectId: unsafe extern "system" fn(this: *mut *mut Self, pobjectid: *mut ::core::ffi::c_void, pindex: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_IndexByObjectId: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICspAlgorithms {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692422, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICspInformation {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeFromName: unsafe extern "system" fn(this: *mut *mut Self, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeFromName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromType: unsafe extern "system" fn(this: *mut *mut Self, r#type: X509ProviderType, palgorithm: *mut ::core::ffi::c_void, machinecontext: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromType: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CspAlgorithms: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CspAlgorithms: usize,
    pub HasHardwareRandomNumberGenerator: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsHardwareDevice: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsRemovable: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub IsSoftwareDevice: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Valid: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub MaxKeyContainerNameLength: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut X509ProviderType) -> ::windows_sys::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub KeySpec: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut X509KeySpec) -> ::windows_sys::core::HRESULT,
    pub IsSmartCard: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDefaultSecurityDescriptor: unsafe extern "system" fn(this: *mut *mut Self, machinecontext: i16, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDefaultSecurityDescriptor: usize,
    pub LegacyCsp: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCspStatusFromOperations: unsafe extern "system" fn(this: *mut *mut Self, palgorithm: *mut ::core::ffi::c_void, operations: AlgorithmOperationFlags, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCspStatusFromOperations: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICspInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692423, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICspInformations {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub AddAvailableCsps: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub get_ItemByName: unsafe extern "system" fn(this: *mut *mut Self, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppcspinformation: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    get_ItemByName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetCspStatusFromProviderName: unsafe extern "system" fn(this: *mut *mut Self, strprovidername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, legacykeyspec: X509KeySpec, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetCspStatusFromProviderName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCspStatusesFromOperations: unsafe extern "system" fn(this: *mut *mut Self, operations: AlgorithmOperationFlags, pcspinformation: *mut ::core::ffi::c_void, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCspStatusesFromOperations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetEncryptionCspAlgorithms: unsafe extern "system" fn(this: *mut *mut Self, pcspinformation: *mut ::core::ffi::c_void, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetEncryptionCspAlgorithms: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetHashAlgorithms: unsafe extern "system" fn(this: *mut *mut Self, pcspinformation: *mut ::core::ffi::c_void, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetHashAlgorithms: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICspInformations {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692424, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICspStatus {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, pcsp: *mut ::core::ffi::c_void, palgorithm: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    pub Ordinal: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetOrdinal: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CspAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CspAlgorithm: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CspInformation: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CspInformation: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnrollmentStatus: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnrollmentStatus: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICspStatus {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692425, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICspStatuses {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub get_ItemByName: unsafe extern "system" fn(this: *mut *mut Self, strcspname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, stralgorithmname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    get_ItemByName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByOrdinal: unsafe extern "system" fn(this: *mut *mut Self, ordinal: i32, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByOrdinal: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub get_ItemByOperations: unsafe extern "system" fn(this: *mut *mut Self, strcspname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, stralgorithmname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, operations: AlgorithmOperationFlags, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    get_ItemByOperations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByProvider: unsafe extern "system" fn(this: *mut *mut Self, pcspstatus: *mut ::core::ffi::c_void, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByProvider: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ICspStatuses {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692426, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[repr(C)]
pub struct IEnroll {
    pub base__: ::windows_sys::core::IUnknown,
    pub createFilePKCS10WStr: unsafe extern "system" fn(this: *mut *mut Self, dnname: ::windows_sys::core::PCWSTR, usage: ::windows_sys::core::PCWSTR, wszpkcs10filename: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub acceptFilePKCS7WStr: unsafe extern "system" fn(this: *mut *mut Self, wszpkcs7filename: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub createPKCS10WStr: unsafe extern "system" fn(this: *mut *mut Self, dnname: ::windows_sys::core::PCWSTR, usage: ::windows_sys::core::PCWSTR, ppkcs10blob: *mut super::CRYPTOAPI_BLOB) -> ::windows_sys::core::HRESULT,
    pub acceptPKCS7Blob: unsafe extern "system" fn(this: *mut *mut Self, pblobpkcs7: *mut super::CRYPTOAPI_BLOB) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub getCertContextFromPKCS7: unsafe extern "system" fn(this: *mut *mut Self, pblobpkcs7: *mut super::CRYPTOAPI_BLOB) -> *mut super::CERT_CONTEXT,
    #[cfg(not(feature = "Win32_Foundation"))]
    getCertContextFromPKCS7: usize,
    pub getMyStore: unsafe extern "system" fn(this: *mut *mut Self) -> super::HCERTSTORE,
    pub getCAStore: unsafe extern "system" fn(this: *mut *mut Self) -> super::HCERTSTORE,
    pub getROOTHStore: unsafe extern "system" fn(this: *mut *mut Self) -> super::HCERTSTORE,
    pub enumProvidersWStr: unsafe extern "system" fn(this: *mut *mut Self, dwindex: i32, dwflags: i32, pbstrprovname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub enumContainersWStr: unsafe extern "system" fn(this: *mut *mut Self, dwindex: i32, pbstr: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub freeRequestInfoBlob: unsafe extern "system" fn(this: *mut *mut Self, pkcs7orpkcs10: super::CRYPTOAPI_BLOB) -> ::windows_sys::core::HRESULT,
    pub MyStoreNameWStr: unsafe extern "system" fn(this: *mut *mut Self, szwname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetMyStoreNameWStr: unsafe extern "system" fn(this: *mut *mut Self, szwname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub MyStoreTypeWStr: unsafe extern "system" fn(this: *mut *mut Self, szwtype: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetMyStoreTypeWStr: unsafe extern "system" fn(this: *mut *mut Self, szwtype: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub MyStoreFlags: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMyStoreFlags: unsafe extern "system" fn(this: *mut *mut Self, dwflags: i32) -> ::windows_sys::core::HRESULT,
    pub CAStoreNameWStr: unsafe extern "system" fn(this: *mut *mut Self, szwname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetCAStoreNameWStr: unsafe extern "system" fn(this: *mut *mut Self, szwname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub CAStoreTypeWStr: unsafe extern "system" fn(this: *mut *mut Self, szwtype: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetCAStoreTypeWStr: unsafe extern "system" fn(this: *mut *mut Self, szwtype: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub CAStoreFlags: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetCAStoreFlags: unsafe extern "system" fn(this: *mut *mut Self, dwflags: i32) -> ::windows_sys::core::HRESULT,
    pub RootStoreNameWStr: unsafe extern "system" fn(this: *mut *mut Self, szwname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetRootStoreNameWStr: unsafe extern "system" fn(this: *mut *mut Self, szwname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub RootStoreTypeWStr: unsafe extern "system" fn(this: *mut *mut Self, szwtype: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetRootStoreTypeWStr: unsafe extern "system" fn(this: *mut *mut Self, szwtype: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub RootStoreFlags: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetRootStoreFlags: unsafe extern "system" fn(this: *mut *mut Self, dwflags: i32) -> ::windows_sys::core::HRESULT,
    pub RequestStoreNameWStr: unsafe extern "system" fn(this: *mut *mut Self, szwname: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetRequestStoreNameWStr: unsafe extern "system" fn(this: *mut *mut Self, szwname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub RequestStoreTypeWStr: unsafe extern "system" fn(this: *mut *mut Self, szwtype: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetRequestStoreTypeWStr: unsafe extern "system" fn(this: *mut *mut Self, szwtype: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub RequestStoreFlags: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetRequestStoreFlags: unsafe extern "system" fn(this: *mut *mut Self, dwflags: i32) -> ::windows_sys::core::HRESULT,
    pub ContainerNameWStr: unsafe extern "system" fn(this: *mut *mut Self, szwcontainer: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetContainerNameWStr: unsafe extern "system" fn(this: *mut *mut Self, szwcontainer: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub ProviderNameWStr: unsafe extern "system" fn(this: *mut *mut Self, szwprovider: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetProviderNameWStr: unsafe extern "system" fn(this: *mut *mut Self, szwprovider: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub ProviderType: unsafe extern "system" fn(this: *mut *mut Self, pdwtype: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetProviderType: unsafe extern "system" fn(this: *mut *mut Self, dwtype: i32) -> ::windows_sys::core::HRESULT,
    pub KeySpec: unsafe extern "system" fn(this: *mut *mut Self, pdw: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetKeySpec: unsafe extern "system" fn(this: *mut *mut Self, dw: i32) -> ::windows_sys::core::HRESULT,
    pub ProviderFlags: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetProviderFlags: unsafe extern "system" fn(this: *mut *mut Self, dwflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UseExistingKeySet: unsafe extern "system" fn(this: *mut *mut Self, fuseexistingkeys: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UseExistingKeySet: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUseExistingKeySet: unsafe extern "system" fn(this: *mut *mut Self, fuseexistingkeys: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUseExistingKeySet: usize,
    pub GenKeyFlags: unsafe extern "system" fn(this: *mut *mut Self, pdwflags: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetGenKeyFlags: unsafe extern "system" fn(this: *mut *mut Self, dwflags: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteRequestCert: unsafe extern "system" fn(this: *mut *mut Self, fdelete: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteRequestCert: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDeleteRequestCert: unsafe extern "system" fn(this: *mut *mut Self, fdelete: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDeleteRequestCert: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteCertToUserDS: unsafe extern "system" fn(this: *mut *mut Self, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteCertToUserDS: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWriteCertToUserDS: unsafe extern "system" fn(this: *mut *mut Self, fbool: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWriteCertToUserDS: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableT61DNEncoding: unsafe extern "system" fn(this: *mut *mut Self, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableT61DNEncoding: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableT61DNEncoding: unsafe extern "system" fn(this: *mut *mut Self, fbool: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableT61DNEncoding: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteCertToCSP: unsafe extern "system" fn(this: *mut *mut Self, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteCertToCSP: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWriteCertToCSP: unsafe extern "system" fn(this: *mut *mut Self, fbool: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWriteCertToCSP: usize,
    pub SPCFileNameWStr: unsafe extern "system" fn(this: *mut *mut Self, szw: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetSPCFileNameWStr: unsafe extern "system" fn(this: *mut *mut Self, szw: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub PVKFileNameWStr: unsafe extern "system" fn(this: *mut *mut Self, szw: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetPVKFileNameWStr: unsafe extern "system" fn(this: *mut *mut Self, szw: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub HashAlgorithmWStr: unsafe extern "system" fn(this: *mut *mut Self, szw: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetHashAlgorithmWStr: unsafe extern "system" fn(this: *mut *mut Self, szw: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RenewalCertificate: unsafe extern "system" fn(this: *mut *mut Self, ppcertcontext: *mut *mut super::CERT_CONTEXT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RenewalCertificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRenewalCertificate: unsafe extern "system" fn(this: *mut *mut Self, pcertcontext: *const super::CERT_CONTEXT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRenewalCertificate: usize,
    pub AddCertTypeToRequestWStr: unsafe extern "system" fn(this: *mut *mut Self, szw: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub AddNameValuePairToSignatureWStr: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, value: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddExtensionsToRequest: unsafe extern "system" fn(this: *mut *mut Self, pcertextensions: *mut super::CERT_EXTENSIONS) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddExtensionsToRequest: usize,
    pub AddAuthenticatedAttributesToPKCS7Request: unsafe extern "system" fn(this: *mut *mut Self, pattributes: *mut super::CRYPT_ATTRIBUTES) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreatePKCS7RequestFromRequest: unsafe extern "system" fn(this: *mut *mut Self, prequest: *mut super::CRYPTOAPI_BLOB, psigningcertcontext: *const super::CERT_CONTEXT, ppkcs7blob: *mut super::CRYPTOAPI_BLOB) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreatePKCS7RequestFromRequest: usize,
}
impl ::windows_sys::core::Interface for IEnroll {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2896853048, data2: 17797, data3: 4561, data4: [171, 87, 0, 192, 79, 194, 149, 225] };
}
#[repr(C)]
pub struct IEnroll2 {
    pub base__: IEnroll,
    pub InstallPKCS7Blob: unsafe extern "system" fn(this: *mut *mut Self, pblobpkcs7: *mut super::CRYPTOAPI_BLOB) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetSupportedKeySpec: unsafe extern "system" fn(this: *mut *mut Self, pdwkeyspec: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetKeyLen: unsafe extern "system" fn(this: *mut *mut Self, fmin: super::super::super::Foundation::BOOL, fexchange: super::super::super::Foundation::BOOL, pdwkeysize: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetKeyLen: usize,
    pub EnumAlgs: unsafe extern "system" fn(this: *mut *mut Self, dwindex: i32, algclass: i32, pdwalgid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetAlgNameWStr: unsafe extern "system" fn(this: *mut *mut Self, algid: i32, ppwsz: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReuseHardwareKeyIfUnableToGenNew: unsafe extern "system" fn(this: *mut *mut Self, freusehardwarekeyifunabletogennew: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReuseHardwareKeyIfUnableToGenNew: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReuseHardwareKeyIfUnableToGenNew: unsafe extern "system" fn(this: *mut *mut Self, freusehardwarekeyifunabletogennew: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReuseHardwareKeyIfUnableToGenNew: usize,
    pub SetHashAlgID: unsafe extern "system" fn(this: *mut *mut Self, hashalgid: i32) -> ::windows_sys::core::HRESULT,
    pub HashAlgID: unsafe extern "system" fn(this: *mut *mut Self, hashalgid: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetHStoreMy: unsafe extern "system" fn(this: *mut *mut Self, hstore: super::HCERTSTORE) -> ::windows_sys::core::HRESULT,
    pub SetHStoreCA: unsafe extern "system" fn(this: *mut *mut Self, hstore: super::HCERTSTORE) -> ::windows_sys::core::HRESULT,
    pub SetHStoreROOT: unsafe extern "system" fn(this: *mut *mut Self, hstore: super::HCERTSTORE) -> ::windows_sys::core::HRESULT,
    pub SetHStoreRequest: unsafe extern "system" fn(this: *mut *mut Self, hstore: super::HCERTSTORE) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLimitExchangeKeyToEncipherment: unsafe extern "system" fn(this: *mut *mut Self, flimitexchangekeytoencipherment: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLimitExchangeKeyToEncipherment: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LimitExchangeKeyToEncipherment: unsafe extern "system" fn(this: *mut *mut Self, flimitexchangekeytoencipherment: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LimitExchangeKeyToEncipherment: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableSMIMECapabilities: unsafe extern "system" fn(this: *mut *mut Self, fenablesmimecapabilities: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableSMIMECapabilities: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableSMIMECapabilities: unsafe extern "system" fn(this: *mut *mut Self, fenablesmimecapabilities: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableSMIMECapabilities: usize,
}
impl ::windows_sys::core::Interface for IEnroll2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3229671833, data2: 47071, data3: 4562, data4: [164, 33, 0, 192, 79, 121, 254, 142] };
}
#[repr(C)]
pub struct IEnroll4 {
    pub base__: IEnroll2,
    pub SetThumbPrintWStr: unsafe extern "system" fn(this: *mut *mut Self, thumbprintblob: super::CRYPTOAPI_BLOB) -> ::windows_sys::core::HRESULT,
    pub ThumbPrintWStr: unsafe extern "system" fn(this: *mut *mut Self, thumbprintblob: *mut super::CRYPTOAPI_BLOB) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPrivateKeyArchiveCertificate: unsafe extern "system" fn(this: *mut *mut Self, pprivatekeyarchivecert: *const super::CERT_CONTEXT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPrivateKeyArchiveCertificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPrivateKeyArchiveCertificate: unsafe extern "system" fn(this: *mut *mut Self) -> *mut super::CERT_CONTEXT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPrivateKeyArchiveCertificate: usize,
    pub binaryBlobToString: unsafe extern "system" fn(this: *mut *mut Self, flags: i32, pblobbinary: *mut super::CRYPTOAPI_BLOB, ppwszstring: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub stringToBinaryBlob: unsafe extern "system" fn(this: *mut *mut Self, flags: i32, pwszstring: ::windows_sys::core::PCWSTR, pblobbinary: *mut super::CRYPTOAPI_BLOB, pdwskip: *mut i32, pdwflags: *mut i32) -> ::windows_sys::core::HRESULT,
    pub addExtensionToRequestWStr: unsafe extern "system" fn(this: *mut *mut Self, flags: i32, pwszname: ::windows_sys::core::PCWSTR, pblobvalue: *mut super::CRYPTOAPI_BLOB) -> ::windows_sys::core::HRESULT,
    pub addAttributeToRequestWStr: unsafe extern "system" fn(this: *mut *mut Self, flags: i32, pwszname: ::windows_sys::core::PCWSTR, pblobvalue: *mut super::CRYPTOAPI_BLOB) -> ::windows_sys::core::HRESULT,
    pub addNameValuePairToRequestWStr: unsafe extern "system" fn(this: *mut *mut Self, flags: i32, pwszname: ::windows_sys::core::PCWSTR, pwszvalue: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub resetExtensions: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub resetAttributes: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub createRequestWStr: unsafe extern "system" fn(this: *mut *mut Self, flags: CERT_CREATE_REQUEST_FLAGS, pwszdnname: ::windows_sys::core::PCWSTR, pwszusage: ::windows_sys::core::PCWSTR, pblobrequest: *mut super::CRYPTOAPI_BLOB) -> ::windows_sys::core::HRESULT,
    pub createFileRequestWStr: unsafe extern "system" fn(this: *mut *mut Self, flags: CERT_CREATE_REQUEST_FLAGS, pwszdnname: ::windows_sys::core::PCWSTR, pwszusage: ::windows_sys::core::PCWSTR, pwszrequestfilename: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub acceptResponseBlob: unsafe extern "system" fn(this: *mut *mut Self, pblobresponse: *mut super::CRYPTOAPI_BLOB) -> ::windows_sys::core::HRESULT,
    pub acceptFileResponseWStr: unsafe extern "system" fn(this: *mut *mut Self, pwszresponsefilename: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub getCertContextFromResponseBlob: unsafe extern "system" fn(this: *mut *mut Self, pblobresponse: *mut super::CRYPTOAPI_BLOB, ppcertcontext: *mut *mut super::CERT_CONTEXT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    getCertContextFromResponseBlob: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub getCertContextFromFileResponseWStr: unsafe extern "system" fn(this: *mut *mut Self, pwszresponsefilename: ::windows_sys::core::PCWSTR, ppcertcontext: *mut *mut super::CERT_CONTEXT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    getCertContextFromFileResponseWStr: usize,
    pub createPFXWStr: unsafe extern "system" fn(this: *mut *mut Self, pwszpassword: ::windows_sys::core::PCWSTR, pblobpfx: *mut super::CRYPTOAPI_BLOB) -> ::windows_sys::core::HRESULT,
    pub createFilePFXWStr: unsafe extern "system" fn(this: *mut *mut Self, pwszpassword: ::windows_sys::core::PCWSTR, pwszpfxfilename: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub setPendingRequestInfoWStr: unsafe extern "system" fn(this: *mut *mut Self, lrequestid: i32, pwszcadns: ::windows_sys::core::PCWSTR, pwszcaname: ::windows_sys::core::PCWSTR, pwszfriendlyname: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub enumPendingRequestWStr: unsafe extern "system" fn(this: *mut *mut Self, lindex: i32, ldesiredproperty: PENDING_REQUEST_DESIRED_PROPERTY, ppproperty: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub removePendingRequestWStr: unsafe extern "system" fn(this: *mut *mut Self, thumbprintblob: super::CRYPTOAPI_BLOB) -> ::windows_sys::core::HRESULT,
    pub GetKeyLenEx: unsafe extern "system" fn(this: *mut *mut Self, lsizespec: XEKL_KEYSIZE, lkeyspec: XEKL_KEYSPEC, pdwkeysize: *mut i32) -> ::windows_sys::core::HRESULT,
    pub InstallPKCS7BlobEx: unsafe extern "system" fn(this: *mut *mut Self, pblobpkcs7: *mut super::CRYPTOAPI_BLOB, plcertinstalled: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddCertTypeToRequestWStrEx: unsafe extern "system" fn(this: *mut *mut Self, ltype: ADDED_CERT_TYPE, pwszoidorname: ::windows_sys::core::PCWSTR, lmajorversion: i32, fminorversion: super::super::super::Foundation::BOOL, lminorversion: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddCertTypeToRequestWStrEx: usize,
    pub getProviderTypeWStr: unsafe extern "system" fn(this: *mut *mut Self, pwszprovname: ::windows_sys::core::PCWSTR, plprovtype: *mut i32) -> ::windows_sys::core::HRESULT,
    pub addBlobPropertyToCertificateWStr: unsafe extern "system" fn(this: *mut *mut Self, lpropertyid: i32, lreserved: i32, pblobproperty: *mut super::CRYPTOAPI_BLOB) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSignerCertificate: unsafe extern "system" fn(this: *mut *mut Self, psignercert: *const super::CERT_CONTEXT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSignerCertificate: usize,
    pub SetClientId: unsafe extern "system" fn(this: *mut *mut Self, lclientid: i32) -> ::windows_sys::core::HRESULT,
    pub ClientId: unsafe extern "system" fn(this: *mut *mut Self, plclientid: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIncludeSubjectKeyID: unsafe extern "system" fn(this: *mut *mut Self, finclude: super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIncludeSubjectKeyID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IncludeSubjectKeyID: unsafe extern "system" fn(this: *mut *mut Self, pfinclude: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IncludeSubjectKeyID: usize,
}
impl ::windows_sys::core::Interface for IEnroll4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4161093605, data2: 30964, data3: 17551, data4: [160, 219, 65, 214, 27, 115, 68, 107] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IEnumCERTVIEWATTRIBUTE {
    pub base__: super::super::super::System::Com::IDispatch,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, pindex: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetValue: usize,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: i32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IEnumCERTVIEWATTRIBUTE {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3883775574, data2: 30291, data3: 4561, data4: [155, 222, 0, 192, 79, 182, 131, 250] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IEnumCERTVIEWCOLUMN {
    pub base__: super::super::super::System::Com::IDispatch,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, pindex: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDisplayName: usize,
    pub GetType: unsafe extern "system" fn(this: *mut *mut Self, ptype: *mut i32) -> ::windows_sys::core::HRESULT,
    pub IsIndexed: unsafe extern "system" fn(this: *mut *mut Self, pindexed: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetMaxLength: unsafe extern "system" fn(this: *mut *mut Self, pmaxlength: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, flags: ENUM_CERT_COLUMN_VALUE_FLAGS, pvarvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetValue: usize,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: i32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IEnumCERTVIEWCOLUMN {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2624805858, data2: 22437, data3: 4561, data4: [155, 219, 0, 192, 79, 182, 131, 250] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IEnumCERTVIEWEXTENSION {
    pub base__: super::super::super::System::Com::IDispatch,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, pindex: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetName: unsafe extern "system" fn(this: *mut *mut Self, pstrout: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetName: usize,
    pub GetFlags: unsafe extern "system" fn(this: *mut *mut Self, pflags: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetValue: unsafe extern "system" fn(this: *mut *mut Self, r#type: CERT_PROPERTY_TYPE, flags: ENUM_CERT_COLUMN_VALUE_FLAGS, pvarvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetValue: usize,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: i32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IEnumCERTVIEWEXTENSION {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3890025574, data2: 30291, data3: 4561, data4: [155, 222, 0, 192, 79, 182, 131, 250] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IEnumCERTVIEWROW {
    pub base__: super::super::super::System::Com::IDispatch,
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, pindex: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumCertViewColumn: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumCertViewColumn: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumCertViewAttribute: unsafe extern "system" fn(this: *mut *mut Self, flags: i32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumCertViewAttribute: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumCertViewExtension: unsafe extern "system" fn(this: *mut *mut Self, flags: i32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumCertViewExtension: usize,
    pub Skip: unsafe extern "system" fn(this: *mut *mut Self, celt: i32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
    pub GetMaxIndex: unsafe extern "system" fn(this: *mut *mut Self, pindex: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IEnumCERTVIEWROW {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3507846988, data2: 23282, data3: 4561, data4: [155, 220, 0, 192, 79, 182, 131, 250] };
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const IF_ENABLEADMINASAUDITOR: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const IF_ENABLEEXITKEYRETRIEVAL: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const IF_ENFORCEENCRYPTICERTADMIN: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const IF_ENFORCEENCRYPTICERTREQUEST: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const IF_LOCKICERTREQUEST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const IF_NOLOCALICERTADMIN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const IF_NOLOCALICERTADMINBACKUP: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const IF_NOLOCALICERTREQUEST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const IF_NOREMOTEICERTADMIN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const IF_NOREMOTEICERTADMINBACKUP: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const IF_NOREMOTEICERTREQUEST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const IF_NORPCICERTREQUEST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const IF_NOSNAPSHOTBACKUP: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const IKF_OVERWRITE: u32 = 65536u32;
#[repr(C)]
pub struct INDESPolicy {
    pub base__: ::windows_sys::core::IUnknown,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Uninitialize: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GenerateChallenge: unsafe extern "system" fn(this: *mut *mut Self, pwsztemplate: ::windows_sys::core::PCWSTR, pwszparams: ::windows_sys::core::PCWSTR, ppwszresponse: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub VerifyRequest: unsafe extern "system" fn(this: *mut *mut Self, pctbrequest: *mut CERTTRANSBLOB, pctbsigningcertencoded: *mut CERTTRANSBLOB, pwsztemplate: ::windows_sys::core::PCWSTR, pwsztransactionid: ::windows_sys::core::PCWSTR, pfverified: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    VerifyRequest: usize,
    pub Notify: unsafe extern "system" fn(this: *mut *mut Self, pwszchallenge: ::windows_sys::core::PCWSTR, pwsztransactionid: ::windows_sys::core::PCWSTR, disposition: X509SCEPDisposition, lasthresult: i32, pctbissuedcertencoded: *mut CERTTRANSBLOB) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for INDESPolicy {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 332026205, data2: 17181, data3: 18124, data4: [140, 46, 29, 162, 105, 187, 214, 37] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IOCSPAdmin {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub OCSPServiceProperties: unsafe extern "system" fn(this: *mut *mut Self, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OCSPServiceProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OCSPCAConfigurationCollection: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OCSPCAConfigurationCollection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetConfiguration: unsafe extern "system" fn(this: *mut *mut Self, bstrservername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bforce: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetConfiguration: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetConfiguration: unsafe extern "system" fn(this: *mut *mut Self, bstrservername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bforce: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetConfiguration: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMyRoles: unsafe extern "system" fn(this: *mut *mut Self, bstrservername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, proles: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMyRoles: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Ping: unsafe extern "system" fn(this: *mut *mut Self, bstrservername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Ping: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSecurity: unsafe extern "system" fn(this: *mut *mut Self, bstrservername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrval: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSecurity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSecurity: unsafe extern "system" fn(this: *mut *mut Self, bstrservername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pval: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSecurity: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetSigningCertificates: unsafe extern "system" fn(this: *mut *mut Self, bstrservername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pcacertvar: *const super::super::super::System::Com::VARIANT, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetSigningCertificates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetHashAlgorithms: unsafe extern "system" fn(this: *mut *mut Self, bstrservername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcaid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetHashAlgorithms: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IOCSPAdmin {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 841909005, data2: 26587, data3: 20457, data4: [149, 119, 69, 150, 217, 240, 146, 148] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IOCSPCAConfiguration {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Identifier: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Identifier: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CACertificate: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CACertificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub HashAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HashAlgorithm: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetHashAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, newval: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetHashAlgorithm: usize,
    pub SigningFlags: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetSigningFlags: unsafe extern "system" fn(this: *mut *mut Self, newval: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SigningCertificate: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SigningCertificate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSigningCertificate: unsafe extern "system" fn(this: *mut *mut Self, newval: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSigningCertificate: usize,
    pub ReminderDuration: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetReminderDuration: unsafe extern "system" fn(this: *mut *mut Self, newval: u32) -> ::windows_sys::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CSPName: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CSPName: usize,
    pub KeySpec: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ProviderCLSID: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProviderCLSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetProviderCLSID: unsafe extern "system" fn(this: *mut *mut Self, newval: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetProviderCLSID: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ProviderProperties: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ProviderProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProviderProperties: unsafe extern "system" fn(this: *mut *mut Self, newval: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProviderProperties: usize,
    pub Modified: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LocalRevocationInformation: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LocalRevocationInformation: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetLocalRevocationInformation: unsafe extern "system" fn(this: *mut *mut Self, newval: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetLocalRevocationInformation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SigningCertificateTemplate: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SigningCertificateTemplate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSigningCertificateTemplate: unsafe extern "system" fn(this: *mut *mut Self, newval: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSigningCertificateTemplate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CAConfig: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CAConfig: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCAConfig: unsafe extern "system" fn(this: *mut *mut Self, newval: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCAConfig: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IOCSPCAConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2932419392, data2: 15686, data3: 17215, data4: [135, 209, 184, 77, 92, 30, 121, 13] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IOCSPCAConfigurationCollection {
    pub base__: super::super::super::System::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_ItemByName: unsafe extern "system" fn(this: *mut *mut Self, bstridentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_ItemByName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateCAConfiguration: unsafe extern "system" fn(this: *mut *mut Self, bstridentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varcacert: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateCAConfiguration: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteCAConfiguration: unsafe extern "system" fn(this: *mut *mut Self, bstridentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteCAConfiguration: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IOCSPCAConfigurationCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 736881163, data2: 24270, data3: 20264, data4: [169, 28, 134, 180, 187, 32, 240, 211] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IOCSPProperty {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, newval: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
    pub Modified: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IOCSPProperty {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1727756345, data2: 24324, data3: 19493, data4: [173, 24, 159, 241, 168, 55, 110, 224] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IOCSPPropertyCollection {
    pub base__: super::super::super::System::Com::IDispatch,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_ItemByName: unsafe extern "system" fn(this: *mut *mut Self, bstrpropname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_ItemByName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateProperty: unsafe extern "system" fn(this: *mut *mut Self, bstrpropname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvarpropvalue: *const super::super::super::System::Com::VARIANT, ppval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteProperty: unsafe extern "system" fn(this: *mut *mut Self, bstrpropname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub InitializeFromProperties: unsafe extern "system" fn(this: *mut *mut Self, pvarproperties: *const super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    InitializeFromProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetAllProperties: unsafe extern "system" fn(this: *mut *mut Self, pvarproperties: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetAllProperties: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IOCSPPropertyCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 630702477, data2: 21734, data3: 19316, data4: [159, 169, 166, 191, 218, 153, 203, 190] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IObjectId {
    pub base__: super::super::super::System::Com::IDispatch,
    pub InitializeFromName: unsafe extern "system" fn(this: *mut *mut Self, name: CERTENROLL_OBJECTID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeFromValue: unsafe extern "system" fn(this: *mut *mut Self, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeFromValue: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeFromAlgorithmName: unsafe extern "system" fn(this: *mut *mut Self, groupid: ObjectIdGroupId, keyflags: ObjectIdPublicKeyFlags, algflags: AlgorithmFlags, stralgorithmname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeFromAlgorithmName: usize,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut CERTENROLL_OBJECTID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FriendlyName: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FriendlyName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFriendlyName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Value: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAlgorithmName: unsafe extern "system" fn(this: *mut *mut Self, groupid: ObjectIdGroupId, keyflags: ObjectIdPublicKeyFlags, pstralgorithmname: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAlgorithmName: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IObjectId {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692416, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IObjectIds {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IObjectIds {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692417, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPolicyQualifier {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeEncode: unsafe extern "system" fn(this: *mut *mut Self, strqualifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, r#type: PolicyQualifierType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeEncode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ObjectId: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ObjectId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Qualifier: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Qualifier: usize,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut PolicyQualifierType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_RawData: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_RawData: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IPolicyQualifier {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692444, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPolicyQualifiers {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IPolicyQualifiers {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692445, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ISSCERT_DEFAULT_DS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ISSCERT_DEFAULT_NODS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ISSCERT_ENABLE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ISSCERT_FILEURL_OLD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ISSCERT_FTPURL_OLD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ISSCERT_HTTPURL_OLD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ISSCERT_LDAPURL_OLD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ISSCERT_URLMASK_OLD: u32 = 255u32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISignerCertificate {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, machinecontext: i16, verifytype: X509PrivateKeyVerify, encoding: EncodingType, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_Certificate: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_Certificate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateKey: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateKey: usize,
    pub Silent: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetSilent: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub ParentWindow: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetParentWindow: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UIContextMessage: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UIContextMessage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUIContextMessage: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUIContextMessage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPin: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPin: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SignatureInformation: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SignatureInformation: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ISignerCertificate {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692477, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISignerCertificates {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Find: unsafe extern "system" fn(this: *mut *mut Self, psignercert: *mut ::core::ffi::c_void, pisignercert: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Find: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ISignerCertificates {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692478, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISmimeCapabilities {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddFromCsp: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddFromCsp: usize,
    pub AddAvailableSmimeCapabilities: unsafe extern "system" fn(this: *mut *mut Self, machinecontext: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ISmimeCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692442, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISmimeCapability {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, pobjectid: *mut ::core::ffi::c_void, bitcount: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ObjectId: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ObjectId: usize,
    pub BitCount: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ISmimeCapability {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692441, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX500DistinguishedName {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Decode: unsafe extern "system" fn(this: *mut *mut Self, strencodedname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, nameflags: X500NameFlags) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Decode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Encode: unsafe extern "system" fn(this: *mut *mut Self, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nameflags: X500NameFlags) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Encode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_EncodedName: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_EncodedName: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX500DistinguishedName {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692419, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509Attribute {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, pobjectid: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Initialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ObjectId: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ObjectId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_RawData: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_RawData: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509Attribute {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692450, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509AttributeArchiveKey {
    pub base__: IX509Attribute,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub InitializeEncode: unsafe extern "system" fn(this: *mut *mut Self, pkey: *mut ::core::ffi::c_void, encoding: EncodingType, strcaxcert: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, palgorithm: *mut ::core::ffi::c_void, encryptionstrength: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    InitializeEncode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeDecode: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeDecode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_EncryptedKeyBlob: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_EncryptedKeyBlob: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EncryptionAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EncryptionAlgorithm: usize,
    pub EncryptionStrength: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509AttributeArchiveKey {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692455, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509AttributeArchiveKeyHash {
    pub base__: IX509Attribute,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeEncodeFromEncryptedKeyBlob: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strencryptedkeyblob: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeEncodeFromEncryptedKeyBlob: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeDecode: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeDecode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_EncryptedKeyHashBlob: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_EncryptedKeyHashBlob: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509AttributeArchiveKeyHash {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692456, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509AttributeClientId {
    pub base__: IX509Attribute,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeEncode: unsafe extern "system" fn(this: *mut *mut Self, clientid: RequestClientInfoClientId, strmachinednsname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strusersamname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strprocessname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeEncode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeDecode: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeDecode: usize,
    pub ClientId: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut RequestClientInfoClientId) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MachineDnsName: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MachineDnsName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UserSamName: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserSamName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ProcessName: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProcessName: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509AttributeClientId {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692453, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509AttributeCspProvider {
    pub base__: IX509Attribute,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeEncode: unsafe extern "system" fn(this: *mut *mut Self, keyspec: X509KeySpec, strprovidername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, strsignature: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeEncode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeDecode: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeDecode: usize,
    pub KeySpec: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut X509KeySpec) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ProviderName: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProviderName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_Signature: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_Signature: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509AttributeCspProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692459, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509AttributeExtensions {
    pub base__: IX509Attribute,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeEncode: unsafe extern "system" fn(this: *mut *mut Self, pextensions: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeEncode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeDecode: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeDecode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub X509Extensions: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    X509Extensions: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509AttributeExtensions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692452, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509AttributeOSVersion {
    pub base__: IX509Attribute,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeEncode: unsafe extern "system" fn(this: *mut *mut Self, strosversion: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeEncode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeDecode: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeDecode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OSVersion: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OSVersion: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509AttributeOSVersion {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692458, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509AttributeRenewalCertificate {
    pub base__: IX509Attribute,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeEncode: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strcert: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeEncode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeDecode: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeDecode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_RenewalCertificate: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_RenewalCertificate: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509AttributeRenewalCertificate {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692454, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509Attributes {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509Attributes {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692451, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRequest {
    pub base__: super::super::super::System::Com::IDispatch,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, context: X509CertificateEnrollmentContext) -> ::windows_sys::core::HRESULT,
    pub Encode: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ResetForEncode: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetInnerRequest: unsafe extern "system" fn(this: *mut *mut Self, level: InnerRequestLevel, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetInnerRequest: usize,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut X509RequestType) -> ::windows_sys::core::HRESULT,
    pub EnrollmentContext: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut X509CertificateEnrollmentContext) -> ::windows_sys::core::HRESULT,
    pub Silent: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetSilent: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub ParentWindow: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetParentWindow: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UIContextMessage: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UIContextMessage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUIContextMessage: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUIContextMessage: usize,
    pub SuppressDefaults: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetSuppressDefaults: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_RenewalCertificate: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_RenewalCertificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub put_RenewalCertificate: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_RenewalCertificate: usize,
    pub ClientId: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut RequestClientInfoClientId) -> ::windows_sys::core::HRESULT,
    pub SetClientId: unsafe extern "system" fn(this: *mut *mut Self, value: RequestClientInfoClientId) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CspInformations: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CspInformations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetCspInformations: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetCspInformations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub HashAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    HashAlgorithm: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetHashAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetHashAlgorithm: usize,
    pub AlternateSignatureAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAlternateSignatureAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_RawData: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_RawData: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509CertificateRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692481, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRequestCertificate {
    pub base__: IX509CertificateRequestPkcs10,
    #[cfg(feature = "Win32_System_Com")]
    pub CheckPublicKeySignature: unsafe extern "system" fn(this: *mut *mut Self, ppublickey: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CheckPublicKeySignature: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Issuer: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Issuer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetIssuer: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetIssuer: usize,
    pub NotBefore: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetNotBefore: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub NotAfter: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetNotAfter: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_SerialNumber: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_SerialNumber: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub put_SerialNumber: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_SerialNumber: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SignerCertificate: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SignerCertificate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSignerCertificate: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSignerCertificate: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509CertificateRequestCertificate {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692483, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRequestCertificate2 {
    pub base__: IX509CertificateRequestCertificate,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromTemplate: unsafe extern "system" fn(this: *mut *mut Self, context: X509CertificateEnrollmentContext, ppolicyserver: *mut ::core::ffi::c_void, ptemplate: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromPrivateKeyTemplate: unsafe extern "system" fn(this: *mut *mut Self, context: X509CertificateEnrollmentContext, pprivatekey: *mut ::core::ffi::c_void, ppolicyserver: *mut ::core::ffi::c_void, ptemplate: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromPrivateKeyTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PolicyServer: unsafe extern "system" fn(this: *mut *mut Self, pppolicyserver: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PolicyServer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Template: unsafe extern "system" fn(this: *mut *mut Self, pptemplate: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Template: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509CertificateRequestCertificate2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692506, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRequestCmc {
    pub base__: IX509CertificateRequestPkcs7,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub InitializeFromInnerRequestTemplateName: unsafe extern "system" fn(this: *mut *mut Self, pinnerrequest: *mut ::core::ffi::c_void, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    InitializeFromInnerRequestTemplateName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub TemplateObjectId: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TemplateObjectId: usize,
    pub NullSigned: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CryptAttributes: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CryptAttributes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NameValuePairs: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NameValuePairs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub X509Extensions: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    X509Extensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CriticalExtensions: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CriticalExtensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SuppressOids: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SuppressOids: usize,
    pub TransactionId: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetTransactionId: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_SenderNonce: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_SenderNonce: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub put_SenderNonce: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_SenderNonce: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SignatureInformation: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SignatureInformation: usize,
    pub ArchivePrivateKey: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetArchivePrivateKey: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_KeyArchivalCertificate: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_KeyArchivalCertificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub put_KeyArchivalCertificate: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_KeyArchivalCertificate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EncryptionAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EncryptionAlgorithm: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetEncryptionAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetEncryptionAlgorithm: usize,
    pub EncryptionStrength: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetEncryptionStrength: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_EncryptedKeyHash: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_EncryptedKeyHash: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SignerCertificates: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SignerCertificates: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509CertificateRequestCmc {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692485, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRequestCmc2 {
    pub base__: IX509CertificateRequestCmc,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromTemplate: unsafe extern "system" fn(this: *mut *mut Self, context: X509CertificateEnrollmentContext, ppolicyserver: *mut ::core::ffi::c_void, ptemplate: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromInnerRequestTemplate: unsafe extern "system" fn(this: *mut *mut Self, pinnerrequest: *mut ::core::ffi::c_void, ppolicyserver: *mut ::core::ffi::c_void, ptemplate: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromInnerRequestTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PolicyServer: unsafe extern "system" fn(this: *mut *mut Self, pppolicyserver: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PolicyServer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Template: unsafe extern "system" fn(this: *mut *mut Self, pptemplate: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Template: usize,
    pub CheckSignature: unsafe extern "system" fn(this: *mut *mut Self, allowedsignaturetypes: Pkcs10AllowedSignatureTypes) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CheckCertificateSignature: unsafe extern "system" fn(this: *mut *mut Self, psignercertificate: *mut ::core::ffi::c_void, validatecertificatechain: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CheckCertificateSignature: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509CertificateRequestCmc2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692509, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRequestPkcs10 {
    pub base__: IX509CertificateRequest,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeFromTemplateName: unsafe extern "system" fn(this: *mut *mut Self, context: X509CertificateEnrollmentContext, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeFromTemplateName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub InitializeFromPrivateKey: unsafe extern "system" fn(this: *mut *mut Self, context: X509CertificateEnrollmentContext, pprivatekey: *mut ::core::ffi::c_void, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    InitializeFromPrivateKey: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub InitializeFromPublicKey: unsafe extern "system" fn(this: *mut *mut Self, context: X509CertificateEnrollmentContext, ppublickey: *mut ::core::ffi::c_void, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    InitializeFromPublicKey: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeFromCertificate: unsafe extern "system" fn(this: *mut *mut Self, context: X509CertificateEnrollmentContext, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, inheritoptions: X509RequestInheritOptions) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeFromCertificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeDecode: unsafe extern "system" fn(this: *mut *mut Self, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeDecode: usize,
    pub CheckSignature: unsafe extern "system" fn(this: *mut *mut Self, allowedsignaturetypes: Pkcs10AllowedSignatureTypes) -> ::windows_sys::core::HRESULT,
    pub IsSmartCard: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub TemplateObjectId: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TemplateObjectId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PublicKey: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PublicKey: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateKey: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateKey: usize,
    pub NullSigned: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub ReuseKey: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_OldCertificate: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_OldCertificate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Subject: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Subject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSubject: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSubject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CspStatuses: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CspStatuses: usize,
    pub SmimeCapabilities: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetSmimeCapabilities: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SignatureInformation: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SignatureInformation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub KeyContainerNamePrefix: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    KeyContainerNamePrefix: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetKeyContainerNamePrefix: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetKeyContainerNamePrefix: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CryptAttributes: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CryptAttributes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub X509Extensions: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    X509Extensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CriticalExtensions: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CriticalExtensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SuppressOids: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SuppressOids: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_RawDataToBeSigned: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_RawDataToBeSigned: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_Signature: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_Signature: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCspStatuses: unsafe extern "system" fn(this: *mut *mut Self, keyspec: X509KeySpec, ppcspstatuses: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCspStatuses: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509CertificateRequestPkcs10 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692482, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRequestPkcs10V2 {
    pub base__: IX509CertificateRequestPkcs10,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromTemplate: unsafe extern "system" fn(this: *mut *mut Self, context: X509CertificateEnrollmentContext, ppolicyserver: *mut ::core::ffi::c_void, ptemplate: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromPrivateKeyTemplate: unsafe extern "system" fn(this: *mut *mut Self, context: X509CertificateEnrollmentContext, pprivatekey: *mut ::core::ffi::c_void, ppolicyserver: *mut ::core::ffi::c_void, ptemplate: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromPrivateKeyTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromPublicKeyTemplate: unsafe extern "system" fn(this: *mut *mut Self, context: X509CertificateEnrollmentContext, ppublickey: *mut ::core::ffi::c_void, ppolicyserver: *mut ::core::ffi::c_void, ptemplate: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromPublicKeyTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PolicyServer: unsafe extern "system" fn(this: *mut *mut Self, pppolicyserver: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PolicyServer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Template: unsafe extern "system" fn(this: *mut *mut Self, pptemplate: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Template: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509CertificateRequestPkcs10V2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692507, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRequestPkcs10V3 {
    pub base__: IX509CertificateRequestPkcs10V2,
    pub AttestPrivateKey: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAttestPrivateKey: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_AttestationEncryptionCertificate: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_AttestationEncryptionCertificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub put_AttestationEncryptionCertificate: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_AttestationEncryptionCertificate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EncryptionAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EncryptionAlgorithm: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetEncryptionAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetEncryptionAlgorithm: usize,
    pub EncryptionStrength: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetEncryptionStrength: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ChallengePassword: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ChallengePassword: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetChallengePassword: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetChallengePassword: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NameValuePairs: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NameValuePairs: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509CertificateRequestPkcs10V3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1424660802, data2: 15718, data3: 17712, data4: [183, 110, 124, 145, 112, 211, 236, 82] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRequestPkcs10V4 {
    pub base__: IX509CertificateRequestPkcs10V3,
    pub ClaimType: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut KeyAttestationClaimType) -> ::windows_sys::core::HRESULT,
    pub SetClaimType: unsafe extern "system" fn(this: *mut *mut Self, value: KeyAttestationClaimType) -> ::windows_sys::core::HRESULT,
    pub AttestPrivateKeyPreferred: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAttestPrivateKeyPreferred: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509CertificateRequestPkcs10V4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692515, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRequestPkcs7 {
    pub base__: IX509CertificateRequest,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeFromTemplateName: unsafe extern "system" fn(this: *mut *mut Self, context: X509CertificateEnrollmentContext, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeFromTemplateName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeFromCertificate: unsafe extern "system" fn(this: *mut *mut Self, context: X509CertificateEnrollmentContext, renewalrequest: i16, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, inheritoptions: X509RequestInheritOptions) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeFromCertificate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromInnerRequest: unsafe extern "system" fn(this: *mut *mut Self, pinnerrequest: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromInnerRequest: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeDecode: unsafe extern "system" fn(this: *mut *mut Self, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeDecode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RequesterName: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequesterName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRequesterName: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRequesterName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SignerCertificate: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SignerCertificate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSignerCertificate: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSignerCertificate: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509CertificateRequestPkcs7 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692484, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRequestPkcs7V2 {
    pub base__: IX509CertificateRequestPkcs7,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromTemplate: unsafe extern "system" fn(this: *mut *mut Self, context: X509CertificateEnrollmentContext, ppolicyserver: *mut ::core::ffi::c_void, ptemplate: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PolicyServer: unsafe extern "system" fn(this: *mut *mut Self, pppolicyserver: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PolicyServer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Template: unsafe extern "system" fn(this: *mut *mut Self, pptemplate: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Template: usize,
    pub CheckCertificateSignature: unsafe extern "system" fn(this: *mut *mut Self, validatecertificatechain: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509CertificateRequestPkcs7V2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692508, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRevocationList {
    pub base__: super::super::super::System::Com::IDispatch,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeDecode: unsafe extern "system" fn(this: *mut *mut Self, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeDecode: usize,
    pub Encode: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ResetForEncode: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CheckPublicKeySignature: unsafe extern "system" fn(this: *mut *mut Self, ppublickey: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CheckPublicKeySignature: usize,
    pub CheckSignature: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Issuer: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Issuer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetIssuer: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetIssuer: usize,
    pub ThisUpdate: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetThisUpdate: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub NextUpdate: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetNextUpdate: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub X509CRLEntries: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    X509CRLEntries: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub X509Extensions: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    X509Extensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CriticalExtensions: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CriticalExtensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SignerCertificate: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SignerCertificate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSignerCertificate: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSignerCertificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_CRLNumber: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_CRLNumber: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub put_CRLNumber: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_CRLNumber: usize,
    pub CAVersion: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetCAVersion: unsafe extern "system" fn(this: *mut *mut Self, pvalue: i32) -> ::windows_sys::core::HRESULT,
    pub BaseCRL: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub NullSigned: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub HashAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    HashAlgorithm: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetHashAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetHashAlgorithm: usize,
    pub AlternateSignatureAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAlternateSignatureAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SignatureInformation: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SignatureInformation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_RawData: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_RawData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_RawDataToBeSigned: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_RawDataToBeSigned: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_Signature: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_Signature: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509CertificateRevocationList {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692512, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRevocationListEntries {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_IndexBySerialNumber: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, serialnumber: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pindex: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_IndexBySerialNumber: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509CertificateRevocationListEntries {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692511, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateRevocationListEntry {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, serialnumber: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, revocationdate: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_SerialNumber: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_SerialNumber: usize,
    pub RevocationDate: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut f64) -> ::windows_sys::core::HRESULT,
    pub RevocationReason: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut CRLRevocationReason) -> ::windows_sys::core::HRESULT,
    pub SetRevocationReason: unsafe extern "system" fn(this: *mut *mut Self, value: CRLRevocationReason) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub X509Extensions: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    X509Extensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CriticalExtensions: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CriticalExtensions: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509CertificateRevocationListEntry {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692510, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateTemplate {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Property: unsafe extern "system" fn(this: *mut *mut Self, property: EnrollmentTemplateProperty, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Property: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509CertificateTemplate {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1411664403, data2: 21850, data3: 20002, data4: [137, 109, 27, 14, 82, 247, 100, 6] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateTemplateWritable {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Commit: unsafe extern "system" fn(this: *mut *mut Self, commitflags: CommitTemplateFlags, strservercontext: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Commit: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Property: unsafe extern "system" fn(this: *mut *mut Self, property: EnrollmentTemplateProperty, pvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Property: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub put_Property: unsafe extern "system" fn(this: *mut *mut Self, property: EnrollmentTemplateProperty, value: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    put_Property: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Template: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Template: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509CertificateTemplateWritable {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4103366311, data2: 14682, data3: 20126, data4: [182, 231, 50, 179, 49, 96, 13, 192] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509CertificateTemplates {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub get_ItemByName: unsafe extern "system" fn(this: *mut *mut Self, bstrname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    get_ItemByName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByOid: unsafe extern "system" fn(this: *mut *mut Self, poid: *mut ::core::ffi::c_void, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByOid: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509CertificateTemplates {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 330797059, data2: 8577, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509EndorsementKey {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub ProviderName: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProviderName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetProviderName: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetProviderName: usize,
    pub Length: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Opened: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddCertificate: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddCertificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveCertificate: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strcertificate: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveCertificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCertificateByIndex: unsafe extern "system" fn(this: *mut *mut Self, manufactureronly: i16, dwindex: i32, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCertificateByIndex: usize,
    pub GetCertificateCount: unsafe extern "system" fn(this: *mut *mut Self, manufactureronly: i16, pcount: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ExportPublicKey: unsafe extern "system" fn(this: *mut *mut Self, pppublickey: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExportPublicKey: usize,
    pub Open: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509EndorsementKey {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2971457621, data2: 62660, data3: 20422, data4: [183, 16, 68, 34, 35, 127, 9, 233] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509Enrollment {
    pub base__: super::super::super::System::Com::IDispatch,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, context: X509CertificateEnrollmentContext) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeFromTemplateName: unsafe extern "system" fn(this: *mut *mut Self, context: X509CertificateEnrollmentContext, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeFromTemplateName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromRequest: unsafe extern "system" fn(this: *mut *mut Self, prequest: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromRequest: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateRequest: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateRequest: usize,
    pub Enroll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InstallResponse: unsafe extern "system" fn(this: *mut *mut Self, restrictions: InstallResponseRestrictionFlags, strresponse: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstallResponse: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreatePFX: unsafe extern "system" fn(this: *mut *mut Self, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, exportoptions: PFXExportOptions, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreatePFX: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Request: usize,
    pub Silent: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetSilent: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub ParentWindow: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetParentWindow: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub NameValuePairs: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NameValuePairs: usize,
    pub EnrollmentContext: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut X509CertificateEnrollmentContext) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Status: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_Certificate: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_Certificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_Response: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_Response: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CertificateFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CertificateFriendlyName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCertificateFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCertificateFriendlyName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CertificateDescription: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CertificateDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCertificateDescription: unsafe extern "system" fn(this: *mut *mut Self, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCertificateDescription: usize,
    pub RequestId: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CAConfigString: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CAConfigString: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509Enrollment {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692486, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509Enrollment2 {
    pub base__: IX509Enrollment,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromTemplate: unsafe extern "system" fn(this: *mut *mut Self, context: X509CertificateEnrollmentContext, ppolicyserver: *mut ::core::ffi::c_void, ptemplate: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromTemplate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InstallResponse2: unsafe extern "system" fn(this: *mut *mut Self, restrictions: InstallResponseRestrictionFlags, strresponse: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strenrollmentpolicyserverurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strenrollmentpolicyserverid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, enrollmentpolicyserverflags: PolicyServerUrlFlags, authflags: X509EnrollmentAuthFlags) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstallResponse2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PolicyServer: unsafe extern "system" fn(this: *mut *mut Self, pppolicyserver: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PolicyServer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Template: unsafe extern "system" fn(this: *mut *mut Self, pptemplate: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Template: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestIdString: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestIdString: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509Enrollment2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692496, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509EnrollmentHelper {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub AddPolicyServer: unsafe extern "system" fn(this: *mut *mut Self, strenrollmentpolicyserveruri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strenrollmentpolicyid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, enrollmentpolicyserverflags: PolicyServerUrlFlags, authflags: X509EnrollmentAuthFlags, strcredential: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddPolicyServer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AddEnrollmentServer: unsafe extern "system" fn(this: *mut *mut Self, strenrollmentserveruri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, authflags: X509EnrollmentAuthFlags, strcredential: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddEnrollmentServer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Enroll: unsafe extern "system" fn(this: *mut *mut Self, strenrollmentpolicyserveruri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, enrollflags: WebEnrollmentFlags, pstrcertificate: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enroll: usize,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, context: X509CertificateEnrollmentContext) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509EnrollmentHelper {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692497, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509EnrollmentPolicyServer {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, bstrpolicyserverurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrpolicyserverid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, authflags: X509EnrollmentAuthFlags, fisuntrusted: i16, context: X509CertificateEnrollmentContext) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub LoadPolicy: unsafe extern "system" fn(this: *mut *mut Self, option: X509EnrollmentPolicyLoadOption) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTemplates: unsafe extern "system" fn(this: *mut *mut Self, ptemplates: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTemplates: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCAsForTemplate: unsafe extern "system" fn(this: *mut *mut Self, ptemplate: *mut ::core::ffi::c_void, ppcas: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCAsForTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCAs: unsafe extern "system" fn(this: *mut *mut Self, ppcas: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCAs: usize,
    pub Validate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCustomOids: unsafe extern "system" fn(this: *mut *mut Self, ppobjectids: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCustomOids: usize,
    pub GetNextUpdateTime: unsafe extern "system" fn(this: *mut *mut Self, pdate: *mut f64) -> ::windows_sys::core::HRESULT,
    pub GetLastUpdateTime: unsafe extern "system" fn(this: *mut *mut Self, pdate: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPolicyServerUrl: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPolicyServerUrl: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPolicyServerId: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPolicyServerId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFriendlyName: usize,
    pub GetIsDefaultCEP: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub GetUseClientId: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub GetAllowUnTrustedCA: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCachePath: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCachePath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCacheDir: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCacheDir: usize,
    pub GetAuthFlags: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut X509EnrollmentAuthFlags) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCredential: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: i32, flag: X509EnrollmentAuthFlags, strcredential: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCredential: usize,
    pub QueryChanges: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub InitializeImport: unsafe extern "system" fn(this: *mut *mut Self, val: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    InitializeImport: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Export: unsafe extern "system" fn(this: *mut *mut Self, exportflags: X509EnrollmentPolicyExportFlags, pval: *mut super::super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Export: usize,
    pub Cost: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetCost: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509EnrollmentPolicyServer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 330797094, data2: 8577, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509EnrollmentStatus {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub AppendText: unsafe extern "system" fn(this: *mut *mut Self, strtext: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AppendText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Text: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetText: usize,
    pub Selected: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut EnrollmentSelectionStatus) -> ::windows_sys::core::HRESULT,
    pub SetSelected: unsafe extern "system" fn(this: *mut *mut Self, value: EnrollmentSelectionStatus) -> ::windows_sys::core::HRESULT,
    pub Display: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut EnrollmentDisplayStatus) -> ::windows_sys::core::HRESULT,
    pub SetDisplay: unsafe extern "system" fn(this: *mut *mut Self, value: EnrollmentDisplayStatus) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut EnrollmentEnrollStatus) -> ::windows_sys::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut *mut Self, value: EnrollmentEnrollStatus) -> ::windows_sys::core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub SetError: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ErrorText: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ErrorText: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509EnrollmentStatus {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692420, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509EnrollmentWebClassFactory {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateObject: unsafe extern "system" fn(this: *mut *mut Self, strprogid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppiunknown: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateObject: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509EnrollmentWebClassFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692489, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509Extension {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, pobjectid: *mut ::core::ffi::c_void, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Initialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ObjectId: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ObjectId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_RawData: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_RawData: usize,
    pub Critical: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetCritical: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509Extension {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692429, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509ExtensionAlternativeNames {
    pub base__: IX509Extension,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeEncode: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeEncode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeDecode: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeDecode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AlternativeNames: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AlternativeNames: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509ExtensionAlternativeNames {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692437, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509ExtensionAuthorityKeyIdentifier {
    pub base__: IX509Extension,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeEncode: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strkeyidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeEncode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeDecode: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeDecode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_AuthorityKeyIdentifier: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_AuthorityKeyIdentifier: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509ExtensionAuthorityKeyIdentifier {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692440, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509ExtensionBasicConstraints {
    pub base__: IX509Extension,
    pub InitializeEncode: unsafe extern "system" fn(this: *mut *mut Self, isca: i16, pathlenconstraint: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeDecode: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeDecode: usize,
    pub IsCA: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub PathLenConstraint: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509ExtensionBasicConstraints {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692438, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509ExtensionCertificatePolicies {
    pub base__: IX509Extension,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeEncode: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeEncode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeDecode: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeDecode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Policies: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Policies: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509ExtensionCertificatePolicies {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692448, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509ExtensionEnhancedKeyUsage {
    pub base__: IX509Extension,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeEncode: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeEncode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeDecode: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeDecode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnhancedKeyUsage: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnhancedKeyUsage: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509ExtensionEnhancedKeyUsage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692432, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509ExtensionKeyUsage {
    pub base__: IX509Extension,
    pub InitializeEncode: unsafe extern "system" fn(this: *mut *mut Self, usageflags: X509KeyUsageFlags) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeDecode: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeDecode: usize,
    pub KeyUsage: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut X509KeyUsageFlags) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509ExtensionKeyUsage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692431, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509ExtensionMSApplicationPolicies {
    pub base__: IX509Extension,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeEncode: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeEncode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeDecode: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeDecode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Policies: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Policies: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509ExtensionMSApplicationPolicies {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692449, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509ExtensionSmimeCapabilities {
    pub base__: IX509Extension,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeEncode: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeEncode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeDecode: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeDecode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SmimeCapabilities: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SmimeCapabilities: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509ExtensionSmimeCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692443, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509ExtensionSubjectKeyIdentifier {
    pub base__: IX509Extension,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeEncode: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strkeyidentifier: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeEncode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeDecode: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeDecode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_SubjectKeyIdentifier: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_SubjectKeyIdentifier: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509ExtensionSubjectKeyIdentifier {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692439, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509ExtensionTemplate {
    pub base__: IX509Extension,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeEncode: unsafe extern "system" fn(this: *mut *mut Self, ptemplateoid: *mut ::core::ffi::c_void, majorversion: i32, minorversion: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeEncode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeDecode: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeDecode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub TemplateOid: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TemplateOid: usize,
    pub MajorVersion: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509ExtensionTemplate {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692434, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509ExtensionTemplateName {
    pub base__: IX509Extension,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeEncode: unsafe extern "system" fn(this: *mut *mut Self, strtemplatename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeEncode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeDecode: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, strencodeddata: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeDecode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TemplateName: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TemplateName: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509ExtensionTemplateName {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692433, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509Extensions {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_IndexByObjectId: unsafe extern "system" fn(this: *mut *mut Self, pobjectid: *mut ::core::ffi::c_void, pindex: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_IndexByObjectId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRange: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRange: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509Extensions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692430, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509MachineEnrollmentFactory {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateObject: unsafe extern "system" fn(this: *mut *mut Self, strprogid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppihelper: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateObject: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509MachineEnrollmentFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692498, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509NameValuePair {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Value: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509NameValuePair {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692479, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509NameValuePairs {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509NameValuePairs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692480, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509PolicyServerListManager {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub get_ItemByIndex: unsafe extern "system" fn(this: *mut *mut Self, index: i32, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_ItemByIndex: usize,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, index: i32) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, context: X509CertificateEnrollmentContext, flags: PolicyServerUrlFlags) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509PolicyServerListManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821451, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509PolicyServerUrl {
    pub base__: super::super::super::System::Com::IDispatch,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, context: X509CertificateEnrollmentContext) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Url: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Url: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUrl: unsafe extern "system" fn(this: *mut *mut Self, pvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUrl: usize,
    pub Default: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetDefault: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub Flags: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut PolicyServerUrlFlags) -> ::windows_sys::core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(this: *mut *mut Self, flags: PolicyServerUrlFlags) -> ::windows_sys::core::HRESULT,
    pub AuthFlags: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut X509EnrollmentAuthFlags) -> ::windows_sys::core::HRESULT,
    pub SetAuthFlags: unsafe extern "system" fn(this: *mut *mut Self, flags: X509EnrollmentAuthFlags) -> ::windows_sys::core::HRESULT,
    pub Cost: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetCost: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetStringProperty: unsafe extern "system" fn(this: *mut *mut Self, propertyid: PolicyServerUrlPropertyID, ppvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetStringProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStringProperty: unsafe extern "system" fn(this: *mut *mut Self, propertyid: PolicyServerUrlPropertyID, pvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStringProperty: usize,
    pub UpdateRegistry: unsafe extern "system" fn(this: *mut *mut Self, context: X509CertificateEnrollmentContext) -> ::windows_sys::core::HRESULT,
    pub RemoveFromRegistry: unsafe extern "system" fn(this: *mut *mut Self, context: X509CertificateEnrollmentContext) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509PolicyServerUrl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2286821450, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509PrivateKey {
    pub base__: super::super::super::System::Com::IDispatch,
    pub Open: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Verify: unsafe extern "system" fn(this: *mut *mut Self, verifytype: X509PrivateKeyVerify) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Import: unsafe extern "system" fn(this: *mut *mut Self, strexporttype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strencodedkey: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Import: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Export: unsafe extern "system" fn(this: *mut *mut Self, strexporttype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, pstrencodedkey: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Export: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ExportPublicKey: unsafe extern "system" fn(this: *mut *mut Self, pppublickey: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExportPublicKey: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ContainerName: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ContainerName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetContainerName: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetContainerName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ContainerNamePrefix: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ContainerNamePrefix: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetContainerNamePrefix: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetContainerNamePrefix: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReaderName: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReaderName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReaderName: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReaderName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CspInformations: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CspInformations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetCspInformations: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetCspInformations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CspStatus: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CspStatus: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetCspStatus: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetCspStatus: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ProviderName: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProviderName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetProviderName: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetProviderName: usize,
    pub ProviderType: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut X509ProviderType) -> ::windows_sys::core::HRESULT,
    pub SetProviderType: unsafe extern "system" fn(this: *mut *mut Self, value: X509ProviderType) -> ::windows_sys::core::HRESULT,
    pub LegacyCsp: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetLegacyCsp: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Algorithm: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Algorithm: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetAlgorithm: usize,
    pub KeySpec: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut X509KeySpec) -> ::windows_sys::core::HRESULT,
    pub SetKeySpec: unsafe extern "system" fn(this: *mut *mut Self, value: X509KeySpec) -> ::windows_sys::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetLength: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub ExportPolicy: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut X509PrivateKeyExportFlags) -> ::windows_sys::core::HRESULT,
    pub SetExportPolicy: unsafe extern "system" fn(this: *mut *mut Self, value: X509PrivateKeyExportFlags) -> ::windows_sys::core::HRESULT,
    pub KeyUsage: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut X509PrivateKeyUsageFlags) -> ::windows_sys::core::HRESULT,
    pub SetKeyUsage: unsafe extern "system" fn(this: *mut *mut Self, value: X509PrivateKeyUsageFlags) -> ::windows_sys::core::HRESULT,
    pub KeyProtection: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut X509PrivateKeyProtection) -> ::windows_sys::core::HRESULT,
    pub SetKeyProtection: unsafe extern "system" fn(this: *mut *mut Self, value: X509PrivateKeyProtection) -> ::windows_sys::core::HRESULT,
    pub MachineContext: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetMachineContext: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SecurityDescriptor: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SecurityDescriptor: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSecurityDescriptor: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSecurityDescriptor: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_Certificate: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_Certificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub put_Certificate: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_Certificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UniqueContainerName: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UniqueContainerName: usize,
    pub Opened: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub DefaultContainer: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub Existing: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetExisting: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub Silent: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetSilent: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub ParentWindow: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetParentWindow: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UIContextMessage: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UIContextMessage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUIContextMessage: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUIContextMessage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPin: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPin: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FriendlyName: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FriendlyName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFriendlyName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509PrivateKey {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692428, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509PrivateKey2 {
    pub base__: IX509PrivateKey,
    pub HardwareKeyUsage: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut X509HardwareKeyUsageFlags) -> ::windows_sys::core::HRESULT,
    pub SetHardwareKeyUsage: unsafe extern "system" fn(this: *mut *mut Self, value: X509HardwareKeyUsageFlags) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AlternateStorageLocation: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AlternateStorageLocation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAlternateStorageLocation: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAlternateStorageLocation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AlgorithmName: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AlgorithmName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAlgorithmName: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAlgorithmName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_AlgorithmParameters: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_AlgorithmParameters: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub put_AlgorithmParameters: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_AlgorithmParameters: usize,
    pub ParametersExportType: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut X509KeyParametersExportType) -> ::windows_sys::core::HRESULT,
    pub SetParametersExportType: unsafe extern "system" fn(this: *mut *mut Self, value: X509KeyParametersExportType) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509PrivateKey2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692514, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509PublicKey {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, pobjectid: *mut ::core::ffi::c_void, strencodedkey: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strencodedparameters: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Initialize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeFromEncodedPublicKeyInfo: unsafe extern "system" fn(this: *mut *mut Self, strencodedpublickeyinfo: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeFromEncodedPublicKeyInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Algorithm: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Algorithm: usize,
    pub Length: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_EncodedKey: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_EncodedKey: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_EncodedParameters: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_EncodedParameters: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ComputeKeyIdentifier: unsafe extern "system" fn(this: *mut *mut Self, algorithm: KeyIdentifierHashAlgorithm, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ComputeKeyIdentifier: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509PublicKey {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692427, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509SCEPEnrollment {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, prequest: *mut ::core::ffi::c_void, strthumbprint: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, thumprintencoding: EncodingType, strservercertificates: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Initialize: usize,
    pub InitializeForPending: unsafe extern "system" fn(this: *mut *mut Self, context: X509CertificateEnrollmentContext) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateRequestMessage: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateRequestMessage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateRetrievePendingMessage: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateRetrievePendingMessage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateRetrieveCertificateMessage: unsafe extern "system" fn(this: *mut *mut Self, context: X509CertificateEnrollmentContext, strissuer: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, issuerencoding: EncodingType, strserialnumber: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, serialnumberencoding: EncodingType, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateRetrieveCertificateMessage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ProcessResponseMessage: unsafe extern "system" fn(this: *mut *mut Self, strresponse: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, pdisposition: *mut X509SCEPDisposition) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProcessResponseMessage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetServerCapabilities: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetServerCapabilities: usize,
    pub FailInfo: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut X509SCEPFailInfo) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SignerCertificate: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SignerCertificate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSignerCertificate: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSignerCertificate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OldCertificate: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OldCertificate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetOldCertificate: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetOldCertificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_TransactionId: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_TransactionId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub put_TransactionId: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_TransactionId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Request: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CertificateFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CertificateFriendlyName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCertificateFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCertificateFriendlyName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Status: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_Certificate: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_Certificate: usize,
    pub Silent: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetSilent: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub DeleteRequest: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509SCEPEnrollment {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692513, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509SCEPEnrollment2 {
    pub base__: IX509SCEPEnrollment,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateChallengeAnswerMessage: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateChallengeAnswerMessage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ProcessResponseMessage2: unsafe extern "system" fn(this: *mut *mut Self, flags: X509SCEPProcessMessageFlags, strresponse: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, encoding: EncodingType, pdisposition: *mut X509SCEPDisposition) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProcessResponseMessage2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ResultMessageText: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ResultMessageText: usize,
    pub DelayRetry: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut DelayRetryAction) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ActivityId: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ActivityId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetActivityId: unsafe extern "system" fn(this: *mut *mut Self, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetActivityId: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509SCEPEnrollment2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692516, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509SCEPEnrollmentHelper {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, strserverurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strrequestheaders: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, prequest: *mut ::core::ffi::c_void, strcacertificatethumbprint: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Initialize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeForPending: unsafe extern "system" fn(this: *mut *mut Self, strserverurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strrequestheaders: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, context: X509CertificateEnrollmentContext, strtransactionid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeForPending: usize,
    pub Enroll: unsafe extern "system" fn(this: *mut *mut Self, processflags: X509SCEPProcessMessageFlags, pdisposition: *mut X509SCEPDisposition) -> ::windows_sys::core::HRESULT,
    pub FetchPending: unsafe extern "system" fn(this: *mut *mut Self, processflags: X509SCEPProcessMessageFlags, pdisposition: *mut X509SCEPDisposition) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub X509SCEPEnrollment: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    X509SCEPEnrollment: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ResultMessageText: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ResultMessageText: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509SCEPEnrollmentHelper {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692517, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IX509SignatureInformation {
    pub base__: super::super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub HashAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    HashAlgorithm: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetHashAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetHashAlgorithm: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PublicKeyAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PublicKeyAlgorithm: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPublicKeyAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPublicKeyAlgorithm: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_Parameters: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_Parameters: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub put_Parameters: unsafe extern "system" fn(this: *mut *mut Self, encoding: EncodingType, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_Parameters: usize,
    pub AlternateSignatureAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAlternateSignatureAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub AlternateSignatureAlgorithmSet: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub NullSigned: unsafe extern "system" fn(this: *mut *mut Self, pvalue: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetNullSigned: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSignatureAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, pkcs7signature: i16, signaturekey: i16, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSignatureAlgorithm: usize,
    pub SetDefaultValues: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IX509SignatureInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1921692476, data2: 8573, data3: 4570, data4: [178, 164, 0, 14, 123, 187, 43, 9] };
}
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type ImportPFXFlags = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ImportNone: ImportPFXFlags = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ImportMachineContext: ImportPFXFlags = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ImportForceOverwrite: ImportPFXFlags = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ImportSilent: ImportPFXFlags = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ImportSaveProperties: ImportPFXFlags = 8i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ImportExportable: ImportPFXFlags = 16i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ImportExportableEncrypted: ImportPFXFlags = 32i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ImportNoUserProtected: ImportPFXFlags = 64i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ImportUserProtected: ImportPFXFlags = 128i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ImportUserProtectedHigh: ImportPFXFlags = 256i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ImportInstallCertificate: ImportPFXFlags = 512i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ImportInstallChain: ImportPFXFlags = 1024i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ImportInstallChainAndRoot: ImportPFXFlags = 2048i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type InnerRequestLevel = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const LevelInnermost: InnerRequestLevel = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const LevelNext: InnerRequestLevel = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type InstallResponseRestrictionFlags = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const AllowNone: InstallResponseRestrictionFlags = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const AllowNoOutstandingRequest: InstallResponseRestrictionFlags = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const AllowUntrustedCertificate: InstallResponseRestrictionFlags = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const AllowUntrustedRoot: InstallResponseRestrictionFlags = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const KRAF_DISABLEUSEDEFAULTPROVIDER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const KRAF_ENABLEARCHIVEALL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const KRAF_ENABLEFOREIGN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const KRAF_SAVEBADREQUESTKEY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const KRA_DISP_EXPIRED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const KRA_DISP_INVALID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const KRA_DISP_NOTFOUND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const KRA_DISP_NOTLOADED: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const KRA_DISP_REVOKED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const KRA_DISP_UNTRUSTED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const KRA_DISP_VALID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const KR_ENABLE_MACHINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const KR_ENABLE_USER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type KeyAttestationClaimType = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_CLAIM_NONE: KeyAttestationClaimType = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_CLAIM_AUTHORITY_AND_SUBJECT: KeyAttestationClaimType = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_CLAIM_AUTHORITY_ONLY: KeyAttestationClaimType = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_CLAIM_SUBJECT_ONLY: KeyAttestationClaimType = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_CLAIM_UNKNOWN: KeyAttestationClaimType = 4096i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type KeyIdentifierHashAlgorithm = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SKIHashDefault: KeyIdentifierHashAlgorithm = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SKIHashSha1: KeyIdentifierHashAlgorithm = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SKIHashCapiSha1: KeyIdentifierHashAlgorithm = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SKIHashSha256: KeyIdentifierHashAlgorithm = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SKIHashHPKP: KeyIdentifierHashAlgorithm = 5i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const LDAPF_SIGNDISABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const LDAPF_SSLENABLE: u32 = 1u32;
pub const OCSPAdmin: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3556193553, data2: 37577, data3: 18379, data4: [143, 242, 141, 137, 26, 124, 77, 228] };
pub const OCSPPropertyCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4181042472, data2: 47754, data3: 19929, data4: [186, 121, 242, 131, 39, 92, 178, 222] };
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type OCSPRequestFlag = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const OCSP_RF_REJECT_SIGNED_REQUESTS: OCSPRequestFlag = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type OCSPSigningFlag = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const OCSP_SF_SILENT: OCSPSigningFlag = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const OCSP_SF_USE_CACERT: OCSPSigningFlag = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const OCSP_SF_ALLOW_SIGNINGCERT_AUTORENEWAL: OCSPSigningFlag = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const OCSP_SF_FORCE_SIGNINGCERT_ISSUER_ISCA: OCSPSigningFlag = 8i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const OCSP_SF_AUTODISCOVER_SIGNINGCERT: OCSPSigningFlag = 16i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const OCSP_SF_MANUAL_ASSIGN_SIGNINGCERT: OCSPSigningFlag = 32i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const OCSP_SF_RESPONDER_ID_KEYHASH: OCSPSigningFlag = 64i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const OCSP_SF_RESPONDER_ID_NAME: OCSPSigningFlag = 128i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const OCSP_SF_ALLOW_NONCE_EXTENSION: OCSPSigningFlag = 256i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const OCSP_SF_ALLOW_SIGNINGCERT_AUTOENROLLMENT: OCSPSigningFlag = 512i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type ObjectIdGroupId = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_ANY_GROUP_ID: ObjectIdGroupId = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_HASH_ALG_OID_GROUP_ID: ObjectIdGroupId = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_ENCRYPT_ALG_OID_GROUP_ID: ObjectIdGroupId = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_PUBKEY_ALG_OID_GROUP_ID: ObjectIdGroupId = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_SIGN_ALG_OID_GROUP_ID: ObjectIdGroupId = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_RDN_ATTR_OID_GROUP_ID: ObjectIdGroupId = 5i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_EXT_OR_ATTR_OID_GROUP_ID: ObjectIdGroupId = 6i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_ENHKEY_USAGE_OID_GROUP_ID: ObjectIdGroupId = 7i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_POLICY_OID_GROUP_ID: ObjectIdGroupId = 8i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_TEMPLATE_OID_GROUP_ID: ObjectIdGroupId = 9i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_KDF_OID_GROUP_ID: ObjectIdGroupId = 10i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_LAST_OID_GROUP_ID: ObjectIdGroupId = 10i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_FIRST_ALG_OID_GROUP_ID: ObjectIdGroupId = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_LAST_ALG_OID_GROUP_ID: ObjectIdGroupId = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_GROUP_ID_MASK: ObjectIdGroupId = 65535i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_OID_PREFER_CNG_ALGID_FLAG: ObjectIdGroupId = 1073741824i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_OID_DISABLE_SEARCH_DS_FLAG: ObjectIdGroupId = -2147483648i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_OID_INFO_OID_GROUP_BIT_LEN_MASK: ObjectIdGroupId = 268369920i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_OID_INFO_OID_GROUP_BIT_LEN_SHIFT: ObjectIdGroupId = 16i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_KEY_LENGTH_MASK: ObjectIdGroupId = 268369920i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type ObjectIdPublicKeyFlags = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_OID_INFO_PUBKEY_ANY: ObjectIdPublicKeyFlags = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_OID_INFO_PUBKEY_SIGN_KEY_FLAG: ObjectIdPublicKeyFlags = -2147483648i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_OID_INFO_PUBKEY_ENCRYPT_KEY_FLAG: ObjectIdPublicKeyFlags = 1073741824i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type PENDING_REQUEST_DESIRED_PROPERTY = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XEPR_CADNS: PENDING_REQUEST_DESIRED_PROPERTY = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XEPR_CAFRIENDLYNAME: PENDING_REQUEST_DESIRED_PROPERTY = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XEPR_CANAME: PENDING_REQUEST_DESIRED_PROPERTY = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XEPR_HASH: PENDING_REQUEST_DESIRED_PROPERTY = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XEPR_REQUESTID: PENDING_REQUEST_DESIRED_PROPERTY = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type PFXExportOptions = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PFXExportEEOnly: PFXExportOptions = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PFXExportChainNoRoot: PFXExportOptions = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PFXExportChainWithRoot: PFXExportOptions = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PROCFLG_ENFORCEGOODKEYS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PROCFLG_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PROPCALLER_ADMIN: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PROPCALLER_EXIT: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PROPCALLER_MASK: u32 = 3840u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PROPCALLER_POLICY: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PROPCALLER_REQUEST: u32 = 1280u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PROPCALLER_SERVER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PROPFLAGS_INDEXED: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PROPTYPE_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type Pkcs10AllowedSignatureTypes = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const AllowedKeySignature: Pkcs10AllowedSignatureTypes = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const AllowedNullSignature: Pkcs10AllowedSignatureTypes = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type PolicyQualifierType = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PolicyQualifierTypeUnknown: PolicyQualifierType = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PolicyQualifierTypeUrl: PolicyQualifierType = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PolicyQualifierTypeUserNotice: PolicyQualifierType = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PolicyQualifierTypeFlags: PolicyQualifierType = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type PolicyServerUrlFlags = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PsfNone: PolicyServerUrlFlags = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PsfLocationGroupPolicy: PolicyServerUrlFlags = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PsfLocationRegistry: PolicyServerUrlFlags = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PsfUseClientId: PolicyServerUrlFlags = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PsfAutoEnrollmentEnabled: PolicyServerUrlFlags = 16i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PsfAllowUnTrustedCA: PolicyServerUrlFlags = 32i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type PolicyServerUrlPropertyID = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PsPolicyID: PolicyServerUrlPropertyID = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PsFriendlyName: PolicyServerUrlPropertyID = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const REQDISP_DEFAULT_ENTERPRISE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const REQDISP_DENY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const REQDISP_ISSUE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const REQDISP_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const REQDISP_PENDING: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const REQDISP_PENDINGFIRST: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const REQDISP_USEREQUESTATTRIBUTE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const REVEXT_ASPENABLE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const REVEXT_CDPENABLE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const REVEXT_CDPFILEURL_OLD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const REVEXT_CDPFTPURL_OLD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const REVEXT_CDPHTTPURL_OLD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const REVEXT_CDPLDAPURL_OLD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const REVEXT_CDPURLMASK_OLD: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const REVEXT_DEFAULT_DS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const REVEXT_DEFAULT_NODS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type RequestClientInfoClientId = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ClientIdNone: RequestClientInfoClientId = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ClientIdXEnroll2003: RequestClientInfoClientId = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ClientIdAutoEnroll2003: RequestClientInfoClientId = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ClientIdWizard2003: RequestClientInfoClientId = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ClientIdCertReq2003: RequestClientInfoClientId = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ClientIdDefaultRequest: RequestClientInfoClientId = 5i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ClientIdAutoEnroll: RequestClientInfoClientId = 6i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ClientIdRequestWizard: RequestClientInfoClientId = 7i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ClientIdEOBO: RequestClientInfoClientId = 8i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ClientIdCertReq: RequestClientInfoClientId = 9i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ClientIdTest: RequestClientInfoClientId = 10i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ClientIdWinRT: RequestClientInfoClientId = 11i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ClientIdUserStart: RequestClientInfoClientId = 1000i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SETUP_ATTEMPT_VROOT_CREATE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SETUP_CLIENT_FLAG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SETUP_CREATEDB_FLAG: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SETUP_DCOM_SECURITY_UPDATED_FLAG: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SETUP_DENIED_FLAG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SETUP_FORCECRL_FLAG: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SETUP_ONLINE_FLAG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SETUP_REQUEST_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SETUP_SECURITY_CHANGED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SETUP_SERVER_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SETUP_SERVER_IS_UP_TO_DATE_FLAG: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SETUP_SERVER_UPGRADED_FLAG: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SETUP_SUSPEND_FLAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SETUP_UPDATE_CAOBJECT_SVRTYPE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SETUP_W2K_SECURITY_NOT_UPGRADED_FLAG: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TP_MACHINEPOLICY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const VR_INSTANT_BAD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const VR_INSTANT_OK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const VR_PENDING: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type WebEnrollmentFlags = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollPrompt: WebEnrollmentFlags = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type WebSecurityLevel = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const LevelUnsafe: WebSecurityLevel = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const LevelSafe: WebSecurityLevel = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type X500NameFlags = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NAME_STR_NONE: X500NameFlags = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_SIMPLE_NAME_STR: X500NameFlags = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_OID_NAME_STR: X500NameFlags = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_X500_NAME_STR: X500NameFlags = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_XML_NAME_STR: X500NameFlags = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NAME_STR_SEMICOLON_FLAG: X500NameFlags = 1073741824i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NAME_STR_NO_PLUS_FLAG: X500NameFlags = 536870912i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NAME_STR_NO_QUOTING_FLAG: X500NameFlags = 268435456i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NAME_STR_CRLF_FLAG: X500NameFlags = 134217728i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NAME_STR_COMMA_FLAG: X500NameFlags = 67108864i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NAME_STR_REVERSE_FLAG: X500NameFlags = 33554432i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NAME_STR_FORWARD_FLAG: X500NameFlags = 16777216i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NAME_STR_AMBIGUOUS_SEPARATOR_FLAGS: X500NameFlags = 1275068416i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NAME_STR_DISABLE_IE4_UTF8_FLAG: X500NameFlags = 65536i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NAME_STR_ENABLE_T61_UNICODE_FLAG: X500NameFlags = 131072i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NAME_STR_ENABLE_UTF8_UNICODE_FLAG: X500NameFlags = 262144i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NAME_STR_FORCE_UTF8_DIR_STR_FLAG: X500NameFlags = 524288i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NAME_STR_DISABLE_UTF8_DIR_STR_FLAG: X500NameFlags = 1048576i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NAME_STR_ENABLE_PUNYCODE_FLAG: X500NameFlags = 2097152i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NAME_STR_DS_ESCAPED: X500NameFlags = 8388608i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type X509CertificateEnrollmentContext = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ContextNone: X509CertificateEnrollmentContext = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ContextUser: X509CertificateEnrollmentContext = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ContextMachine: X509CertificateEnrollmentContext = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ContextAdministratorForceMachine: X509CertificateEnrollmentContext = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type X509CertificateTemplateEnrollmentFlag = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollmentIncludeSymmetricAlgorithms: X509CertificateTemplateEnrollmentFlag = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollmentPendAllRequests: X509CertificateTemplateEnrollmentFlag = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollmentPublishToKRAContainer: X509CertificateTemplateEnrollmentFlag = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollmentPublishToDS: X509CertificateTemplateEnrollmentFlag = 8i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollmentAutoEnrollmentCheckUserDSCertificate: X509CertificateTemplateEnrollmentFlag = 16i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollmentAutoEnrollment: X509CertificateTemplateEnrollmentFlag = 32i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollmentDomainAuthenticationNotRequired: X509CertificateTemplateEnrollmentFlag = 128i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollmentPreviousApprovalValidateReenrollment: X509CertificateTemplateEnrollmentFlag = 64i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollmentUserInteractionRequired: X509CertificateTemplateEnrollmentFlag = 256i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollmentAddTemplateName: X509CertificateTemplateEnrollmentFlag = 512i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollmentRemoveInvalidCertificateFromPersonalStore: X509CertificateTemplateEnrollmentFlag = 1024i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollmentAllowEnrollOnBehalfOf: X509CertificateTemplateEnrollmentFlag = 2048i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollmentAddOCSPNoCheck: X509CertificateTemplateEnrollmentFlag = 4096i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollmentReuseKeyOnFullSmartCard: X509CertificateTemplateEnrollmentFlag = 8192i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollmentNoRevocationInfoInCerts: X509CertificateTemplateEnrollmentFlag = 16384i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollmentIncludeBasicConstraintsForEECerts: X509CertificateTemplateEnrollmentFlag = 32768i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollmentPreviousApprovalKeyBasedValidateReenrollment: X509CertificateTemplateEnrollmentFlag = 65536i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollmentCertificateIssuancePoliciesFromRequest: X509CertificateTemplateEnrollmentFlag = 131072i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const EnrollmentSkipAutoRenewal: X509CertificateTemplateEnrollmentFlag = 262144i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type X509CertificateTemplateGeneralFlag = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const GeneralMachineType: X509CertificateTemplateGeneralFlag = 64i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const GeneralCA: X509CertificateTemplateGeneralFlag = 128i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const GeneralCrossCA: X509CertificateTemplateGeneralFlag = 2048i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const GeneralDefault: X509CertificateTemplateGeneralFlag = 65536i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const GeneralModified: X509CertificateTemplateGeneralFlag = 131072i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const GeneralDonotPersist: X509CertificateTemplateGeneralFlag = 4096i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type X509CertificateTemplatePrivateKeyFlag = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PrivateKeyRequireArchival: X509CertificateTemplatePrivateKeyFlag = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PrivateKeyExportable: X509CertificateTemplatePrivateKeyFlag = 16i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PrivateKeyRequireStrongKeyProtection: X509CertificateTemplatePrivateKeyFlag = 32i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PrivateKeyRequireAlternateSignatureAlgorithm: X509CertificateTemplatePrivateKeyFlag = 64i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PrivateKeyRequireSameKeyRenewal: X509CertificateTemplatePrivateKeyFlag = 128i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PrivateKeyUseLegacyProvider: X509CertificateTemplatePrivateKeyFlag = 256i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PrivateKeyEKTrustOnUse: X509CertificateTemplatePrivateKeyFlag = 512i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PrivateKeyEKValidateCert: X509CertificateTemplatePrivateKeyFlag = 1024i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PrivateKeyEKValidateKey: X509CertificateTemplatePrivateKeyFlag = 2048i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PrivateKeyAttestNone: X509CertificateTemplatePrivateKeyFlag = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PrivateKeyAttestPreferred: X509CertificateTemplatePrivateKeyFlag = 4096i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PrivateKeyAttestRequired: X509CertificateTemplatePrivateKeyFlag = 8192i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PrivateKeyAttestMask: X509CertificateTemplatePrivateKeyFlag = 12288i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PrivateKeyAttestWithoutPolicy: X509CertificateTemplatePrivateKeyFlag = 16384i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PrivateKeyServerVersionMask: X509CertificateTemplatePrivateKeyFlag = 983040i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PrivateKeyServerVersionShift: X509CertificateTemplatePrivateKeyFlag = 16i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PrivateKeyHelloKspKey: X509CertificateTemplatePrivateKeyFlag = 1048576i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PrivateKeyHelloLogonKey: X509CertificateTemplatePrivateKeyFlag = 2097152i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PrivateKeyClientVersionMask: X509CertificateTemplatePrivateKeyFlag = 251658240i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const PrivateKeyClientVersionShift: X509CertificateTemplatePrivateKeyFlag = 24i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type X509CertificateTemplateSubjectNameFlag = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SubjectNameEnrolleeSupplies: X509CertificateTemplateSubjectNameFlag = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SubjectNameRequireDirectoryPath: X509CertificateTemplateSubjectNameFlag = -2147483648i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SubjectNameRequireCommonName: X509CertificateTemplateSubjectNameFlag = 1073741824i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SubjectNameRequireEmail: X509CertificateTemplateSubjectNameFlag = 536870912i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SubjectNameRequireDNS: X509CertificateTemplateSubjectNameFlag = 268435456i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SubjectNameAndAlternativeNameOldCertSupplies: X509CertificateTemplateSubjectNameFlag = 8i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SubjectAlternativeNameEnrolleeSupplies: X509CertificateTemplateSubjectNameFlag = 65536i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SubjectAlternativeNameRequireDirectoryGUID: X509CertificateTemplateSubjectNameFlag = 16777216i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SubjectAlternativeNameRequireUPN: X509CertificateTemplateSubjectNameFlag = 33554432i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SubjectAlternativeNameRequireEmail: X509CertificateTemplateSubjectNameFlag = 67108864i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SubjectAlternativeNameRequireSPN: X509CertificateTemplateSubjectNameFlag = 8388608i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SubjectAlternativeNameRequireDNS: X509CertificateTemplateSubjectNameFlag = 134217728i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SubjectAlternativeNameRequireDomainDNS: X509CertificateTemplateSubjectNameFlag = 4194304i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type X509EnrollmentAuthFlags = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const X509AuthNone: X509EnrollmentAuthFlags = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const X509AuthAnonymous: X509EnrollmentAuthFlags = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const X509AuthKerberos: X509EnrollmentAuthFlags = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const X509AuthUsername: X509EnrollmentAuthFlags = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const X509AuthCertificate: X509EnrollmentAuthFlags = 8i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type X509EnrollmentPolicyExportFlags = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ExportTemplates: X509EnrollmentPolicyExportFlags = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ExportOIDs: X509EnrollmentPolicyExportFlags = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const ExportCAs: X509EnrollmentPolicyExportFlags = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type X509EnrollmentPolicyLoadOption = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const LoadOptionDefault: X509EnrollmentPolicyLoadOption = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const LoadOptionCacheOnly: X509EnrollmentPolicyLoadOption = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const LoadOptionReload: X509EnrollmentPolicyLoadOption = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const LoadOptionRegisterForADChanges: X509EnrollmentPolicyLoadOption = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type X509HardwareKeyUsageFlags = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_PCP_NONE: X509HardwareKeyUsageFlags = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_TPM12_PROVIDER: X509HardwareKeyUsageFlags = 65536i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_PCP_SIGNATURE_KEY: X509HardwareKeyUsageFlags = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_PCP_ENCRYPTION_KEY: X509HardwareKeyUsageFlags = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_PCP_GENERIC_KEY: X509HardwareKeyUsageFlags = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_PCP_STORAGE_KEY: X509HardwareKeyUsageFlags = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_PCP_IDENTITY_KEY: X509HardwareKeyUsageFlags = 8i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type X509KeyParametersExportType = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_OID_USE_CURVE_NONE: X509KeyParametersExportType = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_OID_USE_CURVE_NAME_FOR_ENCODE_FLAG: X509KeyParametersExportType = 536870912i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CRYPT_OID_USE_CURVE_PARAMETERS_FOR_ENCODE_FLAG: X509KeyParametersExportType = 268435456i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type X509KeySpec = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_AT_NONE: X509KeySpec = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_AT_KEYEXCHANGE: X509KeySpec = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_AT_SIGNATURE: X509KeySpec = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type X509KeyUsageFlags = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NO_KEY_USAGE: X509KeyUsageFlags = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_DIGITAL_SIGNATURE_KEY_USAGE: X509KeyUsageFlags = 128i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_NON_REPUDIATION_KEY_USAGE: X509KeyUsageFlags = 64i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_KEY_ENCIPHERMENT_KEY_USAGE: X509KeyUsageFlags = 32i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_DATA_ENCIPHERMENT_KEY_USAGE: X509KeyUsageFlags = 16i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_KEY_AGREEMENT_KEY_USAGE: X509KeyUsageFlags = 8i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_KEY_CERT_SIGN_KEY_USAGE: X509KeyUsageFlags = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_OFFLINE_CRL_SIGN_KEY_USAGE: X509KeyUsageFlags = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_CRL_SIGN_KEY_USAGE: X509KeyUsageFlags = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_ENCIPHER_ONLY_KEY_USAGE: X509KeyUsageFlags = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_CERT_DECIPHER_ONLY_KEY_USAGE: X509KeyUsageFlags = 32768i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type X509PrivateKeyExportFlags = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_ALLOW_EXPORT_NONE: X509PrivateKeyExportFlags = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_ALLOW_EXPORT_FLAG: X509PrivateKeyExportFlags = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_ALLOW_PLAINTEXT_EXPORT_FLAG: X509PrivateKeyExportFlags = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_ALLOW_ARCHIVING_FLAG: X509PrivateKeyExportFlags = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_ALLOW_PLAINTEXT_ARCHIVING_FLAG: X509PrivateKeyExportFlags = 8i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type X509PrivateKeyProtection = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_UI_NO_PROTECTION_FLAG: X509PrivateKeyProtection = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_UI_PROTECT_KEY_FLAG: X509PrivateKeyProtection = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_UI_FORCE_HIGH_PROTECTION_FLAG: X509PrivateKeyProtection = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_UI_FINGERPRINT_PROTECTION_FLAG: X509PrivateKeyProtection = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_UI_APPCONTAINER_ACCESS_MEDIUM_FLAG: X509PrivateKeyProtection = 8i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type X509PrivateKeyUsageFlags = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_ALLOW_USAGES_NONE: X509PrivateKeyUsageFlags = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_ALLOW_DECRYPT_FLAG: X509PrivateKeyUsageFlags = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_ALLOW_SIGNING_FLAG: X509PrivateKeyUsageFlags = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_ALLOW_KEY_AGREEMENT_FLAG: X509PrivateKeyUsageFlags = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_ALLOW_KEY_IMPORT_FLAG: X509PrivateKeyUsageFlags = 8i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_NCRYPT_ALLOW_ALL_USAGES: X509PrivateKeyUsageFlags = 16777215i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type X509PrivateKeyVerify = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const VerifyNone: X509PrivateKeyVerify = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const VerifySilent: X509PrivateKeyVerify = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const VerifySmartCardNone: X509PrivateKeyVerify = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const VerifySmartCardSilent: X509PrivateKeyVerify = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const VerifyAllowUI: X509PrivateKeyVerify = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type X509ProviderType = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_PROV_NONE: X509ProviderType = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_PROV_RSA_FULL: X509ProviderType = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_PROV_RSA_SIG: X509ProviderType = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_PROV_DSS: X509ProviderType = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_PROV_FORTEZZA: X509ProviderType = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_PROV_MS_EXCHANGE: X509ProviderType = 5i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_PROV_SSL: X509ProviderType = 6i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_PROV_RSA_SCHANNEL: X509ProviderType = 12i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_PROV_DSS_DH: X509ProviderType = 13i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_PROV_EC_ECDSA_SIG: X509ProviderType = 14i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_PROV_EC_ECNRA_SIG: X509ProviderType = 15i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_PROV_EC_ECDSA_FULL: X509ProviderType = 16i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_PROV_EC_ECNRA_FULL: X509ProviderType = 17i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_PROV_DH_SCHANNEL: X509ProviderType = 18i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_PROV_SPYRUS_LYNKS: X509ProviderType = 20i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_PROV_RNG: X509ProviderType = 21i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_PROV_INTEL_SEC: X509ProviderType = 22i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_PROV_REPLACE_OWF: X509ProviderType = 23i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XCN_PROV_RSA_AES: X509ProviderType = 24i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type X509RequestInheritOptions = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const InheritDefault: X509RequestInheritOptions = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const InheritNewDefaultKey: X509RequestInheritOptions = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const InheritNewSimilarKey: X509RequestInheritOptions = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const InheritPrivateKey: X509RequestInheritOptions = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const InheritPublicKey: X509RequestInheritOptions = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const InheritKeyMask: X509RequestInheritOptions = 15i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const InheritNone: X509RequestInheritOptions = 16i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const InheritRenewalCertificateFlag: X509RequestInheritOptions = 32i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const InheritTemplateFlag: X509RequestInheritOptions = 64i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const InheritSubjectFlag: X509RequestInheritOptions = 128i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const InheritExtensionsFlag: X509RequestInheritOptions = 256i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const InheritSubjectAltNameFlag: X509RequestInheritOptions = 512i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const InheritValidityPeriodFlag: X509RequestInheritOptions = 1024i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const InheritReserved80000000: X509RequestInheritOptions = -2147483648i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type X509RequestType = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TypeAny: X509RequestType = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TypePkcs10: X509RequestType = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TypePkcs7: X509RequestType = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TypeCmc: X509RequestType = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const TypeCertificate: X509RequestType = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type X509SCEPDisposition = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SCEPDispositionUnknown: X509SCEPDisposition = -1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SCEPDispositionSuccess: X509SCEPDisposition = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SCEPDispositionFailure: X509SCEPDisposition = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SCEPDispositionPending: X509SCEPDisposition = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SCEPDispositionPendingChallenge: X509SCEPDisposition = 11i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type X509SCEPFailInfo = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SCEPFailUnknown: X509SCEPFailInfo = -1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SCEPFailBadAlgorithm: X509SCEPFailInfo = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SCEPFailBadMessageCheck: X509SCEPFailInfo = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SCEPFailBadRequest: X509SCEPFailInfo = 2i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SCEPFailBadTime: X509SCEPFailInfo = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SCEPFailBadCertId: X509SCEPFailInfo = 4i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type X509SCEPMessageType = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SCEPMessageUnknown: X509SCEPMessageType = -1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SCEPMessageCertResponse: X509SCEPMessageType = 3i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SCEPMessagePKCSRequest: X509SCEPMessageType = 19i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SCEPMessageGetCertInitial: X509SCEPMessageType = 20i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SCEPMessageGetCert: X509SCEPMessageType = 21i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SCEPMessageGetCRL: X509SCEPMessageType = 22i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SCEPMessageClaimChallengeAnswer: X509SCEPMessageType = 41i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type X509SCEPProcessMessageFlags = i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SCEPProcessDefault: X509SCEPProcessMessageFlags = 0i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const SCEPProcessSkipCertInstall: X509SCEPProcessMessageFlags = 1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XECI_AUTOENROLL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XECI_CERTREQ: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XECI_DISABLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XECI_REQWIZARD: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XECI_XENROLL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XECP_STRING_PROPERTY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type XEKL_KEYSIZE = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XEKL_KEYSIZE_MIN: XEKL_KEYSIZE = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XEKL_KEYSIZE_MAX: XEKL_KEYSIZE = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XEKL_KEYSIZE_INC: XEKL_KEYSIZE = 3u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XEKL_KEYSIZE_DEFAULT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub type XEKL_KEYSPEC = u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XEKL_KEYSPEC_KEYX: XEKL_KEYSPEC = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XEKL_KEYSPEC_SIG: XEKL_KEYSPEC = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XEPR_DATE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XEPR_ENUM_FIRST: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XEPR_TEMPLATENAME: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XEPR_V1TEMPLATENAME: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XEPR_V2TEMPLATEOID: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const XEPR_VERSION: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const dwCAXCHGOVERLAPPERIODCOUNTDEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const dwCAXCHGVALIDITYPERIODCOUNTDEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const dwCRLDELTAOVERLAPPERIODCOUNTDEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const dwCRLDELTAPERIODCOUNTDEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const dwCRLOVERLAPPERIODCOUNTDEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const dwCRLPERIODCOUNTDEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const dwVALIDITYPERIODCOUNTDEFAULT_ENTERPRISE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const dwVALIDITYPERIODCOUNTDEFAULT_ROOT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const dwVALIDITYPERIODCOUNTDEFAULT_STANDALONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const szBACKUPANNOTATION: &str = "Cert Server Backup Interface";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const szDBBASENAMEPARM: &str = "edb";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const szNAMESEPARATORDEFAULT: &str = "\n";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const szPROPASNTAG: &str = "{asn}";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const szRESTOREANNOTATION: &str = "Cert Server Restore Interface";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszAT_EKCERTINF: &str = "@EKCert";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszAT_TESTROOT: &str = "@TestRoot";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCAPOLICYFILE: &str = "CAPolicy.inf";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCERTEXITMODULE_POSTFIX: &str = ".Exit";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCERTIFICATETRANSPARENCYFLAGS: &str = "CertificateTransparencyFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCERTMANAGE_SUFFIX: &str = "Manage";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCERTPOLICYMODULE_POSTFIX: &str = ".Policy";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCERT_TYPE: &str = "RequestType";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCERT_TYPE_CLIENT: &str = "Client";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCERT_TYPE_CODESIGN: &str = "CodeSign";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCERT_TYPE_CUSTOMER: &str = "SetCustomer";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCERT_TYPE_MERCHANT: &str = "SetMerchant";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCERT_TYPE_PAYMENT: &str = "SetPayment";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCERT_TYPE_SERVER: &str = "Server";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCERT_VERSION: &str = "Version";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCERT_VERSION_1: &str = "1";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCERT_VERSION_2: &str = "2";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCERT_VERSION_3: &str = "3";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCLASS_CERTADMIN: &str = "CertificateAuthority.Admin";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCLASS_CERTCONFIG: &str = "CertificateAuthority.Config";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCLASS_CERTDBMEM: &str = "CertificateAuthority.DBMem";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCLASS_CERTENCODE: &str = "CertificateAuthority.Encode";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCLASS_CERTGETCONFIG: &str = "CertificateAuthority.GetConfig";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCLASS_CERTREQUEST: &str = "CertificateAuthority.Request";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCLASS_CERTSERVEREXIT: &str = "CertificateAuthority.ServerExit";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCLASS_CERTSERVERPOLICY: &str = "CertificateAuthority.ServerPolicy";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCLASS_CERTVIEW: &str = "CertificateAuthority.View";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCMM_PROP_COPYRIGHT: &str = "Copyright";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCMM_PROP_DESCRIPTION: &str = "Description";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCMM_PROP_DISPLAY_HWND: &str = "HWND";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCMM_PROP_FILEVER: &str = "File Version";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCMM_PROP_ISMULTITHREADED: &str = "IsMultiThreaded";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCMM_PROP_NAME: &str = "Name";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCMM_PROP_PRODUCTVER: &str = "Product Version";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCNGENCRYPTIONALGORITHM: &str = "CNGEncryptionAlgorithm";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCNGHASHALGORITHM: &str = "CNGHashAlgorithm";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCNGPUBLICKEYALGORITHM: &str = "CNGPublicKeyAlgorithm";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCONFIG_AUTHORITY: &str = "Authority";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCONFIG_COMMENT: &str = "Comment";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCONFIG_COMMONNAME: &str = "CommonName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCONFIG_CONFIG: &str = "Config";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCONFIG_COUNTRY: &str = "Country";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCONFIG_DESCRIPTION: &str = "Description";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCONFIG_EXCHANGECERTIFICATE: &str = "ExchangeCertificate";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCONFIG_FLAGS: &str = "Flags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCONFIG_LOCALITY: &str = "Locality";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCONFIG_ORGANIZATION: &str = "Organization";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCONFIG_ORGUNIT: &str = "OrgUnit";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCONFIG_SANITIZEDNAME: &str = "SanitizedName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCONFIG_SANITIZEDSHORTNAME: &str = "SanitizedShortName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCONFIG_SERVER: &str = "Server";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCONFIG_SHORTNAME: &str = "ShortName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCONFIG_SIGNATURECERTIFICATE: &str = "SignatureCertificate";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCONFIG_STATE: &str = "State";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCONFIG_WEBENROLLMENTSERVERS: &str = "WebEnrollmentServers";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCRLPUBLISHRETRYCOUNT: &str = "CRLPublishRetryCount";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszCRTFILENAMEEXT: &str = ".crt";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszDATFILENAMEEXT: &str = ".dat";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszDBBACKUPCERTBACKDAT: &str = "certbkxp.dat";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszDBBACKUPSUBDIR: &str = "DataBase";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszDBFILENAMEEXT: &str = ".edb";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszENCRYPTIONALGORITHM: &str = "EncryptionAlgorithm";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszENROLLMENTAGENTRIGHTS: &str = "EnrollmentAgentRights";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszHASHALGORITHM: &str = "HashAlgorithm";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_ALTERNATESIGNATUREALGORITHM: &str = "AlternateSignatureAlgorithm";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_ATTESTPRIVATEKEY: &str = "AttestPrivateKey";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_CACAPABILITIES: &str = "CACapabilities";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_CACERTS: &str = "CACerts";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_CATHUMBPRINT: &str = "CAThumbprint";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_CCDPSYNCDELTATIME: &str = "SyncDeltaTime";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_CHALLENGEPASSWORD: &str = "ChallengePassword";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_CONTINUE: &str = "_continue_";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_CRITICAL: &str = "Critical";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_CRLDELTAPERIODCOUNT: &str = "CRLDeltaPeriodUnits";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_CRLDELTAPERIODSTRING: &str = "CRLDeltaPeriod";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_CRLPERIODCOUNT: &str = "CRLPeriodUnits";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_CRLPERIODSTRING: &str = "CRLPeriod";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_DIRECTORYNAME: &str = "DirectoryName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_DNS: &str = "DNS";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_ECCKEYPARAMETERS: &str = "EccKeyParameters";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_ECCKEYPARAMETERSTYPE: &str = "EccKeyParametersType";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_ECCKEYPARAMETERS_A: &str = "EccKeyParameters_A";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_ECCKEYPARAMETERS_B: &str = "EccKeyParameters_B";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_ECCKEYPARAMETERS_BASE: &str = "EccKeyParameters_Base";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_ECCKEYPARAMETERS_COFACTOR: &str = "EccKeyParameters_Cofactor";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_ECCKEYPARAMETERS_ORDER: &str = "EccKeyParameters_Order";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_ECCKEYPARAMETERS_P: &str = "EccKeyParameters_P";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_ECCKEYPARAMETERS_SEED: &str = "EccKeyParameters_Seed";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_EMAIL: &str = "EMail";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_EMPTY: &str = "Empty";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_ENABLEKEYCOUNTING: &str = "EnableKeyCounting";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_ENCRYPTIONALGORITHM: &str = "EncryptionAlgorithm";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_ENCRYPTIONLENGTH: &str = "EncryptionLength";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_EXCLUDE: &str = "Exclude";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_EXPORTABLE: &str = "Exportable";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_EXPORTABLEENCRYPTED: &str = "ExportableEncrypted";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_FLAGS: &str = "Flags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_FORCEUTF8: &str = "ForceUTF8";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_FRIENDLYNAME: &str = "FriendlyName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_HASHALGORITHM: &str = "HashAlgorithm";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_INCLUDE: &str = "Include";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_INHIBITPOLICYMAPPING: &str = "InhibitPolicyMapping";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_IPADDRESS: &str = "IPAddress";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_KEYALGORITHM: &str = "KeyAlgorithm";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_KEYALGORITHMPARMETERS: &str = "KeyAlgorithmParameters";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_KEYCONTAINER: &str = "KeyContainer";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_KEYLENGTH: &str = "KeyLength";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_KEYPROTECTION: &str = "KeyProtection";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_KEYUSAGEEXTENSION: &str = "KeyUsage";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_KEYUSAGEPROPERTY: &str = "KeyUsageProperty";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_LEGACYKEYSPEC: &str = "KeySpec";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_LOADDEFAULTTEMPLATES: &str = "LoadDefaultTemplates";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_MACHINEKEYSET: &str = "MachineKeySet";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_NOTAFTER: &str = "NotAfter";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_NOTBEFORE: &str = "NotBefore";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_NOTICE: &str = "Notice";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_OID: &str = "OID";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_OTHERNAME: &str = "OtherName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_PATHLENGTH: &str = "PathLength";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_POLICIES: &str = "Policies";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_PRIVATEKEYARCHIVE: &str = "PrivateKeyArchive";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_PROVIDERNAME: &str = "ProviderName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_PROVIDERTYPE: &str = "ProviderType";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_PUBLICKEY: &str = "PublicKey";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_PUBLICKEYPARAMETERS: &str = "PublicKeyParameters";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_READERNAME: &str = "ReaderName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_REGISTEREDID: &str = "RegisteredId";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_RENEWALCERT: &str = "RenewalCert";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_RENEWALKEYLENGTH: &str = "RenewalKeyLength";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_RENEWALVALIDITYPERIODCOUNT: &str = "RenewalValidityPeriodUnits";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_RENEWALVALIDITYPERIODSTRING: &str = "RenewalValidityPeriod";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_REQUESTTYPE: &str = "RequestType";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_REQUIREEXPLICITPOLICY: &str = "RequireExplicitPolicy";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_SECURITYDESCRIPTOR: &str = "SecurityDescriptor";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_SERIALNUMBER: &str = "SerialNumber";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_SHOWALLCSPS: &str = "ShowAllCSPs";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_SILENT: &str = "Silent";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_SMIME: &str = "SMIME";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_SUBJECT: &str = "Subject";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_SUBJECTNAMEFLAGS: &str = "SubjectNameFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_SUBTREE: &str = "SubTree";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_SUPPRESSDEFAULTS: &str = "SuppressDefaults";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_UICONTEXTMESSAGE: &str = "UIContextMessage";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_UPN: &str = "UPN";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_URL: &str = "URL";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_USEEXISTINGKEY: &str = "UseExistingKeySet";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_USERPROTECTED: &str = "UserProtected";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_UTF8: &str = "UTF8";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFKEY_X500NAMEFLAGS: &str = "X500NameFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFSECTION_AIA: &str = "AuthorityInformationAccess";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFSECTION_APPLICATIONPOLICYCONSTRAINTS: &str = "ApplicationPolicyConstraintsExtension";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFSECTION_APPLICATIONPOLICYMAPPINGS: &str = "ApplicationPolicyMappingsExtension";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFSECTION_APPLICATIONPOLICYSTATEMENT: &str = "ApplicationPolicyStatementExtension";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFSECTION_BASICCONSTRAINTS: &str = "BasicConstraintsExtension";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFSECTION_CAPOLICY: &str = "CAPolicy";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFSECTION_CCDP: &str = "CrossCertificateDistributionPointsExtension";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFSECTION_CDP: &str = "CRLDistributionPoint";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFSECTION_CERTSERVER: &str = "certsrv_server";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFSECTION_EKU: &str = "EnhancedKeyUsageExtension";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFSECTION_EXTENSIONS: &str = "Extensions";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFSECTION_NAMECONSTRAINTS: &str = "NameConstraintsExtension";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFSECTION_NEWREQUEST: &str = "NewRequest";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFSECTION_POLICYCONSTRAINTS: &str = "PolicyConstraintsExtension";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFSECTION_POLICYMAPPINGS: &str = "PolicyMappingsExtension";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFSECTION_POLICYSTATEMENT: &str = "PolicyStatementExtension";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFSECTION_PROPERTIES: &str = "Properties";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFSECTION_REQUESTATTRIBUTES: &str = "RequestAttributes";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFVALUE_ENDORSEMENTKEY: &str = "EndorsementKey";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFVALUE_REQUESTTYPE_CERT: &str = "Cert";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFVALUE_REQUESTTYPE_CMC: &str = "CMC";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFVALUE_REQUESTTYPE_PKCS10: &str = "PKCS10";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFVALUE_REQUESTTYPE_PKCS7: &str = "PKCS7";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszINFVALUE_REQUESTTYPE_SCEP: &str = "SCEP";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszLDAPSESSIONOPTIONVALUE: &str = "LDAPSessionOptionValue";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszLOCALIZEDTIMEPERIODUNITS: &str = "LocalizedTimePeriodUnits";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszLOGFILENAMEEXT: &str = ".log";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszLOGPATH: &str = "CertLog";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszMACHINEKEYSET: &str = "MachineKeyset";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszMICROSOFTCERTMODULE_PREFIX: &str = "CertificateAuthority_MicrosoftDefault";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszNETSCAPEREVOCATIONTYPE: &str = "Netscape";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPCAPROP_CACERTIFICATE: &str = "CACertificate";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPCAPROP_CACONFIG: &str = "CAConfig";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPCAPROP_CSPNAME: &str = "CSPName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPCAPROP_ERRORCODE: &str = "ErrorCode";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPCAPROP_HASHALGORITHMID: &str = "HashAlgorithmId";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPCAPROP_KEYSPEC: &str = "KeySpec";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPCAPROP_LOCALREVOCATIONINFORMATION: &str = "LocalRevocationInformation";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPCAPROP_PROVIDERCLSID: &str = "ProviderCLSID";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPCAPROP_PROVIDERPROPERTIES: &str = "Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPCAPROP_REMINDERDURATION: &str = "ReminderDuration";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPCAPROP_SIGNINGCERTIFICATE: &str = "SigningCertificate";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPCAPROP_SIGNINGCERTIFICATETEMPLATE: &str = "SigningCertificateTemplate";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPCAPROP_SIGNINGFLAGS: &str = "SigningFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPCOMMONPROP_MAXINCOMINGMESSAGESIZE: &str = "MaxIncomingMessageSize";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPCOMMONPROP_MAXNUMOFREQUESTENTRIES: &str = "MaxNumOfRequestEntries";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPCOMMONPROP_REQFLAGS: &str = "RequestFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPISAPIPROP_DEBUG: &str = "ISAPIDebug";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPISAPIPROP_MAXAGE: &str = "MaxAge";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPISAPIPROP_MAXNUMOFCACHEENTRIES: &str = "MaxNumOfCacheEntries";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPISAPIPROP_NUMOFBACKENDCONNECTIONS: &str = "NumOfBackendConnections";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPISAPIPROP_NUMOFTHREADS: &str = "NumOfThreads";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPISAPIPROP_REFRESHRATE: &str = "RefreshRate";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPISAPIPROP_VIRTUALROOTNAME: &str = "VirtualRootName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPPROP_ARRAYCONTROLLER: &str = "ArrayController";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPPROP_ARRAYMEMBERS: &str = "ArrayMembers";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPPROP_AUDITFILTER: &str = "AuditFilter";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPPROP_DEBUG: &str = "Debug";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPPROP_ENROLLPOLLINTERVAL: &str = "EnrollPollInterval";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPPROP_LOGLEVEL: &str = "LogLevel";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPREVPROP_BASECRL: &str = "BaseCrl";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPREVPROP_BASECRLURLS: &str = "BaseCrlUrls";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPREVPROP_CRLURLTIMEOUT: &str = "CrlUrlTimeOut";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPREVPROP_DELTACRL: &str = "DeltaCrl";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPREVPROP_DELTACRLURLS: &str = "DeltaCrlUrls";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPREVPROP_ERRORCODE: &str = "RevocationErrorCode";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPREVPROP_REFRESHTIMEOUT: &str = "RefreshTimeOut";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszOCSPREVPROP_SERIALNUMBERSDIRS: &str = "IssuedSerialNumbersDirectories";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPERIODDAYS: &str = "Days";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPERIODHOURS: &str = "Hours";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPERIODMINUTES: &str = "Minutes";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPERIODMONTHS: &str = "Months";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPERIODSECONDS: &str = "Seconds";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPERIODWEEKS: &str = "Weeks";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPERIODYEARS: &str = "Years";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPFXFILENAMEEXT: &str = ".p12";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPATTESTATIONCHALLENGE: &str = "AttestationChallenge";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPATTRIBNAME: &str = "AttributeName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPATTRIBREQUESTID: &str = "AttributeRequestId";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPATTRIBVALUE: &str = "AttributeValue";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCALLERNAME: &str = "CallerName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCATYPE: &str = "CAType";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCERTCLIENTMACHINE: &str = "ccm";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCERTCOUNT: &str = "CertCount";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCERTIFICATEENROLLMENTFLAGS: &str = "EnrollmentFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCERTIFICATEGENERALFLAGS: &str = "GeneralFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCERTIFICATEHASH: &str = "CertificateHash";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCERTIFICATENOTAFTERDATE: &str = "NotAfter";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCERTIFICATENOTBEFOREDATE: &str = "NotBefore";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCERTIFICATEPRIVATEKEYFLAGS: &str = "PrivatekeyFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCERTIFICATEPUBLICKEYALGORITHM: &str = "PublicKeyAlgorithm";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCERTIFICATEPUBLICKEYLENGTH: &str = "PublicKeyLength";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCERTIFICATERAWPUBLICKEY: &str = "RawPublicKey";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCERTIFICATERAWPUBLICKEYALGORITHMPARAMETERS: &str = "RawPublicKeyAlgorithmParameters";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCERTIFICATERAWSMIMECAPABILITIES: &str = "RawSMIMECapabilities";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCERTIFICATEREQUESTID: &str = "RequestID";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCERTIFICATESERIALNUMBER: &str = "SerialNumber";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCERTIFICATESUBJECTKEYIDENTIFIER: &str = "SubjectKeyIdentifier";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCERTIFICATETEMPLATE: &str = "CertificateTemplate";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCERTIFICATETYPE: &str = "CertificateType";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCERTIFICATEUPN: &str = "UPN";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCERTSTATE: &str = "CertState";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCERTSUFFIX: &str = "CertSuffix";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCERTTEMPLATE: &str = "CertificateTemplate";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCERTTYPE: &str = "CertType";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCERTUSAGE: &str = "CertificateUsage";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCHALLENGE: &str = "Challenge";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCLIENTBROWSERMACHINE: &str = "cbm";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCLIENTDCDNS: &str = "cdc";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCOMMONNAME: &str = "CommonName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCONFIGDN: &str = "ConfigDN";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCOUNTRY: &str = "Country";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCRITICALTAG: &str = "{critical}";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCRLCOUNT: &str = "CRLCount";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCRLEFFECTIVE: &str = "CRLEffective";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCRLINDEX: &str = "CRLIndex";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCRLLASTPUBLISHED: &str = "CRLLastPublished";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCRLMINBASE: &str = "CRLMinBase";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCRLNAMEID: &str = "CRLNameId";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCRLNEXTPUBLISH: &str = "CRLNextPublish";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCRLNEXTUPDATE: &str = "CRLNextUpdate";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCRLNUMBER: &str = "CRLNumber";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCRLPROPAGATIONCOMPLETE: &str = "CRLPropagationComplete";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCRLPUBLISHATTEMPTS: &str = "CRLPublishAttempts";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCRLPUBLISHERROR: &str = "CRLPublishError";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCRLPUBLISHFLAGS: &str = "CRLPublishFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCRLPUBLISHSTATUSCODE: &str = "CRLPublishStatusCode";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCRLRAWCRL: &str = "CRLRawCRL";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCRLROWID: &str = "CRLRowId";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCRLSTATE: &str = "CRLState";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCRLSUFFIX: &str = "CRLSuffix";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCRLTHISPUBLISH: &str = "CRLThisPublish";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCRLTHISUPDATE: &str = "CRLThisUpdate";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPCROSSFOREST: &str = "CrossForest";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPDCNAME: &str = "DCName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPDECIMALTAG: &str = "{decimal}";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPDELTACRLSDISABLED: &str = "fDeltaCRLsDisabled";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPDEVICESERIALNUMBER: &str = "DeviceSerialNumber";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPDISPOSITION: &str = "Disposition";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPDISPOSITIONDENY: &str = "Deny";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPDISPOSITIONPENDING: &str = "Pending";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPDISTINGUISHEDNAME: &str = "DistinguishedName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPDN: &str = "dn";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPDNS: &str = "dns";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPDOMAINCOMPONENT: &str = "DomainComponent";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPDOMAINDN: &str = "DomainDN";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPEMAIL: &str = "EMail";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPENDORSEMENTCERTIFICATEHASH: &str = "EndorsementCertificateHash";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPENDORSEMENTKEYHASH: &str = "EndorsementKeyHash";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPEVENTLOGERROR: &str = "EventLogError";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPEVENTLOGEXHAUSTIVE: &str = "EventLogExhaustive";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPEVENTLOGTERSE: &str = "EventLogTerse";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPEVENTLOGVERBOSE: &str = "EventLogVerbose";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPEVENTLOGWARNING: &str = "EventLogWarning";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPEXITCERTFILE: &str = "CertFile";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPEXPECTEDCHALLENGE: &str = "ExpectedChallenge";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPEXPIRATIONDATE: &str = "ExpirationDate";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPEXTFLAGS: &str = "ExtensionFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPEXTNAME: &str = "ExtensionName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPEXTRAWVALUE: &str = "ExtensionRawValue";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPEXTREQUESTID: &str = "ExtensionRequestId";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPFILETAG: &str = "{file}";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPGIVENNAME: &str = "GivenName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPGUID: &str = "guid";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPHEXTAG: &str = "{hex}";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPINITIALS: &str = "Initials";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPIPADDRESS: &str = "ipaddress";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPKEYARCHIVED: &str = "KeyArchived";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPLOCALITY: &str = "Locality";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPLOGLEVEL: &str = "LogLevel";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPMACHINEDNSNAME: &str = "MachineDNSName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPMODULEREGLOC: &str = "ModuleRegistryLocation";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPNAMETYPE: &str = "NameType";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPOCTETTAG: &str = "{octet}";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPOFFICER: &str = "Officer";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPOID: &str = "oid";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPORGANIZATION: &str = "Organization";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPORGUNIT: &str = "OrgUnit";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPPUBLISHEXPIREDCERTINCRL: &str = "PublishExpiredCertInCRL";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPRAWCACERTIFICATE: &str = "RawCACertificate";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPRAWCERTIFICATE: &str = "RawCertificate";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPRAWCRL: &str = "RawCRL";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPRAWDELTACRL: &str = "RawDeltaCRL";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPRAWNAME: &str = "RawName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPRAWPRECERTIFICATE: &str = "RawPrecertificate";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTARCHIVEDKEY: &str = "ArchivedKey";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTATTRIBUTES: &str = "RequestAttributes";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTCSPPROVIDER: &str = "RequestCSPProvider";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTDISPOSITION: &str = "Disposition";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTDISPOSITIONMESSAGE: &str = "DispositionMessage";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTDOT: &str = "Request.";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTERCAACCESS: &str = "RequesterCAAccess";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTERDN: &str = "RequesterDN";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTERNAME: &str = "RequesterName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTERNAMEFROMOLDCERTIFICATE: &str = "RequesterNameFromOldCertificate";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTERSAMNAME: &str = "RequesterSAMName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTERUPN: &str = "RequesterUPN";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTFLAGS: &str = "RequestFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTKEYRECOVERYHASHES: &str = "KeyRecoveryHashes";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTMACHINEDNS: &str = "rmd";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTOSVERSION: &str = "RequestOSVersion";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTRAWARCHIVEDKEY: &str = "RawArchivedKey";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTRAWOLDCERTIFICATE: &str = "RawOldCertificate";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTRAWREQUEST: &str = "RawRequest";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTREQUESTID: &str = "RequestID";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTRESOLVEDWHEN: &str = "ResolvedWhen";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTREVOKEDEFFECTIVEWHEN: &str = "RevokedEffectiveWhen";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTREVOKEDREASON: &str = "RevokedReason";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTREVOKEDWHEN: &str = "RevokedWhen";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTSTATUSCODE: &str = "StatusCode";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTSUBMITTEDWHEN: &str = "SubmittedWhen";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPREQUESTTYPE: &str = "RequestType";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPSANITIZEDCANAME: &str = "SanitizedCAName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPSANITIZEDSHORTNAME: &str = "SanitizedShortName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPSEAUDITFILTER: &str = "SEAuditFilter";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPSEAUDITID: &str = "SEAuditId";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPSERVERUPGRADED: &str = "fServerUpgraded";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPSESSIONCOUNT: &str = "SessionCount";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPSIGNERAPPLICATIONPOLICIES: &str = "SignerApplicationPolicies";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPSIGNERPOLICIES: &str = "SignerPolicies";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPSTATE: &str = "State";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPSTREETADDRESS: &str = "StreetAddress";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPSUBJECTALTNAME2: &str = "san";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPSUBJECTDOT: &str = "Subject.";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPSURNAME: &str = "SurName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPTEMPLATECHANGESEQUENCENUMBER: &str = "TemplateChangeSequenceNumber";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPTEXTTAG: &str = "{text}";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPTITLE: &str = "Title";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPUNSTRUCTUREDADDRESS: &str = "UnstructuredAddress";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPUNSTRUCTUREDNAME: &str = "UnstructuredName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPUPN: &str = "upn";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPURL: &str = "url";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPUSEDS: &str = "fUseDS";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPUSERDN: &str = "UserDN";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPUTF8TAG: &str = "{utf8}";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPVALIDITYPERIODCOUNT: &str = "ValidityPeriodUnits";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPVALIDITYPERIODSTRING: &str = "ValidityPeriod";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszPROPVOLATILEMODE: &str = "VolatileMode";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGACTIVE: &str = "Active";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGAELOGLEVEL_OLD: &str = "AEEventLogLevel";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGAIKCLOUDCAURL: &str = "AIKCloudCAURL";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGAIKKEYALGORITHM: &str = "AIKKeyAlgorithm";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGAIKKEYLENGTH: &str = "AIKKeyLength";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGALLPROVIDERS: &str = "All";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGALTERNATEPUBLISHDOMAINS: &str = "AlternatePublishDomains";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGALTERNATESIGNATUREALGORITHM: &str = "AlternateSignatureAlgorithm";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGAUDITFILTER: &str = "AuditFilter";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGB2ICERTMANAGEMODULE: &str = "ICertManageModule";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGBACKUPLOGDIRECTORY: &str = "BackupLogDirectory";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCACERTFILENAME: &str = "CACertFileName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCACERTHASH: &str = "CACertHash";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCACERTPUBLICATIONURLS: &str = "CACertPublicationURLs";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCADESCRIPTION: &str = "CADescription";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCAPATHLENGTH: &str = "CAPathLength";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCASECURITY: &str = "Security";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCASERIALNUMBER: &str = "CACertSerialNumber";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCASERVERNAME: &str = "CAServerName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCATYPE: &str = "CAType";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCAUSEDS: &str = "UseDS";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCAXCHGCERTHASH: &str = "CAXchgCertHash";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCAXCHGOVERLAPPERIODCOUNT: &str = "CAXchgOverlapPeriodUnits";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCAXCHGOVERLAPPERIODSTRING: &str = "CAXchgOverlapPeriod";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCAXCHGVALIDITYPERIODCOUNT: &str = "CAXchgValidityPeriodUnits";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCAXCHGVALIDITYPERIODSTRING: &str = "CAXchgValidityPeriod";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCERTENROLLCOMPATIBLE: &str = "CertEnrollCompatible";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCERTIFICATETRANSPARENCYINFOOID: &str = "CTInformationExtensionOid";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCERTPUBLISHFLAGS: &str = "PublishCertFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCERTSRVDEBUG: &str = "Debug";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCHECKPOINTFILE: &str = "CheckPointFile";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCLOCKSKEWMINUTES: &str = "ClockSkewMinutes";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCOMMONNAME: &str = "CommonName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCRLATTEMPTREPUBLISH: &str = "CRLAttemptRepublish";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCRLDELTANEXTPUBLISH: &str = "CRLDeltaNextPublish";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCRLDELTAOVERLAPPERIODCOUNT: &str = "CRLDeltaOverlapUnits";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCRLDELTAOVERLAPPERIODSTRING: &str = "CRLDeltaOverlapPeriod";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCRLDELTAPERIODCOUNT: &str = "CRLDeltaPeriodUnits";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCRLDELTAPERIODSTRING: &str = "CRLDeltaPeriod";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCRLEDITFLAGS: &str = "CRLEditFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCRLFLAGS: &str = "CRLFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCRLNEXTPUBLISH: &str = "CRLNextPublish";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCRLOVERLAPPERIODCOUNT: &str = "CRLOverlapUnits";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCRLOVERLAPPERIODSTRING: &str = "CRLOverlapPeriod";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCRLPATH_OLD: &str = "CRLPath";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCRLPERIODCOUNT: &str = "CRLPeriodUnits";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCRLPERIODSTRING: &str = "CRLPeriod";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGCRLPUBLICATIONURLS: &str = "CRLPublicationURLs";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGDATABASERECOVERED: &str = "DatabaseRecovered";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGDBDIRECTORY: &str = "DBDirectory";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGDBFLAGS: &str = "DBFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGDBLASTFULLBACKUP: &str = "DBLastFullBackup";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGDBLASTINCREMENTALBACKUP: &str = "DBLastIncrementalBackup";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGDBLASTRECOVERY: &str = "DBLastRecovery";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGDBLOGDIRECTORY: &str = "DBLogDirectory";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGDBMAXREADSESSIONCOUNT: &str = "DBMaxReadSessionCount";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGDBSESSIONCOUNT: &str = "DBSessionCount";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGDBSYSDIRECTORY: &str = "DBSystemDirectory";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGDBTEMPDIRECTORY: &str = "DBTempDirectory";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGDEFAULTSMIME: &str = "DefaultSMIME";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGDIRECTORY: &str = "ConfigurationDirectory";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGDISABLEEXTENSIONLIST: &str = "DisableExtensionList";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGDSCONFIGDN: &str = "DSConfigDN";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGDSDOMAINDN: &str = "DSDomainDN";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGEDITFLAGS: &str = "EditFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGEKPUBLISTDIRECTORIES: &str = "EndorsementKeyListDirectories";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGEKUOIDSFORPUBLISHEXPIREDCERTINCRL: &str = "EKUOIDsForPublishExpiredCertInCRL";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGEKUOIDSFORVOLATILEREQUESTS: &str = "EKUOIDsforVolatileRequests";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGENABLED: &str = "Enabled";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGENABLEDEKUFORDEFINEDCACERT: &str = "EnabledEKUForDefinedCACert";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGENABLEENROLLEEREQUESTEXTENSIONLIST: &str = "EnableEnrolleeRequestExtensionList";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGENABLEREQUESTEXTENSIONLIST: &str = "EnableRequestExtensionList";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGENFORCEX500NAMELENGTHS: &str = "EnforceX500NameLengths";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGENROLLFLAGS: &str = "EnrollFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGEXITBODYARG: &str = "BodyArg";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGEXITBODYFORMAT: &str = "BodyFormat";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGEXITCRLISSUEDKEY: &str = "CRLIssued";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGEXITDENIEDKEY: &str = "Denied";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGEXITIMPORTEDKEY: &str = "Imported";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGEXITISSUEDKEY: &str = "Issued";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGEXITPENDINGKEY: &str = "Pending";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGEXITPROPNOTFOUND: &str = "???";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGEXITREVOKEDKEY: &str = "Revoked";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGEXITSHUTDOWNKEY: &str = "Shutdown";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGEXITSMTPAUTHENTICATE: &str = "SMTPAuthenticate";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGEXITSMTPCC: &str = "Cc";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGEXITSMTPEVENTFILTER: &str = "EventFilter";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGEXITSMTPFROM: &str = "From";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGEXITSMTPKEY: &str = "SMTP";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGEXITSMTPSERVER: &str = "SMTPServer";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGEXITSMTPTEMPLATES: &str = "Templates";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGEXITSMTPTO: &str = "To";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGEXITSTARTUPKEY: &str = "Startup";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGEXITTITLEARG: &str = "TitleArg";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGEXITTITLEFORMAT: &str = "TitleFormat";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGFILEISSUERCERTURL_OLD: &str = "FileIssuerCertURL";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGFILEREVOCATIONCRLURL_OLD: &str = "FileRevocationCRLURL";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGFORCETELETEX: &str = "ForceTeletex";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGFTPISSUERCERTURL_OLD: &str = "FTPIssuerCertURL";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGFTPREVOCATIONCRLURL_OLD: &str = "FTPRevocationCRLURL";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGHIGHLOGNUMBER: &str = "HighLogNumber";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGHIGHSERIAL: &str = "HighSerial";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGINTERFACEFLAGS: &str = "InterfaceFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGISSUERCERTURLFLAGS: &str = "IssuerCertURLFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGISSUERCERTURL_OLD: &str = "IssuerCertURL";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGKEYBASE: &str = "SYSTEM\\CurrentControlSet\\Services\\CertSvc";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGKEYCERTSVCPATH: &str = "SYSTEM\\CurrentControlSet\\Services\\CertSvc";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGKEYCONFIG: &str = "Configuration";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGKEYCSP: &str = "CSP";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGKEYDBPARAMETERS: &str = "DBParameters";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGKEYENCRYPTIONCSP: &str = "EncryptionCSP";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGKEYENROLLMENT: &str = "Software\\Microsoft\\Cryptography\\AutoEnrollment";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGKEYEXITMODULES: &str = "ExitModules";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGKEYGROUPPOLICYENROLLMENT: &str = "Software\\Policies\\Microsoft\\Cryptography\\AutoEnrollment";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGKEYNOSYSTEMCERTSVCPATH: &str = "CurrentControlSet\\Services\\CertSvc";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGKEYPOLICYMODULES: &str = "PolicyModules";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGKEYREPAIR: &str = "KeyRepair";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGKEYRESTOREINPROGRESS: &str = "RestoreInProgress";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGKEYSIZE: &str = "KeySize";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGKRACERTCOUNT: &str = "KRACertCount";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGKRACERTHASH: &str = "KRACertHash";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGKRAFLAGS: &str = "KRAFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGLDAPFLAGS: &str = "LDAPFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGLDAPISSUERCERTURL_OLD: &str = "LDAPIssuerCertURL";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGLDAPREVOCATIONCRLURL_OLD: &str = "LDAPRevocationCRLURL";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGLDAPREVOCATIONDNTEMPLATE_OLD: &str = "LDAPRevocationDNTemplate";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGLDAPREVOCATIONDN_OLD: &str = "LDAPRevocationDN";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGLDAPSESSIONOPTIONS: &str = "LDAPSessionOptions";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGLOGLEVEL: &str = "LogLevel";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGLOGPATH: &str = "LogPath";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGLOWLOGNUMBER: &str = "LowLogNumber";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGMAXINCOMINGALLOCSIZE: &str = "MaxIncomingAllocSize";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGMAXINCOMINGMESSAGESIZE: &str = "MaxIncomingMessageSize";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGMAXPENDINGREQUESTDAYS: &str = "MaxPendingRequestDays";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGMAXSCTLISTSIZE: &str = "MaxSCTListSize";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGNAMESEPARATOR: &str = "SubjectNameSeparator";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGNETSCAPECERTTYPE: &str = "NetscapeCertType";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGOFFICERRIGHTS: &str = "OfficerRights";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGPARENTCAMACHINE: &str = "ParentCAMachine";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGPARENTCANAME: &str = "ParentCAName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGPOLICYFLAGS: &str = "PolicyFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGPRESERVESCEPDUMMYCERTS: &str = "PreserveSCEPDummyCerts";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGPROCESSINGFLAGS: &str = "ProcessingFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGPROVIDER: &str = "Provider";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGPROVIDERTYPE: &str = "ProviderType";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGREQUESTDISPOSITION: &str = "RequestDisposition";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGREQUESTFILENAME: &str = "RequestFileName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGREQUESTID: &str = "RequestId";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGREQUESTKEYCONTAINER: &str = "RequestKeyContainer";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGREQUESTKEYINDEX: &str = "RequestKeyIndex";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGRESTOREMAP: &str = "RestoreMap";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGRESTOREMAPCOUNT: &str = "RestoreMapCount";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGRESTORESTATUS: &str = "RestoreStatus";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGREVOCATIONCRLURL_OLD: &str = "RevocationCRLURL";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGREVOCATIONTYPE: &str = "RevocationType";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGREVOCATIONURL: &str = "RevocationURL";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGROLESEPARATIONENABLED: &str = "RoleSeparationEnabled";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGSETUPSTATUS: &str = "SetupStatus";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGSP4DEFAULTCONFIGURATION: &str = "DefaultConfiguration";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGSP4KEYSETNAME: &str = "KeySetName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGSP4NAMES: &str = "Names";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGSP4QUERIES: &str = "Queries";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGSP4SUBJECTNAMESEPARATOR: &str = "SubjectNameSeparator";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGSUBJECTALTNAME: &str = "SubjectAltName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGSUBJECTALTNAME2: &str = "SubjectAltName2";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGSUBJECTTEMPLATE: &str = "SubjectTemplate";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGSYMMETRICKEYSIZE: &str = "SymmetricKeySize";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGUNICODE: &str = "Unicode";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGUPNMAP: &str = "UPNMap";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGUSEDEFINEDCACERTINREQ: &str = "UseDefinedCACertInRequest";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGVALIDITYPERIODCOUNT: &str = "ValidityPeriodUnits";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGVALIDITYPERIODSTRING: &str = "ValidityPeriod";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGVERIFYFLAGS: &str = "VerifyFlags";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGVERSION: &str = "Version";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGVIEWAGEMINUTES: &str = "ViewAgeMinutes";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGVIEWIDLEMINUTES: &str = "ViewIdleMinutes";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGWEBCLIENTCAMACHINE: &str = "WebClientCAMachine";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGWEBCLIENTCANAME: &str = "WebClientCAName";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszREGWEBCLIENTCATYPE: &str = "WebClientCAType";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszSECUREDATTRIBUTES: &str = "SignedAttributes";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszSERVICE_NAME: &str = "CertSvc";
#[doc = "*Required features: `\"Win32_Security_Cryptography_Certificates\"`*"]
pub const wszzDEFAULTSIGNEDATTRIBUTES: &str = "RequesterName\u{0}";
