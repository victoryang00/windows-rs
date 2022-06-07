pub type AudioCaptureEffectsManager = *mut ::core::ffi::c_void;
pub type AudioEffect = *mut ::core::ffi::c_void;
pub type AudioEffectDefinition = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Effects\"`*"]
#[repr(transparent)]
pub struct AudioEffectType(pub i32);
impl AudioEffectType {
    pub const Other: Self = Self(0i32);
    pub const AcousticEchoCancellation: Self = Self(1i32);
    pub const NoiseSuppression: Self = Self(2i32);
    pub const AutomaticGainControl: Self = Self(3i32);
    pub const BeamForming: Self = Self(4i32);
    pub const ConstantToneRemoval: Self = Self(5i32);
    pub const Equalizer: Self = Self(6i32);
    pub const LoudnessEqualizer: Self = Self(7i32);
    pub const BassBoost: Self = Self(8i32);
    pub const VirtualSurround: Self = Self(9i32);
    pub const VirtualHeadphones: Self = Self(10i32);
    pub const SpeakerFill: Self = Self(11i32);
    pub const RoomCorrection: Self = Self(12i32);
    pub const BassManagement: Self = Self(13i32);
    pub const EnvironmentalEffects: Self = Self(14i32);
    pub const SpeakerProtection: Self = Self(15i32);
    pub const SpeakerCompensation: Self = Self(16i32);
    pub const DynamicRangeCompression: Self = Self(17i32);
    pub const FarFieldBeamForming: Self = Self(18i32);
    pub const DeepNoiseSuppression: Self = Self(19i32);
}
impl ::core::marker::Copy for AudioEffectType {}
impl ::core::clone::Clone for AudioEffectType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AudioRenderEffectsManager = *mut ::core::ffi::c_void;
pub type CompositeVideoFrameContext = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAudioCaptureEffectsManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AudioCaptureEffectsChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioCaptureEffectsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioCaptureEffectsChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioCaptureEffectsChanged: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAudioCaptureEffects: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAudioCaptureEffects: usize,
}
impl ::windows_sys::core::Interface for IAudioCaptureEffectsManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2407907953, data2: 909, data3: 17299, data4: [130, 152, 84, 1, 16, 96, 142, 239] };
}
#[repr(C)]
pub struct IAudioEffect {
    pub base__: ::windows_sys::core::IInspectable,
    pub AudioEffectType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioEffectType) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 883620433, data2: 37383, data3: 16469, data4: [190, 147, 110, 87, 52, 168, 106, 228] };
}
#[repr(C)]
pub struct IAudioEffectDefinition {
    pub base__: ::windows_sys::core::IInspectable,
    pub ActivatableClassId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
impl ::windows_sys::core::Interface for IAudioEffectDefinition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3839359348, data2: 32128, data3: 20339, data4: [144, 137, 227, 28, 157, 185, 194, 148] };
}
#[repr(C)]
pub struct IAudioEffectDefinitionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithProperties: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING, props: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithProperties: usize,
}
impl ::windows_sys::core::Interface for IAudioEffectDefinitionFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2384307782, data2: 59141, data3: 17901, data4: [138, 43, 252, 78, 79, 64, 90, 151] };
}
#[repr(C)]
pub struct IAudioEffectsManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Render")]
    pub CreateAudioRenderEffectsManager: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, category: super::Render::AudioRenderCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    CreateAudioRenderEffectsManager: usize,
    #[cfg(feature = "Media_Render")]
    pub CreateAudioRenderEffectsManagerWithMode: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, category: super::Render::AudioRenderCategory, mode: super::AudioProcessing, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    CreateAudioRenderEffectsManagerWithMode: usize,
    #[cfg(feature = "Media_Capture")]
    pub CreateAudioCaptureEffectsManager: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, category: super::Capture::MediaCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    CreateAudioCaptureEffectsManager: usize,
    #[cfg(feature = "Media_Capture")]
    pub CreateAudioCaptureEffectsManagerWithMode: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, category: super::Capture::MediaCategory, mode: super::AudioProcessing, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    CreateAudioCaptureEffectsManagerWithMode: usize,
}
impl ::windows_sys::core::Interface for IAudioEffectsManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1715497988, data2: 34554, data3: 18380, data4: [163, 21, 244, 137, 216, 195, 254, 16] };
}
#[repr(C)]
pub struct IAudioRenderEffectsManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AudioRenderEffectsChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioRenderEffectsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioRenderEffectsChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioRenderEffectsChanged: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAudioRenderEffects: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAudioRenderEffects: usize,
}
impl ::windows_sys::core::Interface for IAudioRenderEffectsManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1305053542, data2: 34641, data3: 17074, data4: [191, 203, 57, 202, 120, 100, 189, 71] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAudioRenderEffectsManager2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub EffectsProviderThumbnail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    EffectsProviderThumbnail: usize,
    #[cfg(feature = "deprecated")]
    pub EffectsProviderSettingsLabel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    EffectsProviderSettingsLabel: usize,
    #[cfg(feature = "deprecated")]
    pub ShowSettingsUI: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShowSettingsUI: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAudioRenderEffectsManager2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2823081225, data2: 24268, data3: 17587, data4: [187, 78, 29, 176, 114, 135, 19, 156] };
}
#[repr(C)]
pub struct IBasicAudioEffect {
    pub base__: ::windows_sys::core::IInspectable,
    pub UseInputFrameForOutput: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub SupportedEncodingProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    SupportedEncodingProperties: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetEncodingProperties: unsafe extern "system" fn(this: *mut *mut Self, encodingproperties: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetEncodingProperties: usize,
    pub ProcessFrame: unsafe extern "system" fn(this: *mut *mut Self, context: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self, reason: MediaEffectClosedReason) -> ::windows_sys::core::HRESULT,
    pub DiscardQueuedFrames: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBasicAudioEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2349214803, data2: 27584, data3: 18616, data4: [169, 154, 75, 65, 85, 15, 19, 89] };
}
#[repr(C)]
pub struct IBasicVideoEffect {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SupportedMemoryTypes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaMemoryTypes) -> ::windows_sys::core::HRESULT,
    pub TimeIndependent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub SupportedEncodingProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    SupportedEncodingProperties: usize,
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
    pub SetEncodingProperties: unsafe extern "system" fn(this: *mut *mut Self, encodingproperties: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties")))]
    SetEncodingProperties: usize,
    pub ProcessFrame: unsafe extern "system" fn(this: *mut *mut Self, context: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self, reason: MediaEffectClosedReason) -> ::windows_sys::core::HRESULT,
    pub DiscardQueuedFrames: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IBasicVideoEffect {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2187511791, data2: 45920, data3: 16574, data4: [148, 155, 47, 244, 47, 243, 86, 147] };
}
#[repr(C)]
pub struct ICompositeVideoFrameContext {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11"))]
    pub SurfacesToOverlay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11")))]
    SurfacesToOverlay: usize,
    pub BackgroundFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OutputFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_Editing"))]
    pub GetOverlayForSurface: unsafe extern "system" fn(this: *mut *mut Self, surfacetooverlay: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_Editing")))]
    GetOverlayForSurface: usize,
}
impl ::windows_sys::core::Interface for ICompositeVideoFrameContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1815085643, data2: 62740, data3: 17016, data4: [165, 247, 185, 24, 128, 73, 209, 16] };
}
#[repr(C)]
pub struct IProcessAudioFrameContext {
    pub base__: ::windows_sys::core::IInspectable,
    pub InputFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OutputFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProcessAudioFrameContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1289300294, data2: 4642, data3: 18983, data4: [165, 134, 251, 62, 32, 39, 50, 85] };
}
#[repr(C)]
pub struct IProcessVideoFrameContext {
    pub base__: ::windows_sys::core::IInspectable,
    pub InputFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OutputFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IProcessVideoFrameContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 661589547, data2: 25697, data3: 16414, data4: [186, 120, 15, 218, 214, 17, 78, 236] };
}
#[repr(C)]
pub struct ISlowMotionEffectDefinition {
    pub base__: ::windows_sys::core::IInspectable,
    pub TimeStretchRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetTimeStretchRate: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISlowMotionEffectDefinition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 889535696, data2: 5996, data3: 18275, data4: [130, 196, 27, 2, 219, 227, 23, 55] };
}
#[repr(C)]
pub struct IVideoCompositor {
    pub base__: ::windows_sys::core::IInspectable,
    pub TimeIndependent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
    pub SetEncodingProperties: unsafe extern "system" fn(this: *mut *mut Self, backgroundproperties: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties")))]
    SetEncodingProperties: usize,
    pub CompositeFrame: unsafe extern "system" fn(this: *mut *mut Self, context: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self, reason: MediaEffectClosedReason) -> ::windows_sys::core::HRESULT,
    pub DiscardQueuedFrames: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVideoCompositor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2232464446, data2: 16908, data3: 16911, data4: [150, 199, 124, 152, 187, 161, 252, 85] };
}
#[repr(C)]
pub struct IVideoCompositorDefinition {
    pub base__: ::windows_sys::core::IInspectable,
    pub ActivatableClassId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
impl ::windows_sys::core::Interface for IVideoCompositorDefinition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2034677968, data2: 8208, data3: 19171, data4: [154, 178, 44, 239, 66, 237, 212, 210] };
}
#[repr(C)]
pub struct IVideoCompositorDefinitionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithProperties: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING, props: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithProperties: usize,
}
impl ::windows_sys::core::Interface for IVideoCompositorDefinitionFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1130822928, data2: 26808, data3: 19794, data4: [137, 182, 2, 169, 104, 204, 168, 153] };
}
#[repr(C)]
pub struct IVideoEffectDefinition {
    pub base__: ::windows_sys::core::IInspectable,
    pub ActivatableClassId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
impl ::windows_sys::core::Interface for IVideoEffectDefinition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 972262640, data2: 36111, data3: 20286, data4: [132, 252, 45, 70, 165, 41, 121, 67] };
}
#[repr(C)]
pub struct IVideoEffectDefinitionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithProperties: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING, props: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithProperties: usize,
}
impl ::windows_sys::core::Interface for IVideoEffectDefinitionFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2168691534, data2: 28211, data3: 17039, data4: [157, 33, 181, 170, 254, 247, 97, 124] };
}
#[repr(C)]
pub struct IVideoTransformEffectDefinition {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI")]
    pub PaddingColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    PaddingColor: usize,
    #[cfg(feature = "UI")]
    pub SetPaddingColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetPaddingColor: usize,
    #[cfg(feature = "Foundation")]
    pub OutputSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OutputSize: usize,
    #[cfg(feature = "Foundation")]
    pub SetOutputSize: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetOutputSize: usize,
    #[cfg(feature = "Foundation")]
    pub CropRectangle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CropRectangle: usize,
    #[cfg(feature = "Foundation")]
    pub SetCropRectangle: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCropRectangle: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub Rotation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::MediaProperties::MediaRotation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    Rotation: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetRotation: unsafe extern "system" fn(this: *mut *mut Self, value: super::MediaProperties::MediaRotation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetRotation: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub Mirror: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::MediaProperties::MediaMirroringOptions) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    Mirror: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetMirror: unsafe extern "system" fn(this: *mut *mut Self, value: super::MediaProperties::MediaMirroringOptions) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetMirror: usize,
    #[cfg(feature = "Media_Transcoding")]
    pub SetProcessingAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, value: super::Transcoding::MediaVideoProcessingAlgorithm) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Transcoding"))]
    SetProcessingAlgorithm: usize,
    #[cfg(feature = "Media_Transcoding")]
    pub ProcessingAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Transcoding::MediaVideoProcessingAlgorithm) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Transcoding"))]
    ProcessingAlgorithm: usize,
}
impl ::windows_sys::core::Interface for IVideoTransformEffectDefinition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2523183978, data2: 7846, data3: 19110, data4: [128, 116, 171, 232, 133, 30, 202, 226] };
}
#[repr(C)]
pub struct IVideoTransformEffectDefinition2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SphericalProjection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVideoTransformEffectDefinition2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4037544095, data2: 26312, data3: 18068, data4: [159, 217, 17, 54, 171, 247, 68, 74] };
}
#[repr(C)]
pub struct IVideoTransformSphericalProjection {
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
    #[cfg(feature = "Media_Playback")]
    pub ProjectionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Playback::SphericalVideoProjectionMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    ProjectionMode: usize,
    #[cfg(feature = "Media_Playback")]
    pub SetProjectionMode: unsafe extern "system" fn(this: *mut *mut Self, value: super::Playback::SphericalVideoProjectionMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    SetProjectionMode: usize,
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
}
impl ::windows_sys::core::Interface for IVideoTransformSphericalProjection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3477340656, data2: 39922, data3: 19513, data4: [159, 65, 224, 34, 81, 74, 132, 104] };
}
#[doc = "*Required features: `\"Media_Effects\"`*"]
#[repr(transparent)]
pub struct MediaEffectClosedReason(pub i32);
impl MediaEffectClosedReason {
    pub const Done: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const UnsupportedEncodingFormat: Self = Self(2i32);
    pub const EffectCurrentlyUnloaded: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaEffectClosedReason {}
impl ::core::clone::Clone for MediaEffectClosedReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Effects\"`*"]
#[repr(transparent)]
pub struct MediaMemoryTypes(pub i32);
impl MediaMemoryTypes {
    pub const Gpu: Self = Self(0i32);
    pub const Cpu: Self = Self(1i32);
    pub const GpuAndCpu: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaMemoryTypes {}
impl ::core::clone::Clone for MediaMemoryTypes {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ProcessAudioFrameContext = *mut ::core::ffi::c_void;
pub type ProcessVideoFrameContext = *mut ::core::ffi::c_void;
pub type SlowMotionEffectDefinition = *mut ::core::ffi::c_void;
pub type VideoCompositorDefinition = *mut ::core::ffi::c_void;
pub type VideoEffectDefinition = *mut ::core::ffi::c_void;
pub type VideoTransformEffectDefinition = *mut ::core::ffi::c_void;
pub type VideoTransformSphericalProjection = *mut ::core::ffi::c_void;
