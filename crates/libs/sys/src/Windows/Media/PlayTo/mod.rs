pub type CurrentTimeChangeRequestedEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct ICurrentTimeChangeRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Time: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Time: usize,
}
impl ::windows_sys::core::Interface for ICurrentTimeChangeRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2574324516, data2: 60871, data3: 19445, data4: [145, 246, 60, 134, 39, 219, 89, 229] };
}
#[repr(C)]
pub struct IMuteChangeRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mute: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMuteChangeRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3837064694, data2: 44831, data3: 20254, data4: [180, 55, 125, 163, 36, 0, 225, 212] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPlayToConnection {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PlayToConnectionState) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    State: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub StateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    StateChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveStateChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Transferred: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Transferred: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveTransferred: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveTransferred: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Error: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveError: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveError: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IPlayToConnection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 288341960, data2: 62005, data3: 20446, data4: [141, 65, 155, 242, 124, 158, 154, 64] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPlayToConnectionErrorEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Code: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PlayToConnectionError) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Code: usize,
    #[cfg(feature = "deprecated")]
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Message: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IPlayToConnectionErrorEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3210653094, data2: 35046, data3: 17503, data4: [157, 64, 217, 185, 248, 147, 152, 150] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPlayToConnectionStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub PreviousState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PlayToConnectionState) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PreviousState: usize,
    #[cfg(feature = "deprecated")]
    pub CurrentState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PlayToConnectionState) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CurrentState: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IPlayToConnectionStateChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1757721871, data2: 3104, data3: 18816, data4: [134, 2, 88, 198, 34, 56, 212, 35] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPlayToConnectionTransferredEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub PreviousSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PreviousSource: usize,
    #[cfg(feature = "deprecated")]
    pub CurrentSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CurrentSource: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IPlayToConnectionTransferredEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4209187130, data2: 1667, data3: 18393, data4: [141, 240, 24, 203, 180, 137, 132, 216] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPlayToManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SourceRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SourceRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSourceRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSourceRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SourceSelected: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SourceSelected: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSourceSelected: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSourceSelected: usize,
    #[cfg(feature = "deprecated")]
    pub SetDefaultSourceSelection: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetDefaultSourceSelection: usize,
    #[cfg(feature = "deprecated")]
    pub DefaultSourceSelection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DefaultSourceSelection: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IPlayToManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4117373038, data2: 7031, data3: 17135, data4: [143, 13, 185, 73, 248, 217, 178, 96] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPlayToManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetForCurrentView: usize,
    #[cfg(feature = "deprecated")]
    pub ShowPlayToUI: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShowPlayToUI: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IPlayToManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1692838023, data2: 14722, data3: 20283, data4: [186, 32, 97, 85, 228, 53, 50, 91] };
}
#[repr(C)]
pub struct IPlayToReceiver {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PlayRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlayRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlayRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlayRequested: usize,
    #[cfg(feature = "Foundation")]
    pub PauseRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PauseRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePauseRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePauseRequested: usize,
    #[cfg(feature = "Foundation")]
    pub SourceChangeRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSourceChangeRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSourceChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub PlaybackRateChangeRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackRateChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlaybackRateChangeRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlaybackRateChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub CurrentTimeChangeRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CurrentTimeChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCurrentTimeChangeRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCurrentTimeChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub MuteChangeRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MuteChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMuteChangeRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMuteChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub VolumeChangeRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VolumeChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVolumeChangeRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVolumeChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub TimeUpdateRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeUpdateRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTimeUpdateRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTimeUpdateRequested: usize,
    #[cfg(feature = "Foundation")]
    pub StopRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopRequested: usize,
    pub NotifyVolumeChange: unsafe extern "system" fn(this: *mut *mut Self, volume: f64, mute: bool) -> ::windows_sys::core::HRESULT,
    pub NotifyRateChange: unsafe extern "system" fn(this: *mut *mut Self, rate: f64) -> ::windows_sys::core::HRESULT,
    pub NotifyLoadedMetadata: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NotifyTimeUpdate: unsafe extern "system" fn(this: *mut *mut Self, currenttime: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NotifyTimeUpdate: usize,
    #[cfg(feature = "Foundation")]
    pub NotifyDurationChange: unsafe extern "system" fn(this: *mut *mut Self, duration: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NotifyDurationChange: usize,
    pub NotifySeeking: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub NotifySeeked: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub NotifyPaused: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub NotifyPlaying: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub NotifyEnded: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub NotifyError: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub NotifyStopped: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSupportsImage: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SupportsImage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSupportsAudio: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SupportsAudio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSupportsVideo: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SupportsVideo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopAsync: usize,
}
impl ::windows_sys::core::Interface for IPlayToReceiver {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2887110471, data2: 41314, data3: 19110, data4: [175, 27, 58, 163, 95, 59, 144, 105] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPlayToSource {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Connection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Connection: usize,
    #[cfg(feature = "deprecated")]
    pub Next: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Next: usize,
    #[cfg(feature = "deprecated")]
    pub SetNext: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetNext: usize,
    #[cfg(feature = "deprecated")]
    pub PlayNext: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PlayNext: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IPlayToSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2131986952, data2: 64439, data3: 19209, data4: [131, 86, 170, 95, 78, 51, 92, 49] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPlayToSourceDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Complete: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IPlayToSourceDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1090554141, data2: 10126, data3: 20265, data4: [133, 155, 169, 229, 1, 5, 62, 125] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPlayToSourceRequest {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Deadline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Deadline: usize,
    #[cfg(feature = "deprecated")]
    pub DisplayErrorString: unsafe extern "system" fn(this: *mut *mut Self, errorstring: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DisplayErrorString: usize,
    #[cfg(feature = "deprecated")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetDeferral: usize,
    #[cfg(feature = "deprecated")]
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetSource: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IPlayToSourceRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4166534757, data2: 25844, data3: 17568, data4: [172, 13, 70, 141, 43, 143, 218, 131] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPlayToSourceRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub SourceRequest: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SourceRequest: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IPlayToSourceRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3318596400, data2: 10719, data3: 20166, data4: [157, 169, 159, 189, 252, 252, 27, 62] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPlayToSourceSelectedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub FriendlyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FriendlyName: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub Icon: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    Icon: usize,
    #[cfg(feature = "deprecated")]
    pub SupportsImage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SupportsImage: usize,
    #[cfg(feature = "deprecated")]
    pub SupportsAudio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SupportsAudio: usize,
    #[cfg(feature = "deprecated")]
    pub SupportsVideo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SupportsVideo: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IPlayToSourceSelectedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 211649809, data2: 20994, data3: 19915, data4: [140, 103, 171, 218, 18, 187, 60, 18] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IPlayToSourceWithPreferredSourceUri {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PreferredSourceUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PreferredSourceUri: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetPreferredSourceUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetPreferredSourceUri: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IPlayToSourceWithPreferredSourceUri {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2863813611, data2: 13057, data3: 19908, data4: [175, 186, 178, 242, 237, 150, 53, 160] };
}
#[repr(C)]
pub struct IPlaybackRateChangeRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Rate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlaybackRateChangeRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 257319342, data2: 11400, data3: 19658, data4: [133, 64, 213, 134, 9, 93, 19, 165] };
}
#[repr(C)]
pub struct ISourceChangeRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Stream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Stream: usize,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Author: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Album: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Genre: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Date: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Date: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Foundation")]
    pub Rating: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Rating: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
impl ::windows_sys::core::Interface for ISourceChangeRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4215224982, data2: 31398, data3: 19083, data4: [134, 231, 84, 246, 198, 211, 79, 100] };
}
#[repr(C)]
pub struct IVolumeChangeRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Volume: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVolumeChangeRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1862430044, data2: 53109, data3: 19499, data4: [145, 62, 109, 124, 108, 50, 145, 121] };
}
pub type MuteChangeRequestedEventArgs = *mut ::core::ffi::c_void;
pub type PlayToConnection = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_PlayTo\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PlayToConnectionError(pub i32);
#[cfg(feature = "deprecated")]
impl PlayToConnectionError {
    pub const None: Self = Self(0i32);
    pub const DeviceNotResponding: Self = Self(1i32);
    pub const DeviceError: Self = Self(2i32);
    pub const DeviceLocked: Self = Self(3i32);
    pub const ProtectedPlaybackFailed: Self = Self(4i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for PlayToConnectionError {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToConnectionError {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PlayToConnectionErrorEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_PlayTo\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PlayToConnectionState(pub i32);
#[cfg(feature = "deprecated")]
impl PlayToConnectionState {
    pub const Disconnected: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
    pub const Rendering: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for PlayToConnectionState {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToConnectionState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PlayToConnectionStateChangedEventArgs = *mut ::core::ffi::c_void;
pub type PlayToConnectionTransferredEventArgs = *mut ::core::ffi::c_void;
pub type PlayToManager = *mut ::core::ffi::c_void;
pub type PlayToReceiver = *mut ::core::ffi::c_void;
pub type PlayToSource = *mut ::core::ffi::c_void;
pub type PlayToSourceDeferral = *mut ::core::ffi::c_void;
pub type PlayToSourceRequest = *mut ::core::ffi::c_void;
pub type PlayToSourceRequestedEventArgs = *mut ::core::ffi::c_void;
pub type PlayToSourceSelectedEventArgs = *mut ::core::ffi::c_void;
pub type PlaybackRateChangeRequestedEventArgs = *mut ::core::ffi::c_void;
pub type SourceChangeRequestedEventArgs = *mut ::core::ffi::c_void;
pub type VolumeChangeRequestedEventArgs = *mut ::core::ffi::c_void;
