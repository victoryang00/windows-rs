#[repr(C)]
pub struct IMediaTranscoder {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetTrimStartTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTrimStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub TrimStartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrimStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetTrimStopTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTrimStopTime: usize,
    #[cfg(feature = "Foundation")]
    pub TrimStopTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrimStopTime: usize,
    pub SetAlwaysReencode: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AlwaysReencode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHardwareAccelerationEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub HardwareAccelerationEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AddAudioEffect: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AddAudioEffectWithSettings: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING, effectrequired: bool, configuration: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddAudioEffectWithSettings: usize,
    pub AddVideoEffect: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AddVideoEffectWithSettings: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING, effectrequired: bool, configuration: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddVideoEffectWithSettings: usize,
    pub ClearEffects: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub PrepareFileTranscodeAsync: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage")))]
    PrepareFileTranscodeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub PrepareStreamTranscodeAsync: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams")))]
    PrepareStreamTranscodeAsync: usize,
}
impl ::windows_sys::core::Interface for IMediaTranscoder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 420256210, data2: 41130, data3: 19764, data4: [134, 188, 238, 209, 177, 44, 47, 91] };
}
#[repr(C)]
pub struct IMediaTranscoder2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Media_Core", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub PrepareMediaStreamSourceTranscodeAsync: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Core", feature = "Media_MediaProperties", feature = "Storage_Streams")))]
    PrepareMediaStreamSourceTranscodeAsync: usize,
    pub SetVideoProcessingAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, value: MediaVideoProcessingAlgorithm) -> ::windows_sys::core::HRESULT,
    pub VideoProcessingAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaVideoProcessingAlgorithm) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaTranscoder2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1079188852, data2: 13792, data3: 20228, data4: [133, 116, 202, 139, 196, 229, 160, 130] };
}
#[repr(C)]
pub struct IPrepareTranscodeResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanTranscode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub FailureReason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TranscodeFailureReason) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TranscodeAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TranscodeAsync: usize,
}
impl ::windows_sys::core::Interface for IPrepareTranscodeResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 99769806, data2: 39247, data3: 18996, data4: [157, 104, 151, 204, 206, 23, 48, 214] };
}
pub type MediaTranscoder = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Transcoding\"`*"]
#[repr(transparent)]
pub struct MediaVideoProcessingAlgorithm(pub i32);
impl MediaVideoProcessingAlgorithm {
    pub const Default: Self = Self(0i32);
    pub const MrfCrf444: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaVideoProcessingAlgorithm {}
impl ::core::clone::Clone for MediaVideoProcessingAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PrepareTranscodeResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Transcoding\"`*"]
#[repr(transparent)]
pub struct TranscodeFailureReason(pub i32);
impl TranscodeFailureReason {
    pub const None: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const InvalidProfile: Self = Self(2i32);
    pub const CodecNotFound: Self = Self(3i32);
}
impl ::core::marker::Copy for TranscodeFailureReason {}
impl ::core::clone::Clone for TranscodeFailureReason {
    fn clone(&self) -> Self {
        *self
    }
}
