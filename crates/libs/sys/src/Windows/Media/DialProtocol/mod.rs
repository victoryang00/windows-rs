pub type DialApp = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_DialProtocol\"`*"]
#[repr(transparent)]
pub struct DialAppLaunchResult(pub i32);
impl DialAppLaunchResult {
    pub const Launched: Self = Self(0i32);
    pub const FailedToLaunch: Self = Self(1i32);
    pub const NotFound: Self = Self(2i32);
    pub const NetworkFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for DialAppLaunchResult {}
impl ::core::clone::Clone for DialAppLaunchResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_DialProtocol\"`*"]
#[repr(transparent)]
pub struct DialAppState(pub i32);
impl DialAppState {
    pub const Unknown: Self = Self(0i32);
    pub const Stopped: Self = Self(1i32);
    pub const Running: Self = Self(2i32);
    pub const NetworkFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for DialAppState {}
impl ::core::clone::Clone for DialAppState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DialAppStateDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_DialProtocol\"`*"]
#[repr(transparent)]
pub struct DialAppStopResult(pub i32);
impl DialAppStopResult {
    pub const Stopped: Self = Self(0i32);
    pub const StopFailed: Self = Self(1i32);
    pub const OperationNotSupported: Self = Self(2i32);
    pub const NetworkFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for DialAppStopResult {}
impl ::core::clone::Clone for DialAppStopResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DialDevice = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_DialProtocol\"`*"]
#[repr(transparent)]
pub struct DialDeviceDisplayStatus(pub i32);
impl DialDeviceDisplayStatus {
    pub const None: Self = Self(0i32);
    pub const Connecting: Self = Self(1i32);
    pub const Connected: Self = Self(2i32);
    pub const Disconnecting: Self = Self(3i32);
    pub const Disconnected: Self = Self(4i32);
    pub const Error: Self = Self(5i32);
}
impl ::core::marker::Copy for DialDeviceDisplayStatus {}
impl ::core::clone::Clone for DialDeviceDisplayStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DialDevicePicker = *mut ::core::ffi::c_void;
pub type DialDevicePickerFilter = *mut ::core::ffi::c_void;
pub type DialDeviceSelectedEventArgs = *mut ::core::ffi::c_void;
pub type DialDisconnectButtonClickedEventArgs = *mut ::core::ffi::c_void;
pub type DialReceiverApp = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IDialApp {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestLaunchAsync: unsafe extern "system" fn(this: *mut *mut Self, appargument: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestLaunchAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAppStateAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppStateAsync: usize,
}
#[repr(C)]
pub struct IDialAppStateDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DialAppState) -> ::windows_sys::core::HRESULT,
    pub FullXml: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDialDevice {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDialApp: unsafe extern "system" fn(this: *mut *mut Self, appname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDialDevice2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub FriendlyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
}
#[repr(C)]
pub struct IDialDevicePicker {
    pub base__: ::windows_sys::core::IInspectable,
    pub Filter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub Appearance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    Appearance: usize,
    #[cfg(feature = "Foundation")]
    pub DialDeviceSelected: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DialDeviceSelected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDialDeviceSelected: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDialDeviceSelected: usize,
    #[cfg(feature = "Foundation")]
    pub DisconnectButtonClicked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisconnectButtonClicked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDisconnectButtonClicked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDisconnectButtonClicked: usize,
    #[cfg(feature = "Foundation")]
    pub DialDevicePickerDismissed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DialDevicePickerDismissed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDialDevicePickerDismissed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDialDevicePickerDismissed: usize,
    #[cfg(feature = "Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut *mut Self, selection: super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Show: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowWithPlacement: unsafe extern "system" fn(this: *mut *mut Self, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowWithPlacement: usize,
    #[cfg(feature = "Foundation")]
    pub PickSingleDialDeviceAsync: unsafe extern "system" fn(this: *mut *mut Self, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSingleDialDeviceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub PickSingleDialDeviceAsyncWithPlacement: unsafe extern "system" fn(this: *mut *mut Self, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    PickSingleDialDeviceAsyncWithPlacement: usize,
    pub Hide: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetDisplayStatus: unsafe extern "system" fn(this: *mut *mut Self, device: *mut ::core::ffi::c_void, status: DialDeviceDisplayStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDialDevicePickerFilter {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedAppNames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedAppNames: usize,
}
#[repr(C)]
pub struct IDialDeviceSelectedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SelectedDialDevice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDialDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, appname: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub DeviceInfoSupportsDialAsync: unsafe extern "system" fn(this: *mut *mut Self, device: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    DeviceInfoSupportsDialAsync: usize,
}
#[repr(C)]
pub struct IDialDisconnectButtonClickedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Device: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDialReceiverApp {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAdditionalDataAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAdditionalDataAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetAdditionalDataAsync: unsafe extern "system" fn(this: *mut *mut Self, additionaldata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetAdditionalDataAsync: usize,
}
#[repr(C)]
pub struct IDialReceiverApp2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetUniqueDeviceNameAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetUniqueDeviceNameAsync: usize,
}
#[repr(C)]
pub struct IDialReceiverAppStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Current: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
