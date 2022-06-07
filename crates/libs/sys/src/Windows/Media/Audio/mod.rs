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
impl ::windows_sys::core::Interface for IAudioDeviceInputNode {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2954587105, data2: 28494, data3: 18914, data4: [172, 1, 85, 157, 98, 190, 179, 169] };
}
#[repr(C)]
pub struct IAudioDeviceOutputNode {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Enumeration")]
    pub Device: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    Device: usize,
}
impl ::windows_sys::core::Interface for IAudioDeviceOutputNode {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 909040639, data2: 65308, data3: 17460, data4: [158, 15, 189, 46, 245, 34, 172, 130] };
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
impl ::windows_sys::core::Interface for IAudioFileInputNode {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2421909448, data2: 28517, data3: 19668, data4: [136, 144, 70, 148, 132, 60, 39, 109] };
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
impl ::windows_sys::core::Interface for IAudioFileOutputNode {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1356863872, data2: 20838, data3: 16531, data4: [128, 248, 173, 160, 0, 137, 233, 207] };
}
#[repr(C)]
pub struct IAudioFrameCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Frame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioFrameCompletedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3699147422, data2: 520, data3: 17668, data4: [165, 168, 240, 242, 104, 146, 10, 101] };
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
impl ::windows_sys::core::Interface for IAudioFrameInputNode {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 28468935, data2: 64918, data3: 20469, data4: [163, 197, 210, 122, 155, 244, 66, 55] };
}
#[repr(C)]
pub struct IAudioFrameOutputNode {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioFrameOutputNode {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3091674907, data2: 12953, data3: 17909, data4: [136, 179, 201, 209, 42, 63, 28, 200] };
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
impl ::windows_sys::core::Interface for IAudioGraph {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 450129645, data2: 58508, data3: 19988, data4: [150, 96, 44, 79, 131, 233, 205, 216] };
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
impl ::windows_sys::core::Interface for IAudioGraph2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1313618901, data2: 20417, data3: 17910, data4: [169, 71, 60, 211, 143, 79, 216, 57] };
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
impl ::windows_sys::core::Interface for IAudioGraph3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3721209262, data2: 4485, data3: 17063, data4: [131, 29, 106, 155, 15, 200, 104, 32] };
}
#[repr(C)]
pub struct IAudioGraphConnection {
    pub base__: ::windows_sys::core::IInspectable,
    pub Destination: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetGain: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Gain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioGraphConnection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1982886125, data2: 53326, data3: 20396, data4: [178, 51, 96, 11, 66, 237, 212, 105] };
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
impl ::windows_sys::core::Interface for IAudioGraphSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 492397695, data2: 59134, data3: 17960, data4: [132, 248, 157, 139, 219, 162, 87, 133] };
}
#[repr(C)]
pub struct IAudioGraphSettings2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetMaxPlaybackSpeedFactor: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub MaxPlaybackSpeedFactor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioGraphSettings2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1922144135, data2: 19883, data3: 18147, data4: [180, 201, 216, 225, 162, 99, 96, 98] };
}
#[repr(C)]
pub struct IAudioGraphSettingsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Render")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, audiorendercategory: super::Render::AudioRenderCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IAudioGraphSettingsFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2782469318, data2: 49899, data3: 19041, data4: [162, 20, 29, 102, 215, 95, 131, 218] };
}
#[repr(C)]
pub struct IAudioGraphStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut *mut Self, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
}
impl ::windows_sys::core::Interface for IAudioGraphStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1995190578, data2: 57689, data3: 19127, data4: [168, 42, 23, 190, 180, 179, 30, 148] };
}
#[repr(C)]
pub struct IAudioGraphUnrecoverableErrorOccurredEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioGraphUnrecoverableError) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioGraphUnrecoverableErrorOccurredEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3285830624, data2: 16374, data3: 20403, data4: [178, 98, 80, 212, 53, 197, 84, 35] };
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
impl ::windows_sys::core::Interface for IAudioInputNode {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3511156828, data2: 33832, data3: 18308, data4: [183, 253, 169, 157, 70, 140, 93, 32] };
}
#[repr(C)]
pub struct IAudioInputNode2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Emitter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioInputNode2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2421249719, data2: 51816, data3: 19565, data4: [168, 188, 227, 238, 23, 254, 63, 210] };
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
impl ::windows_sys::core::Interface for IAudioNode {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 356031871, data2: 56280, data3: 18457, data4: [191, 3, 102, 142, 147, 87, 205, 109] };
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
impl ::windows_sys::core::Interface for IAudioNodeEmitter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 913741597, data2: 34826, data3: 18360, data4: [173, 247, 19, 35, 169, 217, 101, 190] };
}
#[repr(C)]
pub struct IAudioNodeEmitter2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SpatialAudioModel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialAudioModel) -> ::windows_sys::core::HRESULT,
    pub SetSpatialAudioModel: unsafe extern "system" fn(this: *mut *mut Self, value: SpatialAudioModel) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioNodeEmitter2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1253502667, data2: 60457, data3: 18424, data4: [129, 140, 182, 182, 96, 165, 174, 177] };
}
#[repr(C)]
pub struct IAudioNodeEmitterConeProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub InnerAngle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub OuterAngle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub OuterAngleGain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioNodeEmitterConeProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3919260910, data2: 714, data3: 17269, data4: [147, 38, 12, 106, 228, 188, 223, 181] };
}
#[repr(C)]
pub struct IAudioNodeEmitterDecayModel {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioNodeEmitterDecayKind) -> ::windows_sys::core::HRESULT,
    pub MinGain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub MaxGain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub NaturalProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioNodeEmitterDecayModel {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 488463095, data2: 3411, data3: 20393, data4: [189, 132, 213, 129, 106, 134, 243, 255] };
}
#[repr(C)]
pub struct IAudioNodeEmitterDecayModelStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateNatural: unsafe extern "system" fn(this: *mut *mut Self, mingain: f64, maxgain: f64, unitygaindistance: f64, cutoffdistance: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateCustom: unsafe extern "system" fn(this: *mut *mut Self, mingain: f64, maxgain: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioNodeEmitterDecayModelStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3346562216, data2: 61816, data3: 17967, data4: [188, 129, 141, 213, 203, 229, 218, 232] };
}
#[repr(C)]
pub struct IAudioNodeEmitterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateAudioNodeEmitter: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, decaymodel: *mut ::core::ffi::c_void, settings: AudioNodeEmitterSettings, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioNodeEmitterFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4257761434, data2: 27350, data3: 19684, data4: [183, 247, 169, 147, 112, 223, 126, 233] };
}
#[repr(C)]
pub struct IAudioNodeEmitterNaturalDecayModelProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub UnityGainDistance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub CutoffDistance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioNodeEmitterNaturalDecayModelProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1217612751, data2: 53036, data3: 20220, data4: [147, 49, 117, 189, 34, 223, 31, 12] };
}
#[repr(C)]
pub struct IAudioNodeEmitterShape {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioNodeEmitterShapeKind) -> ::windows_sys::core::HRESULT,
    pub ConeProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioNodeEmitterShape {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3926069701, data2: 59197, data3: 17596, data4: [133, 156, 69, 85, 59, 188, 72, 40] };
}
#[repr(C)]
pub struct IAudioNodeEmitterShapeStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateCone: unsafe extern "system" fn(this: *mut *mut Self, innerangle: f64, outerangle: f64, outeranglegain: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateOmnidirectional: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioNodeEmitterShapeStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1471883121, data2: 65445, data3: 19334, data4: [167, 121, 226, 100, 174, 185, 20, 95] };
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
impl ::windows_sys::core::Interface for IAudioNodeListener {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3648138774, data2: 3082, data3: 16858, data4: [183, 85, 108, 119, 131, 95, 177, 235] };
}
#[repr(C)]
pub struct IAudioNodeWithListener {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetListener: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Listener: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioNodeWithListener {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 235901052, data2: 31231, data3: 17732, data4: [158, 235, 1, 37, 123, 21, 16, 90] };
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
impl ::windows_sys::core::Interface for IAudioPlaybackConnection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 441196010, data2: 51964, data3: 20711, data4: [135, 24, 234, 63, 129, 203, 250, 81] };
}
#[repr(C)]
pub struct IAudioPlaybackConnectionOpenResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioPlaybackConnectionOpenResultStatus) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioPlaybackConnectionOpenResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1315269359, data2: 14841, data3: 24521, data4: [165, 25, 165, 187, 253, 159, 233, 33] };
}
#[repr(C)]
pub struct IAudioPlaybackConnectionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryCreateFromId: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAudioPlaybackConnectionStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3859375010, data2: 27110, data3: 24572, data4: [158, 19, 130, 74, 133, 33, 61, 175] };
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
impl ::windows_sys::core::Interface for IAudioStateMonitor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 487838006, data2: 409, data3: 19676, data4: [184, 78, 231, 44, 43, 88, 30, 206] };
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
impl ::windows_sys::core::Interface for IAudioStateMonitorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1668606540, data2: 6971, data3: 16385, data4: [148, 217, 221, 34, 83, 48, 250, 64] };
}
#[repr(C)]
pub struct ICreateAudioDeviceInputNodeResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioDeviceNodeCreationStatus) -> ::windows_sys::core::HRESULT,
    pub DeviceInputNode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICreateAudioDeviceInputNodeResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 384747432, data2: 7335, data3: 16623, data4: [145, 164, 211, 70, 224, 170, 27, 186] };
}
#[repr(C)]
pub struct ICreateAudioDeviceInputNodeResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICreateAudioDeviceInputNodeResult2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2451335630, data2: 16181, data3: 16839, data4: [150, 34, 121, 246, 8, 186, 237, 194] };
}
#[repr(C)]
pub struct ICreateAudioDeviceOutputNodeResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioDeviceNodeCreationStatus) -> ::windows_sys::core::HRESULT,
    pub DeviceOutputNode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICreateAudioDeviceOutputNodeResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4151799079, data2: 7578, data3: 18423, data4: [156, 212, 40, 89, 204, 27, 123, 255] };
}
#[repr(C)]
pub struct ICreateAudioDeviceOutputNodeResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICreateAudioDeviceOutputNodeResult2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1214523039, data2: 48590, data3: 19121, data4: [189, 56, 251, 174, 147, 174, 218, 202] };
}
#[repr(C)]
pub struct ICreateAudioFileInputNodeResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioFileNodeCreationStatus) -> ::windows_sys::core::HRESULT,
    pub FileInputNode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICreateAudioFileInputNodeResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3464746524, data2: 58007, data3: 19536, data4: [156, 231, 28, 122, 105, 214, 189, 9] };
}
#[repr(C)]
pub struct ICreateAudioFileInputNodeResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICreateAudioFileInputNodeResult2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4178059296, data2: 15744, data3: 20448, data4: [129, 193, 118, 143, 234, 124, 167, 224] };
}
#[repr(C)]
pub struct ICreateAudioFileOutputNodeResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioFileNodeCreationStatus) -> ::windows_sys::core::HRESULT,
    pub FileOutputNode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICreateAudioFileOutputNodeResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1205254779, data2: 59657, data3: 17727, data4: [134, 110, 85, 64, 205, 167, 52, 255] };
}
#[repr(C)]
pub struct ICreateAudioFileOutputNodeResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICreateAudioFileOutputNodeResult2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2667689229, data2: 13080, data3: 18355, data4: [166, 10, 27, 73, 43, 231, 252, 13] };
}
#[repr(C)]
pub struct ICreateAudioGraphResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AudioGraphCreationStatus) -> ::windows_sys::core::HRESULT,
    pub Graph: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICreateAudioGraphResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1414786942, data2: 31710, data3: 19318, data4: [187, 93, 72, 247, 156, 252, 140, 11] };
}
#[repr(C)]
pub struct ICreateAudioGraphResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICreateAudioGraphResult2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1836289532, data2: 35014, data3: 20427, data4: [165, 52, 133, 206, 221, 64, 80, 161] };
}
#[repr(C)]
pub struct ICreateMediaSourceAudioInputNodeResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaSourceAudioInputNodeCreationStatus) -> ::windows_sys::core::HRESULT,
    pub Node: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICreateMediaSourceAudioInputNodeResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1185306787, data2: 21440, data3: 19801, data4: [158, 81, 204, 29, 16, 68, 164, 196] };
}
#[repr(C)]
pub struct ICreateMediaSourceAudioInputNodeResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICreateMediaSourceAudioInputNodeResult2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1666272488, data2: 27162, data3: 18915, data4: [151, 236, 40, 253, 91, 225, 20, 229] };
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
impl ::windows_sys::core::Interface for IEchoEffectDefinition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 239943594, data2: 14008, data3: 19601, data4: [185, 218, 17, 244, 74, 138, 102, 16] };
}
#[repr(C)]
pub struct IEchoEffectDefinitionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, audiograph: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEchoEffectDefinitionFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 223224407, data2: 43762, data3: 20102, data4: [165, 76, 251, 121, 219, 143, 108, 18] };
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
impl ::windows_sys::core::Interface for IEqualizerBand {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3221903978, data2: 9773, data3: 19333, data4: [155, 183, 67, 40, 11, 98, 237, 12] };
}
#[repr(C)]
pub struct IEqualizerEffectDefinition {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Bands: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Bands: usize,
}
impl ::windows_sys::core::Interface for IEqualizerEffectDefinition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 37711647, data2: 33790, data3: 17562, data4: [168, 34, 198, 150, 68, 45, 22, 176] };
}
#[repr(C)]
pub struct IEqualizerEffectDefinitionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, audiograph: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEqualizerEffectDefinitionFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3532091332, data2: 54288, data3: 20149, data4: [158, 105, 201, 170, 18, 119, 234, 240] };
}
#[repr(C)]
pub struct IFrameInputNodeQuantumStartedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequiredSamples: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFrameInputNodeQuantumStartedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1033622680, data2: 41734, data3: 20230, data4: [189, 159, 233, 239, 200, 34, 99, 4] };
}
#[repr(C)]
pub struct ILimiterEffectDefinition {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetRelease: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub Release: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetLoudness: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub Loudness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILimiterEffectDefinition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1802853657, data2: 9731, data3: 18362, data4: [189, 235, 57, 5, 94, 52, 134, 220] };
}
#[repr(C)]
pub struct ILimiterEffectDefinitionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, audiograph: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILimiterEffectDefinitionFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3971671793, data2: 25087, data3: 17903, data4: [184, 245, 72, 101, 154, 87, 199, 45] };
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
impl ::windows_sys::core::Interface for IMediaSourceAudioInputNode {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2581108795, data2: 43146, data3: 16449, data4: [142, 79, 221, 186, 192, 201, 31, 211] };
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
impl ::windows_sys::core::Interface for IReverbEffectDefinition {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1174841993, data2: 62819, data3: 19722, data4: [143, 110, 240, 205, 223, 243, 93, 132] };
}
#[repr(C)]
pub struct IReverbEffectDefinitionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, audiograph: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IReverbEffectDefinitionFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2815806462, data2: 4107, data3: 20464, data4: [157, 166, 220, 78, 5, 167, 89, 240] };
}
#[repr(C)]
pub struct ISetDefaultSpatialAudioFormatResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SetDefaultSpatialAudioFormatStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISetDefaultSpatialAudioFormatResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 472556817, data2: 5120, data3: 24176, data4: [158, 169, 174, 21, 18, 65, 232, 234] };
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
impl ::windows_sys::core::Interface for ISpatialAudioDeviceConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4001562676, data2: 25039, data3: 22345, data4: [157, 164, 16, 240, 254, 2, 129, 153] };
}
#[repr(C)]
pub struct ISpatialAudioDeviceConfigurationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForDeviceId: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialAudioDeviceConfigurationStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1052999547, data2: 37741, data3: 19972, data4: [151, 40, 40, 39, 217, 247, 88, 196] };
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
impl ::windows_sys::core::Interface for ISpatialAudioFormatConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 853477800, data2: 20720, data3: 21397, data4: [153, 35, 125, 68, 202, 113, 237, 109] };
}
#[repr(C)]
pub struct ISpatialAudioFormatConfigurationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialAudioFormatConfigurationStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 727707505, data2: 26569, data3: 20063, data4: [163, 91, 65, 104, 7, 17, 248, 199] };
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
impl ::windows_sys::core::Interface for ISpatialAudioFormatSubtypeStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3017706055, data2: 33774, data3: 16998, data4: [169, 69, 190, 223, 80, 122, 254, 237] };
}
#[repr(C)]
pub struct ISpatialAudioFormatSubtypeStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DTSXForHomeTheater: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialAudioFormatSubtypeStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1164306123, data2: 55643, data3: 22049, data4: [182, 175, 14, 136, 73, 197, 124, 128] };
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
