#[doc = "*Required features: `\"Devices_Input_Preview\"`*"]
#[repr(transparent)]
pub struct GazeDeviceConfigurationStatePreview(pub i32);
impl GazeDeviceConfigurationStatePreview {
    pub const Unknown: Self = Self(0i32);
    pub const Ready: Self = Self(1i32);
    pub const Configuring: Self = Self(2i32);
    pub const ScreenSetupNeeded: Self = Self(3i32);
    pub const UserCalibrationNeeded: Self = Self(4i32);
}
impl ::core::marker::Copy for GazeDeviceConfigurationStatePreview {}
impl ::core::clone::Clone for GazeDeviceConfigurationStatePreview {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GazeDevicePreview = *mut ::core::ffi::c_void;
pub type GazeDeviceWatcherAddedPreviewEventArgs = *mut ::core::ffi::c_void;
pub type GazeDeviceWatcherPreview = *mut ::core::ffi::c_void;
pub type GazeDeviceWatcherRemovedPreviewEventArgs = *mut ::core::ffi::c_void;
pub type GazeDeviceWatcherUpdatedPreviewEventArgs = *mut ::core::ffi::c_void;
pub type GazeEnteredPreviewEventArgs = *mut ::core::ffi::c_void;
pub type GazeExitedPreviewEventArgs = *mut ::core::ffi::c_void;
pub type GazeInputSourcePreview = *mut ::core::ffi::c_void;
pub type GazeMovedPreviewEventArgs = *mut ::core::ffi::c_void;
pub type GazePointPreview = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IGazeDevicePreview {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CanTrackEyes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanTrackHead: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ConfigurationState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GazeDeviceConfigurationStatePreview) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestCalibrationAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestCalibrationAsync: usize,
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))]
    pub GetNumericControlDescriptions: unsafe extern "system" fn(this: *mut *mut Self, usagepage: u16, usageid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections")))]
    GetNumericControlDescriptions: usize,
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))]
    pub GetBooleanControlDescriptions: unsafe extern "system" fn(this: *mut *mut Self, usagepage: u16, usageid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections")))]
    GetBooleanControlDescriptions: usize,
}
impl ::windows_sys::core::Interface for IGazeDevicePreview {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3885924073, data2: 45961, data3: 4583, data4: [178, 1, 200, 211, 255, 183, 87, 33] };
}
#[repr(C)]
pub struct IGazeDeviceWatcherAddedPreviewEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Device: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGazeDeviceWatcherAddedPreviewEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3885924077, data2: 45961, data3: 4583, data4: [178, 1, 200, 211, 255, 183, 87, 33] };
}
#[repr(C)]
pub struct IGazeDeviceWatcherPreview {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGazeDeviceWatcherPreview {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3885924071, data2: 45961, data3: 4583, data4: [178, 1, 200, 211, 255, 183, 87, 33] };
}
#[repr(C)]
pub struct IGazeDeviceWatcherRemovedPreviewEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Device: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGazeDeviceWatcherRemovedPreviewEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4066582280, data2: 3647, data3: 17183, data4: [166, 6, 80, 179, 90, 249, 74, 28] };
}
#[repr(C)]
pub struct IGazeDeviceWatcherUpdatedPreviewEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Device: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGazeDeviceWatcherUpdatedPreviewEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2145923311, data2: 32520, data3: 18231, data4: [136, 225, 74, 131, 174, 78, 72, 133] };
}
#[repr(C)]
pub struct IGazeEnteredPreviewEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CurrentPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGazeEnteredPreviewEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 627556163, data2: 4645, data3: 18591, data4: [157, 209, 218, 167, 197, 15, 191, 75] };
}
#[repr(C)]
pub struct IGazeExitedPreviewEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CurrentPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGazeExitedPreviewEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1560998014, data2: 32131, data3: 16623, data4: [159, 10, 251, 193, 187, 220, 197, 172] };
}
#[repr(C)]
pub struct IGazeInputSourcePreview {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GazeMoved: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GazeMoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGazeMoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGazeMoved: usize,
    #[cfg(feature = "Foundation")]
    pub GazeEntered: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GazeEntered: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGazeEntered: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGazeEntered: usize,
    #[cfg(feature = "Foundation")]
    pub GazeExited: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GazeExited: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGazeExited: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGazeExited: usize,
}
impl ::windows_sys::core::Interface for IGazeInputSourcePreview {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3885924072, data2: 45961, data3: 4583, data4: [178, 1, 200, 211, 255, 183, 87, 33] };
}
#[repr(C)]
pub struct IGazeInputSourcePreviewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGazeInputSourcePreviewStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3885924070, data2: 45961, data3: 4583, data4: [178, 1, 200, 211, 255, 183, 87, 33] };
}
#[repr(C)]
pub struct IGazeMovedPreviewEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CurrentPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetIntermediatePoints: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetIntermediatePoints: usize,
}
impl ::windows_sys::core::Interface for IGazeMovedPreviewEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3885924075, data2: 45961, data3: 4583, data4: [178, 1, 200, 211, 255, 183, 87, 33] };
}
#[repr(C)]
pub struct IGazePointPreview {
    pub base__: ::windows_sys::core::IInspectable,
    pub SourceDevice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EyeGazePosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EyeGazePosition: usize,
    #[cfg(feature = "Foundation")]
    pub HeadGazePosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HeadGazePosition: usize,
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_HumanInterfaceDevice")]
    pub HidInputReport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_HumanInterfaceDevice"))]
    HidInputReport: usize,
}
impl ::windows_sys::core::Interface for IGazePointPreview {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3885924074, data2: 45961, data3: 4583, data4: [178, 1, 200, 211, 255, 183, 87, 33] };
}
