#[cfg(feature = "Security_Authentication_Identity_Core")]
pub mod Core;
#[cfg(feature = "Security_Authentication_Identity_Provider")]
pub mod Provider;
pub type EnterpriseKeyCredentialRegistrationInfo = *mut ::core::ffi::c_void;
pub type EnterpriseKeyCredentialRegistrationManager = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IEnterpriseKeyCredentialRegistrationInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub TenantId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TenantName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub KeyId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub KeyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEnterpriseKeyCredentialRegistrationManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRegistrationsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRegistrationsAsync: usize,
}
#[repr(C)]
pub struct IEnterpriseKeyCredentialRegistrationManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Current: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
