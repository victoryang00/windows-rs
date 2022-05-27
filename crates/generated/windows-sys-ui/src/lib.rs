
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#![no_std]
#[cfg(feature = "Accessibility")]
pub mod Accessibility;
#[cfg(feature = "ApplicationSettings")]
pub mod ApplicationSettings;
#[cfg(feature = "Composition")]
pub mod Composition;
#[cfg(feature = "Core")]
pub mod Core;
#[cfg(feature = "Input")]
pub mod Input;
#[cfg(feature = "Notifications")]
pub mod Notifications;
#[cfg(feature = "Popups")]
pub mod Popups;
#[cfg(feature = "Shell")]
pub mod Shell;
#[cfg(feature = "StartScreen")]
pub mod StartScreen;
#[cfg(feature = "Text")]
pub mod Text;
#[cfg(feature = "UIAutomation")]
pub mod UIAutomation;
#[cfg(feature = "ViewManagement")]
pub mod ViewManagement;
#[cfg(feature = "WebUI")]
pub mod WebUI;
#[cfg(feature = "WindowManagement")]
pub mod WindowManagement;
#[cfg(feature = "Xaml")]
pub mod Xaml;
#[repr(C)]
pub struct Color {
    pub A: u8,
    pub R: u8,
    pub G: u8,
    pub B: u8,
}
impl ::core::marker::Copy for Color {}
impl ::core::clone::Clone for Color {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ColorHelper = *mut ::core::ffi::c_void;
pub type Colors = *mut ::core::ffi::c_void;
pub type UIContentRoot = *mut ::core::ffi::c_void;
pub type UIContext = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct WindowId {
    pub Value: u64,
}
impl ::core::marker::Copy for WindowId {}
impl ::core::clone::Clone for WindowId {
    fn clone(&self) -> Self {
        *self
    }
}
