#[repr(C)]
pub struct IInstalledVoicesStatic {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AllVoices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllVoices: usize,
    pub DefaultVoice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInstalledVoicesStatic2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TrySetDefaultVoiceAsync: unsafe extern "system" fn(this: *mut *mut Self, voice: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySetDefaultVoiceAsync: usize,
}
#[repr(C)]
pub struct ISpeechSynthesisStream {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Markers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Markers: usize,
}
#[repr(C)]
pub struct ISpeechSynthesizer {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SynthesizeTextToStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SynthesizeTextToStreamAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SynthesizeSsmlToStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, ssml: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SynthesizeSsmlToStreamAsync: usize,
    pub SetVoice: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Voice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpeechSynthesizer2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Options: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpeechSynthesizerOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub IncludeWordBoundaryMetadata: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIncludeWordBoundaryMetadata: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IncludeSentenceBoundaryMetadata: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIncludeSentenceBoundaryMetadata: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpeechSynthesizerOptions2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AudioVolume: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetAudioVolume: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub SpeakingRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetSpeakingRate: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub AudioPitch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetAudioPitch: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpeechSynthesizerOptions3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppendedSilence: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpeechAppendedSilence) -> ::windows_sys::core::HRESULT,
    pub SetAppendedSilence: unsafe extern "system" fn(this: *mut *mut Self, value: SpeechAppendedSilence) -> ::windows_sys::core::HRESULT,
    pub PunctuationSilence: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpeechPunctuationSilence) -> ::windows_sys::core::HRESULT,
    pub SetPunctuationSilence: unsafe extern "system" fn(this: *mut *mut Self, value: SpeechPunctuationSilence) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVoiceInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Gender: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VoiceGender) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"Media_SpeechSynthesis\"`*"]
#[repr(transparent)]
pub struct SpeechAppendedSilence(pub i32);
impl SpeechAppendedSilence {
    pub const Default: Self = Self(0i32);
    pub const Min: Self = Self(1i32);
}
impl ::core::marker::Copy for SpeechAppendedSilence {}
impl ::core::clone::Clone for SpeechAppendedSilence {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_SpeechSynthesis\"`*"]
#[repr(transparent)]
pub struct SpeechPunctuationSilence(pub i32);
impl SpeechPunctuationSilence {
    pub const Default: Self = Self(0i32);
    pub const Min: Self = Self(1i32);
}
impl ::core::marker::Copy for SpeechPunctuationSilence {}
impl ::core::clone::Clone for SpeechPunctuationSilence {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SpeechSynthesisStream = *mut ::core::ffi::c_void;
pub type SpeechSynthesizer = *mut ::core::ffi::c_void;
pub type SpeechSynthesizerOptions = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_SpeechSynthesis\"`*"]
#[repr(transparent)]
pub struct VoiceGender(pub i32);
impl VoiceGender {
    pub const Male: Self = Self(0i32);
    pub const Female: Self = Self(1i32);
}
impl ::core::marker::Copy for VoiceGender {}
impl ::core::clone::Clone for VoiceGender {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VoiceInformation = *mut ::core::ffi::c_void;
