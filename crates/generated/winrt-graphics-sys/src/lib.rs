
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#![no_std]
#[cfg(feature = "Capture")]
pub mod Capture;
#[cfg(feature = "DirectX")]
pub mod DirectX;
#[cfg(feature = "Display")]
pub mod Display;
#[cfg(feature = "Effects")]
pub mod Effects;
#[cfg(feature = "Holographic")]
pub mod Holographic;
#[cfg(feature = "Imaging")]
pub mod Imaging;
#[cfg(feature = "Printing")]
pub mod Printing;
#[cfg(feature = "Printing3D")]
pub mod Printing3D;
#[repr(C)]
pub struct DisplayAdapterId {
    pub LowPart: u32,
    pub HighPart: i32,
}
impl ::core::marker::Copy for DisplayAdapterId {}
impl ::core::clone::Clone for DisplayAdapterId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DisplayId {
    pub Value: u64,
}
impl ::core::marker::Copy for DisplayId {}
impl ::core::clone::Clone for DisplayId {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IGeometrySource2D = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct PointInt32 {
    pub X: i32,
    pub Y: i32,
}
impl ::core::marker::Copy for PointInt32 {}
impl ::core::clone::Clone for PointInt32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RectInt32 {
    pub X: i32,
    pub Y: i32,
    pub Width: i32,
    pub Height: i32,
}
impl ::core::marker::Copy for RectInt32 {}
impl ::core::clone::Clone for RectInt32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SizeInt32 {
    pub Width: i32,
    pub Height: i32,
}
impl ::core::marker::Copy for SizeInt32 {}
impl ::core::clone::Clone for SizeInt32 {
    fn clone(&self) -> Self {
        *self
    }
}
