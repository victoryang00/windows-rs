#[repr(C)]
pub struct IInstalledVoicesStatic {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AllVoices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllVoices: usize,
    pub DefaultVoice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInstalledVoicesStatic {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2102554316, data2: 30003, data3: 19519, data4: [133, 190, 136, 140, 43, 174, 235, 220] };
}
#[repr(C)]
pub struct IInstalledVoicesStatic2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TrySetDefaultVoiceAsync: unsafe extern "system" fn(this: *mut *mut Self, voice: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySetDefaultVoiceAsync: usize,
}
impl ::windows_sys::core::Interface for IInstalledVoicesStatic2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1680170798, data2: 13709, data3: 16472, data4: [190, 154, 253, 63, 203, 66, 53, 48] };
}
#[repr(C)]
pub struct ISpeechSynthesisStream {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Markers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Markers: usize,
}
impl ::windows_sys::core::Interface for ISpeechSynthesisStream {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2212785811, data2: 9292, data3: 17954, data4: [186, 11, 98, 41, 196, 208, 214, 93] };
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
impl ::windows_sys::core::Interface for ISpeechSynthesizer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3466558582, data2: 38900, data3: 19693, data4: [173, 104, 213, 28, 69, 142, 69, 198] };
}
#[repr(C)]
pub struct ISpeechSynthesizer2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Options: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpeechSynthesizer2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2814766258, data2: 17209, data3: 19818, data4: [187, 248, 199, 164, 241, 84, 76, 46] };
}
#[repr(C)]
pub struct ISpeechSynthesizerOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub IncludeWordBoundaryMetadata: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIncludeWordBoundaryMetadata: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IncludeSentenceBoundaryMetadata: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIncludeSentenceBoundaryMetadata: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpeechSynthesizerOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2699180145, data2: 52285, data3: 17353, data4: [145, 177, 238, 24, 83, 36, 216, 61] };
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
impl ::windows_sys::core::Interface for ISpeechSynthesizerOptions2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 482276878, data2: 4508, data3: 19437, data4: [177, 24, 210, 80, 195, 162, 87, 147] };
}
#[repr(C)]
pub struct ISpeechSynthesizerOptions3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppendedSilence: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpeechAppendedSilence) -> ::windows_sys::core::HRESULT,
    pub SetAppendedSilence: unsafe extern "system" fn(this: *mut *mut Self, value: SpeechAppendedSilence) -> ::windows_sys::core::HRESULT,
    pub PunctuationSilence: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpeechPunctuationSilence) -> ::windows_sys::core::HRESULT,
    pub SetPunctuationSilence: unsafe extern "system" fn(this: *mut *mut Self, value: SpeechPunctuationSilence) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpeechSynthesizerOptions3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1075763319, data2: 36908, data3: 18452, data4: [165, 130, 165, 208, 192, 118, 159, 168] };
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
impl ::windows_sys::core::Interface for IVoiceInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2972178084, data2: 4753, data3: 17924, data4: [170, 156, 131, 19, 64, 131, 53, 44] };
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
