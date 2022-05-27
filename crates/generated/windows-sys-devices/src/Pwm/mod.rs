#[cfg(feature = "Provider")]
pub mod Provider;
pub type PwmController = *mut ::core::ffi::c_void;
pub type PwmPin = *mut ::core::ffi::c_void;
#[repr(transparent)]
pub struct PwmPulsePolarity(pub i32);
impl PwmPulsePolarity {
    pub const ActiveHigh: Self = Self(0i32);
    pub const ActiveLow: Self = Self(1i32);
}
impl ::core::marker::Copy for PwmPulsePolarity {}
impl ::core::clone::Clone for PwmPulsePolarity {
    fn clone(&self) -> Self {
        *self
    }
}
