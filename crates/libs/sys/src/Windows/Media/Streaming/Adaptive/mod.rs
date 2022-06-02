pub type AdaptiveMediaSource = *mut ::core::ffi::c_void;
pub type AdaptiveMediaSourceAdvancedSettings = *mut ::core::ffi::c_void;
pub type AdaptiveMediaSourceCorrelatedTimes = *mut ::core::ffi::c_void;
pub type AdaptiveMediaSourceCreationResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
#[repr(transparent)]
pub struct AdaptiveMediaSourceCreationStatus(pub i32);
impl AdaptiveMediaSourceCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const ManifestDownloadFailure: Self = Self(1i32);
    pub const ManifestParseFailure: Self = Self(2i32);
    pub const UnsupportedManifestContentType: Self = Self(3i32);
    pub const UnsupportedManifestVersion: Self = Self(4i32);
    pub const UnsupportedManifestProfile: Self = Self(5i32);
    pub const UnknownFailure: Self = Self(6i32);
}
impl ::core::marker::Copy for AdaptiveMediaSourceCreationStatus {}
impl ::core::clone::Clone for AdaptiveMediaSourceCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AdaptiveMediaSourceDiagnosticAvailableEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
#[repr(transparent)]
pub struct AdaptiveMediaSourceDiagnosticType(pub i32);
impl AdaptiveMediaSourceDiagnosticType {
    pub const ManifestUnchangedUponReload: Self = Self(0i32);
    pub const ManifestMismatchUponReload: Self = Self(1i32);
    pub const ManifestSignaledEndOfLiveEventUponReload: Self = Self(2i32);
    pub const MediaSegmentSkipped: Self = Self(3i32);
    pub const ResourceNotFound: Self = Self(4i32);
    pub const ResourceTimedOut: Self = Self(5i32);
    pub const ResourceParsingError: Self = Self(6i32);
    pub const BitrateDisabled: Self = Self(7i32);
    pub const FatalMediaSourceError: Self = Self(8i32);
}
impl ::core::marker::Copy for AdaptiveMediaSourceDiagnosticType {}
impl ::core::clone::Clone for AdaptiveMediaSourceDiagnosticType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AdaptiveMediaSourceDiagnostics = *mut ::core::ffi::c_void;
pub type AdaptiveMediaSourceDownloadBitrateChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadBitrateChangedReason(pub i32);
impl AdaptiveMediaSourceDownloadBitrateChangedReason {
    pub const SufficientInboundBitsPerSecond: Self = Self(0i32);
    pub const InsufficientInboundBitsPerSecond: Self = Self(1i32);
    pub const LowBufferLevel: Self = Self(2i32);
    pub const PositionChanged: Self = Self(3i32);
    pub const TrackSelectionChanged: Self = Self(4i32);
    pub const DesiredBitratesChanged: Self = Self(5i32);
    pub const ErrorInPreviousBitrate: Self = Self(6i32);
}
impl ::core::marker::Copy for AdaptiveMediaSourceDownloadBitrateChangedReason {}
impl ::core::clone::Clone for AdaptiveMediaSourceDownloadBitrateChangedReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AdaptiveMediaSourceDownloadCompletedEventArgs = *mut ::core::ffi::c_void;
pub type AdaptiveMediaSourceDownloadFailedEventArgs = *mut ::core::ffi::c_void;
pub type AdaptiveMediaSourceDownloadRequestedDeferral = *mut ::core::ffi::c_void;
pub type AdaptiveMediaSourceDownloadRequestedEventArgs = *mut ::core::ffi::c_void;
pub type AdaptiveMediaSourceDownloadResult = *mut ::core::ffi::c_void;
pub type AdaptiveMediaSourceDownloadStatistics = *mut ::core::ffi::c_void;
pub type AdaptiveMediaSourcePlaybackBitrateChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
#[repr(transparent)]
pub struct AdaptiveMediaSourceResourceType(pub i32);
impl AdaptiveMediaSourceResourceType {
    pub const Manifest: Self = Self(0i32);
    pub const InitializationSegment: Self = Self(1i32);
    pub const MediaSegment: Self = Self(2i32);
    pub const Key: Self = Self(3i32);
    pub const InitializationVector: Self = Self(4i32);
    pub const MediaSegmentIndex: Self = Self(5i32);
}
impl ::core::marker::Copy for AdaptiveMediaSourceResourceType {}
impl ::core::clone::Clone for AdaptiveMediaSourceResourceType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IAdaptiveMediaSource {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsLive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DesiredLiveOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesiredLiveOffset: usize,
    #[cfg(feature = "Foundation")]
    pub SetDesiredLiveOffset: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredLiveOffset: usize,
    pub InitialBitrate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetInitialBitrate: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub CurrentDownloadBitrate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CurrentPlaybackBitrate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AvailableBitrates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AvailableBitrates: usize,
    #[cfg(feature = "Foundation")]
    pub DesiredMinBitrate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesiredMinBitrate: usize,
    #[cfg(feature = "Foundation")]
    pub SetDesiredMinBitrate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredMinBitrate: usize,
    #[cfg(feature = "Foundation")]
    pub DesiredMaxBitrate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesiredMaxBitrate: usize,
    #[cfg(feature = "Foundation")]
    pub SetDesiredMaxBitrate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredMaxBitrate: usize,
    pub AudioOnlyPlayback: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub InboundBitsPerSecond: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InboundBitsPerSecondWindow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InboundBitsPerSecondWindow: usize,
    #[cfg(feature = "Foundation")]
    pub SetInboundBitsPerSecondWindow: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInboundBitsPerSecondWindow: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadBitrateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadBitrateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDownloadBitrateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDownloadBitrateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PlaybackBitrateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackBitrateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlaybackBitrateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlaybackBitrateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDownloadRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDownloadRequested: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDownloadCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDownloadCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadFailed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDownloadFailed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDownloadFailed: usize,
}
#[repr(C)]
pub struct IAdaptiveMediaSource2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AdvancedSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdaptiveMediaSource3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub MinLiveOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinLiveOffset: usize,
    #[cfg(feature = "Foundation")]
    pub MaxSeekableWindowSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxSeekableWindowSize: usize,
    #[cfg(feature = "Foundation")]
    pub DesiredSeekableWindowSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesiredSeekableWindowSize: usize,
    #[cfg(feature = "Foundation")]
    pub SetDesiredSeekableWindowSize: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredSeekableWindowSize: usize,
    pub Diagnostics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCorrelatedTimes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdaptiveMediaSourceAdvancedSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllSegmentsIndependent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllSegmentsIndependent: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DesiredBitrateHeadroomRatio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesiredBitrateHeadroomRatio: usize,
    #[cfg(feature = "Foundation")]
    pub SetDesiredBitrateHeadroomRatio: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredBitrateHeadroomRatio: usize,
    #[cfg(feature = "Foundation")]
    pub BitrateDowngradeTriggerRatio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BitrateDowngradeTriggerRatio: usize,
    #[cfg(feature = "Foundation")]
    pub SetBitrateDowngradeTriggerRatio: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBitrateDowngradeTriggerRatio: usize,
}
#[repr(C)]
pub struct IAdaptiveMediaSourceCorrelatedTimes {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub PresentationTimeStamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PresentationTimeStamp: usize,
    #[cfg(feature = "Foundation")]
    pub ProgramDateTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProgramDateTime: usize,
}
#[repr(C)]
pub struct IAdaptiveMediaSourceCreationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AdaptiveMediaSourceCreationStatus) -> ::windows_sys::core::HRESULT,
    pub MediaSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Web_Http")]
    pub HttpResponseMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    HttpResponseMessage: usize,
}
#[repr(C)]
pub struct IAdaptiveMediaSourceCreationResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub DiagnosticType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AdaptiveMediaSourceDiagnosticType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestId: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub SegmentId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SegmentId: usize,
    #[cfg(feature = "Foundation")]
    pub ResourceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceType: usize,
    #[cfg(feature = "Foundation")]
    pub ResourceUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceUri: usize,
    #[cfg(feature = "Foundation")]
    pub ResourceByteRangeOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceByteRangeOffset: usize,
    #[cfg(feature = "Foundation")]
    pub ResourceByteRangeLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceByteRangeLength: usize,
    #[cfg(feature = "Foundation")]
    pub Bitrate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Bitrate: usize,
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ResourceDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceDuration: usize,
    pub ResourceContentType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDiagnostics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub DiagnosticAvailable: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DiagnosticAvailable: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDiagnosticAvailable: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDiagnosticAvailable: usize,
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub OldValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub NewValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AdaptiveMediaSourceDownloadBitrateChangedReason) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ResourceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AdaptiveMediaSourceResourceType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ResourceUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceUri: usize,
    #[cfg(feature = "Foundation")]
    pub ResourceByteRangeOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceByteRangeOffset: usize,
    #[cfg(feature = "Foundation")]
    pub ResourceByteRangeLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceByteRangeLength: usize,
    #[cfg(feature = "Web_Http")]
    pub HttpResponseMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    HttpResponseMessage: usize,
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Statistics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ResourceDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceDuration: usize,
    pub ResourceContentType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ResourceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AdaptiveMediaSourceResourceType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ResourceUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceUri: usize,
    #[cfg(feature = "Foundation")]
    pub ResourceByteRangeOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceByteRangeOffset: usize,
    #[cfg(feature = "Foundation")]
    pub ResourceByteRangeLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceByteRangeLength: usize,
    #[cfg(feature = "Web_Http")]
    pub HttpResponseMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    HttpResponseMessage: usize,
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub Statistics: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ResourceDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceDuration: usize,
    pub ResourceContentType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadRequestedDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ResourceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AdaptiveMediaSourceResourceType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ResourceUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceUri: usize,
    #[cfg(feature = "Foundation")]
    pub ResourceByteRangeOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceByteRangeOffset: usize,
    #[cfg(feature = "Foundation")]
    pub ResourceByteRangeLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceByteRangeLength: usize,
    pub Result: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ResourceDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceDuration: usize,
    pub ResourceContentType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ResourceUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetResourceUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetResourceUri: usize,
    #[cfg(feature = "Storage_Streams")]
    pub InputStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InputStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetInputStream: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetInputStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Buffer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Buffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetBuffer: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetBuffer: usize,
    pub ContentType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetContentType: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetExtendedStatus: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ResourceByteRangeOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceByteRangeOffset: usize,
    #[cfg(feature = "Foundation")]
    pub SetResourceByteRangeOffset: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetResourceByteRangeOffset: usize,
    #[cfg(feature = "Foundation")]
    pub ResourceByteRangeLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceByteRangeLength: usize,
    #[cfg(feature = "Foundation")]
    pub SetResourceByteRangeLength: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetResourceByteRangeLength: usize,
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadStatistics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContentBytesReceivedCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TimeToHeadersReceived: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeToHeadersReceived: usize,
    #[cfg(feature = "Foundation")]
    pub TimeToFirstByteReceived: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeToFirstByteReceived: usize,
    #[cfg(feature = "Foundation")]
    pub TimeToLastByteReceived: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeToLastByteReceived: usize,
}
#[repr(C)]
pub struct IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub OldValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub NewValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub AudioOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAdaptiveMediaSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsContentTypeSupported: unsafe extern "system" fn(this: *mut *mut Self, contenttype: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateFromUriAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFromUriAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Http"))]
    pub CreateFromUriWithDownloaderAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, httpclient: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Http")))]
    CreateFromUriWithDownloaderAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateFromStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, contenttype: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateFromStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Http"))]
    pub CreateFromStreamWithDownloaderAsync: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, contenttype: ::windows_sys::core::HSTRING, httpclient: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Http")))]
    CreateFromStreamWithDownloaderAsync: usize,
}
