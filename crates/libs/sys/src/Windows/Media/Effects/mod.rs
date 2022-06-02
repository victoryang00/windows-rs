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
#[repr(C)]
pub struct IAudioEffect {
    pub base__: ::windows_sys::core::IInspectable,
    pub AudioEffectType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioEffectType) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IAudioEffectDefinitionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithProperties: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING, props: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithProperties: usize,
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
#[repr(C)]
pub struct IProcessAudioFrameContext {
    pub base__: ::windows_sys::core::IInspectable,
    pub InputFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OutputFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProcessVideoFrameContext {
    pub base__: ::windows_sys::core::IInspectable,
    pub InputFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OutputFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISlowMotionEffectDefinition {
    pub base__: ::windows_sys::core::IInspectable,
    pub TimeStretchRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetTimeStretchRate: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IVideoCompositorDefinition {
    pub base__: ::windows_sys::core::IInspectable,
    pub ActivatableClassId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
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
#[repr(C)]
pub struct IVideoEffectDefinition {
    pub base__: ::windows_sys::core::IInspectable,
    pub ActivatableClassId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
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
#[repr(C)]
pub struct IVideoTransformEffectDefinition2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SphericalProjection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
