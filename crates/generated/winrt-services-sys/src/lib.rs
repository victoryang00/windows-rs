
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#![no_std]
#[cfg(feature = "Cortana")]
pub mod Cortana;
#[cfg(feature = "Maps")]
pub mod Maps;
#[cfg(feature = "Store")]
pub mod Store;
#[cfg(feature = "TargetedContent")]
pub mod TargetedContent;
