
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#![no_std]
#[cfg(feature = "AppBroadcasting")]
pub mod AppBroadcasting;
#[cfg(feature = "AppRecording")]
pub mod AppRecording;
#[cfg(feature = "Audio")]
pub mod Audio;
#[cfg(feature = "Capture")]
pub mod Capture;
#[cfg(feature = "Casting")]
pub mod Casting;
#[cfg(feature = "ClosedCaptioning")]
pub mod ClosedCaptioning;
#[cfg(feature = "ContentRestrictions")]
pub mod ContentRestrictions;
#[cfg(feature = "Control")]
pub mod Control;
#[cfg(feature = "Core")]
pub mod Core;
#[cfg(feature = "Devices")]
pub mod Devices;
#[cfg(feature = "DialProtocol")]
pub mod DialProtocol;
#[cfg(feature = "Editing")]
pub mod Editing;
#[cfg(feature = "Effects")]
pub mod Effects;
#[cfg(feature = "FaceAnalysis")]
pub mod FaceAnalysis;
#[cfg(feature = "Import")]
pub mod Import;
#[cfg(feature = "MediaProperties")]
pub mod MediaProperties;
#[cfg(feature = "Miracast")]
pub mod Miracast;
#[cfg(feature = "Ocr")]
pub mod Ocr;
#[cfg(feature = "PlayTo")]
pub mod PlayTo;
#[cfg(feature = "Playback")]
pub mod Playback;
#[cfg(feature = "Playlists")]
pub mod Playlists;
#[cfg(feature = "Protection")]
pub mod Protection;
#[cfg(feature = "Render")]
pub mod Render;
#[cfg(feature = "SpeechRecognition")]
pub mod SpeechRecognition;
#[cfg(feature = "SpeechSynthesis")]
pub mod SpeechSynthesis;
#[cfg(feature = "Streaming")]
pub mod Streaming;
#[cfg(feature = "Transcoding")]
pub mod Transcoding;
pub type AudioBuffer = *mut ::core::ffi::c_void;
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
pub type IMediaExtension = *mut ::core::ffi::c_void;
pub type IMediaFrame = *mut ::core::ffi::c_void;
pub type IMediaMarker = *mut ::core::ffi::c_void;
pub type IMediaMarkers = *mut ::core::ffi::c_void;
pub type ImageDisplayProperties = *mut ::core::ffi::c_void;
pub type MediaExtensionManager = *mut ::core::ffi::c_void;
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
pub struct MediaTimeRange {
    pub Start: ::winrt_foundation_sys::TimeSpan,
    pub End: ::winrt_foundation_sys::TimeSpan,
}
impl ::core::marker::Copy for MediaTimeRange {}
impl ::core::clone::Clone for MediaTimeRange {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaTimelineController = *mut ::core::ffi::c_void;
pub type MediaTimelineControllerFailedEventArgs = *mut ::core::ffi::c_void;
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
