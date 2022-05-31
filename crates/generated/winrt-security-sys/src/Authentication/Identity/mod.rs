#[cfg(feature = "Core")]
pub mod Core;
#[cfg(feature = "Provider")]
pub mod Provider;
pub type EnterpriseKeyCredentialRegistrationInfo = *mut ::core::ffi::c_void;
pub type EnterpriseKeyCredentialRegistrationManager = *mut ::core::ffi::c_void;
