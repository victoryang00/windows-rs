#[cfg(feature = "Core")]
pub mod Core;
#[cfg(feature = "Management")]
pub mod Management;
pub type ResourceLoader = *mut ::core::ffi::c_void;
