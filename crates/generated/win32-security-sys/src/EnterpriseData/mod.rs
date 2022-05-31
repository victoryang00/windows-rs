#[link(name = "windows")]
extern "system" {
    pub fn ProtectFileToEnterpriseIdentity(fileorfolderpath: ::windows_core_sys::PCWSTR, identity: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn SrpCloseThreadNetworkContext(threadnetworkcontext: *mut HTHREAD_NETWORK_CONTEXT) -> ::windows_core_sys::HRESULT;
    pub fn SrpCreateThreadNetworkContext(enterpriseid: ::windows_core_sys::PCWSTR, threadnetworkcontext: *mut HTHREAD_NETWORK_CONTEXT) -> ::windows_core_sys::HRESULT;
    pub fn SrpDisablePermissiveModeFileEncryption() -> ::windows_core_sys::HRESULT;
    #[cfg(feature = "win32-storage-sys")]
    pub fn SrpDoesPolicyAllowAppExecution(packageid: *const ::win32_storage_sys::Packaging::Appx::PACKAGE_ID, isallowed: *mut ::win32_foundation_sys::BOOL) -> ::windows_core_sys::HRESULT;
    pub fn SrpEnablePermissiveModeFileEncryption(enterpriseid: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn SrpGetEnterpriseIds(tokenhandle: ::win32_foundation_sys::HANDLE, numberofbytes: *mut u32, enterpriseids: *mut ::windows_core_sys::PWSTR, enterpriseidcount: *mut u32) -> ::windows_core_sys::HRESULT;
    pub fn SrpGetEnterprisePolicy(tokenhandle: ::win32_foundation_sys::HANDLE, policyflags: *mut ENTERPRISE_DATA_POLICIES) -> ::windows_core_sys::HRESULT;
    pub fn SrpHostingInitialize(version: SRPHOSTING_VERSION, r#type: SRPHOSTING_TYPE, pvdata: *const ::core::ffi::c_void, cbdata: u32) -> ::windows_core_sys::HRESULT;
    pub fn SrpHostingTerminate(r#type: SRPHOSTING_TYPE);
    pub fn SrpIsTokenService(tokenhandle: ::win32_foundation_sys::HANDLE, istokenservice: *mut u8) -> ::win32_foundation_sys::NTSTATUS;
    pub fn SrpSetTokenEnterpriseId(tokenhandle: ::win32_foundation_sys::HANDLE, enterpriseid: ::windows_core_sys::PCWSTR) -> ::windows_core_sys::HRESULT;
    pub fn UnprotectFile(fileorfolderpath: ::windows_core_sys::PCWSTR, options: *const FILE_UNPROTECT_OPTIONS) -> ::windows_core_sys::HRESULT;
}
pub type ENTERPRISE_DATA_POLICIES = u32;
pub const ENTERPRISE_POLICY_NONE: ENTERPRISE_DATA_POLICIES = 0u32;
pub const ENTERPRISE_POLICY_ALLOWED: ENTERPRISE_DATA_POLICIES = 1u32;
pub const ENTERPRISE_POLICY_ENLIGHTENED: ENTERPRISE_DATA_POLICIES = 2u32;
pub const ENTERPRISE_POLICY_EXEMPT: ENTERPRISE_DATA_POLICIES = 4u32;
#[repr(C)]
pub struct FILE_UNPROTECT_OPTIONS {
    pub audit: bool,
}
impl ::core::marker::Copy for FILE_UNPROTECT_OPTIONS {}
impl ::core::clone::Clone for FILE_UNPROTECT_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct HTHREAD_NETWORK_CONTEXT {
    pub ThreadId: u32,
    pub ThreadContext: ::win32_foundation_sys::HANDLE,
}
impl ::core::marker::Copy for HTHREAD_NETWORK_CONTEXT {}
impl ::core::clone::Clone for HTHREAD_NETWORK_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IProtectionPolicyManagerInterop = *mut ::core::ffi::c_void;
pub type IProtectionPolicyManagerInterop2 = *mut ::core::ffi::c_void;
pub type IProtectionPolicyManagerInterop3 = *mut ::core::ffi::c_void;
pub type SRPHOSTING_TYPE = i32;
pub const SRPHOSTING_TYPE_NONE: SRPHOSTING_TYPE = 0i32;
pub const SRPHOSTING_TYPE_WINHTTP: SRPHOSTING_TYPE = 1i32;
pub const SRPHOSTING_TYPE_WININET: SRPHOSTING_TYPE = 2i32;
pub type SRPHOSTING_VERSION = i32;
pub const SRPHOSTING_VERSION1: SRPHOSTING_VERSION = 1i32;
