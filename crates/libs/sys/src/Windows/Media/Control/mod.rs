pub type CurrentSessionChangedEventArgs = *mut ::core::ffi::c_void;
pub type GlobalSystemMediaTransportControlsSession = *mut ::core::ffi::c_void;
pub type GlobalSystemMediaTransportControlsSessionManager = *mut ::core::ffi::c_void;
pub type GlobalSystemMediaTransportControlsSessionMediaProperties = *mut ::core::ffi::c_void;
pub type GlobalSystemMediaTransportControlsSessionPlaybackControls = *mut ::core::ffi::c_void;
pub type GlobalSystemMediaTransportControlsSessionPlaybackInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Control\"`*"]
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionPlaybackStatus(pub i32);
impl GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    pub const Closed: Self = Self(0i32);
    pub const Opened: Self = Self(1i32);
    pub const Changing: Self = Self(2i32);
    pub const Stopped: Self = Self(3i32);
    pub const Playing: Self = Self(4i32);
    pub const Paused: Self = Self(5i32);
}
impl ::core::marker::Copy for GlobalSystemMediaTransportControlsSessionPlaybackStatus {}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GlobalSystemMediaTransportControlsSessionTimelineProperties = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct ICurrentSessionChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICurrentSessionChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1768540985, data2: 3066, data3: 24544, data4: [141, 115, 9, 204, 94, 84, 8, 225] };
}
#[repr(C)]
pub struct IGlobalSystemMediaTransportControlsSession {
    pub base__: ::windows_sys::core::IInspectable,
    pub SourceAppUserModelId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryGetMediaPropertiesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetMediaPropertiesAsync: usize,
    pub GetTimelineProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPlaybackInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryPlayAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryPlayAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryPauseAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryPauseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryStopAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryStopAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryRecordAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryRecordAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryFastForwardAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryFastForwardAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryRewindAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryRewindAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySkipNextAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySkipNextAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySkipPreviousAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySkipPreviousAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryChangeChannelUpAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryChangeChannelUpAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryChangeChannelDownAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryChangeChannelDownAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryTogglePlayPauseAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryTogglePlayPauseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryChangeAutoRepeatModeAsync: unsafe extern "system" fn(this: *mut *mut Self, requestedautorepeatmode: super::MediaPlaybackAutoRepeatMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryChangeAutoRepeatModeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryChangePlaybackRateAsync: unsafe extern "system" fn(this: *mut *mut Self, requestedplaybackrate: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryChangePlaybackRateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryChangeShuffleActiveAsync: unsafe extern "system" fn(this: *mut *mut Self, requestedshufflestate: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryChangeShuffleActiveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryChangePlaybackPositionAsync: unsafe extern "system" fn(this: *mut *mut Self, requestedplaybackposition: i64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryChangePlaybackPositionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TimelinePropertiesChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimelinePropertiesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTimelinePropertiesChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTimelinePropertiesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PlaybackInfoChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackInfoChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlaybackInfoChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlaybackInfoChanged: usize,
    #[cfg(feature = "Foundation")]
    pub MediaPropertiesChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MediaPropertiesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMediaPropertiesChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMediaPropertiesChanged: usize,
}
impl ::windows_sys::core::Interface for IGlobalSystemMediaTransportControlsSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1900595253, data2: 39700, data3: 23266, data4: [171, 133, 220, 155, 28, 20, 225, 168] };
}
#[repr(C)]
pub struct IGlobalSystemMediaTransportControlsSessionManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetCurrentSession: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSessions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSessions: usize,
    #[cfg(feature = "Foundation")]
    pub CurrentSessionChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CurrentSessionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCurrentSessionChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCurrentSessionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SessionsChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SessionsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSessionsChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSessionsChanged: usize,
}
impl ::windows_sys::core::Interface for IGlobalSystemMediaTransportControlsSessionManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3402534572, data2: 59502, data3: 20554, data4: [171, 49, 95, 248, 255, 27, 206, 73] };
}
#[repr(C)]
pub struct IGlobalSystemMediaTransportControlsSessionManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAsync: usize,
}
impl ::windows_sys::core::Interface for IGlobalSystemMediaTransportControlsSessionManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 542164206, data2: 4512, data3: 22494, data4: [174, 215, 201, 124, 112, 51, 130, 69] };
}
#[repr(C)]
pub struct IGlobalSystemMediaTransportControlsSessionMediaProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AlbumArtist: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Artist: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AlbumTitle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TrackNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Genres: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Genres: usize,
    pub AlbumTrackCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PlaybackType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackType: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
}
impl ::windows_sys::core::Interface for IGlobalSystemMediaTransportControlsSessionMediaProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1753574646, data2: 44468, data3: 21682, data4: [172, 22, 5, 131, 121, 7, 172, 182] };
}
#[repr(C)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackControls {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPlayEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsPauseEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsStopEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsRecordEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsFastForwardEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsRewindEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsNextEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsPreviousEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsChannelUpEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsChannelDownEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsPlayPauseToggleEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsShuffleEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsRepeatEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsPlaybackRateEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsPlaybackPositionEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGlobalSystemMediaTransportControlsSessionPlaybackControls {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1694606310, data2: 48250, data3: 20538, data4: [187, 27, 104, 241, 88, 243, 251, 3] };
}
#[repr(C)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub Controls: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlaybackStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GlobalSystemMediaTransportControlsSessionPlaybackStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PlaybackType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackType: usize,
    #[cfg(feature = "Foundation")]
    pub AutoRepeatMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AutoRepeatMode: usize,
    #[cfg(feature = "Foundation")]
    pub PlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackRate: usize,
    #[cfg(feature = "Foundation")]
    pub IsShuffleActive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsShuffleActive: usize,
}
impl ::windows_sys::core::Interface for IGlobalSystemMediaTransportControlsSessionPlaybackInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2494871247, data2: 59578, data3: 20909, data4: [135, 167, 193, 10, 222, 16, 97, 39] };
}
#[repr(C)]
pub struct IGlobalSystemMediaTransportControlsSessionTimelineProperties {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub EndTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndTime: usize,
    #[cfg(feature = "Foundation")]
    pub MinSeekTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinSeekTime: usize,
    #[cfg(feature = "Foundation")]
    pub MaxSeekTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxSeekTime: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub LastUpdatedTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastUpdatedTime: usize,
}
impl ::windows_sys::core::Interface for IGlobalSystemMediaTransportControlsSessionTimelineProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3991093558, data2: 28453, data3: 22669, data4: [142, 207, 234, 91, 103, 53, 170, 165] };
}
#[repr(C)]
pub struct IMediaPropertiesChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IMediaPropertiesChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2100773323, data2: 44528, data3: 23791, data4: [145, 186, 207, 171, 205, 215, 118, 120] };
}
#[repr(C)]
pub struct IPlaybackInfoChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IPlaybackInfoChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2020038338, data2: 48141, data3: 20645, data4: [136, 7, 5, 66, 145, 254, 241, 57] };
}
#[repr(C)]
pub struct ISessionsChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ISessionsChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3153120562, data2: 17092, data3: 23128, data4: [179, 23, 243, 75, 191, 189, 38, 224] };
}
#[repr(C)]
pub struct ITimelinePropertiesChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ITimelinePropertiesChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 688077359, data2: 51491, data3: 23159, data4: [188, 175, 5, 95, 244, 21, 173, 50] };
}
pub type MediaPropertiesChangedEventArgs = *mut ::core::ffi::c_void;
pub type PlaybackInfoChangedEventArgs = *mut ::core::ffi::c_void;
pub type SessionsChangedEventArgs = *mut ::core::ffi::c_void;
pub type TimelinePropertiesChangedEventArgs = *mut ::core::ffi::c_void;
