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
impl ::windows_sys::core::Interface for IAdaptiveMediaSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1282618095, data2: 54175, data3: 17302, data4: [180, 217, 4, 57, 87, 167, 201, 100] };
}
#[repr(C)]
pub struct IAdaptiveMediaSource2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AdvancedSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdaptiveMediaSource2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 394855234, data2: 26464, data3: 19385, data4: [165, 138, 247, 170, 152, 176, 140, 14] };
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
impl ::windows_sys::core::Interface for IAdaptiveMediaSource3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3127911421, data2: 49972, data3: 17947, data4: [163, 110, 201, 159, 84, 247, 23, 74] };
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
impl ::windows_sys::core::Interface for IAdaptiveMediaSourceAdvancedSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1440421504, data2: 6891, data3: 18396, data4: [170, 8, 154, 17, 97, 11, 164, 90] };
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
impl ::windows_sys::core::Interface for IAdaptiveMediaSourceCorrelatedTimes {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 84969351, data2: 57394, data3: 18657, data4: [171, 141, 0, 43, 11, 48, 81, 223] };
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
impl ::windows_sys::core::Interface for IAdaptiveMediaSourceCreationResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1183233714, data2: 32783, data3: 20017, data4: [144, 147, 118, 212, 120, 32, 19, 231] };
}
#[repr(C)]
pub struct IAdaptiveMediaSourceCreationResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdaptiveMediaSourceCreationResult2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 473056191, data2: 7236, data3: 16459, data4: [162, 1, 223, 69, 172, 120, 152, 232] };
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
impl ::windows_sys::core::Interface for IAdaptiveMediaSourceDiagnosticAvailableEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 989220614, data2: 28060, data3: 18762, data4: [183, 169, 179, 165, 222, 230, 173, 104] };
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdaptiveMediaSourceDiagnosticAvailableEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2356009047, data2: 5797, data3: 19871, data4: [129, 14, 0, 189, 144, 27, 62, 249] };
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
impl ::windows_sys::core::Interface for IAdaptiveMediaSourceDiagnosticAvailableEventArgs3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3278179541, data2: 56043, data3: 16643, data4: [132, 218, 104, 118, 154, 213, 19, 255] };
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
impl ::windows_sys::core::Interface for IAdaptiveMediaSourceDiagnostics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2602888808, data2: 38446, data3: 17548, data4: [174, 191, 178, 155, 86, 9, 142, 35] };
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub OldValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub NewValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1728842308, data2: 57422, data3: 20223, data4: [129, 106, 23, 57, 159, 120, 244, 186] };
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AdaptiveMediaSourceDownloadBitrateChangedReason) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4092720196, data2: 38574, data3: 19936, data4: [181, 64, 43, 50, 70, 230, 150, 140] };
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
impl ::windows_sys::core::Interface for IAdaptiveMediaSourceDownloadCompletedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 421793219, data2: 23351, data3: 18970, data4: [137, 112, 214, 33, 203, 108, 168, 59] };
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
impl ::windows_sys::core::Interface for IAdaptiveMediaSourceDownloadCompletedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1883718852, data2: 38474, data3: 16612, data4: [175, 149, 145, 119, 221, 109, 250, 0] };
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
impl ::windows_sys::core::Interface for IAdaptiveMediaSourceDownloadCompletedEventArgs3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 260738001, data2: 37810, data3: 18374, data4: [186, 220, 139, 226, 200, 247, 246, 232] };
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
impl ::windows_sys::core::Interface for IAdaptiveMediaSourceDownloadFailedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 930320456, data2: 62635, data3: 16548, data4: [177, 53, 198, 223, 216, 189, 127, 241] };
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
impl ::windows_sys::core::Interface for IAdaptiveMediaSourceDownloadFailedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1888589160, data2: 38524, data3: 18822, data4: [144, 197, 198, 252, 75, 49, 226, 216] };
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
impl ::windows_sys::core::Interface for IAdaptiveMediaSourceDownloadFailedEventArgs3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3493152073, data2: 4402, data3: 18960, data4: [145, 90, 194, 33, 27, 91, 148, 9] };
}
#[repr(C)]
pub struct IAdaptiveMediaSourceDownloadRequestedDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdaptiveMediaSourceDownloadRequestedDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 96898916, data2: 64032, data3: 19901, data4: [152, 33, 75, 244, 201, 191, 119, 171] };
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
impl ::windows_sys::core::Interface for IAdaptiveMediaSourceDownloadRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3359629309, data2: 17577, data3: 18338, data4: [191, 150, 3, 57, 139, 75, 250, 175] };
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
impl ::windows_sys::core::Interface for IAdaptiveMediaSourceDownloadRequestedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3011349502, data2: 43588, data3: 19842, data4: [130, 91, 97, 29, 227, 188, 254, 203] };
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
impl ::windows_sys::core::Interface for IAdaptiveMediaSourceDownloadRequestedEventArgs3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 859590909, data2: 20322, data3: 17537, data4: [171, 68, 30, 71, 176, 87, 66, 37] };
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
impl ::windows_sys::core::Interface for IAdaptiveMediaSourceDownloadResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4105165939, data2: 48366, data3: 19050, data4: [159, 10, 254, 196, 30, 35, 57, 176] };
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
impl ::windows_sys::core::Interface for IAdaptiveMediaSourceDownloadResult2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 357903543, data2: 31616, data3: 19140, data4: [134, 96, 164, 185, 127, 124, 112, 240] };
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
impl ::windows_sys::core::Interface for IAdaptiveMediaSourceDownloadStatistics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2735132411, data2: 59754, data3: 19967, data4: [169, 184, 26, 224, 140, 1, 174, 152] };
}
#[repr(C)]
pub struct IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub OldValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub NewValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub AudioOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 597860205, data2: 32218, data3: 19025, data4: [135, 169, 111, 168, 197, 178, 146, 190] };
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
impl ::windows_sys::core::Interface for IAdaptiveMediaSourceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1353104733, data2: 26351, data3: 19667, data4: [149, 121, 158, 102, 5, 7, 220, 63] };
}
