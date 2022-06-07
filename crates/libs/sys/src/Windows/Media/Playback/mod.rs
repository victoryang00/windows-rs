#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct AutoLoadedDisplayPropertyKind(pub i32);
impl AutoLoadedDisplayPropertyKind {
    pub const None: Self = Self(0i32);
    pub const MusicOrVideo: Self = Self(1i32);
    pub const Music: Self = Self(2i32);
    pub const Video: Self = Self(3i32);
}
impl ::core::marker::Copy for AutoLoadedDisplayPropertyKind {}
impl ::core::clone::Clone for AutoLoadedDisplayPropertyKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CurrentMediaPlaybackItemChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct FailedMediaStreamKind(pub i32);
impl FailedMediaStreamKind {
    pub const Unknown: Self = Self(0i32);
    pub const Audio: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for FailedMediaStreamKind {}
impl ::core::clone::Clone for FailedMediaStreamKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IBackgroundMediaPlayerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Current: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Current: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub MessageReceivedFromBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    MessageReceivedFromBackground: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveMessageReceivedFromBackground: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveMessageReceivedFromBackground: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub MessageReceivedFromForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    MessageReceivedFromForeground: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveMessageReceivedFromForeground: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveMessageReceivedFromForeground: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub SendMessageToBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    SendMessageToBackground: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub SendMessageToForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    SendMessageToForeground: usize,
    #[cfg(feature = "deprecated")]
    pub IsMediaPlaying: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsMediaPlaying: usize,
    #[cfg(feature = "deprecated")]
    pub Shutdown: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Shutdown: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IBackgroundMediaPlayerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2238569409, data2: 22007, data3: 18207, data4: [160, 242, 104, 172, 76, 144, 69, 146] };
}
#[repr(C)]
pub struct ICurrentMediaPlaybackItemChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub NewItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OldItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICurrentMediaPlaybackItemChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 390310034, data2: 23619, data3: 18965, data4: [150, 122, 87, 45, 45, 15, 38, 198] };
}
#[repr(C)]
pub struct ICurrentMediaPlaybackItemChangedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaPlaybackItemChangedReason) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICurrentMediaPlaybackItemChangedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 494970142, data2: 39278, data3: 16553, data4: [190, 72, 230, 110, 201, 11, 43, 125] };
}
#[repr(C)]
pub struct IMediaBreak {
    pub base__: ::windows_sys::core::IInspectable,
    pub PlaybackList: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PresentationPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PresentationPosition: usize,
    pub InsertionMethod: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaBreakInsertionMethod) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CustomProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CustomProperties: usize,
    pub CanStart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanStart: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaBreak {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1900798576, data2: 3567, data3: 20156, data4: [164, 137, 107, 52, 147, 14, 21, 88] };
}
#[repr(C)]
pub struct IMediaBreakEndedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub MediaBreak: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaBreakEndedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 850997878, data2: 7261, data3: 20462, data4: [135, 50, 35, 109, 195, 168, 133, 128] };
}
#[repr(C)]
pub struct IMediaBreakFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, insertionmethod: MediaBreakInsertionMethod, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateWithPresentationPosition: unsafe extern "system" fn(this: *mut *mut Self, insertionmethod: MediaBreakInsertionMethod, presentationposition: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWithPresentationPosition: usize,
}
impl ::windows_sys::core::Interface for IMediaBreakFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1159127042, data2: 6368, data3: 16505, data4: [139, 95, 211, 52, 149, 193, 93, 46] };
}
#[repr(C)]
pub struct IMediaBreakManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub BreaksSeekedOver: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BreaksSeekedOver: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBreaksSeekedOver: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBreaksSeekedOver: usize,
    #[cfg(feature = "Foundation")]
    pub BreakStarted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BreakStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBreakStarted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBreakStarted: usize,
    #[cfg(feature = "Foundation")]
    pub BreakEnded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BreakEnded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBreakEnded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBreakEnded: usize,
    #[cfg(feature = "Foundation")]
    pub BreakSkipped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BreakSkipped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBreakSkipped: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBreakSkipped: usize,
    pub CurrentBreak: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlaybackSession: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlayBreak: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SkipCurrentBreak: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaBreakManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2824134065, data2: 65204, data3: 19867, data4: [157, 151, 15, 219, 229, 142, 94, 57] };
}
#[repr(C)]
pub struct IMediaBreakSchedule {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ScheduleChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScheduleChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveScheduleChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveScheduleChanged: usize,
    pub InsertMidrollBreak: unsafe extern "system" fn(this: *mut *mut Self, mediabreak: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveMidrollBreak: unsafe extern "system" fn(this: *mut *mut Self, mediabreak: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MidrollBreaks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MidrollBreaks: usize,
    pub SetPrerollBreak: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PrerollBreak: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPostrollBreak: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PostrollBreak: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlaybackItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaBreakSchedule {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2711246867, data2: 39094, data3: 16856, data4: [131, 218, 249, 113, 210, 43, 123, 186] };
}
#[repr(C)]
pub struct IMediaBreakSeekedOverEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SeekedOverBreaks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SeekedOverBreaks: usize,
    #[cfg(feature = "Foundation")]
    pub OldPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OldPosition: usize,
    #[cfg(feature = "Foundation")]
    pub NewPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NewPosition: usize,
}
impl ::windows_sys::core::Interface for IMediaBreakSeekedOverEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3853150022, data2: 1542, data3: 17554, data4: [185, 211, 195, 200, 253, 224, 164, 234] };
}
#[repr(C)]
pub struct IMediaBreakSkippedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub MediaBreak: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaBreakSkippedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1860783109, data2: 12116, data3: 19006, data4: [163, 171, 36, 195, 178, 112, 180, 163] };
}
#[repr(C)]
pub struct IMediaBreakStartedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub MediaBreak: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaBreakStartedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2826894961, data2: 57300, data3: 17738, data4: [149, 110, 10, 74, 100, 131, 149, 248] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IMediaEnginePlaybackSource {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub CurrentItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CurrentItem: usize,
    #[cfg(feature = "deprecated")]
    pub SetPlaybackSource: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetPlaybackSource: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IMediaEnginePlaybackSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1545407399, data2: 14422, data3: 18617, data4: [141, 198, 36, 75, 241, 7, 191, 140] };
}
#[repr(C)]
pub struct IMediaItemDisplayProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::MediaPlaybackType) -> ::windows_sys::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut *mut Self, value: super::MediaPlaybackType) -> ::windows_sys::core::HRESULT,
    pub MusicProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VideoProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetThumbnail: usize,
    pub ClearAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaItemDisplayProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 507255624, data2: 28823, data3: 17284, data4: [162, 23, 193, 41, 29, 250, 140, 22] };
}
#[repr(C)]
pub struct IMediaPlaybackCommandManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub MediaPlayer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PlayBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PauseBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NextBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PreviousBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FastForwardBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RewindBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ShuffleBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AutoRepeatModeBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PositionBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RateBehavior: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PlayReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlayReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlayReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlayReceived: usize,
    #[cfg(feature = "Foundation")]
    pub PauseReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PauseReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePauseReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePauseReceived: usize,
    #[cfg(feature = "Foundation")]
    pub NextReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NextReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNextReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNextReceived: usize,
    #[cfg(feature = "Foundation")]
    pub PreviousReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PreviousReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePreviousReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePreviousReceived: usize,
    #[cfg(feature = "Foundation")]
    pub FastForwardReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FastForwardReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFastForwardReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFastForwardReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RewindReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RewindReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRewindReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRewindReceived: usize,
    #[cfg(feature = "Foundation")]
    pub ShuffleReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShuffleReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShuffleReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShuffleReceived: usize,
    #[cfg(feature = "Foundation")]
    pub AutoRepeatModeReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AutoRepeatModeReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAutoRepeatModeReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAutoRepeatModeReceived: usize,
    #[cfg(feature = "Foundation")]
    pub PositionReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PositionReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePositionReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePositionReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RateReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RateReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRateReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRateReceived: usize,
}
impl ::windows_sys::core::Interface for IMediaPlaybackCommandManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1523508646, data2: 23734, data3: 19034, data4: [133, 33, 204, 134, 177, 193, 237, 55] };
}
#[repr(C)]
pub struct IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AutoRepeatMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::MediaPlaybackAutoRepeatMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1030704931, data2: 21040, data3: 17425, data4: [160, 233, 186, 217, 76, 42, 4, 92] };
}
#[repr(C)]
pub struct IMediaPlaybackCommandManagerCommandBehavior {
    pub base__: ::windows_sys::core::IInspectable,
    pub CommandManager: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub EnablingRule: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaCommandEnablingRule) -> ::windows_sys::core::HRESULT,
    pub SetEnablingRule: unsafe extern "system" fn(this: *mut *mut Self, value: MediaCommandEnablingRule) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsEnabledChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsEnabledChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsEnabledChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsEnabledChanged: usize,
}
impl ::windows_sys::core::Interface for IMediaPlaybackCommandManagerCommandBehavior {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2020351608, data2: 52856, data3: 18960, data4: [175, 214, 132, 63, 203, 185, 12, 46] };
}
#[repr(C)]
pub struct IMediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IMediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 821060825, data2: 46225, data3: 19722, data4: [188, 33, 48, 152, 189, 19, 50, 233] };
}
#[repr(C)]
pub struct IMediaPlaybackCommandManagerNextReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IMediaPlaybackCommandManagerNextReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3780133939, data2: 41648, data3: 17876, data4: [185, 222, 95, 66, 172, 20, 168, 57] };
}
#[repr(C)]
pub struct IMediaPlaybackCommandManagerPauseReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IMediaPlaybackCommandManagerPauseReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1559022876, data2: 49756, data3: 16929, data4: [177, 108, 195, 201, 140, 224, 18, 214] };
}
#[repr(C)]
pub struct IMediaPlaybackCommandManagerPlayReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IMediaPlaybackCommandManagerPlayReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2599419982, data2: 22411, data3: 19542, data4: [160, 6, 22, 21, 157, 136, 138, 72] };
}
#[repr(C)]
pub struct IMediaPlaybackCommandManagerPositionReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IMediaPlaybackCommandManagerPositionReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1435608916, data2: 54823, data3: 19421, data4: [169, 13, 134, 160, 21, 178, 73, 2] };
}
#[repr(C)]
pub struct IMediaPlaybackCommandManagerPreviousReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IMediaPlaybackCommandManagerPreviousReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1381904513, data2: 17970, data3: 20342, data4: [153, 177, 215, 113, 98, 63, 98, 135] };
}
#[repr(C)]
pub struct IMediaPlaybackCommandManagerRateReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub PlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IMediaPlaybackCommandManagerRateReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 418003257, data2: 18966, data3: 16745, data4: [139, 5, 62, 185, 245, 255, 120, 235] };
}
#[repr(C)]
pub struct IMediaPlaybackCommandManagerRewindReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IMediaPlaybackCommandManagerRewindReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2668124487, data2: 41920, data3: 16989, data4: [170, 239, 151, 186, 120, 152, 177, 65] };
}
#[repr(C)]
pub struct IMediaPlaybackCommandManagerShuffleReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsShuffleRequested: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IMediaPlaybackCommandManagerShuffleReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1352686831, data2: 25582, data3: 19094, data4: [183, 181, 254, 224, 139, 159, 249, 12] };
}
#[repr(C)]
pub struct IMediaPlaybackItem {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AudioTracksChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AudioTracksChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioTracksChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioTracksChanged: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub VideoTracksChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    VideoTracksChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVideoTracksChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVideoTracksChanged: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TimedMetadataTracksChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TimedMetadataTracksChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTimedMetadataTracksChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTimedMetadataTracksChanged: usize,
    #[cfg(feature = "Media_Core")]
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    Source: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AudioTracks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AudioTracks: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub VideoTracks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    VideoTracks: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TimedMetadataTracks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TimedMetadataTracks: usize,
}
impl ::windows_sys::core::Interface for IMediaPlaybackItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 74487762, data2: 58543, data3: 18603, data4: [178, 131, 105, 41, 230, 116, 236, 226] };
}
#[repr(C)]
pub struct IMediaPlaybackItem2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BreakSchedule: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub DurationLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DurationLimit: usize,
    pub CanSkip: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanSkip: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub GetDisplayProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ApplyDisplayProperties: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaPlaybackItem2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3629764977, data2: 55279, data3: 19329, data4: [172, 31, 244, 4, 147, 203, 176, 145] };
}
#[repr(C)]
pub struct IMediaPlaybackItem3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsDisabledInPlaybackList: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsDisabledInPlaybackList: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub TotalDownloadProgress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub AutoLoadedDisplayProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AutoLoadedDisplayPropertyKind) -> ::windows_sys::core::HRESULT,
    pub SetAutoLoadedDisplayProperties: unsafe extern "system" fn(this: *mut *mut Self, value: AutoLoadedDisplayPropertyKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaPlaybackItem3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 221413920, data2: 47114, data3: 19721, data4: [159, 248, 248, 112, 148, 161, 200, 49] };
}
#[repr(C)]
pub struct IMediaPlaybackItemError {
    pub base__: ::windows_sys::core::IInspectable,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaPlaybackItemErrorCode) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaPlaybackItemError {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1778118443, data2: 56534, data3: 19961, data4: [164, 80, 219, 244, 198, 241, 194, 194] };
}
#[repr(C)]
pub struct IMediaPlaybackItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Core")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IMediaPlaybackItemFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1899232481, data2: 5993, data3: 20473, data4: [167, 193, 56, 210, 196, 212, 35, 96] };
}
#[repr(C)]
pub struct IMediaPlaybackItemFactory2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub CreateWithStartTime: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, starttime: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Core")))]
    CreateWithStartTime: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub CreateWithStartTimeAndDurationLimit: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, starttime: super::super::Foundation::TimeSpan, durationlimit: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Core")))]
    CreateWithStartTimeAndDurationLimit: usize,
}
impl ::windows_sys::core::Interface for IMediaPlaybackItemFactory2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3615285050, data2: 47431, data3: 18802, data4: [179, 93, 173, 251, 147, 26, 113, 230] };
}
#[repr(C)]
pub struct IMediaPlaybackItemFailedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaPlaybackItemFailedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1996690250, data2: 59815, data3: 18371, data4: [134, 44, 198, 86, 211, 6, 131, 212] };
}
#[repr(C)]
pub struct IMediaPlaybackItemOpenedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaPlaybackItemOpenedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3420044674, data2: 12343, data3: 20414, data4: [174, 143, 57, 252, 57, 237, 244, 239] };
}
#[repr(C)]
pub struct IMediaPlaybackItemStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Core")]
    pub FindFromMediaSource: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    FindFromMediaSource: usize,
}
impl ::windows_sys::core::Interface for IMediaPlaybackItemStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1260120052, data2: 17221, data3: 16444, data4: [138, 103, 245, 222, 145, 223, 76, 134] };
}
#[repr(C)]
pub struct IMediaPlaybackList {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ItemFailed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemFailed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemFailed: usize,
    #[cfg(feature = "Foundation")]
    pub CurrentItemChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CurrentItemChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCurrentItemChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCurrentItemChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ItemOpened: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemOpened: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemOpened: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemOpened: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
    pub AutoRepeatEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAutoRepeatEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ShuffleEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShuffleEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CurrentItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CurrentItemIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MovePrevious: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MoveTo: unsafe extern "system" fn(this: *mut *mut Self, itemindex: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaPlaybackList {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2138566300, data2: 56386, data3: 20006, data4: [169, 141, 120, 80, 223, 142, 201, 37] };
}
#[repr(C)]
pub struct IMediaPlaybackList2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub MaxPrefetchTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxPrefetchTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxPrefetchTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxPrefetchTime: usize,
    pub StartingItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetStartingItem: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ShuffledItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ShuffledItems: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetShuffledItems: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetShuffledItems: usize,
}
impl ::windows_sys::core::Interface for IMediaPlaybackList2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 235517048, data2: 24586, data3: 17012, data4: [161, 75, 11, 103, 35, 208, 244, 139] };
}
#[repr(C)]
pub struct IMediaPlaybackList3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub MaxPlayedItemsToKeepOpen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxPlayedItemsToKeepOpen: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxPlayedItemsToKeepOpen: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxPlayedItemsToKeepOpen: usize,
}
impl ::windows_sys::core::Interface for IMediaPlaybackList3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3710172073, data2: 48199, data3: 17507, data4: [170, 144, 193, 139, 126, 95, 253, 225] };
}
#[repr(C)]
pub struct IMediaPlaybackSession {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PlaybackStateChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlaybackStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlaybackStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PlaybackRateChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackRateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlaybackRateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlaybackRateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SeekCompleted: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SeekCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSeekCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSeekCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub BufferingStarted: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BufferingStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBufferingStarted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBufferingStarted: usize,
    #[cfg(feature = "Foundation")]
    pub BufferingEnded: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BufferingEnded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBufferingEnded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBufferingEnded: usize,
    #[cfg(feature = "Foundation")]
    pub BufferingProgressChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BufferingProgressChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBufferingProgressChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBufferingProgressChanged: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadProgressChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadProgressChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDownloadProgressChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDownloadProgressChanged: usize,
    #[cfg(feature = "Foundation")]
    pub NaturalDurationChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NaturalDurationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNaturalDurationChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNaturalDurationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PositionChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePositionChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub NaturalVideoSizeChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NaturalVideoSizeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNaturalVideoSizeChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNaturalVideoSizeChanged: usize,
    pub MediaPlayer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NaturalDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NaturalDuration: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub SetPosition: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPosition: usize,
    pub PlaybackState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaPlaybackState) -> ::windows_sys::core::HRESULT,
    pub CanSeek: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanPause: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsProtected: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub PlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetPlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub BufferingProgress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub DownloadProgress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub NaturalVideoHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub NaturalVideoWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NormalizedSourceRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NormalizedSourceRect: usize,
    #[cfg(feature = "Foundation")]
    pub SetNormalizedSourceRect: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetNormalizedSourceRect: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub StereoscopicVideoPackingMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::MediaProperties::StereoscopicVideoPackingMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    StereoscopicVideoPackingMode: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetStereoscopicVideoPackingMode: unsafe extern "system" fn(this: *mut *mut Self, value: super::MediaProperties::StereoscopicVideoPackingMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetStereoscopicVideoPackingMode: usize,
}
impl ::windows_sys::core::Interface for IMediaPlaybackSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3274401853, data2: 1031, data3: 16826, data4: [137, 70, 139, 52, 90, 90, 84, 53] };
}
#[repr(C)]
pub struct IMediaPlaybackSession2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub BufferedRangesChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BufferedRangesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBufferedRangesChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBufferedRangesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PlayedRangesChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlayedRangesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlayedRangesChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlayedRangesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SeekableRangesChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SeekableRangesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSeekableRangesChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSeekableRangesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SupportedPlaybackRatesChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SupportedPlaybackRatesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSupportedPlaybackRatesChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSupportedPlaybackRatesChanged: usize,
    pub SphericalVideoProjection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsMirroring: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsMirroring: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetBufferedRanges: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetBufferedRanges: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPlayedRanges: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPlayedRanges: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSeekableRanges: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSeekableRanges: usize,
    pub IsSupportedPlaybackRateRange: unsafe extern "system" fn(this: *mut *mut Self, rate1: f64, rate2: f64, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaPlaybackSession2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4172971129, data2: 8136, data3: 16535, data4: [173, 112, 192, 250, 24, 204, 0, 80] };
}
#[repr(C)]
pub struct IMediaPlaybackSession3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_MediaProperties")]
    pub PlaybackRotation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::MediaProperties::MediaRotation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    PlaybackRotation: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetPlaybackRotation: unsafe extern "system" fn(this: *mut *mut Self, value: super::MediaProperties::MediaRotation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetPlaybackRotation: usize,
    pub GetOutputDegradationPolicyState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaPlaybackSession3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2074260506, data2: 41954, data3: 16479, data4: [183, 123, 164, 129, 44, 35, 139, 102] };
}
#[repr(C)]
pub struct IMediaPlaybackSessionBufferingStartedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPlaybackInterruption: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaPlaybackSessionBufferingStartedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3446321133, data2: 29922, data3: 17333, data4: [177, 21, 118, 35, 108, 51, 121, 26] };
}
#[repr(C)]
pub struct IMediaPlaybackSessionOutputDegradationPolicyState {
    pub base__: ::windows_sys::core::IInspectable,
    pub VideoConstrictionReason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaPlaybackSessionVideoConstrictionReason) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaPlaybackSessionOutputDegradationPolicyState {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1435398781, data2: 63027, data3: 18937, data4: [150, 90, 171, 170, 29, 183, 9, 190] };
}
#[repr(C)]
pub struct IMediaPlaybackSource {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IMediaPlaybackSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4020093628, data2: 37655, data3: 18070, data4: [176, 81, 43, 173, 100, 49, 119, 181] };
}
#[repr(C)]
pub struct IMediaPlaybackSphericalVideoProjection {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub FrameFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::MediaProperties::SphericalVideoFrameFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    FrameFormat: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetFrameFormat: unsafe extern "system" fn(this: *mut *mut Self, value: super::MediaProperties::SphericalVideoFrameFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetFrameFormat: usize,
    pub HorizontalFieldOfViewInDegrees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalFieldOfViewInDegrees: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub ViewOrientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ViewOrientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetViewOrientation: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Quaternion) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetViewOrientation: usize,
    pub ProjectionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SphericalVideoProjectionMode) -> ::windows_sys::core::HRESULT,
    pub SetProjectionMode: unsafe extern "system" fn(this: *mut *mut Self, value: SphericalVideoProjectionMode) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaPlaybackSphericalVideoProjection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3557143420, data2: 28430, data3: 18017, data4: [184, 238, 212, 135, 186, 151, 82, 213] };
}
#[repr(C)]
pub struct IMediaPlaybackTimedMetadataTrackList {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub PresentationModeChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PresentationModeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePresentationModeChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePresentationModeChanged: usize,
    pub GetPresentationMode: unsafe extern "system" fn(this: *mut *mut Self, index: u32, result__: *mut TimedMetadataTrackPresentationMode) -> ::windows_sys::core::HRESULT,
    pub SetPresentationMode: unsafe extern "system" fn(this: *mut *mut Self, index: u32, value: TimedMetadataTrackPresentationMode) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaPlaybackTimedMetadataTrackList {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1924403993, data2: 48123, data3: 18083, data4: [147, 114, 156, 156, 116, 75, 148, 56] };
}
#[repr(C)]
pub struct IMediaPlayer {
    pub base__: ::windows_sys::core::IInspectable,
    pub AutoPlay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAutoPlay: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub NaturalDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    NaturalDuration: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Position: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetPosition: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetPosition: usize,
    #[cfg(feature = "deprecated")]
    pub BufferingProgress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BufferingProgress: usize,
    #[cfg(feature = "deprecated")]
    pub CurrentState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaPlayerState) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CurrentState: usize,
    #[cfg(feature = "deprecated")]
    pub CanSeek: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CanSeek: usize,
    #[cfg(feature = "deprecated")]
    pub CanPause: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CanPause: usize,
    pub IsLoopingEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsLoopingEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub IsProtected: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsProtected: usize,
    pub IsMuted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsMuted: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub PlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PlaybackRate: usize,
    #[cfg(feature = "deprecated")]
    pub SetPlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetPlaybackRate: usize,
    pub Volume: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub PlaybackMediaMarkers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PlaybackMediaMarkers: usize,
    #[cfg(feature = "Foundation")]
    pub MediaOpened: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MediaOpened: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMediaOpened: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMediaOpened: usize,
    #[cfg(feature = "Foundation")]
    pub MediaEnded: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MediaEnded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMediaEnded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMediaEnded: usize,
    #[cfg(feature = "Foundation")]
    pub MediaFailed: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MediaFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMediaFailed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMediaFailed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CurrentStateChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CurrentStateChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveCurrentStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveCurrentStateChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PlaybackMediaMarkerReached: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PlaybackMediaMarkerReached: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemovePlaybackMediaMarkerReached: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemovePlaybackMediaMarkerReached: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub MediaPlayerRateChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    MediaPlayerRateChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveMediaPlayerRateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveMediaPlayerRateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub VolumeChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VolumeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVolumeChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVolumeChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SeekCompleted: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SeekCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSeekCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSeekCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub BufferingStarted: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    BufferingStarted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveBufferingStarted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveBufferingStarted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub BufferingEnded: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    BufferingEnded: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveBufferingEnded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveBufferingEnded: usize,
    pub Play: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetUriSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetUriSource: usize,
}
impl ::windows_sys::core::Interface for IMediaPlayer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 941261771, data2: 28671, data3: 18843, data4: [141, 100, 40, 133, 223, 193, 36, 158] };
}
#[repr(C)]
pub struct IMediaPlayer2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SystemMediaTransportControls: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AudioCategory: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaPlayerAudioCategory) -> ::windows_sys::core::HRESULT,
    pub SetAudioCategory: unsafe extern "system" fn(this: *mut *mut Self, value: MediaPlayerAudioCategory) -> ::windows_sys::core::HRESULT,
    pub AudioDeviceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaPlayerAudioDeviceType) -> ::windows_sys::core::HRESULT,
    pub SetAudioDeviceType: unsafe extern "system" fn(this: *mut *mut Self, value: MediaPlayerAudioDeviceType) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaPlayer2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1015288344, data2: 8483, data3: 20421, data4: [144, 130, 47, 136, 63, 119, 189, 245] };
}
#[repr(C)]
pub struct IMediaPlayer3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub IsMutedChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsMutedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsMutedChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsMutedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SourceChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSourceChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSourceChanged: usize,
    pub AudioBalance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetAudioBalance: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub RealTimePlayback: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetRealTimePlayback: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub StereoscopicVideoRenderMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StereoscopicVideoRenderMode) -> ::windows_sys::core::HRESULT,
    pub SetStereoscopicVideoRenderMode: unsafe extern "system" fn(this: *mut *mut Self, value: StereoscopicVideoRenderMode) -> ::windows_sys::core::HRESULT,
    pub BreakManager: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CommandManager: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub AudioDevice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    AudioDevice: usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub SetAudioDevice: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    SetAudioDevice: usize,
    pub TimelineController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTimelineController: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TimelineControllerPositionOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimelineControllerPositionOffset: usize,
    #[cfg(feature = "Foundation")]
    pub SetTimelineControllerPositionOffset: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTimelineControllerPositionOffset: usize,
    pub PlaybackSession: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StepForwardOneFrame: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub StepBackwardOneFrame: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_Casting")]
    pub GetAsCastingSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Casting"))]
    GetAsCastingSource: usize,
}
impl ::windows_sys::core::Interface for IMediaPlayer3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3993395418, data2: 795, data3: 20459, data4: [189, 155, 146, 224, 160, 168, 210, 153] };
}
#[repr(C)]
pub struct IMediaPlayer4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetSurfaceSize: unsafe extern "system" fn(this: *mut *mut Self, size: super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSurfaceSize: usize,
    #[cfg(feature = "UI_Composition")]
    pub GetSurface: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetSurface: usize,
}
impl ::windows_sys::core::Interface for IMediaPlayer4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2147704240, data2: 29768, data3: 18288, data4: [175, 207, 42, 87, 69, 9, 20, 197] };
}
#[repr(C)]
pub struct IMediaPlayer5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub VideoFrameAvailable: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VideoFrameAvailable: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVideoFrameAvailable: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVideoFrameAvailable: usize,
    pub IsVideoFrameServerEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsVideoFrameServerEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CopyFrameToVideoSurface: unsafe extern "system" fn(this: *mut *mut Self, destination: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CopyFrameToVideoSurface: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub CopyFrameToVideoSurfaceWithTargetRectangle: unsafe extern "system" fn(this: *mut *mut Self, destination: *mut ::core::ffi::c_void, targetrectangle: super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11")))]
    CopyFrameToVideoSurfaceWithTargetRectangle: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CopyFrameToStereoscopicVideoSurfaces: unsafe extern "system" fn(this: *mut *mut Self, destinationlefteye: *mut ::core::ffi::c_void, destinationrighteye: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CopyFrameToStereoscopicVideoSurfaces: usize,
}
impl ::windows_sys::core::Interface for IMediaPlayer5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3487905789, data2: 63594, data3: 17478, data4: [191, 77, 200, 231, 146, 183, 180, 179] };
}
#[repr(C)]
pub struct IMediaPlayer6 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SubtitleFrameChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SubtitleFrameChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSubtitleFrameChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSubtitleFrameChanged: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub RenderSubtitlesToSurface: unsafe extern "system" fn(this: *mut *mut Self, destination: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    RenderSubtitlesToSurface: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub RenderSubtitlesToSurfaceWithTargetRectangle: unsafe extern "system" fn(this: *mut *mut Self, destination: *mut ::core::ffi::c_void, targetrectangle: super::super::Foundation::Rect, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11")))]
    RenderSubtitlesToSurfaceWithTargetRectangle: usize,
}
impl ::windows_sys::core::Interface for IMediaPlayer6 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3771375750, data2: 44645, data3: 16716, data4: [176, 16, 139, 197, 95, 0, 230, 146] };
}
#[repr(C)]
pub struct IMediaPlayer7 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Audio")]
    pub AudioStateMonitor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Audio"))]
    AudioStateMonitor: usize,
}
impl ::windows_sys::core::Interface for IMediaPlayer7 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1562231928, data2: 17664, data3: 17713, data4: [179, 244, 119, 122, 113, 73, 31, 127] };
}
#[repr(C)]
pub struct IMediaPlayerDataReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Data: usize,
}
impl ::windows_sys::core::Interface for IMediaPlayerDataReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3344602117, data2: 51201, data3: 16682, data4: [131, 91, 131, 252, 14, 98, 42, 142] };
}
#[repr(C)]
pub struct IMediaPlayerEffects {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AddAudioEffect: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING, effectoptional: bool, configuration: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddAudioEffect: usize,
    pub RemoveAllEffects: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaPlayerEffects {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2241978074, data2: 51894, data3: 19648, data4: [139, 227, 96, 53, 244, 222, 37, 145] };
}
#[repr(C)]
pub struct IMediaPlayerEffects2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AddVideoEffect: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING, effectoptional: bool, effectconfiguration: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddVideoEffect: usize,
}
impl ::windows_sys::core::Interface for IMediaPlayerEffects2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4198603385, data2: 7102, data3: 18117, data4: [174, 31, 142, 230, 159, 179, 194, 199] };
}
#[repr(C)]
pub struct IMediaPlayerFailedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaPlayerError) -> ::windows_sys::core::HRESULT,
    pub ExtendedErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub ErrorMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaPlayerFailedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 658827705, data2: 42979, data3: 20246, data4: [186, 196, 121, 20, 235, 192, 131, 1] };
}
#[repr(C)]
pub struct IMediaPlayerRateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub NewRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaPlayerRateChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1080036696, data2: 15201, data3: 19378, data4: [152, 159, 252, 101, 96, 139, 108, 171] };
}
#[repr(C)]
pub struct IMediaPlayerSource {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Protection")]
    pub ProtectionManager: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Protection"))]
    ProtectionManager: usize,
    #[cfg(feature = "Media_Protection")]
    pub SetProtectionManager: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Protection"))]
    SetProtectionManager: usize,
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub SetFileSource: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Storage", feature = "deprecated")))]
    SetFileSource: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub SetStreamSource: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    SetStreamSource: usize,
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub SetMediaSource: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "deprecated")))]
    SetMediaSource: usize,
}
impl ::windows_sys::core::Interface for IMediaPlayerSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3176106135, data2: 5155, data3: 19518, data4: [130, 197, 15, 177, 175, 148, 247, 21] };
}
#[repr(C)]
pub struct IMediaPlayerSource2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaPlayerSource2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2185534367, data2: 29474, data3: 19467, data4: [176, 59, 62, 105, 164, 130, 96, 197] };
}
#[repr(C)]
pub struct IMediaPlayerSurface {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Composition")]
    pub CompositionSurface: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CompositionSurface: usize,
    #[cfg(feature = "UI_Composition")]
    pub Compositor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    Compositor: usize,
    pub MediaPlayer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaPlayerSurface {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 248927164, data2: 46902, data3: 18883, data4: [131, 11, 118, 74, 56, 69, 49, 58] };
}
#[repr(C)]
pub struct IPlaybackMediaMarker {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Time: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Time: usize,
    pub MediaMarkerType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlaybackMediaMarker {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3302109020, data2: 15388, data3: 17476, data4: [182, 185, 119, 139, 4, 34, 212, 26] };
}
#[repr(C)]
pub struct IPlaybackMediaMarkerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateFromTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFromTime: usize,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan, mediamarkettype: ::windows_sys::core::HSTRING, text: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IPlaybackMediaMarkerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2354252408, data2: 57518, data3: 19994, data4: [168, 200, 226, 63, 152, 42, 147, 123] };
}
#[repr(C)]
pub struct IPlaybackMediaMarkerReachedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub PlaybackMediaMarker: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlaybackMediaMarkerReachedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1468846521, data2: 37090, data3: 20064, data4: [171, 196, 135, 64, 176, 31, 97, 150] };
}
#[repr(C)]
pub struct IPlaybackMediaMarkerSequence {
    pub base__: ::windows_sys::core::IInspectable,
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Insert: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlaybackMediaMarkerSequence {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4068543726, data2: 25483, data3: 18127, data4: [136, 23, 29, 17, 31, 233, 216, 196] };
}
#[repr(C)]
pub struct ITimedMetadataPresentationModeChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Core")]
    pub Track: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    Track: usize,
    pub OldPresentationMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedMetadataTrackPresentationMode) -> ::windows_sys::core::HRESULT,
    pub NewPresentationMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedMetadataTrackPresentationMode) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITimedMetadataPresentationModeChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3512950937, data2: 26079, data3: 17838, data4: [140, 239, 220, 11, 83, 253, 194, 187] };
}
pub type MediaBreak = *mut ::core::ffi::c_void;
pub type MediaBreakEndedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaBreakInsertionMethod(pub i32);
impl MediaBreakInsertionMethod {
    pub const Interrupt: Self = Self(0i32);
    pub const Replace: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaBreakInsertionMethod {}
impl ::core::clone::Clone for MediaBreakInsertionMethod {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaBreakManager = *mut ::core::ffi::c_void;
pub type MediaBreakSchedule = *mut ::core::ffi::c_void;
pub type MediaBreakSeekedOverEventArgs = *mut ::core::ffi::c_void;
pub type MediaBreakSkippedEventArgs = *mut ::core::ffi::c_void;
pub type MediaBreakStartedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaCommandEnablingRule(pub i32);
impl MediaCommandEnablingRule {
    pub const Auto: Self = Self(0i32);
    pub const Always: Self = Self(1i32);
    pub const Never: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaCommandEnablingRule {}
impl ::core::clone::Clone for MediaCommandEnablingRule {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaItemDisplayProperties = *mut ::core::ffi::c_void;
pub type MediaPlaybackAudioTrackList = *mut ::core::ffi::c_void;
pub type MediaPlaybackCommandManager = *mut ::core::ffi::c_void;
pub type MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs = *mut ::core::ffi::c_void;
pub type MediaPlaybackCommandManagerCommandBehavior = *mut ::core::ffi::c_void;
pub type MediaPlaybackCommandManagerFastForwardReceivedEventArgs = *mut ::core::ffi::c_void;
pub type MediaPlaybackCommandManagerNextReceivedEventArgs = *mut ::core::ffi::c_void;
pub type MediaPlaybackCommandManagerPauseReceivedEventArgs = *mut ::core::ffi::c_void;
pub type MediaPlaybackCommandManagerPlayReceivedEventArgs = *mut ::core::ffi::c_void;
pub type MediaPlaybackCommandManagerPositionReceivedEventArgs = *mut ::core::ffi::c_void;
pub type MediaPlaybackCommandManagerPreviousReceivedEventArgs = *mut ::core::ffi::c_void;
pub type MediaPlaybackCommandManagerRateReceivedEventArgs = *mut ::core::ffi::c_void;
pub type MediaPlaybackCommandManagerRewindReceivedEventArgs = *mut ::core::ffi::c_void;
pub type MediaPlaybackCommandManagerShuffleReceivedEventArgs = *mut ::core::ffi::c_void;
pub type MediaPlaybackItem = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackItemChangedReason(pub i32);
impl MediaPlaybackItemChangedReason {
    pub const InitialItem: Self = Self(0i32);
    pub const EndOfStream: Self = Self(1i32);
    pub const Error: Self = Self(2i32);
    pub const AppRequested: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaPlaybackItemChangedReason {}
impl ::core::clone::Clone for MediaPlaybackItemChangedReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaPlaybackItemError = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackItemErrorCode(pub i32);
impl MediaPlaybackItemErrorCode {
    pub const None: Self = Self(0i32);
    pub const Aborted: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const DecodeError: Self = Self(3i32);
    pub const SourceNotSupportedError: Self = Self(4i32);
    pub const EncryptionError: Self = Self(5i32);
}
impl ::core::marker::Copy for MediaPlaybackItemErrorCode {}
impl ::core::clone::Clone for MediaPlaybackItemErrorCode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaPlaybackItemFailedEventArgs = *mut ::core::ffi::c_void;
pub type MediaPlaybackItemOpenedEventArgs = *mut ::core::ffi::c_void;
pub type MediaPlaybackList = *mut ::core::ffi::c_void;
pub type MediaPlaybackSession = *mut ::core::ffi::c_void;
pub type MediaPlaybackSessionBufferingStartedEventArgs = *mut ::core::ffi::c_void;
pub type MediaPlaybackSessionOutputDegradationPolicyState = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackSessionVideoConstrictionReason(pub i32);
impl MediaPlaybackSessionVideoConstrictionReason {
    pub const None: Self = Self(0i32);
    pub const VirtualMachine: Self = Self(1i32);
    pub const UnsupportedDisplayAdapter: Self = Self(2i32);
    pub const UnsignedDriver: Self = Self(3i32);
    pub const FrameServerEnabled: Self = Self(4i32);
    pub const OutputProtectionFailed: Self = Self(5i32);
    pub const Unknown: Self = Self(6i32);
}
impl ::core::marker::Copy for MediaPlaybackSessionVideoConstrictionReason {}
impl ::core::clone::Clone for MediaPlaybackSessionVideoConstrictionReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaPlaybackSphericalVideoProjection = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackState(pub i32);
impl MediaPlaybackState {
    pub const None: Self = Self(0i32);
    pub const Opening: Self = Self(1i32);
    pub const Buffering: Self = Self(2i32);
    pub const Playing: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaPlaybackState {}
impl ::core::clone::Clone for MediaPlaybackState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaPlaybackTimedMetadataTrackList = *mut ::core::ffi::c_void;
pub type MediaPlaybackVideoTrackList = *mut ::core::ffi::c_void;
pub type MediaPlayer = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlayerAudioCategory(pub i32);
impl MediaPlayerAudioCategory {
    pub const Other: Self = Self(0i32);
    pub const Communications: Self = Self(3i32);
    pub const Alerts: Self = Self(4i32);
    pub const SoundEffects: Self = Self(5i32);
    pub const GameEffects: Self = Self(6i32);
    pub const GameMedia: Self = Self(7i32);
    pub const GameChat: Self = Self(8i32);
    pub const Speech: Self = Self(9i32);
    pub const Movie: Self = Self(10i32);
    pub const Media: Self = Self(11i32);
}
impl ::core::marker::Copy for MediaPlayerAudioCategory {}
impl ::core::clone::Clone for MediaPlayerAudioCategory {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlayerAudioDeviceType(pub i32);
impl MediaPlayerAudioDeviceType {
    pub const Console: Self = Self(0i32);
    pub const Multimedia: Self = Self(1i32);
    pub const Communications: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaPlayerAudioDeviceType {}
impl ::core::clone::Clone for MediaPlayerAudioDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaPlayerDataReceivedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlayerError(pub i32);
impl MediaPlayerError {
    pub const Unknown: Self = Self(0i32);
    pub const Aborted: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const DecodingError: Self = Self(3i32);
    pub const SourceNotSupported: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaPlayerError {}
impl ::core::clone::Clone for MediaPlayerError {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaPlayerFailedEventArgs = *mut ::core::ffi::c_void;
pub type MediaPlayerRateChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Playback\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct MediaPlayerState(pub i32);
#[cfg(feature = "deprecated")]
impl MediaPlayerState {
    pub const Closed: Self = Self(0i32);
    pub const Opening: Self = Self(1i32);
    pub const Buffering: Self = Self(2i32);
    pub const Playing: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
    pub const Stopped: Self = Self(5i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for MediaPlayerState {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for MediaPlayerState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaPlayerSurface = *mut ::core::ffi::c_void;
pub type PlaybackMediaMarker = *mut ::core::ffi::c_void;
pub type PlaybackMediaMarkerReachedEventArgs = *mut ::core::ffi::c_void;
pub type PlaybackMediaMarkerSequence = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct SphericalVideoProjectionMode(pub i32);
impl SphericalVideoProjectionMode {
    pub const Spherical: Self = Self(0i32);
    pub const Flat: Self = Self(1i32);
}
impl ::core::marker::Copy for SphericalVideoProjectionMode {}
impl ::core::clone::Clone for SphericalVideoProjectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct StereoscopicVideoRenderMode(pub i32);
impl StereoscopicVideoRenderMode {
    pub const Mono: Self = Self(0i32);
    pub const Stereo: Self = Self(1i32);
}
impl ::core::marker::Copy for StereoscopicVideoRenderMode {}
impl ::core::clone::Clone for StereoscopicVideoRenderMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TimedMetadataPresentationModeChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct TimedMetadataTrackPresentationMode(pub i32);
impl TimedMetadataTrackPresentationMode {
    pub const Disabled: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
    pub const ApplicationPresented: Self = Self(2i32);
    pub const PlatformPresented: Self = Self(3i32);
}
impl ::core::marker::Copy for TimedMetadataTrackPresentationMode {}
impl ::core::clone::Clone for TimedMetadataTrackPresentationMode {
    fn clone(&self) -> Self {
        *self
    }
}
