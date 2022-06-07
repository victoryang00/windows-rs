#[cfg(feature = "Media_AppBroadcasting")]
pub mod AppBroadcasting;
#[cfg(feature = "Media_AppRecording")]
pub mod AppRecording;
#[cfg(feature = "Media_Audio")]
pub mod Audio;
#[cfg(feature = "Media_Capture")]
pub mod Capture;
#[cfg(feature = "Media_Casting")]
pub mod Casting;
#[cfg(feature = "Media_ClosedCaptioning")]
pub mod ClosedCaptioning;
#[cfg(feature = "Media_ContentRestrictions")]
pub mod ContentRestrictions;
#[cfg(feature = "Media_Control")]
pub mod Control;
#[cfg(feature = "Media_Core")]
pub mod Core;
#[cfg(feature = "Media_Devices")]
pub mod Devices;
#[cfg(feature = "Media_DialProtocol")]
pub mod DialProtocol;
#[cfg(feature = "Media_Editing")]
pub mod Editing;
#[cfg(feature = "Media_Effects")]
pub mod Effects;
#[cfg(feature = "Media_FaceAnalysis")]
pub mod FaceAnalysis;
#[cfg(feature = "Media_Import")]
pub mod Import;
#[cfg(feature = "Media_MediaProperties")]
pub mod MediaProperties;
#[cfg(feature = "Media_Miracast")]
pub mod Miracast;
#[cfg(feature = "Media_Ocr")]
pub mod Ocr;
#[cfg(feature = "Media_PlayTo")]
pub mod PlayTo;
#[cfg(feature = "Media_Playback")]
pub mod Playback;
#[cfg(feature = "Media_Playlists")]
pub mod Playlists;
#[cfg(feature = "Media_Protection")]
pub mod Protection;
#[cfg(feature = "Media_Render")]
pub mod Render;
#[cfg(feature = "Media_SpeechRecognition")]
pub mod SpeechRecognition;
#[cfg(feature = "Media_SpeechSynthesis")]
pub mod SpeechSynthesis;
#[cfg(feature = "Media_Streaming")]
pub mod Streaming;
#[cfg(feature = "Media_Transcoding")]
pub mod Transcoding;
pub type AudioBuffer = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct AudioBufferAccessMode(pub i32);
impl AudioBufferAccessMode {
    pub const Read: Self = Self(0i32);
    pub const ReadWrite: Self = Self(1i32);
    pub const Write: Self = Self(2i32);
}
impl ::core::marker::Copy for AudioBufferAccessMode {}
impl ::core::clone::Clone for AudioBufferAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AudioFrame = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct AudioProcessing(pub i32);
impl AudioProcessing {
    pub const Default: Self = Self(0i32);
    pub const Raw: Self = Self(1i32);
}
impl ::core::marker::Copy for AudioProcessing {}
impl ::core::clone::Clone for AudioProcessing {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AutoRepeatModeChangeRequestedEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAudioBuffer {
    pub base__: ::windows_sys::core::IInspectable,
    pub Capacity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetLength: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioBuffer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 890722343, data2: 29259, data3: 19562, data4: [177, 48, 246, 83, 127, 154, 224, 208] };
}
#[repr(C)]
pub struct IAudioFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub LockBuffer: unsafe extern "system" fn(this: *mut *mut Self, mode: AudioBufferAccessMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioFrame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3815424772, data2: 43698, data3: 17015, data4: [158, 208, 67, 206, 223, 142, 41, 198] };
}
#[repr(C)]
pub struct IAudioFrameFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, capacity: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioFrameFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2443774686, data2: 9250, data3: 16550, data4: [185, 173, 48, 208, 36, 4, 49, 125] };
}
#[repr(C)]
pub struct IAutoRepeatModeChangeRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestedAutoRepeatMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaPlaybackAutoRepeatMode) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAutoRepeatModeChangeRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3927146234, data2: 55378, data3: 17294, data4: [136, 43, 201, 144, 16, 154, 120, 244] };
}
#[repr(C)]
pub struct IImageDisplayProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSubtitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IImageDisplayProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3440101359, data2: 21735, data3: 16671, data4: [153, 51, 240, 233, 139, 10, 150, 210] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IMediaControl {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SoundLevelChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SoundLevelChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSoundLevelChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSoundLevelChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PlayPressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PlayPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemovePlayPressed: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemovePlayPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PausePressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PausePressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemovePausePressed: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemovePausePressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub StopPressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    StopPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveStopPressed: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveStopPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PlayPauseTogglePressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PlayPauseTogglePressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemovePlayPauseTogglePressed: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemovePlayPauseTogglePressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RecordPressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RecordPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveRecordPressed: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveRecordPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub NextTrackPressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    NextTrackPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveNextTrackPressed: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveNextTrackPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PreviousTrackPressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PreviousTrackPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemovePreviousTrackPressed: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemovePreviousTrackPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FastForwardPressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FastForwardPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveFastForwardPressed: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveFastForwardPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RewindPressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RewindPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveRewindPressed: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveRewindPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ChannelUpPressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ChannelUpPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveChannelUpPressed: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveChannelUpPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ChannelDownPressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ChannelDownPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveChannelDownPressed: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveChannelDownPressed: usize,
    #[cfg(feature = "deprecated")]
    pub SoundLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SoundLevel) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SoundLevel: usize,
    #[cfg(feature = "deprecated")]
    pub SetTrackName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetTrackName: usize,
    #[cfg(feature = "deprecated")]
    pub TrackName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TrackName: usize,
    #[cfg(feature = "deprecated")]
    pub SetArtistName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetArtistName: usize,
    #[cfg(feature = "deprecated")]
    pub ArtistName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ArtistName: usize,
    #[cfg(feature = "deprecated")]
    pub SetIsPlaying: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetIsPlaying: usize,
    #[cfg(feature = "deprecated")]
    pub IsPlaying: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsPlaying: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetAlbumArt: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetAlbumArt: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AlbumArt: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AlbumArt: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IMediaControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2565995489, data2: 31373, data3: 17099, data4: [182, 254, 143, 230, 152, 38, 79, 19] };
}
#[repr(C)]
pub struct IMediaExtension {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SetProperties: unsafe extern "system" fn(this: *mut *mut Self, configuration: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetProperties: usize,
}
impl ::windows_sys::core::Interface for IMediaExtension {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 126963992, data2: 17887, data3: 17451, data4: [138, 63, 247, 130, 106, 99, 112, 171] };
}
#[repr(C)]
pub struct IMediaExtensionManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub RegisterSchemeHandler: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING, scheme: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterSchemeHandlerWithSettings: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING, scheme: ::windows_sys::core::HSTRING, configuration: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterSchemeHandlerWithSettings: usize,
    pub RegisterByteStreamHandler: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING, fileextension: ::windows_sys::core::HSTRING, mimetype: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterByteStreamHandlerWithSettings: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING, fileextension: ::windows_sys::core::HSTRING, mimetype: ::windows_sys::core::HSTRING, configuration: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterByteStreamHandlerWithSettings: usize,
    pub RegisterAudioDecoder: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING, inputsubtype: ::windows_sys::core::GUID, outputsubtype: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterAudioDecoderWithSettings: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING, inputsubtype: ::windows_sys::core::GUID, outputsubtype: ::windows_sys::core::GUID, configuration: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterAudioDecoderWithSettings: usize,
    pub RegisterAudioEncoder: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING, inputsubtype: ::windows_sys::core::GUID, outputsubtype: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterAudioEncoderWithSettings: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING, inputsubtype: ::windows_sys::core::GUID, outputsubtype: ::windows_sys::core::GUID, configuration: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterAudioEncoderWithSettings: usize,
    pub RegisterVideoDecoder: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING, inputsubtype: ::windows_sys::core::GUID, outputsubtype: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterVideoDecoderWithSettings: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING, inputsubtype: ::windows_sys::core::GUID, outputsubtype: ::windows_sys::core::GUID, configuration: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterVideoDecoderWithSettings: usize,
    pub RegisterVideoEncoder: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING, inputsubtype: ::windows_sys::core::GUID, outputsubtype: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterVideoEncoderWithSettings: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING, inputsubtype: ::windows_sys::core::GUID, outputsubtype: ::windows_sys::core::GUID, configuration: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterVideoEncoderWithSettings: usize,
}
impl ::windows_sys::core::Interface for IMediaExtensionManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1243998965, data2: 9261, data3: 19963, data4: [151, 244, 105, 183, 196, 37, 118, 255] };
}
#[repr(C)]
pub struct IMediaExtensionManager2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_AppService")]
    pub RegisterMediaExtensionForAppService: unsafe extern "system" fn(this: *mut *mut Self, extension: *mut ::core::ffi::c_void, connection: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))]
    RegisterMediaExtensionForAppService: usize,
}
impl ::windows_sys::core::Interface for IMediaExtensionManager2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1540276039, data2: 16451, data3: 20461, data4: [172, 175, 84, 236, 41, 223, 177, 247] };
}
#[repr(C)]
pub struct IMediaFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetRelativeTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRelativeTime: usize,
    #[cfg(feature = "Foundation")]
    pub RelativeTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RelativeTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetSystemRelativeTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSystemRelativeTime: usize,
    #[cfg(feature = "Foundation")]
    pub SystemRelativeTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativeTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub SetIsDiscontinuous: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsDiscontinuous: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ExtendedProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExtendedProperties: usize,
}
impl ::windows_sys::core::Interface for IMediaFrame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3216322444, data2: 22851, data3: 18392, data4: [142, 16, 5, 48, 138, 165, 251, 208] };
}
#[repr(C)]
pub struct IMediaMarker {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Time: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Time: usize,
    pub MediaMarkerType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaMarker {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 402906872, data2: 56485, data3: 19311, data4: [156, 32, 227, 211, 192, 100, 54, 37] };
}
#[repr(C)]
pub struct IMediaMarkerTypesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Bookmark: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaMarkerTypesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3139010624, data2: 18479, data3: 18243, data4: [136, 50, 69, 133, 56, 33, 236, 224] };
}
#[repr(C)]
pub struct IMediaMarkers {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Markers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Markers: usize,
}
impl ::windows_sys::core::Interface for IMediaMarkers {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2951393673, data2: 63709, data3: 18030, data4: [170, 16, 146, 11, 82, 53, 63, 223] };
}
#[repr(C)]
pub struct IMediaProcessingTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Arguments: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Arguments: usize,
}
impl ::windows_sys::core::Interface for IMediaProcessingTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3951387820, data2: 41809, data3: 20302, data4: [180, 240, 155, 242, 64, 137, 147, 219] };
}
#[repr(C)]
pub struct IMediaTimelineController {
    pub base__: ::windows_sys::core::IInspectable,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub SetPosition: unsafe extern "system" fn(this: *mut *mut Self, value: super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPosition: usize,
    pub ClockRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetClockRate: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaTimelineControllerState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PositionChanged: unsafe extern "system" fn(this: *mut *mut Self, positionchangedeventhandler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePositionChanged: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut *mut Self, statechangedeventhandler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
}
impl ::windows_sys::core::Interface for IMediaTimelineController {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2396217843, data2: 2936, data3: 17248, data4: [191, 113, 12, 132, 25, 153, 234, 27] };
}
#[repr(C)]
pub struct IMediaTimelineController2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    pub IsLoopingEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsLoopingEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Failed: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Failed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFailed: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFailed: usize,
    #[cfg(feature = "Foundation")]
    pub Ended: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Ended: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnded: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnded: usize,
}
impl ::windows_sys::core::Interface for IMediaTimelineController2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4017416760, data2: 40562, data3: 19961, data4: [131, 85, 110, 144, 200, 27, 186, 221] };
}
#[repr(C)]
pub struct IMediaTimelineControllerFailedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaTimelineControllerFailedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2283927581, data2: 15991, data3: 17403, data4: [190, 38, 79, 200, 122, 4, 72, 52] };
}
#[repr(C)]
pub struct IMusicDisplayProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AlbumArtist: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAlbumArtist: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Artist: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetArtist: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMusicDisplayProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1807682649, data2: 53408, data3: 19750, data4: [146, 160, 249, 120, 225, 209, 142, 123] };
}
#[repr(C)]
pub struct IMusicDisplayProperties2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AlbumTitle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAlbumTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TrackNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetTrackNumber: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Genres: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Genres: usize,
}
impl ::windows_sys::core::Interface for IMusicDisplayProperties2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3572834, data2: 38867, data3: 17593, data4: [176, 15, 0, 138, 252, 239, 175, 24] };
}
#[repr(C)]
pub struct IMusicDisplayProperties3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AlbumTrackCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetAlbumTrackCount: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMusicDisplayProperties3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1303714497, data2: 1665, data3: 20108, data4: [148, 1, 184, 21, 157, 158, 239, 199] };
}
#[repr(C)]
pub struct IPlaybackPositionChangeRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestedPlaybackPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestedPlaybackPosition: usize,
}
impl ::windows_sys::core::Interface for IPlaybackPositionChangeRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3024699272, data2: 60200, data3: 18785, data4: [156, 20, 51, 94, 68, 243, 225, 37] };
}
#[repr(C)]
pub struct IPlaybackRateChangeRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestedPlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlaybackRateChangeRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 753058847, data2: 15574, data3: 20343, data4: [155, 167, 235, 39, 194, 106, 33, 64] };
}
#[repr(C)]
pub struct IShuffleEnabledChangeRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestedShuffleEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IShuffleEnabledChangeRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1236636670, data2: 20432, data3: 18022, data4: [163, 20, 192, 224, 25, 64, 211, 2] };
}
#[repr(C)]
pub struct ISystemMediaTransportControls {
    pub base__: ::windows_sys::core::IInspectable,
    pub PlaybackStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaPlaybackStatus) -> ::windows_sys::core::HRESULT,
    pub SetPlaybackStatus: unsafe extern "system" fn(this: *mut *mut Self, value: MediaPlaybackStatus) -> ::windows_sys::core::HRESULT,
    pub DisplayUpdater: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SoundLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SoundLevel) -> ::windows_sys::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsPlayEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsPlayEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsStopEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsStopEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsPauseEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsPauseEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsRecordEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsRecordEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsFastForwardEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsFastForwardEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsRewindEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsRewindEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsPreviousEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsPreviousEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsNextEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsNextEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsChannelUpEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsChannelUpEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsChannelDownEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsChannelDownEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ButtonPressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveButtonPressed: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveButtonPressed: usize,
    #[cfg(feature = "Foundation")]
    pub PropertyChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PropertyChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePropertyChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePropertyChanged: usize,
}
impl ::windows_sys::core::Interface for ISystemMediaTransportControls {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2583314420, data2: 5954, data3: 17062, data4: [144, 46, 8, 125, 65, 249, 101, 236] };
}
#[repr(C)]
pub struct ISystemMediaTransportControls2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AutoRepeatMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaPlaybackAutoRepeatMode) -> ::windows_sys::core::HRESULT,
    pub SetAutoRepeatMode: unsafe extern "system" fn(this: *mut *mut Self, value: MediaPlaybackAutoRepeatMode) -> ::windows_sys::core::HRESULT,
    pub ShuffleEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShuffleEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub PlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetPlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub UpdateTimelineProperties: unsafe extern "system" fn(this: *mut *mut Self, timelineproperties: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PlaybackPositionChangeRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackPositionChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlaybackPositionChangeRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlaybackPositionChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub PlaybackRateChangeRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackRateChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlaybackRateChangeRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlaybackRateChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ShuffleEnabledChangeRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShuffleEnabledChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShuffleEnabledChangeRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShuffleEnabledChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub AutoRepeatModeChangeRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AutoRepeatModeChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAutoRepeatModeChangeRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAutoRepeatModeChangeRequested: usize,
}
impl ::windows_sys::core::Interface for ISystemMediaTransportControls2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3935884022, data2: 32572, data3: 19186, data4: [165, 134, 114, 136, 152, 8, 239, 177] };
}
#[repr(C)]
pub struct ISystemMediaTransportControlsButtonPressedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Button: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SystemMediaTransportControlsButton) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemMediaTransportControlsButtonPressedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3086250262, data2: 42351, data3: 19912, data4: [158, 17, 146, 3, 31, 74, 135, 194] };
}
#[repr(C)]
pub struct ISystemMediaTransportControlsDisplayUpdater {
    pub base__: ::windows_sys::core::IInspectable,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaPlaybackType) -> ::windows_sys::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut *mut Self, value: MediaPlaybackType) -> ::windows_sys::core::HRESULT,
    pub AppMediaId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAppMediaId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetThumbnail: usize,
    pub MusicProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VideoProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ImageProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CopyFromFileAsync: unsafe extern "system" fn(this: *mut *mut Self, r#type: MediaPlaybackType, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CopyFromFileAsync: usize,
    pub ClearAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemMediaTransportControlsDisplayUpdater {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2327561534, data2: 64085, data3: 20175, data4: [173, 142, 201, 132, 229, 221, 21, 80] };
}
#[repr(C)]
pub struct ISystemMediaTransportControlsPropertyChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SystemMediaTransportControlsProperty) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemMediaTransportControlsPropertyChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3502901558, data2: 13211, data3: 19635, data4: [142, 235, 115, 118, 7, 245, 110, 8] };
}
#[repr(C)]
pub struct ISystemMediaTransportControlsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISystemMediaTransportControlsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1136277514, data2: 60580, data3: 18482, data4: [145, 171, 212, 21, 250, 228, 132, 198] };
}
#[repr(C)]
pub struct ISystemMediaTransportControlsTimelineProperties {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub EndTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetEndTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEndTime: usize,
    #[cfg(feature = "Foundation")]
    pub MinSeekTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinSeekTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetMinSeekTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMinSeekTime: usize,
    #[cfg(feature = "Foundation")]
    pub MaxSeekTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxSeekTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxSeekTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxSeekTime: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub SetPosition: unsafe extern "system" fn(this: *mut *mut Self, value: super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPosition: usize,
}
impl ::windows_sys::core::Interface for ISystemMediaTransportControlsTimelineProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1361391978, data2: 50082, data3: 18267, data4: [133, 7, 147, 83, 77, 200, 143, 21] };
}
#[repr(C)]
pub struct IVideoDisplayProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSubtitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVideoDisplayProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1443495345, data2: 23853, data3: 18546, data4: [129, 112, 69, 222, 229, 188, 47, 92] };
}
#[repr(C)]
pub struct IVideoDisplayProperties2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Genres: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Genres: usize,
}
impl ::windows_sys::core::Interface for IVideoDisplayProperties2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3021005262, data2: 43858, data3: 16811, data4: [164, 134, 204, 16, 250, 177, 82, 249] };
}
#[repr(C)]
pub struct IVideoEffectsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub VideoStabilization: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVideoEffectsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 533571048, data2: 47857, data3: 17697, data4: [152, 12, 59, 206, 187, 68, 207, 56] };
}
#[repr(C)]
pub struct IVideoFrame {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_Imaging")]
    pub SoftwareBitmap: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SoftwareBitmap: usize,
    #[cfg(feature = "Foundation")]
    pub CopyToAsync: unsafe extern "system" fn(this: *mut *mut Self, frame: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CopyToAsync: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3DSurface: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3DSurface: usize,
}
impl ::windows_sys::core::Interface for IVideoFrame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 213935653, data2: 37116, data3: 19602, data4: [189, 149, 125, 237, 33, 129, 157, 28] };
}
#[repr(C)]
pub struct IVideoFrame2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub CopyToWithBoundsAsync: unsafe extern "system" fn(this: *mut *mut Self, frame: *mut ::core::ffi::c_void, sourcebounds: *mut ::core::ffi::c_void, destinationbounds: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    CopyToWithBoundsAsync: usize,
}
impl ::windows_sys::core::Interface for IVideoFrame2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 943162381, data2: 13164, data3: 17254, data4: [141, 70, 6, 7, 152, 115, 108, 93] };
}
#[repr(C)]
pub struct IVideoFrameFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_Imaging")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    Create: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub CreateWithAlpha: unsafe extern "system" fn(this: *mut *mut Self, format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32, alpha: super::Graphics::Imaging::BitmapAlphaMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    CreateWithAlpha: usize,
}
impl ::windows_sys::core::Interface for IVideoFrameFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 21720425, data2: 8744, data3: 19602, data4: [146, 255, 80, 195, 128, 211, 231, 118] };
}
#[repr(C)]
pub struct IVideoFrameStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_DirectX")]
    pub CreateAsDirect3D11SurfaceBacked: unsafe extern "system" fn(this: *mut *mut Self, format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    CreateAsDirect3D11SurfaceBacked: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateAsDirect3D11SurfaceBackedWithDevice: unsafe extern "system" fn(this: *mut *mut Self, format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32, device: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateAsDirect3D11SurfaceBackedWithDevice: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub CreateWithSoftwareBitmap: unsafe extern "system" fn(this: *mut *mut Self, bitmap: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    CreateWithSoftwareBitmap: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateWithDirect3D11Surface: unsafe extern "system" fn(this: *mut *mut Self, surface: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateWithDirect3D11Surface: usize,
}
impl ::windows_sys::core::Interface for IVideoFrameStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2871678319, data2: 24849, data3: 19251, data4: [142, 195, 43, 32, 154, 2, 225, 122] };
}
pub type ImageDisplayProperties = *mut ::core::ffi::c_void;
pub type MediaExtensionManager = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackAutoRepeatMode(pub i32);
impl MediaPlaybackAutoRepeatMode {
    pub const None: Self = Self(0i32);
    pub const Track: Self = Self(1i32);
    pub const List: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaPlaybackAutoRepeatMode {}
impl ::core::clone::Clone for MediaPlaybackAutoRepeatMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackStatus(pub i32);
impl MediaPlaybackStatus {
    pub const Closed: Self = Self(0i32);
    pub const Changing: Self = Self(1i32);
    pub const Stopped: Self = Self(2i32);
    pub const Playing: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaPlaybackStatus {}
impl ::core::clone::Clone for MediaPlaybackStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackType(pub i32);
impl MediaPlaybackType {
    pub const Unknown: Self = Self(0i32);
    pub const Music: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
    pub const Image: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaPlaybackType {}
impl ::core::clone::Clone for MediaPlaybackType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaProcessingTriggerDetails = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct MediaTimeRange {
    pub Start: super::Foundation::TimeSpan,
    pub End: super::Foundation::TimeSpan,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for MediaTimeRange {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for MediaTimeRange {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaTimelineController = *mut ::core::ffi::c_void;
pub type MediaTimelineControllerFailedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct MediaTimelineControllerState(pub i32);
impl MediaTimelineControllerState {
    pub const Paused: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Stalled: Self = Self(2i32);
    pub const Error: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaTimelineControllerState {}
impl ::core::clone::Clone for MediaTimelineControllerState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MusicDisplayProperties = *mut ::core::ffi::c_void;
pub type PlaybackPositionChangeRequestedEventArgs = *mut ::core::ffi::c_void;
pub type PlaybackRateChangeRequestedEventArgs = *mut ::core::ffi::c_void;
pub type ShuffleEnabledChangeRequestedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct SoundLevel(pub i32);
impl SoundLevel {
    pub const Muted: Self = Self(0i32);
    pub const Low: Self = Self(1i32);
    pub const Full: Self = Self(2i32);
}
impl ::core::marker::Copy for SoundLevel {}
impl ::core::clone::Clone for SoundLevel {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SystemMediaTransportControls = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct SystemMediaTransportControlsButton(pub i32);
impl SystemMediaTransportControlsButton {
    pub const Play: Self = Self(0i32);
    pub const Pause: Self = Self(1i32);
    pub const Stop: Self = Self(2i32);
    pub const Record: Self = Self(3i32);
    pub const FastForward: Self = Self(4i32);
    pub const Rewind: Self = Self(5i32);
    pub const Next: Self = Self(6i32);
    pub const Previous: Self = Self(7i32);
    pub const ChannelUp: Self = Self(8i32);
    pub const ChannelDown: Self = Self(9i32);
}
impl ::core::marker::Copy for SystemMediaTransportControlsButton {}
impl ::core::clone::Clone for SystemMediaTransportControlsButton {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SystemMediaTransportControlsButtonPressedEventArgs = *mut ::core::ffi::c_void;
pub type SystemMediaTransportControlsDisplayUpdater = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct SystemMediaTransportControlsProperty(pub i32);
impl SystemMediaTransportControlsProperty {
    pub const SoundLevel: Self = Self(0i32);
}
impl ::core::marker::Copy for SystemMediaTransportControlsProperty {}
impl ::core::clone::Clone for SystemMediaTransportControlsProperty {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SystemMediaTransportControlsPropertyChangedEventArgs = *mut ::core::ffi::c_void;
pub type SystemMediaTransportControlsTimelineProperties = *mut ::core::ffi::c_void;
pub type VideoDisplayProperties = *mut ::core::ffi::c_void;
pub type VideoFrame = *mut ::core::ffi::c_void;
