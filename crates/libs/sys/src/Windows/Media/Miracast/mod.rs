#[repr(C)]
pub struct IMiracastReceiver {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefaultSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCurrentSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetCurrentSettingsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCurrentSettingsAsync: usize,
    pub DisconnectAllAndApplySettings: unsafe extern "system" fn(this: *mut *mut Self, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DisconnectAllAndApplySettingsAsync: unsafe extern "system" fn(this: *mut *mut Self, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisconnectAllAndApplySettingsAsync: usize,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetStatusAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetStatusAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
    #[cfg(feature = "ApplicationModel_Core")]
    pub CreateSession: unsafe extern "system" fn(this: *mut *mut Self, view: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Core"))]
    CreateSession: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub CreateSessionAsync: unsafe extern "system" fn(this: *mut *mut Self, view: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))]
    CreateSessionAsync: usize,
    pub ClearKnownTransmitters: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RemoveKnownTransmitter: unsafe extern "system" fn(this: *mut *mut Self, transmitter: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMiracastReceiver {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2050052696, data2: 58436, data3: 20916, data4: [175, 247, 184, 141, 170, 18, 41, 224] };
}
#[repr(C)]
pub struct IMiracastReceiverApplySettingsResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MiracastReceiverApplySettingsStatus) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMiracastReceiverApplySettingsResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3500827250, data2: 2509, data3: 22753, data4: [164, 242, 93, 81, 67, 211, 18, 249] };
}
#[repr(C)]
pub struct IMiracastReceiverConnection {
    pub base__: ::windows_sys::core::IInspectable,
    pub Disconnect: unsafe extern "system" fn(this: *mut *mut Self, reason: MiracastReceiverDisconnectReason) -> ::windows_sys::core::HRESULT,
    pub DisconnectWithMessage: unsafe extern "system" fn(this: *mut *mut Self, reason: MiracastReceiverDisconnectReason, message: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PauseAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PauseAsync: usize,
    pub Resume: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ResumeAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResumeAsync: usize,
    pub Transmitter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InputDevices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CursorImageChannel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StreamControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMiracastReceiverConnection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1883975478, data2: 53989, data3: 21791, data4: [168, 84, 248, 34, 183, 145, 125, 40] };
}
#[repr(C)]
pub struct IMiracastReceiverConnectionCreatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Connection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Pin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IMiracastReceiverConnectionCreatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2106456633, data2: 12410, data3: 23567, data4: [148, 189, 208, 198, 157, 22, 153, 130] };
}
#[repr(C)]
pub struct IMiracastReceiverCursorImageChannel {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Graphics")]
    pub MaxImageSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::SizeInt32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    MaxImageSize: usize,
    #[cfg(feature = "Graphics")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::PointInt32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    Position: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ImageStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ImageStream: usize,
    #[cfg(feature = "Foundation")]
    pub ImageStreamChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImageStreamChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveImageStreamChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveImageStreamChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PositionChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePositionChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePositionChanged: usize,
}
impl ::windows_sys::core::Interface for IMiracastReceiverCursorImageChannel {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3651941165, data2: 29242, data3: 23197, data4: [185, 10, 129, 21, 62, 250, 42, 15] };
}
#[repr(C)]
pub struct IMiracastReceiverCursorImageChannelSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Graphics")]
    pub MaxImageSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::SizeInt32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    MaxImageSize: usize,
    #[cfg(feature = "Graphics")]
    pub SetMaxImageSize: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Graphics::SizeInt32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    SetMaxImageSize: usize,
}
impl ::windows_sys::core::Interface for IMiracastReceiverCursorImageChannelSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3436965375, data2: 48384, data3: 23452, data4: [142, 76, 0, 202, 207, 134, 182, 52] };
}
#[repr(C)]
pub struct IMiracastReceiverDisconnectedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Connection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMiracastReceiverDisconnectedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3651231326, data2: 24558, data3: 22502, data4: [180, 176, 4, 114, 125, 185, 50, 41] };
}
#[repr(C)]
pub struct IMiracastReceiverGameControllerDevice {
    pub base__: ::windows_sys::core::IInspectable,
    pub TransmitInput: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetTransmitInput: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsRequestedByTransmitter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsTransmittingInput: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MiracastReceiverGameControllerDeviceUsageMode) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, value: MiracastReceiverGameControllerDeviceUsageMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Changed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChanged: usize,
}
impl ::windows_sys::core::Interface for IMiracastReceiverGameControllerDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 762409448, data2: 48852, data3: 20760, data4: [160, 88, 226, 71, 126, 181, 136, 141] };
}
#[repr(C)]
pub struct IMiracastReceiverInputDevices {
    pub base__: ::windows_sys::core::IInspectable,
    pub Keyboard: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GameController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMiracastReceiverInputDevices {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3660954370, data2: 10410, data3: 24296, data4: [150, 245, 164, 41, 1, 198, 111, 0] };
}
#[repr(C)]
pub struct IMiracastReceiverKeyboardDevice {
    pub base__: ::windows_sys::core::IInspectable,
    pub TransmitInput: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetTransmitInput: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsRequestedByTransmitter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsTransmittingInput: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Changed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChanged: usize,
}
impl ::windows_sys::core::Interface for IMiracastReceiverKeyboardDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3199627890, data2: 1728, data3: 21759, data4: [172, 150, 33, 116, 100, 255, 37, 1] };
}
#[repr(C)]
pub struct IMiracastReceiverMediaSourceCreatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Connection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_Core")]
    pub MediaSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    MediaSource: usize,
    pub CursorImageChannelSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IMiracastReceiverMediaSourceCreatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 399462814, data2: 4678, data3: 21277, data4: [148, 90, 107, 21, 142, 57, 195, 170] };
}
#[repr(C)]
pub struct IMiracastReceiverSession {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ConnectionCreated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionCreated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionCreated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionCreated: usize,
    #[cfg(feature = "Foundation")]
    pub MediaSourceCreated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MediaSourceCreated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMediaSourceCreated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMediaSourceCreated: usize,
    #[cfg(feature = "Foundation")]
    pub Disconnected: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Disconnected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDisconnected: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDisconnected: usize,
    pub AllowConnectionTakeover: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowConnectionTakeover: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub MaxSimultaneousConnections: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxSimultaneousConnections: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
}
impl ::windows_sys::core::Interface for IMiracastReceiverSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 489409972, data2: 61323, data3: 21001, data4: [191, 201, 195, 33, 22, 80, 72, 3] };
}
#[repr(C)]
pub struct IMiracastReceiverSessionStartResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MiracastReceiverSessionStartStatus) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMiracastReceiverSessionStartResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3083170798, data2: 16586, data3: 20991, data4: [149, 242, 201, 222, 52, 242, 233, 14] };
}
#[repr(C)]
pub struct IMiracastReceiverSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub FriendlyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ModelName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetModelName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ModelNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetModelNumber: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AuthorizationMethod: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MiracastReceiverAuthorizationMethod) -> ::windows_sys::core::HRESULT,
    pub SetAuthorizationMethod: unsafe extern "system" fn(this: *mut *mut Self, value: MiracastReceiverAuthorizationMethod) -> ::windows_sys::core::HRESULT,
    pub RequireAuthorizationFromKnownTransmitters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetRequireAuthorizationFromKnownTransmitters: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMiracastReceiverSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1473064740, data2: 50522, data3: 24510, data4: [148, 100, 235, 5, 48, 119, 5, 221] };
}
#[repr(C)]
pub struct IMiracastReceiverStatus {
    pub base__: ::windows_sys::core::IInspectable,
    pub ListeningStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MiracastReceiverListeningStatus) -> ::windows_sys::core::HRESULT,
    pub WiFiStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MiracastReceiverWiFiStatus) -> ::windows_sys::core::HRESULT,
    pub IsConnectionTakeoverSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MaxSimultaneousConnections: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub KnownTransmitters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    KnownTransmitters: usize,
}
impl ::windows_sys::core::Interface for IMiracastReceiverStatus {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3263845777, data2: 9131, data3: 20894, data4: [173, 9, 144, 191, 246, 220, 200, 126] };
}
#[repr(C)]
pub struct IMiracastReceiverStreamControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetVideoStreamSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetVideoStreamSettingsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetVideoStreamSettingsAsync: usize,
    pub SuggestVideoStreamSettings: unsafe extern "system" fn(this: *mut *mut Self, settings: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SuggestVideoStreamSettingsAsync: unsafe extern "system" fn(this: *mut *mut Self, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SuggestVideoStreamSettingsAsync: usize,
    pub MuteAudio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetMuteAudio: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMiracastReceiverStreamControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 954871179, data2: 10089, data3: 23255, data4: [138, 138, 37, 75, 157, 247, 186, 130] };
}
#[repr(C)]
pub struct IMiracastReceiverVideoStreamSettings {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics")]
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::SizeInt32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    Size: usize,
    #[cfg(feature = "Graphics")]
    pub SetSize: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Graphics::SizeInt32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    SetSize: usize,
    pub Bitrate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetBitrate: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMiracastReceiverVideoStreamSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 379280923, data2: 5277, data3: 21200, data4: [177, 38, 111, 137, 116, 78, 79, 80] };
}
#[repr(C)]
pub struct IMiracastTransmitter {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AuthorizationStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MiracastTransmitterAuthorizationStatus) -> ::windows_sys::core::HRESULT,
    pub SetAuthorizationStatus: unsafe extern "system" fn(this: *mut *mut Self, value: MiracastTransmitterAuthorizationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConnections: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConnections: usize,
    pub MacAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastConnectionTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastConnectionTime: usize,
}
impl ::windows_sys::core::Interface for IMiracastTransmitter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 875395581, data2: 11876, data3: 21768, data4: [138, 48, 131, 61, 30, 172, 112, 208] };
}
pub type MiracastReceiver = *mut ::core::ffi::c_void;
pub type MiracastReceiverApplySettingsResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverApplySettingsStatus(pub i32);
impl MiracastReceiverApplySettingsStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const MiracastNotSupported: Self = Self(2i32);
    pub const AccessDenied: Self = Self(3i32);
    pub const FriendlyNameTooLong: Self = Self(4i32);
    pub const ModelNameTooLong: Self = Self(5i32);
    pub const ModelNumberTooLong: Self = Self(6i32);
    pub const InvalidSettings: Self = Self(7i32);
}
impl ::core::marker::Copy for MiracastReceiverApplySettingsStatus {}
impl ::core::clone::Clone for MiracastReceiverApplySettingsStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverAuthorizationMethod(pub i32);
impl MiracastReceiverAuthorizationMethod {
    pub const None: Self = Self(0i32);
    pub const ConfirmConnection: Self = Self(1i32);
    pub const PinDisplayIfRequested: Self = Self(2i32);
    pub const PinDisplayRequired: Self = Self(3i32);
}
impl ::core::marker::Copy for MiracastReceiverAuthorizationMethod {}
impl ::core::clone::Clone for MiracastReceiverAuthorizationMethod {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MiracastReceiverConnection = *mut ::core::ffi::c_void;
pub type MiracastReceiverConnectionCreatedEventArgs = *mut ::core::ffi::c_void;
pub type MiracastReceiverCursorImageChannel = *mut ::core::ffi::c_void;
pub type MiracastReceiverCursorImageChannelSettings = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverDisconnectReason(pub i32);
impl MiracastReceiverDisconnectReason {
    pub const Finished: Self = Self(0i32);
    pub const AppSpecificError: Self = Self(1i32);
    pub const ConnectionNotAccepted: Self = Self(2i32);
    pub const DisconnectedByUser: Self = Self(3i32);
    pub const FailedToStartStreaming: Self = Self(4i32);
    pub const MediaDecodingError: Self = Self(5i32);
    pub const MediaStreamingError: Self = Self(6i32);
    pub const MediaDecryptionError: Self = Self(7i32);
}
impl ::core::marker::Copy for MiracastReceiverDisconnectReason {}
impl ::core::clone::Clone for MiracastReceiverDisconnectReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MiracastReceiverDisconnectedEventArgs = *mut ::core::ffi::c_void;
pub type MiracastReceiverGameControllerDevice = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverGameControllerDeviceUsageMode(pub i32);
impl MiracastReceiverGameControllerDeviceUsageMode {
    pub const AsGameController: Self = Self(0i32);
    pub const AsMouseAndKeyboard: Self = Self(1i32);
}
impl ::core::marker::Copy for MiracastReceiverGameControllerDeviceUsageMode {}
impl ::core::clone::Clone for MiracastReceiverGameControllerDeviceUsageMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MiracastReceiverInputDevices = *mut ::core::ffi::c_void;
pub type MiracastReceiverKeyboardDevice = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverListeningStatus(pub i32);
impl MiracastReceiverListeningStatus {
    pub const NotListening: Self = Self(0i32);
    pub const Listening: Self = Self(1i32);
    pub const ConnectionPending: Self = Self(2i32);
    pub const Connected: Self = Self(3i32);
    pub const DisabledByPolicy: Self = Self(4i32);
    pub const TemporarilyDisabled: Self = Self(5i32);
}
impl ::core::marker::Copy for MiracastReceiverListeningStatus {}
impl ::core::clone::Clone for MiracastReceiverListeningStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MiracastReceiverMediaSourceCreatedEventArgs = *mut ::core::ffi::c_void;
pub type MiracastReceiverSession = *mut ::core::ffi::c_void;
pub type MiracastReceiverSessionStartResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverSessionStartStatus(pub i32);
impl MiracastReceiverSessionStartStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const MiracastNotSupported: Self = Self(2i32);
    pub const AccessDenied: Self = Self(3i32);
}
impl ::core::marker::Copy for MiracastReceiverSessionStartStatus {}
impl ::core::clone::Clone for MiracastReceiverSessionStartStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MiracastReceiverSettings = *mut ::core::ffi::c_void;
pub type MiracastReceiverStatus = *mut ::core::ffi::c_void;
pub type MiracastReceiverStreamControl = *mut ::core::ffi::c_void;
pub type MiracastReceiverVideoStreamSettings = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverWiFiStatus(pub i32);
impl MiracastReceiverWiFiStatus {
    pub const MiracastSupportUndetermined: Self = Self(0i32);
    pub const MiracastNotSupported: Self = Self(1i32);
    pub const MiracastSupportNotOptimized: Self = Self(2i32);
    pub const MiracastSupported: Self = Self(3i32);
}
impl ::core::marker::Copy for MiracastReceiverWiFiStatus {}
impl ::core::clone::Clone for MiracastReceiverWiFiStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MiracastTransmitter = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastTransmitterAuthorizationStatus(pub i32);
impl MiracastTransmitterAuthorizationStatus {
    pub const Undecided: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const AlwaysPrompt: Self = Self(2i32);
    pub const Blocked: Self = Self(3i32);
}
impl ::core::marker::Copy for MiracastTransmitterAuthorizationStatus {}
impl ::core::clone::Clone for MiracastTransmitterAuthorizationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
