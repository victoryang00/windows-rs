#[link(name = "windows")]
extern "system" {
    pub fn CryptSIPAddProvider(psnewprov: *mut SIP_ADD_NEWPROVIDER) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-security-sys")]
    pub fn CryptSIPCreateIndirectData(psubjectinfo: *mut SIP_SUBJECTINFO, pcbindirectdata: *mut u32, pindirectdata: *mut SIP_INDIRECT_DATA) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-security-sys")]
    pub fn CryptSIPGetCaps(psubjinfo: *const SIP_SUBJECTINFO, pcaps: *mut SIP_CAP_SET_V3) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-security-sys")]
    pub fn CryptSIPGetSealedDigest(psubjectinfo: *const SIP_SUBJECTINFO, psig: *const u8, dwsig: u32, pbdigest: *mut u8, pcbdigest: *mut u32) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-security-sys")]
    pub fn CryptSIPGetSignedDataMsg(psubjectinfo: *mut SIP_SUBJECTINFO, pdwencodingtype: *mut super::CERT_QUERY_ENCODING_TYPE, dwindex: u32, pcbsigneddatamsg: *mut u32, pbsigneddatamsg: *mut u8) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-security-sys")]
    pub fn CryptSIPLoad(pgsubject: *const ::windows_core_sys::GUID, dwflags: u32, psipdispatch: *mut SIP_DISPATCH_INFO) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-security-sys")]
    pub fn CryptSIPPutSignedDataMsg(psubjectinfo: *mut SIP_SUBJECTINFO, dwencodingtype: super::CERT_QUERY_ENCODING_TYPE, pdwindex: *mut u32, cbsigneddatamsg: u32, pbsigneddatamsg: *mut u8) -> ::win32_foundation_sys::BOOL;
    pub fn CryptSIPRemoveProvider(pgprov: *mut ::windows_core_sys::GUID) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-security-sys")]
    pub fn CryptSIPRemoveSignedDataMsg(psubjectinfo: *mut SIP_SUBJECTINFO, dwindex: u32) -> ::win32_foundation_sys::BOOL;
    pub fn CryptSIPRetrieveSubjectGuid(filename: ::windows_core_sys::PCWSTR, hfilein: ::win32_foundation_sys::HANDLE, pgsubject: *mut ::windows_core_sys::GUID) -> ::win32_foundation_sys::BOOL;
    pub fn CryptSIPRetrieveSubjectGuidForCatalogFile(filename: ::windows_core_sys::PCWSTR, hfilein: ::win32_foundation_sys::HANDLE, pgsubject: *mut ::windows_core_sys::GUID) -> ::win32_foundation_sys::BOOL;
    #[cfg(feature = "win32-security-sys")]
    pub fn CryptSIPVerifyIndirectData(psubjectinfo: *mut SIP_SUBJECTINFO, pindirectdata: *mut SIP_INDIRECT_DATA) -> ::win32_foundation_sys::BOOL;
}
pub const MSSIP_ADDINFO_BLOB: u32 = 3u32;
pub const MSSIP_ADDINFO_CATMEMBER: u32 = 2u32;
pub const MSSIP_ADDINFO_FLAT: u32 = 1u32;
pub const MSSIP_ADDINFO_NONE: u32 = 0u32;
pub const MSSIP_ADDINFO_NONMSSIP: u32 = 500u32;
pub const MSSIP_FLAGS_MULTI_HASH: u32 = 262144u32;
pub const MSSIP_FLAGS_PROHIBIT_RESIZE_ON_CREATE: u32 = 65536u32;
pub const MSSIP_FLAGS_USE_CATALOG: u32 = 131072u32;
#[repr(C)]
pub struct MS_ADDINFO_BLOB {
    pub cbStruct: u32,
    pub cbMemObject: u32,
    pub pbMemObject: *mut u8,
    pub cbMemSignedMsg: u32,
    pub pbMemSignedMsg: *mut u8,
}
impl ::core::marker::Copy for MS_ADDINFO_BLOB {}
impl ::core::clone::Clone for MS_ADDINFO_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MS_ADDINFO_FLAT {
    pub cbStruct: u32,
    pub pIndirectData: *mut SIP_INDIRECT_DATA,
}
impl ::core::marker::Copy for MS_ADDINFO_FLAT {}
impl ::core::clone::Clone for MS_ADDINFO_FLAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SIP_ADD_NEWPROVIDER {
    pub cbStruct: u32,
    pub pgSubject: *mut ::windows_core_sys::GUID,
    pub pwszDLLFileName: ::windows_core_sys::PWSTR,
    pub pwszMagicNumber: ::windows_core_sys::PWSTR,
    pub pwszIsFunctionName: ::windows_core_sys::PWSTR,
    pub pwszGetFuncName: ::windows_core_sys::PWSTR,
    pub pwszPutFuncName: ::windows_core_sys::PWSTR,
    pub pwszCreateFuncName: ::windows_core_sys::PWSTR,
    pub pwszVerifyFuncName: ::windows_core_sys::PWSTR,
    pub pwszRemoveFuncName: ::windows_core_sys::PWSTR,
    pub pwszIsFunctionNameFmt2: ::windows_core_sys::PWSTR,
    pub pwszGetCapFuncName: ::windows_core_sys::PWSTR,
}
impl ::core::marker::Copy for SIP_ADD_NEWPROVIDER {}
impl ::core::clone::Clone for SIP_ADD_NEWPROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SIP_CAP_FLAG_SEALING: u32 = 1u32;
pub const SIP_CAP_SET_CUR_VER: u32 = 3u32;
#[repr(C)]
pub struct SIP_CAP_SET_V2 {
    pub cbSize: u32,
    pub dwVersion: u32,
    pub isMultiSign: ::win32_foundation_sys::BOOL,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for SIP_CAP_SET_V2 {}
impl ::core::clone::Clone for SIP_CAP_SET_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SIP_CAP_SET_V3 {
    pub cbSize: u32,
    pub dwVersion: u32,
    pub isMultiSign: ::win32_foundation_sys::BOOL,
    pub Anonymous: SIP_CAP_SET_V3_0,
}
impl ::core::marker::Copy for SIP_CAP_SET_V3 {}
impl ::core::clone::Clone for SIP_CAP_SET_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union SIP_CAP_SET_V3_0 {
    pub dwFlags: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for SIP_CAP_SET_V3_0 {}
impl ::core::clone::Clone for SIP_CAP_SET_V3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SIP_CAP_SET_VERSION_2: u32 = 2u32;
pub const SIP_CAP_SET_VERSION_3: u32 = 3u32;
#[repr(C)]
#[cfg(feature = "win32-security-sys")]
pub struct SIP_DISPATCH_INFO {
    pub cbSize: u32,
    pub hSIP: ::win32_foundation_sys::HANDLE,
    pub pfGet: pCryptSIPGetSignedDataMsg,
    pub pfPut: pCryptSIPPutSignedDataMsg,
    pub pfCreate: pCryptSIPCreateIndirectData,
    pub pfVerify: pCryptSIPVerifyIndirectData,
    pub pfRemove: pCryptSIPRemoveSignedDataMsg,
}
#[cfg(feature = "win32-security-sys")]
impl ::core::marker::Copy for SIP_DISPATCH_INFO {}
#[cfg(feature = "win32-security-sys")]
impl ::core::clone::Clone for SIP_DISPATCH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SIP_INDIRECT_DATA {
    pub Data: super::CRYPT_ATTRIBUTE_TYPE_VALUE,
    pub DigestAlgorithm: super::CRYPT_ALGORITHM_IDENTIFIER,
    pub Digest: super::CRYPTOAPI_BLOB,
}
impl ::core::marker::Copy for SIP_INDIRECT_DATA {}
impl ::core::clone::Clone for SIP_INDIRECT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SIP_MAX_MAGIC_NUMBER: u32 = 4u32;
#[repr(C)]
#[cfg(feature = "win32-security-sys")]
pub struct SIP_SUBJECTINFO {
    pub cbSize: u32,
    pub pgSubjectType: *mut ::windows_core_sys::GUID,
    pub hFile: ::win32_foundation_sys::HANDLE,
    pub pwsFileName: ::windows_core_sys::PCWSTR,
    pub pwsDisplayName: ::windows_core_sys::PCWSTR,
    pub dwReserved1: u32,
    pub dwIntVersion: u32,
    pub hProv: usize,
    pub DigestAlgorithm: super::CRYPT_ALGORITHM_IDENTIFIER,
    pub dwFlags: u32,
    pub dwEncodingType: u32,
    pub dwReserved2: u32,
    pub fdwCAPISettings: u32,
    pub fdwSecuritySettings: u32,
    pub dwIndex: u32,
    pub dwUnionChoice: u32,
    pub Anonymous: SIP_SUBJECTINFO_0,
    pub pClientData: *mut ::core::ffi::c_void,
}
#[cfg(feature = "win32-security-sys")]
impl ::core::marker::Copy for SIP_SUBJECTINFO {}
#[cfg(feature = "win32-security-sys")]
impl ::core::clone::Clone for SIP_SUBJECTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "win32-security-sys")]
pub union SIP_SUBJECTINFO_0 {
    pub psFlat: *mut MS_ADDINFO_FLAT,
    pub psCatMember: *mut super::Catalog::MS_ADDINFO_CATALOGMEMBER,
    pub psBlob: *mut MS_ADDINFO_BLOB,
}
#[cfg(feature = "win32-security-sys")]
impl ::core::marker::Copy for SIP_SUBJECTINFO_0 {}
#[cfg(feature = "win32-security-sys")]
impl ::core::clone::Clone for SIP_SUBJECTINFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SPC_DIGEST_GENERATE_FLAG: u32 = 512u32;
pub const SPC_DIGEST_SIGN_EX_FLAG: u32 = 16384u32;
pub const SPC_DIGEST_SIGN_FLAG: u32 = 1024u32;
pub const SPC_EXC_PE_PAGE_HASHES_FLAG: u32 = 16u32;
pub const SPC_INC_PE_DEBUG_INFO_FLAG: u32 = 64u32;
pub const SPC_INC_PE_IMPORT_ADDR_TABLE_FLAG: u32 = 32u32;
pub const SPC_INC_PE_PAGE_HASHES_FLAG: u32 = 256u32;
pub const SPC_INC_PE_RESOURCES_FLAG: u32 = 128u32;
pub const SPC_MARKER_CHECK_CURRENTLY_SUPPORTED_FLAGS: u32 = 1u32;
pub const SPC_MARKER_CHECK_SKIP_SIP_INDIRECT_DATA_FLAG: u32 = 1u32;
pub const SPC_RELAXED_PE_MARKER_CHECK: u32 = 2048u32;
#[cfg(feature = "win32-security-sys")]
pub type pCryptSIPCreateIndirectData = ::core::option::Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, pcbindirectdata: *mut u32, pindirectdata: *mut SIP_INDIRECT_DATA) -> ::win32_foundation_sys::BOOL>;
#[cfg(feature = "win32-security-sys")]
pub type pCryptSIPGetCaps = ::core::option::Option<unsafe extern "system" fn(psubjinfo: *const SIP_SUBJECTINFO, pcaps: *mut SIP_CAP_SET_V3) -> ::win32_foundation_sys::BOOL>;
#[cfg(feature = "win32-security-sys")]
pub type pCryptSIPGetSealedDigest = ::core::option::Option<unsafe extern "system" fn(psubjectinfo: *const SIP_SUBJECTINFO, psig: *const u8, dwsig: u32, pbdigest: *mut u8, pcbdigest: *mut u32) -> ::win32_foundation_sys::BOOL>;
#[cfg(feature = "win32-security-sys")]
pub type pCryptSIPGetSignedDataMsg = ::core::option::Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, pdwencodingtype: *mut u32, dwindex: u32, pcbsigneddatamsg: *mut u32, pbsigneddatamsg: *mut u8) -> ::win32_foundation_sys::BOOL>;
#[cfg(feature = "win32-security-sys")]
pub type pCryptSIPPutSignedDataMsg = ::core::option::Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, dwencodingtype: u32, pdwindex: *mut u32, cbsigneddatamsg: u32, pbsigneddatamsg: *mut u8) -> ::win32_foundation_sys::BOOL>;
#[cfg(feature = "win32-security-sys")]
pub type pCryptSIPRemoveSignedDataMsg = ::core::option::Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, dwindex: u32) -> ::win32_foundation_sys::BOOL>;
#[cfg(feature = "win32-security-sys")]
pub type pCryptSIPVerifyIndirectData = ::core::option::Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, pindirectdata: *mut SIP_INDIRECT_DATA) -> ::win32_foundation_sys::BOOL>;
pub type pfnIsFileSupported = ::core::option::Option<unsafe extern "system" fn(hfile: ::win32_foundation_sys::HANDLE, pgsubject: *mut ::windows_core_sys::GUID) -> ::win32_foundation_sys::BOOL>;
pub type pfnIsFileSupportedName = ::core::option::Option<unsafe extern "system" fn(pwszfilename: ::windows_core_sys::PCWSTR, pgsubject: *mut ::windows_core_sys::GUID) -> ::win32_foundation_sys::BOOL>;
