
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#[cfg(feature = "Html")]
pub mod Html;
#[cfg(feature = "Json")]
pub mod Json;
#[cfg(feature = "Pdf")]
pub mod Pdf;
#[cfg(feature = "Text")]
pub mod Text;
#[cfg(feature = "Xml")]
pub mod Xml;
