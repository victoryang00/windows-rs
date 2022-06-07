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
impl ::windows_sys::core::Interface for ITwoPanelHingedDevicePosturePreview {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1914985521, data2: 19257, data3: 17062, data4: [142, 115, 114, 53, 173, 225, 104, 83] };
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
impl ::windows_sys::core::Interface for ITwoPanelHingedDevicePosturePreviewReading {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2686784594, data2: 19158, data3: 19256, data4: [132, 38, 197, 154, 21, 73, 58, 125] };
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
impl ::windows_sys::core::Interface for ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 757930950, data2: 718, data3: 18250, data4: [165, 86, 167, 91, 28, 249, 58, 3] };
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
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ITwoPanelHingedDevicePosturePreviewStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 205992914, data2: 22496, data3: 16768, data4: [189, 94, 243, 26, 33, 56, 66, 62] };
}
pub type TwoPanelHingedDevicePosturePreview = *mut ::core::ffi::c_void;
pub type TwoPanelHingedDevicePosturePreviewReading = *mut ::core::ffi::c_void;
pub type TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs = *mut ::core::ffi::c_void;
