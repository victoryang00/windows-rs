#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct HingeState(pub i32);
#[cfg(feature = "winrt-")]
impl HingeState {
    pub const Unknown: Self = Self(0i32);
    pub const Closed: Self = Self(1i32);
    pub const Concave: Self = Self(2i32);
    pub const Flat: Self = Self(3i32);
    pub const Convex: Self = Self(4i32);
    pub const Full: Self = Self(5i32);
}
#[cfg(feature = "winrt-")]
impl ::core::marker::Copy for HingeState {}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for HingeState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TwoPanelHingedDevicePosturePreview = *mut ::core::ffi::c_void;
pub type TwoPanelHingedDevicePosturePreviewReading = *mut ::core::ffi::c_void;
pub type TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs = *mut ::core::ffi::c_void;
