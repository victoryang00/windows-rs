pub type AudioDeviceInputNode = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioDeviceNodeCreationStatus(pub i32);
impl AudioDeviceNodeCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const DeviceNotAvailable: Self = Self(1i32);
    pub const FormatNotSupported: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
    pub const AccessDenied: Self = Self(4i32);
}
impl ::core::marker::Copy for AudioDeviceNodeCreationStatus {}
impl ::core::clone::Clone for AudioDeviceNodeCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AudioDeviceOutputNode = *mut ::core::ffi::c_void;
pub type AudioFileInputNode = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioFileNodeCreationStatus(pub i32);
impl AudioFileNodeCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const FileNotFound: Self = Self(1i32);
    pub const InvalidFileType: Self = Self(2i32);
    pub const FormatNotSupported: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
}
impl ::core::marker::Copy for AudioFileNodeCreationStatus {}
impl ::core::clone::Clone for AudioFileNodeCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AudioFileOutputNode = *mut ::core::ffi::c_void;
pub type AudioFrameCompletedEventArgs = *mut ::core::ffi::c_void;
pub type AudioFrameInputNode = *mut ::core::ffi::c_void;
pub type AudioFrameOutputNode = *mut ::core::ffi::c_void;
pub type AudioGraph = *mut ::core::ffi::c_void;
pub type AudioGraphBatchUpdater = *mut ::core::ffi::c_void;
pub type AudioGraphConnection = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioGraphCreationStatus(pub i32);
impl AudioGraphCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const DeviceNotAvailable: Self = Self(1i32);
    pub const FormatNotSupported: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for AudioGraphCreationStatus {}
impl ::core::clone::Clone for AudioGraphCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AudioGraphSettings = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioGraphUnrecoverableError(pub i32);
impl AudioGraphUnrecoverableError {
    pub const None: Self = Self(0i32);
    pub const AudioDeviceLost: Self = Self(1i32);
    pub const AudioSessionDisconnected: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for AudioGraphUnrecoverableError {}
impl ::core::clone::Clone for AudioGraphUnrecoverableError {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AudioGraphUnrecoverableErrorOccurredEventArgs = *mut ::core::ffi::c_void;
pub type AudioNodeEmitter = *mut ::core::ffi::c_void;
pub type AudioNodeEmitterConeProperties = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioNodeEmitterDecayKind(pub i32);
impl AudioNodeEmitterDecayKind {
    pub const Natural: Self = Self(0i32);
    pub const Custom: Self = Self(1i32);
}
impl ::core::marker::Copy for AudioNodeEmitterDecayKind {}
impl ::core::clone::Clone for AudioNodeEmitterDecayKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AudioNodeEmitterDecayModel = *mut ::core::ffi::c_void;
pub type AudioNodeEmitterNaturalDecayModelProperties = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioNodeEmitterSettings(pub u32);
impl AudioNodeEmitterSettings {
    pub const None: Self = Self(0u32);
    pub const DisableDoppler: Self = Self(1u32);
}
impl ::core::marker::Copy for AudioNodeEmitterSettings {}
impl ::core::clone::Clone for AudioNodeEmitterSettings {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AudioNodeEmitterShape = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioNodeEmitterShapeKind(pub i32);
impl AudioNodeEmitterShapeKind {
    pub const Omnidirectional: Self = Self(0i32);
    pub const Cone: Self = Self(1i32);
}
impl ::core::marker::Copy for AudioNodeEmitterShapeKind {}
impl ::core::clone::Clone for AudioNodeEmitterShapeKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AudioNodeListener = *mut ::core::ffi::c_void;
pub type AudioPlaybackConnection = *mut ::core::ffi::c_void;
pub type AudioPlaybackConnectionOpenResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioPlaybackConnectionOpenResultStatus(pub i32);
impl AudioPlaybackConnectionOpenResultStatus {
    pub const Success: Self = Self(0i32);
    pub const RequestTimedOut: Self = Self(1i32);
    pub const DeniedBySystem: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for AudioPlaybackConnectionOpenResultStatus {}
impl ::core::clone::Clone for AudioPlaybackConnectionOpenResultStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioPlaybackConnectionState(pub i32);
impl AudioPlaybackConnectionState {
    pub const Closed: Self = Self(0i32);
    pub const Opened: Self = Self(1i32);
}
impl ::core::marker::Copy for AudioPlaybackConnectionState {}
impl ::core::clone::Clone for AudioPlaybackConnectionState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AudioStateMonitor = *mut ::core::ffi::c_void;
pub type AudioSubmixNode = *mut ::core::ffi::c_void;
pub type CreateAudioDeviceInputNodeResult = *mut ::core::ffi::c_void;
pub type CreateAudioDeviceOutputNodeResult = *mut ::core::ffi::c_void;
pub type CreateAudioFileInputNodeResult = *mut ::core::ffi::c_void;
pub type CreateAudioFileOutputNodeResult = *mut ::core::ffi::c_void;
pub type CreateAudioGraphResult = *mut ::core::ffi::c_void;
pub type CreateMediaSourceAudioInputNodeResult = *mut ::core::ffi::c_void;
pub type EchoEffectDefinition = *mut ::core::ffi::c_void;
pub type EqualizerBand = *mut ::core::ffi::c_void;
pub type EqualizerEffectDefinition = *mut ::core::ffi::c_void;
pub type FrameInputNodeQuantumStartedEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAudioDeviceInputNode {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Enumeration")]
    pub Device: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    Device: usize,
}
#[repr(C)]
pub struct IAudioDeviceOutputNode {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Enumeration")]
    pub Device: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    Device: usize,
}
#[repr(C)]
pub struct IAudioFileInputNode {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetPlaybackSpeedFactor: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub PlaybackSpeedFactor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub Seek: unsafe extern "system" fn(this: *mut *mut Self, position: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Seek: usize,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub EndTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetEndTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEndTime: usize,
    #[cfg(feature = "Foundation")]
    pub LoopCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoopCount: usize,
    #[cfg(feature = "Foundation")]
    pub SetLoopCount: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLoopCount: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Storage")]
    pub SourceFile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SourceFile: usize,
    #[cfg(feature = "Foundation")]
    pub FileCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FileCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFileCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFileCompleted: usize,
}
#[repr(C)]
pub struct IAudioFileOutputNode {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub FileEncodingProfile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    FileEncodingProfile: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Transcoding"))]
    pub FinalizeAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Transcoding")))]
    FinalizeAsync: usize,
}
#[repr(C)]
pub struct IAudioFrameCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Frame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioFrameInputNode {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetPlaybackSpeedFactor: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub PlaybackSpeedFactor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub AddFrame: unsafe extern "system" fn(this: *mut *mut Self, frame: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DiscardQueuedFrames: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub QueuedSampleCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AudioFrameCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioFrameCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioFrameCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioFrameCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub QuantumStarted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QuantumStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveQuantumStarted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveQuantumStarted: usize,
}
#[repr(C)]
pub struct IAudioFrameOutputNode {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioGraph {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFrameInputNode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub CreateFrameInputNodeWithFormat: unsafe extern "system" fn(this: *mut *mut Self, encodingproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    CreateFrameInputNodeWithFormat: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture"))]
    pub CreateDeviceInputNodeAsync: unsafe extern "system" fn(this: *mut *mut Self, category: super::Capture::MediaCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture")))]
    CreateDeviceInputNodeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub CreateDeviceInputNodeWithFormatAsync: unsafe extern "system" fn(this: *mut *mut Self, category: super::Capture::MediaCategory, encodingproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties")))]
    CreateDeviceInputNodeWithFormatAsync: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub CreateDeviceInputNodeWithFormatOnDeviceAsync: unsafe extern "system" fn(this: *mut *mut Self, category: super::Capture::MediaCategory, encodingproperties: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties")))]
    CreateDeviceInputNodeWithFormatOnDeviceAsync: usize,
    pub CreateFrameOutputNode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub CreateFrameOutputNodeWithFormat: unsafe extern "system" fn(this: *mut *mut Self, encodingproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    CreateFrameOutputNodeWithFormat: usize,
    #[cfg(feature = "Foundation")]
    pub CreateDeviceOutputNodeAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateDeviceOutputNodeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateFileInputNodeAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateFileInputNodeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateFileOutputNodeAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateFileOutputNodeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub CreateFileOutputNodeWithFileProfileAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, fileencodingprofile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage")))]
    CreateFileOutputNodeWithFileProfileAsync: usize,
    pub CreateSubmixNode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub CreateSubmixNodeWithFormat: unsafe extern "system" fn(this: *mut *mut Self, encodingproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    CreateSubmixNodeWithFormat: usize,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ResetAllNodes: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub QuantumStarted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QuantumStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveQuantumStarted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveQuantumStarted: usize,
    #[cfg(feature = "Foundation")]
    pub QuantumProcessed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QuantumProcessed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveQuantumProcessed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveQuantumProcessed: usize,
    #[cfg(feature = "Foundation")]
    pub UnrecoverableErrorOccurred: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnrecoverableErrorOccurred: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUnrecoverableErrorOccurred: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUnrecoverableErrorOccurred: usize,
    pub CompletedQuantumCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub EncodingProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    EncodingProperties: usize,
    pub LatencyInSamples: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub PrimaryRenderDevice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    PrimaryRenderDevice: usize,
    pub RenderDeviceAudioProcessing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::AudioProcessing) -> ::windows_sys::core::HRESULT,
    pub SamplesPerQuantum: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioGraph2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_MediaProperties")]
    pub CreateFrameInputNodeWithFormatAndEmitter: unsafe extern "system" fn(this: *mut *mut Self, encodingproperties: *mut ::core::ffi::c_void, emitter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    CreateFrameInputNodeWithFormatAndEmitter: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync: unsafe extern "system" fn(this: *mut *mut Self, category: super::Capture::MediaCategory, encodingproperties: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, emitter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties")))]
    CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateFileInputNodeWithEmitterAsync: unsafe extern "system" fn(this: *mut *mut Self, file: *mut ::core::ffi::c_void, emitter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateFileInputNodeWithEmitterAsync: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub CreateSubmixNodeWithFormatAndEmitter: unsafe extern "system" fn(this: *mut *mut Self, encodingproperties: *mut ::core::ffi::c_void, emitter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    CreateSubmixNodeWithFormatAndEmitter: usize,
    #[cfg(feature = "Foundation")]
    pub CreateBatchUpdater: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateBatchUpdater: usize,
}
#[repr(C)]
pub struct IAudioGraph3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub CreateMediaSourceAudioInputNodeAsync: unsafe extern "system" fn(this: *mut *mut Self, mediasource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Core")))]
    CreateMediaSourceAudioInputNodeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub CreateMediaSourceAudioInputNodeWithEmitterAsync: unsafe extern "system" fn(this: *mut *mut Self, mediasource: *mut ::core::ffi::c_void, emitter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Core")))]
    CreateMediaSourceAudioInputNodeWithEmitterAsync: usize,
}
#[repr(C)]
pub struct IAudioGraphConnection {
    pub base__: ::windows_sys::core::IInspectable,
    pub Destination: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetGain: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Gain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioGraphSettings {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_MediaProperties")]
    pub EncodingProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    EncodingProperties: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetEncodingProperties: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetEncodingProperties: usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub PrimaryRenderDevice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    PrimaryRenderDevice: usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub SetPrimaryRenderDevice: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    SetPrimaryRenderDevice: usize,
    pub QuantumSizeSelectionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut QuantumSizeSelectionMode) -> ::windows_sys::core::HRESULT,
    pub SetQuantumSizeSelectionMode: unsafe extern "system" fn(this: *mut *mut Self, value: QuantumSizeSelectionMode) -> ::windows_sys::core::HRESULT,
    pub DesiredSamplesPerQuantum: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetDesiredSamplesPerQuantum: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_Render")]
    pub AudioRenderCategory: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Render::AudioRenderCategory) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    AudioRenderCategory: usize,
    #[cfg(feature = "Media_Render")]
    pub SetAudioRenderCategory: unsafe extern "system" fn(this: *mut *mut Self, value: super::Render::AudioRenderCategory) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    SetAudioRenderCategory: usize,
    pub DesiredRenderDeviceAudioProcessing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::AudioProcessing) -> ::windows_sys::core::HRESULT,
    pub SetDesiredRenderDeviceAudioProcessing: unsafe extern "system" fn(this: *mut *mut Self, value: super::AudioProcessing) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioGraphSettings2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetMaxPlaybackSpeedFactor: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub MaxPlaybackSpeedFactor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioGraphSettingsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Render")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, audiorendercategory: super::Render::AudioRenderCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    Create: usize,
}
#[repr(C)]
pub struct IAudioGraphStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut *mut Self, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
}
#[repr(C)]
pub struct IAudioGraphUnrecoverableErrorOccurredEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioGraphUnrecoverableError) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioInputNode {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub OutgoingConnections: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OutgoingConnections: usize,
    pub AddOutgoingConnection: unsafe extern "system" fn(this: *mut *mut Self, destination: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddOutgoingConnectionWithGain: unsafe extern "system" fn(this: *mut *mut Self, destination: *mut ::core::ffi::c_void, gain: f64) -> ::windows_sys::core::HRESULT,
    pub RemoveOutgoingConnection: unsafe extern "system" fn(this: *mut *mut Self, destination: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioInputNode2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Emitter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioNode {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub EffectDefinitions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Effects")))]
    EffectDefinitions: usize,
    pub SetOutgoingGain: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub OutgoingGain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub EncodingProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    EncodingProperties: usize,
    pub ConsumeInput: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetConsumeInput: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_Effects")]
    pub DisableEffectsByDefinition: unsafe extern "system" fn(this: *mut *mut Self, definition: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Effects"))]
    DisableEffectsByDefinition: usize,
    #[cfg(feature = "Media_Effects")]
    pub EnableEffectsByDefinition: unsafe extern "system" fn(this: *mut *mut Self, definition: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Effects"))]
    EnableEffectsByDefinition: usize,
}
#[repr(C)]
pub struct IAudioNodeEmitter {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetPosition: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Direction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Direction: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetDirection: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetDirection: usize,
    pub Shape: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DecayModel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Gain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetGain: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub DistanceScale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDistanceScale: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub DopplerScale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDopplerScale: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub DopplerVelocity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DopplerVelocity: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetDopplerVelocity: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetDopplerVelocity: usize,
    pub IsDopplerDisabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioNodeEmitter2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SpatialAudioModel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialAudioModel) -> ::windows_sys::core::HRESULT,
    pub SetSpatialAudioModel: unsafe extern "system" fn(this: *mut *mut Self, value: SpatialAudioModel) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioNodeEmitterConeProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub InnerAngle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub OuterAngle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub OuterAngleGain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioNodeEmitterDecayModel {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioNodeEmitterDecayKind) -> ::windows_sys::core::HRESULT,
    pub MinGain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub MaxGain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub NaturalProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioNodeEmitterDecayModelStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateNatural: unsafe extern "system" fn(this: *mut *mut Self, mingain: f64, maxgain: f64, unitygaindistance: f64, cutoffdistance: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateCustom: unsafe extern "system" fn(this: *mut *mut Self, mingain: f64, maxgain: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioNodeEmitterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateAudioNodeEmitter: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, decaymodel: *mut ::core::ffi::c_void, settings: AudioNodeEmitterSettings, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioNodeEmitterNaturalDecayModelProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub UnityGainDistance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub CutoffDistance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioNodeEmitterShape {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioNodeEmitterShapeKind) -> ::windows_sys::core::HRESULT,
    pub ConeProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioNodeEmitterShapeStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateCone: unsafe extern "system" fn(this: *mut *mut Self, innerangle: f64, outerangle: f64, outeranglegain: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateOmnidirectional: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioNodeListener {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetPosition: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Orientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetOrientation: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Quaternion) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetOrientation: usize,
    pub SpeedOfSound: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetSpeedOfSound: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub DopplerVelocity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DopplerVelocity: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetDopplerVelocity: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetDopplerVelocity: usize,
}
#[repr(C)]
pub struct IAudioNodeWithListener {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetListener: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Listener: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioPlaybackConnection {
    pub base__: ::windows_sys::core::IInspectable,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioPlaybackConnectionState) -> ::windows_sys::core::HRESULT,
    pub Open: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OpenAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
}
#[repr(C)]
pub struct IAudioPlaybackConnectionOpenResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioPlaybackConnectionOpenResultStatus) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioPlaybackConnectionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryCreateFromId: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioStateMonitor {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SoundLevelChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SoundLevelChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSoundLevelChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSoundLevelChanged: usize,
    pub SoundLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::SoundLevel) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAudioStateMonitorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateForRenderMonitoring: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_Render")]
    pub CreateForRenderMonitoringWithCategory: unsafe extern "system" fn(this: *mut *mut Self, category: super::Render::AudioRenderCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    CreateForRenderMonitoringWithCategory: usize,
    #[cfg(all(feature = "Media_Devices", feature = "Media_Render"))]
    pub CreateForRenderMonitoringWithCategoryAndDeviceRole: unsafe extern "system" fn(this: *mut *mut Self, category: super::Render::AudioRenderCategory, role: super::Devices::AudioDeviceRole, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Media_Devices", feature = "Media_Render")))]
    CreateForRenderMonitoringWithCategoryAndDeviceRole: usize,
    #[cfg(feature = "Media_Render")]
    pub CreateForRenderMonitoringWithCategoryAndDeviceId: unsafe extern "system" fn(this: *mut *mut Self, category: super::Render::AudioRenderCategory, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    CreateForRenderMonitoringWithCategoryAndDeviceId: usize,
    pub CreateForCaptureMonitoring: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_Capture")]
    pub CreateForCaptureMonitoringWithCategory: unsafe extern "system" fn(this: *mut *mut Self, category: super::Capture::MediaCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    CreateForCaptureMonitoringWithCategory: usize,
    #[cfg(all(feature = "Media_Capture", feature = "Media_Devices"))]
    pub CreateForCaptureMonitoringWithCategoryAndDeviceRole: unsafe extern "system" fn(this: *mut *mut Self, category: super::Capture::MediaCategory, role: super::Devices::AudioDeviceRole, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Media_Capture", feature = "Media_Devices")))]
    CreateForCaptureMonitoringWithCategoryAndDeviceRole: usize,
    #[cfg(feature = "Media_Capture")]
    pub CreateForCaptureMonitoringWithCategoryAndDeviceId: unsafe extern "system" fn(this: *mut *mut Self, category: super::Capture::MediaCategory, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    CreateForCaptureMonitoringWithCategoryAndDeviceId: usize,
}
#[repr(C)]
pub struct ICreateAudioDeviceInputNodeResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioDeviceNodeCreationStatus) -> ::windows_sys::core::HRESULT,
    pub DeviceInputNode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICreateAudioDeviceInputNodeResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICreateAudioDeviceOutputNodeResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioDeviceNodeCreationStatus) -> ::windows_sys::core::HRESULT,
    pub DeviceOutputNode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICreateAudioDeviceOutputNodeResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICreateAudioFileInputNodeResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioFileNodeCreationStatus) -> ::windows_sys::core::HRESULT,
    pub FileInputNode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICreateAudioFileInputNodeResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICreateAudioFileOutputNodeResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioFileNodeCreationStatus) -> ::windows_sys::core::HRESULT,
    pub FileOutputNode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICreateAudioFileOutputNodeResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICreateAudioGraphResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioGraphCreationStatus) -> ::windows_sys::core::HRESULT,
    pub Graph: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICreateAudioGraphResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICreateMediaSourceAudioInputNodeResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaSourceAudioInputNodeCreationStatus) -> ::windows_sys::core::HRESULT,
    pub Node: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICreateMediaSourceAudioInputNodeResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEchoEffectDefinition {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWetDryMix: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub WetDryMix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFeedback: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Feedback: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDelay: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Delay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEchoEffectDefinitionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, audiograph: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEqualizerBand {
    pub base__: ::windows_sys::core::IInspectable,
    pub Bandwidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetBandwidth: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub FrequencyCenter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFrequencyCenter: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Gain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetGain: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IEqualizerEffectDefinition {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Bands: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Bands: usize,
}
#[repr(C)]
pub struct IEqualizerEffectDefinitionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, audiograph: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFrameInputNodeQuantumStartedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequiredSamples: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILimiterEffectDefinition {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetRelease: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub Release: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetLoudness: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub Loudness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILimiterEffectDefinitionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, audiograph: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaSourceAudioInputNode {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetPlaybackSpeedFactor: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub PlaybackSpeedFactor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub Seek: unsafe extern "system" fn(this: *mut *mut Self, position: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Seek: usize,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub EndTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetEndTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEndTime: usize,
    #[cfg(feature = "Foundation")]
    pub LoopCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoopCount: usize,
    #[cfg(feature = "Foundation")]
    pub SetLoopCount: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLoopCount: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Media_Core")]
    pub MediaSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    MediaSource: usize,
    #[cfg(feature = "Foundation")]
    pub MediaSourceCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MediaSourceCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMediaSourceCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMediaSourceCompleted: usize,
}
#[repr(C)]
pub struct IReverbEffectDefinition {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetWetDryMix: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub WetDryMix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetReflectionsDelay: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ReflectionsDelay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetReverbDelay: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
    pub ReverbDelay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetRearDelay: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
    pub RearDelay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetPositionLeft: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
    pub PositionLeft: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetPositionRight: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
    pub PositionRight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetPositionMatrixLeft: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
    pub PositionMatrixLeft: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetPositionMatrixRight: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
    pub PositionMatrixRight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetEarlyDiffusion: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
    pub EarlyDiffusion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetLateDiffusion: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
    pub LateDiffusion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetLowEQGain: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
    pub LowEQGain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetLowEQCutoff: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
    pub LowEQCutoff: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetHighEQGain: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
    pub HighEQGain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetHighEQCutoff: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
    pub HighEQCutoff: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetRoomFilterFreq: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub RoomFilterFreq: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRoomFilterMain: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub RoomFilterMain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRoomFilterHF: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub RoomFilterHF: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetReflectionsGain: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ReflectionsGain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetReverbGain: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ReverbGain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDecayTime: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub DecayTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDensity: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Density: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRoomSize: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub RoomSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDisableLateField: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub DisableLateField: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IReverbEffectDefinitionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, audiograph: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISetDefaultSpatialAudioFormatResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SetDefaultSpatialAudioFormatStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpatialAudioDeviceConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsSpatialAudioSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsSpatialAudioFormatSupported: unsafe extern "system" fn(this: *mut *mut Self, subtype: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ActiveSpatialAudioFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DefaultSpatialAudioFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetDefaultSpatialAudioFormatAsync: unsafe extern "system" fn(this: *mut *mut Self, subtype: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDefaultSpatialAudioFormatAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ConfigurationChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConfigurationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConfigurationChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConfigurationChanged: usize,
}
#[repr(C)]
pub struct ISpatialAudioDeviceConfigurationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForDeviceId: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpatialAudioFormatConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReportLicenseChangedAsync: unsafe extern "system" fn(this: *mut *mut Self, subtype: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportLicenseChangedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportConfigurationChangedAsync: unsafe extern "system" fn(this: *mut *mut Self, subtype: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportConfigurationChangedAsync: usize,
    pub MixedRealityExclusiveModePolicy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MixedRealitySpatialAudioFormatPolicy) -> ::windows_sys::core::HRESULT,
    pub SetMixedRealityExclusiveModePolicy: unsafe extern "system" fn(this: *mut *mut Self, value: MixedRealitySpatialAudioFormatPolicy) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpatialAudioFormatConfigurationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpatialAudioFormatSubtypeStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub WindowsSonic: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DolbyAtmosForHeadphones: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DolbyAtmosForHomeTheater: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DolbyAtmosForSpeakers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DTSHeadphoneX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DTSXUltra: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpatialAudioFormatSubtypeStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DTSXForHomeTheater: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
pub type LimiterEffectDefinition = *mut ::core::ffi::c_void;
pub type MediaSourceAudioInputNode = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct MediaSourceAudioInputNodeCreationStatus(pub i32);
impl MediaSourceAudioInputNodeCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const FormatNotSupported: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaSourceAudioInputNodeCreationStatus {}
impl ::core::clone::Clone for MediaSourceAudioInputNodeCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct MixedRealitySpatialAudioFormatPolicy(pub i32);
impl MixedRealitySpatialAudioFormatPolicy {
    pub const UseMixedRealityDefaultSpatialAudioFormat: Self = Self(0i32);
    pub const UseDeviceConfigurationDefaultSpatialAudioFormat: Self = Self(1i32);
}
impl ::core::marker::Copy for MixedRealitySpatialAudioFormatPolicy {}
impl ::core::clone::Clone for MixedRealitySpatialAudioFormatPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct QuantumSizeSelectionMode(pub i32);
impl QuantumSizeSelectionMode {
    pub const SystemDefault: Self = Self(0i32);
    pub const LowestLatency: Self = Self(1i32);
    pub const ClosestToDesired: Self = Self(2i32);
}
impl ::core::marker::Copy for QuantumSizeSelectionMode {}
impl ::core::clone::Clone for QuantumSizeSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ReverbEffectDefinition = *mut ::core::ffi::c_void;
pub type SetDefaultSpatialAudioFormatResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct SetDefaultSpatialAudioFormatStatus(pub i32);
impl SetDefaultSpatialAudioFormatStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const AccessDenied: Self = Self(1i32);
    pub const LicenseExpired: Self = Self(2i32);
    pub const LicenseNotValidForAudioEndpoint: Self = Self(3i32);
    pub const NotSupportedOnAudioEndpoint: Self = Self(4i32);
    pub const UnknownError: Self = Self(5i32);
}
impl ::core::marker::Copy for SetDefaultSpatialAudioFormatStatus {}
impl ::core::clone::Clone for SetDefaultSpatialAudioFormatStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SpatialAudioDeviceConfiguration = *mut ::core::ffi::c_void;
pub type SpatialAudioFormatConfiguration = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct SpatialAudioModel(pub i32);
impl SpatialAudioModel {
    pub const ObjectBased: Self = Self(0i32);
    pub const FoldDown: Self = Self(1i32);
}
impl ::core::marker::Copy for SpatialAudioModel {}
impl ::core::clone::Clone for SpatialAudioModel {
    fn clone(&self) -> Self {
        *self
    }
}
