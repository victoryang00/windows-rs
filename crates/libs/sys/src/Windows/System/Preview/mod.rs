#[doc = "*Required features: `\"System_Preview\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct HingeState(pub i32);
#[cfg(feature = "deprecated")]
impl HingeState {
    pub const Unknown: Self = Self(0i32);
    pub const Closed: Self = Self(1i32);
    pub const Concave: Self = Self(2i32);
    pub const Flat: Self = Self(3i32);
    pub const Convex: Self = Self(4i32);
    pub const Full: Self = Self(5i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for HingeState {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for HingeState {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ITwoPanelHingedDevicePosturePreview {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetCurrentPostureAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetCurrentPostureAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PostureChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PostureChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemovePostureChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemovePostureChanged: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ITwoPanelHingedDevicePosturePreviewReading {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Timestamp: usize,
    #[cfg(feature = "deprecated")]
    pub HingeState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HingeState) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    HingeState: usize,
    #[cfg(all(feature = "Devices_Sensors", feature = "deprecated"))]
    pub Panel1Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Devices::Sensors::SimpleOrientation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Sensors", feature = "deprecated")))]
    Panel1Orientation: usize,
    #[cfg(feature = "deprecated")]
    pub Panel1Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Panel1Id: usize,
    #[cfg(all(feature = "Devices_Sensors", feature = "deprecated"))]
    pub Panel2Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Devices::Sensors::SimpleOrientation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Sensors", feature = "deprecated")))]
    Panel2Orientation: usize,
    #[cfg(feature = "deprecated")]
    pub Panel2Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Panel2Id: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Reading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Reading: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ITwoPanelHingedDevicePosturePreviewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetDefaultAsync: usize,
}
pub type TwoPanelHingedDevicePosturePreview = *mut ::core::ffi::c_void;
pub type TwoPanelHingedDevicePosturePreviewReading = *mut ::core::ffi::c_void;
pub type TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs = *mut ::core::ffi::c_void;
