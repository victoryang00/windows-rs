
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#![no_std]
#[cfg(feature = "Authentication")]
pub mod Authentication;
#[cfg(feature = "Authorization")]
pub mod Authorization;
#[cfg(feature = "Credentials")]
pub mod Credentials;
#[cfg(feature = "Cryptography")]
pub mod Cryptography;
#[cfg(feature = "DataProtection")]
pub mod DataProtection;
#[cfg(feature = "EnterpriseData")]
pub mod EnterpriseData;
#[cfg(feature = "ExchangeActiveSyncProvisioning")]
pub mod ExchangeActiveSyncProvisioning;
#[cfg(feature = "Isolation")]
pub mod Isolation;
