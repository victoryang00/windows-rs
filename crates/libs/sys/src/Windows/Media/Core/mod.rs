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
impl ::windows_sys::core::Interface for IAudioStreamDescriptor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 506893028, data2: 16423, data3: 18503, data4: [167, 11, 223, 29, 154, 42, 123, 4] };
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
impl ::windows_sys::core::Interface for IAudioStreamDescriptor2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 778629622, data2: 42056, data3: 18811, data4: [136, 64, 133, 8, 38, 101, 172, 249] };
}
#[repr(C)]
pub struct IAudioStreamDescriptor3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Copy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioStreamDescriptor3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1294077345, data2: 36483, data3: 17647, data4: [137, 115, 47, 99, 233, 147, 243, 107] };
}
#[repr(C)]
pub struct IAudioStreamDescriptorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_MediaProperties")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, encodingproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IAudioStreamDescriptorFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1250348702, data2: 19633, data3: 17280, data4: [142, 12, 131, 80, 75, 127, 91, 243] };
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
impl ::windows_sys::core::Interface for IAudioTrack {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4063981175, data2: 16119, data3: 16606, data4: [185, 67, 6, 139, 19, 33, 112, 29] };
}
#[repr(C)]
pub struct IAudioTrackOpenFailedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioTrackOpenFailedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4007508409, data2: 47996, data3: 16658, data4: [191, 118, 147, 132, 103, 111, 130, 75] };
}
#[repr(C)]
pub struct IAudioTrackSupportInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub DecoderStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaDecoderStatus) -> ::windows_sys::core::HRESULT,
    pub Degradation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioDecoderDegradation) -> ::windows_sys::core::HRESULT,
    pub DegradationReason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioDecoderDegradationReason) -> ::windows_sys::core::HRESULT,
    pub MediaSourceStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaSourceStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioTrackSupportInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 395046903, data2: 52281, data3: 17574, data4: [185, 81, 74, 86, 83, 240, 115, 250] };
}
#[repr(C)]
pub struct IChapterCue {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChapterCue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1923710977, data2: 54154, data3: 19466, data4: [143, 166, 117, 205, 218, 244, 102, 76] };
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
impl ::windows_sys::core::Interface for ICodecInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1374199685, data2: 60055, data3: 18844, data4: [134, 172, 76, 229, 231, 63, 58, 66] };
}
#[repr(C)]
pub struct ICodecQuery {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut *mut Self, kind: CodecKind, category: CodecCategory, subtype: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
}
impl ::windows_sys::core::Interface for ICodecQuery {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 573216058, data2: 44897, data3: 19972, data4: [128, 138, 164, 99, 78, 47, 58, 196] };
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
impl ::windows_sys::core::Interface for ICodecSubtypesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2792015090, data2: 34955, data3: 16932, data4: [140, 246, 42, 141, 78, 176, 35, 130] };
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
impl ::windows_sys::core::Interface for IDataCue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2088724333, data2: 8124, data3: 20013, data4: [154, 135, 238, 56, 189, 29, 198, 55] };
}
#[repr(C)]
pub struct IDataCue2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
impl ::windows_sys::core::Interface for IDataCue2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3159759637, data2: 38386, data3: 18920, data4: [150, 241, 141, 213, 218, 198, 141, 147] };
}
#[repr(C)]
pub struct IFaceDetectedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ResultFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFaceDetectedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 428966950, data2: 50779, data3: 18106, data4: [133, 248, 19, 136, 5, 118, 201, 10] };
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
impl ::windows_sys::core::Interface for IFaceDetectionEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2920672210, data2: 1346, data3: 17065, data4: [188, 144, 242, 131, 162, 159, 70, 193] };
}
#[repr(C)]
pub struct IFaceDetectionEffectDefinition {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetDetectionMode: unsafe extern "system" fn(this: *mut *mut Self, value: FaceDetectionMode) -> ::windows_sys::core::HRESULT,
    pub DetectionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FaceDetectionMode) -> ::windows_sys::core::HRESULT,
    pub SetSynchronousDetectionEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SynchronousDetectionEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFaceDetectionEffectDefinition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1138532481, data2: 47176, data3: 20275, data4: [183, 2, 31, 210, 98, 79, 176, 22] };
}
#[repr(C)]
pub struct IFaceDetectionEffectFrame {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_FaceAnalysis"))]
    pub DetectedFaces: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_FaceAnalysis")))]
    DetectedFaces: usize,
}
impl ::windows_sys::core::Interface for IFaceDetectionEffectFrame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2326825363, data2: 24008, data3: 17531, data4: [162, 71, 82, 112, 189, 128, 46, 206] };
}
#[repr(C)]
pub struct IHighDynamicRangeControl {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHighDynamicRangeControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1441900462, data2: 55639, data3: 19913, data4: [157, 28, 133, 83, 168, 42, 125, 153] };
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
impl ::windows_sys::core::Interface for IHighDynamicRangeOutput {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 257392747, data2: 9531, data3: 16665, data4: [187, 64, 58, 144, 229, 19, 132, 247] };
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
impl ::windows_sys::core::Interface for IImageCue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1384284802, data2: 13947, data3: 17419, data4: [145, 22, 60, 132, 87, 13, 210, 112] };
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
impl ::windows_sys::core::Interface for IInitializeMediaStreamSourceRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 633095649, data2: 39688, data3: 19502, data4: [168, 85, 69, 66, 241, 167, 93, 235] };
}
#[repr(C)]
pub struct ILowLightFusionResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_Imaging")]
    pub Frame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    Frame: usize,
}
impl ::windows_sys::core::Interface for ILowLightFusionResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2028846645, data2: 10144, data3: 17120, data4: [156, 211, 115, 141, 32, 137, 222, 156] };
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
impl ::windows_sys::core::Interface for ILowLightFusionStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1392836973, data2: 49822, data3: 16610, data4: [135, 169, 158, 31, 210, 241, 146, 245] };
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
impl ::windows_sys::core::Interface for IMediaBinder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 729694378, data2: 56839, data3: 16975, data4: [131, 241, 241, 222, 70, 196, 250, 46] };
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
impl ::windows_sys::core::Interface for IMediaBindingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3055333978, data2: 7021, data3: 17968, data4: [168, 109, 47, 8, 55, 247, 18, 229] };
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
impl ::windows_sys::core::Interface for IMediaBindingEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 73714923, data2: 47962, data3: 18479, data4: [184, 186, 240, 40, 76, 105, 101, 103] };
}
#[repr(C)]
pub struct IMediaBindingEventArgs3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Networking_BackgroundTransfer")]
    pub SetDownloadOperation: unsafe extern "system" fn(this: *mut *mut Self, downloadoperation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_BackgroundTransfer"))]
    SetDownloadOperation: usize,
}
impl ::windows_sys::core::Interface for IMediaBindingEventArgs3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4176168798, data2: 6590, data3: 17660, data4: [165, 237, 122, 186, 49, 80, 55, 249] };
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
impl ::windows_sys::core::Interface for IMediaCue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3352387165, data2: 23004, data3: 17183, data4: [160, 238, 39, 116, 67, 35, 179, 109] };
}
#[repr(C)]
pub struct IMediaCueEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Cue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaCueEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3509536759, data2: 24484, data3: 20072, data4: [159, 229, 50, 22, 13, 206, 229, 126] };
}
#[repr(C)]
pub struct IMediaSource {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IMediaSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3888100761, data2: 41117, data3: 19489, data4: [188, 223, 32, 175, 79, 134, 179, 217] };
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
impl ::windows_sys::core::Interface for IMediaSource2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 783683656, data2: 25951, data3: 19511, data4: [184, 19, 180, 228, 93, 250, 10, 190] };
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
impl ::windows_sys::core::Interface for IMediaSource3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3047099803, data2: 19310, data3: 16877, data4: [187, 180, 124, 117, 9, 169, 148, 173] };
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
impl ::windows_sys::core::Interface for IMediaSource4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3182406999, data2: 36607, data3: 19555, data4: [133, 166, 132, 222, 10, 227, 228, 242] };
}
#[repr(C)]
pub struct IMediaSource5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Networking_BackgroundTransfer")]
    pub DownloadOperation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_BackgroundTransfer"))]
    DownloadOperation: usize,
}
impl ::windows_sys::core::Interface for IMediaSource5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 857350830, data2: 60718, data3: 18978, data4: [148, 200, 183, 67, 169, 43, 48, 34] };
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
impl ::windows_sys::core::Interface for IMediaSourceAppServiceConnection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1642195607, data2: 6422, data3: 18448, data4: [183, 244, 182, 66, 190, 130, 149, 150] };
}
#[repr(C)]
pub struct IMediaSourceAppServiceConnectionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_AppService")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, appserviceconnection: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IMediaSourceAppServiceConnectionFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1706627819, data2: 32953, data3: 17657, data4: [156, 30, 225, 32, 246, 217, 40, 56] };
}
#[repr(C)]
pub struct IMediaSourceError {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaSourceError {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1544194405, data2: 14277, data3: 20125, data4: [141, 33, 28, 222, 233, 12, 236, 198] };
}
#[repr(C)]
pub struct IMediaSourceOpenOperationCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaSourceOpenOperationCompletedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4234685675, data2: 57985, data3: 18300, data4: [168, 224, 26, 205, 101, 65, 20, 200] };
}
#[repr(C)]
pub struct IMediaSourceStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub OldState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaSourceState) -> ::windows_sys::core::HRESULT,
    pub NewState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaSourceState) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaSourceStateChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 170962818, data2: 36977, data3: 19372, data4: [188, 57, 202, 42, 147, 183, 23, 169] };
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
impl ::windows_sys::core::Interface for IMediaSourceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4152192932, data2: 18002, data3: 16654, data4: [177, 216, 233, 165, 226, 69, 164, 92] };
}
#[repr(C)]
pub struct IMediaSourceStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromMediaBinder: unsafe extern "system" fn(this: *mut *mut Self, binder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaSourceStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4007748004, data2: 32531, data3: 18582, data4: [184, 203, 223, 13, 229, 188, 185, 241] };
}
#[repr(C)]
pub struct IMediaSourceStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Capture_Frames")]
    pub CreateFromMediaFrameSource: unsafe extern "system" fn(this: *mut *mut Self, framesource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Capture_Frames"))]
    CreateFromMediaFrameSource: usize,
}
impl ::windows_sys::core::Interface for IMediaSourceStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1161441494, data2: 11242, data3: 16674, data4: [159, 115, 234, 206, 4, 82, 110, 53] };
}
#[repr(C)]
pub struct IMediaSourceStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Networking_BackgroundTransfer")]
    pub CreateFromDownloadOperation: unsafe extern "system" fn(this: *mut *mut Self, downloadoperation: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_BackgroundTransfer"))]
    CreateFromDownloadOperation: usize,
}
impl ::windows_sys::core::Interface for IMediaSourceStatics4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 672873468, data2: 58634, data3: 17448, data4: [165, 0, 156, 78, 217, 24, 211, 240] };
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
impl ::windows_sys::core::Interface for IMediaStreamDescriptor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2163306094, data2: 37623, data3: 17694, data4: [151, 210, 175, 216, 7, 66, 218, 112] };
}
#[repr(C)]
pub struct IMediaStreamDescriptor2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetLabel: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaStreamDescriptor2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1349714191, data2: 59570, data3: 16497, data4: [176, 11, 235, 243, 55, 167, 107, 88] };
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
impl ::windows_sys::core::Interface for IMediaStreamSample {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1552791079, data2: 19328, data3: 17249, data4: [152, 55, 108, 183, 72, 26, 217, 214] };
}
#[repr(C)]
pub struct IMediaStreamSample2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3D11Surface: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3D11Surface: usize,
}
impl ::windows_sys::core::Interface for IMediaStreamSample2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1158121105, data2: 64744, data3: 18246, data4: [161, 200, 16, 194, 93, 61, 124, 211] };
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
impl ::windows_sys::core::Interface for IMediaStreamSampleProtectionProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1320714898, data2: 60639, data3: 18750, data4: [132, 29, 221, 74, 221, 124, 172, 162] };
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
impl ::windows_sys::core::Interface for IMediaStreamSampleStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3755942287, data2: 42703, data3: 17785, data4: [190, 65, 115, 221, 148, 26, 217, 114] };
}
#[repr(C)]
pub struct IMediaStreamSampleStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub CreateFromDirect3D11Surface: unsafe extern "system" fn(this: *mut *mut Self, surface: *mut ::core::ffi::c_void, timestamp: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11")))]
    CreateFromDirect3D11Surface: usize,
}
impl ::windows_sys::core::Interface for IMediaStreamSampleStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2667484449, data2: 27974, data3: 18764, data4: [162, 248, 214, 98, 146, 46, 45, 215] };
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
impl ::windows_sys::core::Interface for IMediaStreamSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 923981123, data2: 17899, data3: 16696, data4: [170, 98, 192, 30, 38, 243, 132, 63] };
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
impl ::windows_sys::core::Interface for IMediaStreamSource2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3965046957, data2: 11882, data3: 20340, data4: [173, 187, 181, 98, 209, 83, 56, 73] };
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
impl ::windows_sys::core::Interface for IMediaStreamSource3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1781147462, data2: 15837, data3: 19935, data4: [161, 33, 148, 4, 94, 207, 148, 64] };
}
#[repr(C)]
pub struct IMediaStreamSource4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetIsLive: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsLive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaStreamSource4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 487390379, data2: 33549, data3: 16764, data4: [163, 169, 36, 84, 253, 100, 21, 199] };
}
#[repr(C)]
pub struct IMediaStreamSourceClosedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaStreamSourceClosedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3448536754, data2: 18454, data3: 20004, data4: [136, 240, 73, 30, 247, 56, 100, 6] };
}
#[repr(C)]
pub struct IMediaStreamSourceClosedRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaStreamSourceClosedReason) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaStreamSourceClosedRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2424045801, data2: 6307, data3: 18769, data4: [136, 122, 44, 30, 235, 213, 198, 158] };
}
#[repr(C)]
pub struct IMediaStreamSourceFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromDescriptor: unsafe extern "system" fn(this: *mut *mut Self, descriptor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromDescriptors: unsafe extern "system" fn(this: *mut *mut Self, descriptor: *mut ::core::ffi::c_void, descriptor2: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaStreamSourceFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4017610969, data2: 53592, data3: 19322, data4: [134, 63, 32, 51, 66, 251, 253, 65] };
}
#[repr(C)]
pub struct IMediaStreamSourceSampleRenderedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SampleLag: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SampleLag: usize,
}
impl ::windows_sys::core::Interface for IMediaStreamSourceSampleRenderedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2640935685, data2: 54514, data3: 19578, data4: [157, 254, 141, 108, 208, 179, 238, 132] };
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
impl ::windows_sys::core::Interface for IMediaStreamSourceSampleRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1303593385, data2: 13569, data3: 19867, data4: [131, 249, 143, 35, 92, 130, 37, 50] };
}
#[repr(C)]
pub struct IMediaStreamSourceSampleRequestDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaStreamSourceSampleRequestDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2023083010, data2: 63874, data3: 17352, data4: [157, 22, 198, 45, 153, 147, 25, 190] };
}
#[repr(C)]
pub struct IMediaStreamSourceSampleRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaStreamSourceSampleRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 284801950, data2: 29125, data3: 18735, data4: [132, 127, 13, 161, 243, 94, 129, 248] };
}
#[repr(C)]
pub struct IMediaStreamSourceStartingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaStreamSourceStartingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4094978290, data2: 49780, data3: 18752, data4: [165, 187, 40, 165, 114, 69, 47, 167] };
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
impl ::windows_sys::core::Interface for IMediaStreamSourceStartingRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 714118116, data2: 13764, data3: 19227, data4: [167, 145, 13, 153, 219, 86, 221, 29] };
}
#[repr(C)]
pub struct IMediaStreamSourceStartingRequestDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaStreamSourceStartingRequestDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1058231973, data2: 25408, data3: 19908, data4: [153, 16, 6, 142, 217, 245, 152, 248] };
}
#[repr(C)]
pub struct IMediaStreamSourceSwitchStreamsRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub OldStreamDescriptor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NewStreamDescriptor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaStreamSourceSwitchStreamsRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1102610574, data2: 14505, data3: 20163, data4: [155, 160, 182, 155, 133, 80, 30, 144] };
}
#[repr(C)]
pub struct IMediaStreamSourceSwitchStreamsRequestDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaStreamSourceSwitchStreamsRequestDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3202603061, data2: 42245, data3: 20378, data4: [185, 67, 43, 140, 177, 180, 187, 217] };
}
#[repr(C)]
pub struct IMediaStreamSourceSwitchStreamsRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaStreamSourceSwitchStreamsRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1109404530, data2: 28321, data3: 18039, data4: [152, 30, 53, 10, 13, 164, 18, 170] };
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
impl ::windows_sys::core::Interface for IMediaTrack {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 65141500, data2: 51505, data3: 18714, data4: [180, 107, 193, 14, 232, 194, 86, 183] };
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
impl ::windows_sys::core::Interface for IMseSourceBuffer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 203072483, data2: 57229, data3: 16505, data4: [163, 254, 104, 73, 24, 75, 78, 47] };
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
impl ::windows_sys::core::Interface for IMseSourceBufferList {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2516248807, data2: 43239, data3: 20159, data4: [137, 39, 20, 94, 148, 11, 165, 17] };
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
impl ::windows_sys::core::Interface for IMseStreamSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2964593037, data2: 756, data3: 18723, data4: [136, 221, 129, 188, 63, 54, 15, 250] };
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
impl ::windows_sys::core::Interface for IMseStreamSource2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1727364407, data2: 63975, data3: 16778, data4: [156, 222, 160, 32, 233, 86, 85, 43] };
}
#[repr(C)]
pub struct IMseStreamSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsContentTypeSupported: unsafe extern "system" fn(this: *mut *mut Self, contenttype: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMseStreamSourceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1180460957, data2: 54640, data3: 17358, data4: [186, 33, 11, 255, 95, 63, 189, 10] };
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
impl ::windows_sys::core::Interface for ISceneAnalysisEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3226182425, data2: 51777, data3: 18451, data4: [191, 253, 123, 8, 176, 237, 37, 87] };
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
impl ::windows_sys::core::Interface for ISceneAnalysisEffectFrame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3635482188, data2: 32729, data3: 17121, data4: [133, 235, 101, 114, 194, 151, 201, 135] };
}
#[repr(C)]
pub struct ISceneAnalysisEffectFrame2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AnalysisRecommendation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SceneAnalysisRecommendation) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISceneAnalysisEffectFrame2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 760097214, data2: 1567, data3: 18350, data4: [153, 21, 2, 82, 75, 95, 154, 95] };
}
#[repr(C)]
pub struct ISceneAnalyzedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ResultFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISceneAnalyzedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 342594952, data2: 10321, data3: 17892, data4: [173, 85, 68, 207, 141, 248, 219, 77] };
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
impl ::windows_sys::core::Interface for ISingleSelectMediaTrackList {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1998614303, data2: 49999, data3: 18767, data4: [128, 119, 43, 173, 159, 244, 236, 241] };
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
impl ::windows_sys::core::Interface for ISpeechCue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2934068444, data2: 5925, data3: 19373, data4: [128, 67, 169, 132, 153, 176, 23, 162] };
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
impl ::windows_sys::core::Interface for ITimedMetadataStreamDescriptor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 322123455, data2: 10602, data3: 17982, data4: [159, 249, 1, 205, 37, 105, 20, 8] };
}
#[repr(C)]
pub struct ITimedMetadataStreamDescriptorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_MediaProperties")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, encodingproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for ITimedMetadataStreamDescriptorFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3223838256, data2: 29538, data3: 20473, data4: [152, 177, 45, 253, 11, 141, 28, 174] };
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
impl ::windows_sys::core::Interface for ITimedMetadataTrack {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2657807774, data2: 63098, data3: 18857, data4: [179, 48, 207, 3, 176, 233, 207, 7] };
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
impl ::windows_sys::core::Interface for ITimedMetadataTrack2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 565491272, data2: 40861, data3: 16570, data4: [168, 243, 26, 146, 117, 58, 239, 11] };
}
#[repr(C)]
pub struct ITimedMetadataTrackError {
    pub base__: ::windows_sys::core::IInspectable,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimedMetadataTrackErrorCode) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITimedMetadataTrackError {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3010885909, data2: 16660, data3: 18457, data4: [185, 217, 221, 118, 8, 158, 114, 248] };
}
#[repr(C)]
pub struct ITimedMetadataTrackFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, language: ::windows_sys::core::HSTRING, kind: TimedMetadataKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITimedMetadataTrackFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2379576849, data2: 38835, data3: 19999, data4: [133, 44, 15, 72, 44, 129, 173, 38] };
}
#[repr(C)]
pub struct ITimedMetadataTrackFailedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITimedMetadataTrackFailedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2776615377, data2: 26505, data3: 19789, data4: [176, 127, 132, 180, 243, 26, 203, 112] };
}
#[repr(C)]
pub struct ITimedMetadataTrackProvider {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub TimedMetadataTracks: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TimedMetadataTracks: usize,
}
impl ::windows_sys::core::Interface for ITimedMetadataTrackProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 998187044, data2: 63310, data3: 19166, data4: [147, 197, 33, 157, 160, 91, 104, 86] };
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
impl ::windows_sys::core::Interface for ITimedTextBouten {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3641059203, data2: 21911, data3: 20626, data4: [130, 12, 143, 115, 142, 15, 119, 74] };
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
impl ::windows_sys::core::Interface for ITimedTextCue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1372036689, data2: 15238, data3: 18765, data4: [179, 89, 187, 46, 167, 172, 169, 169] };
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
impl ::windows_sys::core::Interface for ITimedTextLine {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2542632162, data2: 29448, data3: 19558, data4: [190, 80, 101, 119, 114, 137, 245, 223] };
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
impl ::windows_sys::core::Interface for ITimedTextRegion {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 516982815, data2: 35334, data3: 16930, data4: [159, 89, 178, 27, 244, 1, 36, 180] };
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
impl ::windows_sys::core::Interface for ITimedTextRuby {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 271801385, data2: 23356, data3: 22163, data4: [153, 89, 208, 90, 11, 210, 70, 40] };
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
impl ::windows_sys::core::Interface for ITimedTextSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3303906214, data2: 4127, data3: 16461, data4: [169, 73, 130, 243, 63, 205, 147, 183] };
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
impl ::windows_sys::core::Interface for ITimedTextSourceResolveResultEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1217428636, data2: 56536, data3: 19507, data4: [154, 211, 108, 220, 231, 177, 197, 102] };
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
impl ::windows_sys::core::Interface for ITimedTextSourceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2117146707, data2: 39610, data3: 19140, data4: [187, 152, 47, 177, 118, 195, 191, 221] };
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
impl ::windows_sys::core::Interface for ITimedTextSourceStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3060495874, data2: 37438, data3: 17402, data4: [150, 51, 88, 112, 117, 129, 45, 181] };
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
impl ::windows_sys::core::Interface for ITimedTextStyle {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 464664653, data2: 43045, data3: 16578, data4: [167, 245, 40, 30, 174, 223, 59, 85] };
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
impl ::windows_sys::core::Interface for ITimedTextStyle2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1700743469, data2: 24849, data3: 18311, data4: [137, 204, 104, 111, 236, 229, 126, 20] };
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
impl ::windows_sys::core::Interface for ITimedTextStyle3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4161009979, data2: 16025, data3: 22878, data4: [187, 183, 120, 162, 250, 19, 194, 112] };
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
impl ::windows_sys::core::Interface for ITimedTextSubformat {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3608367151, data2: 12897, data3: 18210, data4: [160, 194, 185, 55, 178, 57, 15, 20] };
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
impl ::windows_sys::core::Interface for IVideoStabilizationEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 134784592, data2: 38552, data3: 20055, data4: [135, 123, 189, 124, 178, 238, 15, 138] };
}
#[repr(C)]
pub struct IVideoStabilizationEffectEnabledChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VideoStabilizationEffectEnabledChangedReason) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVideoStabilizationEffectEnabledChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 410976040, data2: 26555, data3: 18195, data4: [185, 0, 65, 104, 218, 22, 69, 41] };
}
#[repr(C)]
pub struct IVideoStreamDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_MediaProperties")]
    pub EncodingProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    EncodingProperties: usize,
}
impl ::windows_sys::core::Interface for IVideoStreamDescriptor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 317590869, data2: 39979, data3: 17472, data4: [128, 87, 44, 122, 144, 240, 203, 236] };
}
#[repr(C)]
pub struct IVideoStreamDescriptor2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Copy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVideoStreamDescriptor2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2335206928, data2: 17726, data3: 16520, data4: [131, 45, 195, 111, 164, 249, 74, 243] };
}
#[repr(C)]
pub struct IVideoStreamDescriptorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_MediaProperties")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, encodingproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IVideoStreamDescriptorFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1229911761, data2: 47989, data3: 17362, data4: [158, 94, 123, 121, 163, 175, 206, 212] };
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
impl ::windows_sys::core::Interface for IVideoTrack {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2582886387, data2: 58008, data3: 17302, data4: [187, 106, 165, 27, 230, 162, 162, 10] };
}
#[repr(C)]
pub struct IVideoTrackOpenFailedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVideoTrackOpenFailedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1987699249, data2: 1273, data3: 19586, data4: [164, 238, 134, 2, 200, 187, 71, 84] };
}
#[repr(C)]
pub struct IVideoTrackSupportInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub DecoderStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaDecoderStatus) -> ::windows_sys::core::HRESULT,
    pub MediaSourceStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaSourceStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVideoTrackSupportInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1270166688, data2: 64607, data3: 17677, data4: [143, 240, 119, 141, 89, 4, 134, 222] };
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
