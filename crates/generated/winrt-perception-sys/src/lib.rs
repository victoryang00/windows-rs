
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#![no_std]
#[cfg(feature = "Automation")]
pub mod Automation;
#[cfg(feature = "People")]
pub mod People;
#[cfg(feature = "Spatial")]
pub mod Spatial;
pub type PerceptionTimestamp = *mut ::core::ffi::c_void;
