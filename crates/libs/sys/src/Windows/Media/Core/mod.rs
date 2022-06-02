#[cfg(feature = "Media_Core_Preview")]
pub mod Preview;
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct AudioDecoderDegradation(pub i32);
impl AudioDecoderDegradation {
    pub const None: Self = Self(0i32);
    pub const DownmixTo2Channels: Self = Self(1i32);
    pub const DownmixTo6Channels: Self = Self(2i32);
    pub const DownmixTo8Channels: Self = Self(3i32);
}
impl ::core::marker::Copy for AudioDecoderDegradation {}
impl ::core::clone::Clone for AudioDecoderDegradation {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct AudioDecoderDegradationReason(pub i32);
impl AudioDecoderDegradationReason {
    pub const None: Self = Self(0i32);
    pub const LicensingRequirement: Self = Self(1i32);
    pub const SpatialAudioNotSupported: Self = Self(2i32);
}
impl ::core::marker::Copy for AudioDecoderDegradationReason {}
impl ::core::clone::Clone for AudioDecoderDegradationReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AudioStreamDescriptor = *mut ::core::ffi::c_void;
pub type AudioTrack = *mut ::core::ffi::c_void;
pub type AudioTrackOpenFailedEventArgs = *mut ::core::ffi::c_void;
pub type AudioTrackSupportInfo = *mut ::core::ffi::c_void;
pub type ChapterCue = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct CodecCategory(pub i32);
impl CodecCategory {
    pub const Encoder: Self = Self(0i32);
    pub const Decoder: Self = Self(1i32);
}
impl ::core::marker::Copy for CodecCategory {}
impl ::core::clone::Clone for CodecCategory {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CodecInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct CodecKind(pub i32);
impl CodecKind {
    pub const Audio: Self = Self(0i32);
    pub const Video: Self = Self(1i32);
}
impl ::core::marker::Copy for CodecKind {}
impl ::core::clone::Clone for CodecKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CodecQuery = *mut ::core::ffi::c_void;
pub type DataCue = *mut ::core::ffi::c_void;
pub type FaceDetectedEventArgs = *mut ::core::ffi::c_void;
pub type FaceDetectionEffect = *mut ::core::ffi::c_void;
pub type FaceDetectionEffectDefinition = *mut ::core::ffi::c_void;
pub type FaceDetectionEffectFrame = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct FaceDetectionMode(pub i32);
impl FaceDetectionMode {
    pub const HighPerformance: Self = Self(0i32);
    pub const Balanced: Self = Self(1i32);
    pub const HighQuality: Self = Self(2i32);
}
impl ::core::marker::Copy for FaceDetectionMode {}
impl ::core::clone::Clone for FaceDetectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HighDynamicRangeControl = *mut ::core::ffi::c_void;
pub type HighDynamicRangeOutput = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAudioStreamDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_MediaProperties")]
    pub EncodingProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    EncodingProperties: usize,
}
#[repr(C)]
pub struct IAudioStreamDescriptor2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetLeadingEncoderPadding: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLeadingEncoderPadding: usize,
    #[cfg(feature = "Foundation")]
    pub LeadingEncoderPadding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LeadingEncoderPadding: usize,
    #[cfg(feature = "Foundation")]
    pub SetTrailingEncoderPadding: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTrailingEncoderPadding: usize,
    #[cfg(feature = "Foundation")]
    pub TrailingEncoderPadding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrailingEncoderPadding: usize,
}
#[repr(C)]
pub struct IAudioStreamDescriptor3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Copy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioStreamDescriptorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_MediaProperties")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, encodingproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    Create: usize,
}
#[repr(C)]
pub struct IAudioTrack {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub OpenFailed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOpenFailed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOpenFailed: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetEncodingProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetEncodingProperties: usize,
    #[cfg(feature = "Media_Playback")]
    pub PlaybackItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    PlaybackItem: usize,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SupportInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioTrackOpenFailedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioTrackSupportInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub DecoderStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaDecoderStatus) -> ::windows_sys::core::HRESULT,
    pub Degradation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioDecoderDegradation) -> ::windows_sys::core::HRESULT,
    pub DegradationReason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioDecoderDegradationReason) -> ::windows_sys::core::HRESULT,
    pub MediaSourceStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaSourceStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IChapterCue {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICodecInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CodecKind) -> ::windows_sys::core::HRESULT,
    pub Category: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CodecCategory) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Subtypes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Subtypes: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsTrusted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICodecQuery {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut *mut Self, kind: CodecKind, category: CodecCategory, subtype: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
}
#[repr(C)]
pub struct ICodecSubtypesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub VideoFormatDV25: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatDV50: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatDvc: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatDvh1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatDvhD: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatDvsd: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatDvsl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatH263: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatH264: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatH265: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatH264ES: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatHevc: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatHevcES: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatM4S2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatMjpg: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatMP43: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatMP4S: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatMP4V: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatMpeg2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatVP80: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatVP90: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatMpg1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatMss1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatMss2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatWmv1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatWmv2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatWmv3: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormatWvc1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoFormat420O: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AudioFormatAac: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AudioFormatAdts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AudioFormatAlac: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AudioFormatAmrNB: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AudioFormatAmrWB: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AudioFormatAmrWP: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AudioFormatDolbyAC3: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AudioFormatDolbyAC3Spdif: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AudioFormatDolbyDDPlus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AudioFormatDrm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AudioFormatDts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AudioFormatFlac: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AudioFormatFloat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AudioFormatMP3: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AudioFormatMPeg: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AudioFormatMsp1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AudioFormatOpus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AudioFormatPcm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AudioFormatWmaSpdif: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AudioFormatWMAudioLossless: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AudioFormatWMAudioV8: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AudioFormatWMAudioV9: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDataCue {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
}
#[repr(C)]
pub struct IDataCue2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct IFaceDetectedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ResultFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFaceDetectionEffect {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetDesiredDetectionInterval: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredDetectionInterval: usize,
    #[cfg(feature = "Foundation")]
    pub DesiredDetectionInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesiredDetectionInterval: usize,
    #[cfg(feature = "Foundation")]
    pub FaceDetected: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FaceDetected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFaceDetected: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFaceDetected: usize,
}
#[repr(C)]
pub struct IFaceDetectionEffectDefinition {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetDetectionMode: unsafe extern "system" fn(this: *mut *mut Self, value: FaceDetectionMode) -> ::windows_sys::core::HRESULT,
    pub DetectionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FaceDetectionMode) -> ::windows_sys::core::HRESULT,
    pub SetSynchronousDetectionEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SynchronousDetectionEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFaceDetectionEffectFrame {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_FaceAnalysis"))]
    pub DetectedFaces: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_FaceAnalysis")))]
    DetectedFaces: usize,
}
#[repr(C)]
pub struct IHighDynamicRangeControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHighDynamicRangeOutput {
    pub base__: ::windows_sys::core::IInspectable,
    pub Certainty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Devices_Core"))]
    pub FrameControllers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Devices_Core")))]
    FrameControllers: usize,
}
#[repr(C)]
pub struct IImageCue {
    pub base__: ::windows_sys::core::IInspectable,
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedTextPoint) -> ::windows_sys::core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut *mut Self, value: TimedTextPoint) -> ::windows_sys::core::HRESULT,
    pub Extent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedTextSize) -> ::windows_sys::core::HRESULT,
    pub SetExtent: unsafe extern "system" fn(this: *mut *mut Self, value: TimedTextSize) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetSoftwareBitmap: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetSoftwareBitmap: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SoftwareBitmap: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SoftwareBitmap: usize,
}
#[repr(C)]
pub struct IInitializeMediaStreamSourceRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub RandomAccessStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RandomAccessStream: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(C)]
pub struct ILowLightFusionResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_Imaging")]
    pub Frame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    Frame: usize,
}
#[repr(C)]
pub struct ILowLightFusionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub SupportedBitmapPixelFormats: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    SupportedBitmapPixelFormats: usize,
    pub MaxSupportedFrameCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub FuseAsync: unsafe extern "system" fn(this: *mut *mut Self, frameset: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    FuseAsync: usize,
}
#[repr(C)]
pub struct IMediaBinder {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Binding: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Binding: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBinding: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBinding: usize,
    pub Token: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetToken: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaBindingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Canceled: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Canceled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCanceled: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCanceled: usize,
    pub MediaBinder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStream: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, contenttype: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStreamReference: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, contenttype: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStreamReference: usize,
}
#[repr(C)]
pub struct IMediaBindingEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Streaming_Adaptive")]
    pub SetAdaptiveMediaSource: unsafe extern "system" fn(this: *mut *mut Self, mediasource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Streaming_Adaptive"))]
    SetAdaptiveMediaSource: usize,
    #[cfg(feature = "Storage")]
    pub SetStorageFile: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SetStorageFile: usize,
}
#[repr(C)]
pub struct IMediaBindingEventArgs3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Networking_BackgroundTransfer")]
    pub SetDownloadOperation: unsafe extern "system" fn(this: *mut *mut Self, downloadoperation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_BackgroundTransfer"))]
    SetDownloadOperation: usize,
}
#[repr(C)]
pub struct IMediaCue {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaCueEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Cue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaSource {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IMediaSource2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub OpenOperationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenOperationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOpenOperationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOpenOperationCompleted: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CustomProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CustomProperties: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub IsOpen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ExternalTimedTextSources: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExternalTimedTextSources: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ExternalTimedMetadataTracks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExternalTimedMetadataTracks: usize,
}
#[repr(C)]
pub struct IMediaSource3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaSourceState) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaSource4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Streaming_Adaptive")]
    pub AdaptiveMediaSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Streaming_Adaptive"))]
    AdaptiveMediaSource: usize,
    pub MediaStreamSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MseStreamSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub OpenAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenAsync: usize,
}
#[repr(C)]
pub struct IMediaSource5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Networking_BackgroundTransfer")]
    pub DownloadOperation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_BackgroundTransfer"))]
    DownloadOperation: usize,
}
#[repr(C)]
pub struct IMediaSourceAppServiceConnection {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub InitializeMediaStreamSourceRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InitializeMediaStreamSourceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInitializeMediaStreamSourceRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInitializeMediaStreamSourceRequested: usize,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaSourceAppServiceConnectionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_AppService")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, appserviceconnection: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))]
    Create: usize,
}
#[repr(C)]
pub struct IMediaSourceError {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaSourceOpenOperationCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaSourceStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub OldState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaSourceState) -> ::windows_sys::core::HRESULT,
    pub NewState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaSourceState) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Streaming_Adaptive")]
    pub CreateFromAdaptiveMediaSource: unsafe extern "system" fn(this: *mut *mut Self, mediasource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Streaming_Adaptive"))]
    CreateFromAdaptiveMediaSource: usize,
    pub CreateFromMediaStreamSource: unsafe extern "system" fn(this: *mut *mut Self, mediasource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromMseStreamSource: unsafe extern "system" fn(this: *mut *mut Self, mediasource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromIMediaSource: unsafe extern "system" fn(this: *mut *mut Self, mediasource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage")]
    pub CreateFromStorageFile: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CreateFromStorageFile: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStream: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, contenttype: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStreamReference: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, contenttype: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStreamReference: usize,
    #[cfg(feature = "Foundation")]
    pub CreateFromUri: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFromUri: usize,
}
#[repr(C)]
pub struct IMediaSourceStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromMediaBinder: unsafe extern "system" fn(this: *mut *mut Self, binder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaSourceStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Capture_Frames")]
    pub CreateFromMediaFrameSource: unsafe extern "system" fn(this: *mut *mut Self, framesource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Capture_Frames"))]
    CreateFromMediaFrameSource: usize,
}
#[repr(C)]
pub struct IMediaSourceStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Networking_BackgroundTransfer")]
    pub CreateFromDownloadOperation: unsafe extern "system" fn(this: *mut *mut Self, downloadoperation: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_BackgroundTransfer"))]
    CreateFromDownloadOperation: usize,
}
#[repr(C)]
pub struct IMediaStreamDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSelected: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaStreamDescriptor2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetLabel: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaStreamSample {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Processed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Processed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProcessed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProcessed: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Buffer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Buffer: usize,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ExtendedProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExtendedProperties: usize,
    pub Protection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetDecodeTimestamp: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDecodeTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub DecodeTimestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DecodeTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub SetKeyFrame: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub KeyFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetDiscontinuous: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Discontinuous: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaStreamSample2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3D11Surface: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3D11Surface: usize,
}
#[repr(C)]
pub struct IMediaStreamSampleProtectionProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetKeyIdentifier: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const u8) -> ::windows_sys::core::HRESULT,
    pub GetKeyIdentifier: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetInitializationVector: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const u8) -> ::windows_sys::core::HRESULT,
    pub GetInitializationVector: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetSubSampleMapping: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const u8) -> ::windows_sys::core::HRESULT,
    pub GetSubSampleMapping: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaStreamSampleStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, buffer: *mut ::core::ffi::c_void, timestamp: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateFromBuffer: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateFromStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, count: u32, timestamp: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateFromStreamAsync: usize,
}
#[repr(C)]
pub struct IMediaStreamSampleStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub CreateFromDirect3D11Surface: unsafe extern "system" fn(this: *mut *mut Self, surface: *mut ::core::ffi::c_void, timestamp: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11")))]
    CreateFromDirect3D11Surface: usize,
}
#[repr(C)]
pub struct IMediaStreamSource {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    #[cfg(feature = "Foundation")]
    pub Starting: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Starting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStarting: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStarting: usize,
    #[cfg(feature = "Foundation")]
    pub Paused: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Paused: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePaused: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePaused: usize,
    #[cfg(feature = "Foundation")]
    pub SampleRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SampleRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSampleRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSampleRequested: usize,
    #[cfg(feature = "Foundation")]
    pub SwitchStreamsRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SwitchStreamsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSwitchStreamsRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSwitchStreamsRequested: usize,
    pub NotifyError: unsafe extern "system" fn(this: *mut *mut Self, errorstatus: MediaStreamSourceErrorStatus) -> ::windows_sys::core::HRESULT,
    pub AddStreamDescriptor: unsafe extern "system" fn(this: *mut *mut Self, descriptor: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_Protection")]
    pub SetMediaProtectionManager: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Protection"))]
    SetMediaProtectionManager: usize,
    #[cfg(feature = "Media_Protection")]
    pub MediaProtectionManager: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Protection"))]
    MediaProtectionManager: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub SetCanSeek: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CanSeek: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetBufferTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBufferTime: usize,
    #[cfg(feature = "Foundation")]
    pub BufferTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BufferTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetBufferedRange: unsafe extern "system" fn(this: *mut *mut Self, startoffset: super::super::Foundation::TimeSpan, endoffset: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBufferedRange: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub MusicProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    MusicProperties: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub VideoProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    VideoProperties: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetThumbnail: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    pub AddProtectionKey: unsafe extern "system" fn(this: *mut *mut Self, streamdescriptor: *mut ::core::ffi::c_void, keyIdentifier_array_size: u32, keyidentifier: *const u8, licenseData_array_size: u32, licensedata: *const u8) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaStreamSource2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SampleRendered: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SampleRendered: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSampleRendered: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSampleRendered: usize,
}
#[repr(C)]
pub struct IMediaStreamSource3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetMaxSupportedPlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxSupportedPlaybackRate: usize,
    #[cfg(feature = "Foundation")]
    pub MaxSupportedPlaybackRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxSupportedPlaybackRate: usize,
}
#[repr(C)]
pub struct IMediaStreamSource4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetIsLive: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsLive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaStreamSourceClosedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaStreamSourceClosedRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaStreamSourceClosedReason) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaStreamSourceFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromDescriptor: unsafe extern "system" fn(this: *mut *mut Self, descriptor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromDescriptors: unsafe extern "system" fn(this: *mut *mut Self, descriptor: *mut ::core::ffi::c_void, descriptor2: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaStreamSourceSampleRenderedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SampleLag: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SampleLag: usize,
}
#[repr(C)]
pub struct IMediaStreamSourceSampleRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub StreamDescriptor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSample: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Sample: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ReportSampleProgress: unsafe extern "system" fn(this: *mut *mut Self, progress: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaStreamSourceSampleRequestDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaStreamSourceSampleRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaStreamSourceStartingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaStreamSourceStartingRequest {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub StartPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartPosition: usize,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetActualStartPosition: unsafe extern "system" fn(this: *mut *mut Self, position: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetActualStartPosition: usize,
}
#[repr(C)]
pub struct IMediaStreamSourceStartingRequestDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaStreamSourceSwitchStreamsRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub OldStreamDescriptor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NewStreamDescriptor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaStreamSourceSwitchStreamsRequestDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaStreamSourceSwitchStreamsRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaTrack {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TrackKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaTrackKind) -> ::windows_sys::core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMseSourceBuffer {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub UpdateStarting: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdateStarting: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdateStarting: usize,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateEnded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateEnded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdateEnded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdateEnded: usize,
    #[cfg(feature = "Foundation")]
    pub ErrorOccurred: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ErrorOccurred: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveErrorOccurred: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveErrorOccurred: usize,
    #[cfg(feature = "Foundation")]
    pub Aborted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Aborted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAborted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAborted: usize,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MseAppendMode) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, value: MseAppendMode) -> ::windows_sys::core::HRESULT,
    pub IsUpdating: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Buffered: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Buffered: usize,
    #[cfg(feature = "Foundation")]
    pub TimestampOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimestampOffset: usize,
    #[cfg(feature = "Foundation")]
    pub SetTimestampOffset: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTimestampOffset: usize,
    #[cfg(feature = "Foundation")]
    pub AppendWindowStart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppendWindowStart: usize,
    #[cfg(feature = "Foundation")]
    pub SetAppendWindowStart: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAppendWindowStart: usize,
    #[cfg(feature = "Foundation")]
    pub AppendWindowEnd: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppendWindowEnd: usize,
    #[cfg(feature = "Foundation")]
    pub SetAppendWindowEnd: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAppendWindowEnd: usize,
    #[cfg(feature = "Storage_Streams")]
    pub AppendBuffer: unsafe extern "system" fn(this: *mut *mut Self, buffer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AppendBuffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub AppendStream: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AppendStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub AppendStreamMaxSize: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, maxsize: u64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AppendStreamMaxSize: usize,
    pub Abort: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, start: super::super::Foundation::TimeSpan, end: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Remove: usize,
}
#[repr(C)]
pub struct IMseSourceBufferList {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SourceBufferAdded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceBufferAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSourceBufferAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSourceBufferAdded: usize,
    #[cfg(feature = "Foundation")]
    pub SourceBufferRemoved: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceBufferRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSourceBufferRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSourceBufferRemoved: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Buffers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Buffers: usize,
}
#[repr(C)]
pub struct IMseStreamSource {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Opened: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Opened: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOpened: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOpened: usize,
    #[cfg(feature = "Foundation")]
    pub Ended: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Ended: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnded: usize,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    pub SourceBuffers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ActiveSourceBuffers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ReadyState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MseReadyState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    pub AddSourceBuffer: unsafe extern "system" fn(this: *mut *mut Self, mimetype: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveSourceBuffer: unsafe extern "system" fn(this: *mut *mut Self, buffer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EndOfStream: unsafe extern "system" fn(this: *mut *mut Self, status: MseEndOfStreamStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMseStreamSource2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub LiveSeekableRange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LiveSeekableRange: usize,
    #[cfg(feature = "Foundation")]
    pub SetLiveSeekableRange: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLiveSeekableRange: usize,
}
#[repr(C)]
pub struct IMseStreamSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsContentTypeSupported: unsafe extern "system" fn(this: *mut *mut Self, contenttype: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISceneAnalysisEffect {
    pub base__: ::windows_sys::core::IInspectable,
    pub HighDynamicRangeAnalyzer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetDesiredAnalysisInterval: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredAnalysisInterval: usize,
    #[cfg(feature = "Foundation")]
    pub DesiredAnalysisInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesiredAnalysisInterval: usize,
    #[cfg(feature = "Foundation")]
    pub SceneAnalyzed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SceneAnalyzed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSceneAnalyzed: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSceneAnalyzed: usize,
}
#[repr(C)]
pub struct ISceneAnalysisEffectFrame {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Capture")]
    pub FrameControlValues: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    FrameControlValues: usize,
    pub HighDynamicRange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISceneAnalysisEffectFrame2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AnalysisRecommendation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SceneAnalysisRecommendation) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISceneAnalyzedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ResultFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISingleSelectMediaTrackList {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SelectedIndexChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectedIndexChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSelectedIndexChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSelectedIndexChanged: usize,
    pub SetSelectedIndex: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub SelectedIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpeechCue {
    pub base__: ::windows_sys::core::IInspectable,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartPositionInInput: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartPositionInInput: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartPositionInInput: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartPositionInInput: usize,
    #[cfg(feature = "Foundation")]
    pub EndPositionInInput: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndPositionInInput: usize,
    #[cfg(feature = "Foundation")]
    pub SetEndPositionInInput: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEndPositionInInput: usize,
}
#[repr(C)]
pub struct ITimedMetadataStreamDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_MediaProperties")]
    pub EncodingProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    EncodingProperties: usize,
    pub Copy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimedMetadataStreamDescriptorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_MediaProperties")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, encodingproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    Create: usize,
}
#[repr(C)]
pub struct ITimedMetadataTrack {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CueEntered: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CueEntered: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCueEntered: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCueEntered: usize,
    #[cfg(feature = "Foundation")]
    pub CueExited: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CueExited: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCueExited: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCueExited: usize,
    #[cfg(feature = "Foundation")]
    pub TrackFailed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrackFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTrackFailed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTrackFailed: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Cues: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Cues: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ActiveCues: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ActiveCues: usize,
    pub TimedMetadataKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedMetadataKind) -> ::windows_sys::core::HRESULT,
    pub DispatchType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AddCue: unsafe extern "system" fn(this: *mut *mut Self, cue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveCue: unsafe extern "system" fn(this: *mut *mut Self, cue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimedMetadataTrack2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Playback")]
    pub PlaybackItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    PlaybackItem: usize,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimedMetadataTrackError {
    pub base__: ::windows_sys::core::IInspectable,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedMetadataTrackErrorCode) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimedMetadataTrackFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, language: ::windows_sys::core::HSTRING, kind: TimedMetadataKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimedMetadataTrackFailedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimedMetadataTrackProvider {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub TimedMetadataTracks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TimedMetadataTracks: usize,
}
#[repr(C)]
pub struct ITimedTextBouten {
    pub base__: ::windows_sys::core::IInspectable,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedTextBoutenType) -> ::windows_sys::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut *mut Self, value: TimedTextBoutenType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI")]
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Color: usize,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedTextBoutenPosition) -> ::windows_sys::core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut *mut Self, value: TimedTextBoutenPosition) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimedTextCue {
    pub base__: ::windows_sys::core::IInspectable,
    pub CueRegion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCueRegion: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CueStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCueStyle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Lines: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Lines: usize,
}
#[repr(C)]
pub struct ITimedTextLine {
    pub base__: ::windows_sys::core::IInspectable,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Subformats: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Subformats: usize,
}
#[repr(C)]
pub struct ITimedTextRegion {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedTextPoint) -> ::windows_sys::core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut *mut Self, value: TimedTextPoint) -> ::windows_sys::core::HRESULT,
    pub Extent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedTextSize) -> ::windows_sys::core::HRESULT,
    pub SetExtent: unsafe extern "system" fn(this: *mut *mut Self, value: TimedTextSize) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI")]
    pub Background: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Background: usize,
    #[cfg(feature = "UI")]
    pub SetBackground: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetBackground: usize,
    pub WritingMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedTextWritingMode) -> ::windows_sys::core::HRESULT,
    pub SetWritingMode: unsafe extern "system" fn(this: *mut *mut Self, value: TimedTextWritingMode) -> ::windows_sys::core::HRESULT,
    pub DisplayAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedTextDisplayAlignment) -> ::windows_sys::core::HRESULT,
    pub SetDisplayAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: TimedTextDisplayAlignment) -> ::windows_sys::core::HRESULT,
    pub LineHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedTextDouble) -> ::windows_sys::core::HRESULT,
    pub SetLineHeight: unsafe extern "system" fn(this: *mut *mut Self, value: TimedTextDouble) -> ::windows_sys::core::HRESULT,
    pub IsOverflowClipped: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsOverflowClipped: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Padding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedTextPadding) -> ::windows_sys::core::HRESULT,
    pub SetPadding: unsafe extern "system" fn(this: *mut *mut Self, value: TimedTextPadding) -> ::windows_sys::core::HRESULT,
    pub TextWrapping: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedTextWrapping) -> ::windows_sys::core::HRESULT,
    pub SetTextWrapping: unsafe extern "system" fn(this: *mut *mut Self, value: TimedTextWrapping) -> ::windows_sys::core::HRESULT,
    pub ZIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetZIndex: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub ScrollMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedTextScrollMode) -> ::windows_sys::core::HRESULT,
    pub SetScrollMode: unsafe extern "system" fn(this: *mut *mut Self, value: TimedTextScrollMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimedTextRuby {
    pub base__: ::windows_sys::core::IInspectable,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedTextRubyPosition) -> ::windows_sys::core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut *mut Self, value: TimedTextRubyPosition) -> ::windows_sys::core::HRESULT,
    pub Align: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedTextRubyAlign) -> ::windows_sys::core::HRESULT,
    pub SetAlign: unsafe extern "system" fn(this: *mut *mut Self, value: TimedTextRubyAlign) -> ::windows_sys::core::HRESULT,
    pub Reserve: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedTextRubyReserve) -> ::windows_sys::core::HRESULT,
    pub SetReserve: unsafe extern "system" fn(this: *mut *mut Self, value: TimedTextRubyReserve) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimedTextSource {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Resolved: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Resolved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveResolved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveResolved: usize,
}
#[repr(C)]
pub struct ITimedTextSourceResolveResultEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Tracks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Tracks: usize,
}
#[repr(C)]
pub struct ITimedTextSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStream: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStream: usize,
    #[cfg(feature = "Foundation")]
    pub CreateFromUri: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFromUri: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStreamWithLanguage: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, defaultlanguage: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStreamWithLanguage: usize,
    #[cfg(feature = "Foundation")]
    pub CreateFromUriWithLanguage: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, defaultlanguage: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFromUriWithLanguage: usize,
}
#[repr(C)]
pub struct ITimedTextSourceStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStreamWithIndex: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, indexstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStreamWithIndex: usize,
    #[cfg(feature = "Foundation")]
    pub CreateFromUriWithIndex: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, indexuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFromUriWithIndex: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStreamWithIndexAndLanguage: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, indexstream: *mut ::core::ffi::c_void, defaultlanguage: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStreamWithIndexAndLanguage: usize,
    #[cfg(feature = "Foundation")]
    pub CreateFromUriWithIndexAndLanguage: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, indexuri: *mut ::core::ffi::c_void, defaultlanguage: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFromUriWithIndexAndLanguage: usize,
}
#[repr(C)]
pub struct ITimedTextStyle {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FontFamily: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetFontFamily: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FontSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedTextDouble) -> ::windows_sys::core::HRESULT,
    pub SetFontSize: unsafe extern "system" fn(this: *mut *mut Self, value: TimedTextDouble) -> ::windows_sys::core::HRESULT,
    pub FontWeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedTextWeight) -> ::windows_sys::core::HRESULT,
    pub SetFontWeight: unsafe extern "system" fn(this: *mut *mut Self, value: TimedTextWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI")]
    pub Foreground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Foreground: usize,
    #[cfg(feature = "UI")]
    pub SetForeground: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetForeground: usize,
    #[cfg(feature = "UI")]
    pub Background: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Background: usize,
    #[cfg(feature = "UI")]
    pub SetBackground: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetBackground: usize,
    pub IsBackgroundAlwaysShown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsBackgroundAlwaysShown: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub FlowDirection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedTextFlowDirection) -> ::windows_sys::core::HRESULT,
    pub SetFlowDirection: unsafe extern "system" fn(this: *mut *mut Self, value: TimedTextFlowDirection) -> ::windows_sys::core::HRESULT,
    pub LineAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedTextLineAlignment) -> ::windows_sys::core::HRESULT,
    pub SetLineAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: TimedTextLineAlignment) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI")]
    pub OutlineColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    OutlineColor: usize,
    #[cfg(feature = "UI")]
    pub SetOutlineColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetOutlineColor: usize,
    pub OutlineThickness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedTextDouble) -> ::windows_sys::core::HRESULT,
    pub SetOutlineThickness: unsafe extern "system" fn(this: *mut *mut Self, value: TimedTextDouble) -> ::windows_sys::core::HRESULT,
    pub OutlineRadius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedTextDouble) -> ::windows_sys::core::HRESULT,
    pub SetOutlineRadius: unsafe extern "system" fn(this: *mut *mut Self, value: TimedTextDouble) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimedTextStyle2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub FontStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedTextFontStyle) -> ::windows_sys::core::HRESULT,
    pub SetFontStyle: unsafe extern "system" fn(this: *mut *mut Self, value: TimedTextFontStyle) -> ::windows_sys::core::HRESULT,
    pub IsUnderlineEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsUnderlineEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsLineThroughEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsLineThroughEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsOverlineEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsOverlineEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimedTextStyle3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Ruby: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Bouten: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsTextCombined: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsTextCombined: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub FontAngleInDegrees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFontAngleInDegrees: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITimedTextSubformat {
    pub base__: ::windows_sys::core::IInspectable,
    pub StartIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetStartIndex: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetLength: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub SubformatStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSubformatStyle: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVideoStabilizationEffect {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnabledChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnabledChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnabledChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnabledChanged: usize,
    #[cfg(all(feature = "Media_Capture", feature = "Media_Devices", feature = "Media_MediaProperties"))]
    pub GetRecommendedStreamConfiguration: unsafe extern "system" fn(this: *mut *mut Self, controller: *mut ::core::ffi::c_void, desiredproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Media_Capture", feature = "Media_Devices", feature = "Media_MediaProperties")))]
    GetRecommendedStreamConfiguration: usize,
}
#[repr(C)]
pub struct IVideoStabilizationEffectEnabledChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VideoStabilizationEffectEnabledChangedReason) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVideoStreamDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_MediaProperties")]
    pub EncodingProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    EncodingProperties: usize,
}
#[repr(C)]
pub struct IVideoStreamDescriptor2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Copy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVideoStreamDescriptorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_MediaProperties")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, encodingproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    Create: usize,
}
#[repr(C)]
pub struct IVideoTrack {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub OpenFailed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOpenFailed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOpenFailed: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetEncodingProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetEncodingProperties: usize,
    #[cfg(feature = "Media_Playback")]
    pub PlaybackItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    PlaybackItem: usize,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SupportInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVideoTrackOpenFailedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVideoTrackSupportInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub DecoderStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaDecoderStatus) -> ::windows_sys::core::HRESULT,
    pub MediaSourceStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaSourceStatus) -> ::windows_sys::core::HRESULT,
}
pub type ImageCue = *mut ::core::ffi::c_void;
pub type InitializeMediaStreamSourceRequestedEventArgs = *mut ::core::ffi::c_void;
pub type LowLightFusionResult = *mut ::core::ffi::c_void;
pub type MediaBinder = *mut ::core::ffi::c_void;
pub type MediaBindingEventArgs = *mut ::core::ffi::c_void;
pub type MediaCueEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaDecoderStatus(pub i32);
impl MediaDecoderStatus {
    pub const FullySupported: Self = Self(0i32);
    pub const UnsupportedSubtype: Self = Self(1i32);
    pub const UnsupportedEncoderProperties: Self = Self(2i32);
    pub const Degraded: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaDecoderStatus {}
impl ::core::clone::Clone for MediaDecoderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaSource = *mut ::core::ffi::c_void;
pub type MediaSourceAppServiceConnection = *mut ::core::ffi::c_void;
pub type MediaSourceError = *mut ::core::ffi::c_void;
pub type MediaSourceOpenOperationCompletedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaSourceState(pub i32);
impl MediaSourceState {
    pub const Initial: Self = Self(0i32);
    pub const Opening: Self = Self(1i32);
    pub const Opened: Self = Self(2i32);
    pub const Failed: Self = Self(3i32);
    pub const Closed: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaSourceState {}
impl ::core::clone::Clone for MediaSourceState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaSourceStateChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaSourceStatus(pub i32);
impl MediaSourceStatus {
    pub const FullySupported: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaSourceStatus {}
impl ::core::clone::Clone for MediaSourceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaStreamSample = *mut ::core::ffi::c_void;
pub type MediaStreamSamplePropertySet = *mut ::core::ffi::c_void;
pub type MediaStreamSampleProtectionProperties = *mut ::core::ffi::c_void;
pub type MediaStreamSource = *mut ::core::ffi::c_void;
pub type MediaStreamSourceClosedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaStreamSourceClosedReason(pub i32);
impl MediaStreamSourceClosedReason {
    pub const Done: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const AppReportedError: Self = Self(2i32);
    pub const UnsupportedProtectionSystem: Self = Self(3i32);
    pub const ProtectionSystemFailure: Self = Self(4i32);
    pub const UnsupportedEncodingFormat: Self = Self(5i32);
    pub const MissingSampleRequestedEventHandler: Self = Self(6i32);
}
impl ::core::marker::Copy for MediaStreamSourceClosedReason {}
impl ::core::clone::Clone for MediaStreamSourceClosedReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaStreamSourceClosedRequest = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaStreamSourceErrorStatus(pub i32);
impl MediaStreamSourceErrorStatus {
    pub const Other: Self = Self(0i32);
    pub const OutOfMemory: Self = Self(1i32);
    pub const FailedToOpenFile: Self = Self(2i32);
    pub const FailedToConnectToServer: Self = Self(3i32);
    pub const ConnectionToServerLost: Self = Self(4i32);
    pub const UnspecifiedNetworkError: Self = Self(5i32);
    pub const DecodeError: Self = Self(6i32);
    pub const UnsupportedMediaFormat: Self = Self(7i32);
}
impl ::core::marker::Copy for MediaStreamSourceErrorStatus {}
impl ::core::clone::Clone for MediaStreamSourceErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaStreamSourceSampleRenderedEventArgs = *mut ::core::ffi::c_void;
pub type MediaStreamSourceSampleRequest = *mut ::core::ffi::c_void;
pub type MediaStreamSourceSampleRequestDeferral = *mut ::core::ffi::c_void;
pub type MediaStreamSourceSampleRequestedEventArgs = *mut ::core::ffi::c_void;
pub type MediaStreamSourceStartingEventArgs = *mut ::core::ffi::c_void;
pub type MediaStreamSourceStartingRequest = *mut ::core::ffi::c_void;
pub type MediaStreamSourceStartingRequestDeferral = *mut ::core::ffi::c_void;
pub type MediaStreamSourceSwitchStreamsRequest = *mut ::core::ffi::c_void;
pub type MediaStreamSourceSwitchStreamsRequestDeferral = *mut ::core::ffi::c_void;
pub type MediaStreamSourceSwitchStreamsRequestedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaTrackKind(pub i32);
impl MediaTrackKind {
    pub const Audio: Self = Self(0i32);
    pub const Video: Self = Self(1i32);
    pub const TimedMetadata: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaTrackKind {}
impl ::core::clone::Clone for MediaTrackKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MseAppendMode(pub i32);
impl MseAppendMode {
    pub const Segments: Self = Self(0i32);
    pub const Sequence: Self = Self(1i32);
}
impl ::core::marker::Copy for MseAppendMode {}
impl ::core::clone::Clone for MseAppendMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MseEndOfStreamStatus(pub i32);
impl MseEndOfStreamStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const DecodeError: Self = Self(2i32);
    pub const UnknownError: Self = Self(3i32);
}
impl ::core::marker::Copy for MseEndOfStreamStatus {}
impl ::core::clone::Clone for MseEndOfStreamStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MseReadyState(pub i32);
impl MseReadyState {
    pub const Closed: Self = Self(0i32);
    pub const Open: Self = Self(1i32);
    pub const Ended: Self = Self(2i32);
}
impl ::core::marker::Copy for MseReadyState {}
impl ::core::clone::Clone for MseReadyState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MseSourceBuffer = *mut ::core::ffi::c_void;
pub type MseSourceBufferList = *mut ::core::ffi::c_void;
pub type MseStreamSource = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Media_Core\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct MseTimeRange {
    pub Start: super::super::Foundation::TimeSpan,
    pub End: super::super::Foundation::TimeSpan,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for MseTimeRange {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for MseTimeRange {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SceneAnalysisEffect = *mut ::core::ffi::c_void;
pub type SceneAnalysisEffectDefinition = *mut ::core::ffi::c_void;
pub type SceneAnalysisEffectFrame = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct SceneAnalysisRecommendation(pub i32);
impl SceneAnalysisRecommendation {
    pub const Standard: Self = Self(0i32);
    pub const Hdr: Self = Self(1i32);
    pub const LowLight: Self = Self(2i32);
}
impl ::core::marker::Copy for SceneAnalysisRecommendation {}
impl ::core::clone::Clone for SceneAnalysisRecommendation {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SceneAnalyzedEventArgs = *mut ::core::ffi::c_void;
pub type SpeechCue = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedMetadataKind(pub i32);
impl TimedMetadataKind {
    pub const Caption: Self = Self(0i32);
    pub const Chapter: Self = Self(1i32);
    pub const Custom: Self = Self(2i32);
    pub const Data: Self = Self(3i32);
    pub const Description: Self = Self(4i32);
    pub const Subtitle: Self = Self(5i32);
    pub const ImageSubtitle: Self = Self(6i32);
    pub const Speech: Self = Self(7i32);
}
impl ::core::marker::Copy for TimedMetadataKind {}
impl ::core::clone::Clone for TimedMetadataKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TimedMetadataStreamDescriptor = *mut ::core::ffi::c_void;
pub type TimedMetadataTrack = *mut ::core::ffi::c_void;
pub type TimedMetadataTrackError = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedMetadataTrackErrorCode(pub i32);
impl TimedMetadataTrackErrorCode {
    pub const None: Self = Self(0i32);
    pub const DataFormatError: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const InternalError: Self = Self(3i32);
}
impl ::core::marker::Copy for TimedMetadataTrackErrorCode {}
impl ::core::clone::Clone for TimedMetadataTrackErrorCode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TimedMetadataTrackFailedEventArgs = *mut ::core::ffi::c_void;
pub type TimedTextBouten = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedTextBoutenPosition(pub i32);
impl TimedTextBoutenPosition {
    pub const Before: Self = Self(0i32);
    pub const After: Self = Self(1i32);
    pub const Outside: Self = Self(2i32);
}
impl ::core::marker::Copy for TimedTextBoutenPosition {}
impl ::core::clone::Clone for TimedTextBoutenPosition {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedTextBoutenType(pub i32);
impl TimedTextBoutenType {
    pub const None: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
    pub const FilledCircle: Self = Self(2i32);
    pub const OpenCircle: Self = Self(3i32);
    pub const FilledDot: Self = Self(4i32);
    pub const OpenDot: Self = Self(5i32);
    pub const FilledSesame: Self = Self(6i32);
    pub const OpenSesame: Self = Self(7i32);
}
impl ::core::marker::Copy for TimedTextBoutenType {}
impl ::core::clone::Clone for TimedTextBoutenType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TimedTextCue = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedTextDisplayAlignment(pub i32);
impl TimedTextDisplayAlignment {
    pub const Before: Self = Self(0i32);
    pub const After: Self = Self(1i32);
    pub const Center: Self = Self(2i32);
}
impl ::core::marker::Copy for TimedTextDisplayAlignment {}
impl ::core::clone::Clone for TimedTextDisplayAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Media_Core\"`*"]
pub struct TimedTextDouble {
    pub Value: f64,
    pub Unit: TimedTextUnit,
}
impl ::core::marker::Copy for TimedTextDouble {}
impl ::core::clone::Clone for TimedTextDouble {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedTextFlowDirection(pub i32);
impl TimedTextFlowDirection {
    pub const LeftToRight: Self = Self(0i32);
    pub const RightToLeft: Self = Self(1i32);
}
impl ::core::marker::Copy for TimedTextFlowDirection {}
impl ::core::clone::Clone for TimedTextFlowDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedTextFontStyle(pub i32);
impl TimedTextFontStyle {
    pub const Normal: Self = Self(0i32);
    pub const Oblique: Self = Self(1i32);
    pub const Italic: Self = Self(2i32);
}
impl ::core::marker::Copy for TimedTextFontStyle {}
impl ::core::clone::Clone for TimedTextFontStyle {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TimedTextLine = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedTextLineAlignment(pub i32);
impl TimedTextLineAlignment {
    pub const Start: Self = Self(0i32);
    pub const End: Self = Self(1i32);
    pub const Center: Self = Self(2i32);
}
impl ::core::marker::Copy for TimedTextLineAlignment {}
impl ::core::clone::Clone for TimedTextLineAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Media_Core\"`*"]
pub struct TimedTextPadding {
    pub Before: f64,
    pub After: f64,
    pub Start: f64,
    pub End: f64,
    pub Unit: TimedTextUnit,
}
impl ::core::marker::Copy for TimedTextPadding {}
impl ::core::clone::Clone for TimedTextPadding {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Media_Core\"`*"]
pub struct TimedTextPoint {
    pub X: f64,
    pub Y: f64,
    pub Unit: TimedTextUnit,
}
impl ::core::marker::Copy for TimedTextPoint {}
impl ::core::clone::Clone for TimedTextPoint {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TimedTextRegion = *mut ::core::ffi::c_void;
pub type TimedTextRuby = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedTextRubyAlign(pub i32);
impl TimedTextRubyAlign {
    pub const Center: Self = Self(0i32);
    pub const Start: Self = Self(1i32);
    pub const End: Self = Self(2i32);
    pub const SpaceAround: Self = Self(3i32);
    pub const SpaceBetween: Self = Self(4i32);
    pub const WithBase: Self = Self(5i32);
}
impl ::core::marker::Copy for TimedTextRubyAlign {}
impl ::core::clone::Clone for TimedTextRubyAlign {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedTextRubyPosition(pub i32);
impl TimedTextRubyPosition {
    pub const Before: Self = Self(0i32);
    pub const After: Self = Self(1i32);
    pub const Outside: Self = Self(2i32);
}
impl ::core::marker::Copy for TimedTextRubyPosition {}
impl ::core::clone::Clone for TimedTextRubyPosition {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedTextRubyReserve(pub i32);
impl TimedTextRubyReserve {
    pub const None: Self = Self(0i32);
    pub const Before: Self = Self(1i32);
    pub const After: Self = Self(2i32);
    pub const Both: Self = Self(3i32);
    pub const Outside: Self = Self(4i32);
}
impl ::core::marker::Copy for TimedTextRubyReserve {}
impl ::core::clone::Clone for TimedTextRubyReserve {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedTextScrollMode(pub i32);
impl TimedTextScrollMode {
    pub const Popon: Self = Self(0i32);
    pub const Rollup: Self = Self(1i32);
}
impl ::core::marker::Copy for TimedTextScrollMode {}
impl ::core::clone::Clone for TimedTextScrollMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Media_Core\"`*"]
pub struct TimedTextSize {
    pub Height: f64,
    pub Width: f64,
    pub Unit: TimedTextUnit,
}
impl ::core::marker::Copy for TimedTextSize {}
impl ::core::clone::Clone for TimedTextSize {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TimedTextSource = *mut ::core::ffi::c_void;
pub type TimedTextSourceResolveResultEventArgs = *mut ::core::ffi::c_void;
pub type TimedTextStyle = *mut ::core::ffi::c_void;
pub type TimedTextSubformat = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedTextUnit(pub i32);
impl TimedTextUnit {
    pub const Pixels: Self = Self(0i32);
    pub const Percentage: Self = Self(1i32);
}
impl ::core::marker::Copy for TimedTextUnit {}
impl ::core::clone::Clone for TimedTextUnit {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedTextWeight(pub i32);
impl TimedTextWeight {
    pub const Normal: Self = Self(400i32);
    pub const Bold: Self = Self(700i32);
}
impl ::core::marker::Copy for TimedTextWeight {}
impl ::core::clone::Clone for TimedTextWeight {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedTextWrapping(pub i32);
impl TimedTextWrapping {
    pub const NoWrap: Self = Self(0i32);
    pub const Wrap: Self = Self(1i32);
}
impl ::core::marker::Copy for TimedTextWrapping {}
impl ::core::clone::Clone for TimedTextWrapping {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedTextWritingMode(pub i32);
impl TimedTextWritingMode {
    pub const LeftRightTopBottom: Self = Self(0i32);
    pub const RightLeftTopBottom: Self = Self(1i32);
    pub const TopBottomRightLeft: Self = Self(2i32);
    pub const TopBottomLeftRight: Self = Self(3i32);
    pub const LeftRight: Self = Self(4i32);
    pub const RightLeft: Self = Self(5i32);
    pub const TopBottom: Self = Self(6i32);
}
impl ::core::marker::Copy for TimedTextWritingMode {}
impl ::core::clone::Clone for TimedTextWritingMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VideoStabilizationEffect = *mut ::core::ffi::c_void;
pub type VideoStabilizationEffectDefinition = *mut ::core::ffi::c_void;
pub type VideoStabilizationEffectEnabledChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct VideoStabilizationEffectEnabledChangedReason(pub i32);
impl VideoStabilizationEffectEnabledChangedReason {
    pub const Programmatic: Self = Self(0i32);
    pub const PixelRateTooHigh: Self = Self(1i32);
    pub const RunningSlowly: Self = Self(2i32);
}
impl ::core::marker::Copy for VideoStabilizationEffectEnabledChangedReason {}
impl ::core::clone::Clone for VideoStabilizationEffectEnabledChangedReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VideoStreamDescriptor = *mut ::core::ffi::c_void;
pub type VideoTrack = *mut ::core::ffi::c_void;
pub type VideoTrackOpenFailedEventArgs = *mut ::core::ffi::c_void;
pub type VideoTrackSupportInfo = *mut ::core::ffi::c_void;
