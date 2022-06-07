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
impl ::windows_sys::core::Interface for IEnterpriseKeyCredentialRegistrationInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 942807756, data2: 26411, data3: 18467, data4: [182, 3, 107, 60, 117, 61, 175, 151] };
}
#[repr(C)]
pub struct IEnterpriseKeyCredentialRegistrationManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRegistrationsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRegistrationsAsync: usize,
}
impl ::windows_sys::core::Interface for IEnterpriseKeyCredentialRegistrationManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2213789247, data2: 41567, data3: 19642, data4: [187, 142, 189, 195, 45, 3, 194, 151] };
}
#[repr(C)]
pub struct IEnterpriseKeyCredentialRegistrationManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Current: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEnterpriseKeyCredentialRegistrationManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2008571550, data2: 44276, data3: 19392, data4: [186, 194, 64, 187, 70, 239, 187, 63] };
}
