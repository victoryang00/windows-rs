
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#![no_std]
#[cfg(feature = "Collation")]
pub mod Collation;
#[cfg(feature = "DateTimeFormatting")]
pub mod DateTimeFormatting;
#[cfg(feature = "Fonts")]
pub mod Fonts;
#[cfg(feature = "NumberFormatting")]
pub mod NumberFormatting;
#[cfg(feature = "PhoneNumberFormatting")]
pub mod PhoneNumberFormatting;
pub type Calendar = *mut ::core::ffi::c_void;
pub type CurrencyAmount = *mut ::core::ffi::c_void;
#[repr(transparent)]
pub struct DayOfWeek(pub i32);
impl DayOfWeek {
    pub const Sunday: Self = Self(0i32);
    pub const Monday: Self = Self(1i32);
    pub const Tuesday: Self = Self(2i32);
    pub const Wednesday: Self = Self(3i32);
    pub const Thursday: Self = Self(4i32);
    pub const Friday: Self = Self(5i32);
    pub const Saturday: Self = Self(6i32);
}
impl ::core::marker::Copy for DayOfWeek {}
impl ::core::clone::Clone for DayOfWeek {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GeographicRegion = *mut ::core::ffi::c_void;
pub type JapanesePhoneme = *mut ::core::ffi::c_void;
pub type Language = *mut ::core::ffi::c_void;
#[repr(transparent)]
pub struct LanguageLayoutDirection(pub i32);
impl LanguageLayoutDirection {
    pub const Ltr: Self = Self(0i32);
    pub const Rtl: Self = Self(1i32);
    pub const TtbLtr: Self = Self(2i32);
    pub const TtbRtl: Self = Self(3i32);
}
impl ::core::marker::Copy for LanguageLayoutDirection {}
impl ::core::clone::Clone for LanguageLayoutDirection {
    fn clone(&self) -> Self {
        *self
    }
}
