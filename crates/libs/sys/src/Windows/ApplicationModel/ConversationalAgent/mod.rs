pub type ActivationSignalDetectionConfiguration = *mut ::core::ffi::c_void;
pub type ActivationSignalDetectionConfigurationCreationResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ActivationSignalDetectionConfigurationCreationStatus(pub i32);
impl ActivationSignalDetectionConfigurationCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const SignalIdNotAvailable: Self = Self(1i32);
    pub const ModelIdNotSupported: Self = Self(2i32);
    pub const InvalidSignalId: Self = Self(3i32);
    pub const InvalidModelId: Self = Self(4i32);
    pub const InvalidDisplayName: Self = Self(5i32);
    pub const ConfigurationAlreadyExists: Self = Self(6i32);
    pub const CreationNotSupported: Self = Self(7i32);
}
impl ::core::marker::Copy for ActivationSignalDetectionConfigurationCreationStatus {}
impl ::core::clone::Clone for ActivationSignalDetectionConfigurationCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ActivationSignalDetectionConfigurationRemovalResult(pub i32);
impl ActivationSignalDetectionConfigurationRemovalResult {
    pub const Success: Self = Self(0i32);
    pub const NotFound: Self = Self(1i32);
    pub const CurrentlyEnabled: Self = Self(2i32);
    pub const RemovalNotSupported: Self = Self(3i32);
}
impl ::core::marker::Copy for ActivationSignalDetectionConfigurationRemovalResult {}
impl ::core::clone::Clone for ActivationSignalDetectionConfigurationRemovalResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ActivationSignalDetectionConfigurationSetModelDataResult(pub i32);
impl ActivationSignalDetectionConfigurationSetModelDataResult {
    pub const Success: Self = Self(0i32);
    pub const EmptyModelData: Self = Self(1i32);
    pub const UnsupportedFormat: Self = Self(2i32);
    pub const ConfigurationCurrentlyEnabled: Self = Self(3i32);
    pub const InvalidData: Self = Self(4i32);
    pub const SetModelDataNotSupported: Self = Self(5i32);
    pub const ConfigurationNotFound: Self = Self(6i32);
    pub const UnknownError: Self = Self(7i32);
}
impl ::core::marker::Copy for ActivationSignalDetectionConfigurationSetModelDataResult {}
impl ::core::clone::Clone for ActivationSignalDetectionConfigurationSetModelDataResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ActivationSignalDetectionConfigurationStateChangeResult(pub i32);
impl ActivationSignalDetectionConfigurationStateChangeResult {
    pub const Success: Self = Self(0i32);
    pub const NoModelData: Self = Self(1i32);
    pub const ConfigurationNotFound: Self = Self(2i32);
}
impl ::core::marker::Copy for ActivationSignalDetectionConfigurationStateChangeResult {}
impl ::core::clone::Clone for ActivationSignalDetectionConfigurationStateChangeResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ActivationSignalDetectionTrainingDataFormat(pub i32);
impl ActivationSignalDetectionTrainingDataFormat {
    pub const Voice8kHz8BitMono: Self = Self(0i32);
    pub const Voice8kHz16BitMono: Self = Self(1i32);
    pub const Voice16kHz8BitMono: Self = Self(2i32);
    pub const Voice16kHz16BitMono: Self = Self(3i32);
    pub const VoiceOEMDefined: Self = Self(4i32);
    pub const Audio44kHz8BitMono: Self = Self(5i32);
    pub const Audio44kHz16BitMono: Self = Self(6i32);
    pub const Audio48kHz8BitMono: Self = Self(7i32);
    pub const Audio48kHz16BitMono: Self = Self(8i32);
    pub const AudioOEMDefined: Self = Self(9i32);
    pub const OtherOEMDefined: Self = Self(10i32);
}
impl ::core::marker::Copy for ActivationSignalDetectionTrainingDataFormat {}
impl ::core::clone::Clone for ActivationSignalDetectionTrainingDataFormat {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ActivationSignalDetector = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ActivationSignalDetectorKind(pub i32);
impl ActivationSignalDetectorKind {
    pub const AudioPattern: Self = Self(0i32);
    pub const AudioImpulse: Self = Self(1i32);
    pub const HardwareEvent: Self = Self(2i32);
}
impl ::core::marker::Copy for ActivationSignalDetectorKind {}
impl ::core::clone::Clone for ActivationSignalDetectorKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ActivationSignalDetectorPowerState(pub i32);
impl ActivationSignalDetectorPowerState {
    pub const HighPower: Self = Self(0i32);
    pub const ConnectedLowPower: Self = Self(1i32);
    pub const DisconnectedLowPower: Self = Self(2i32);
}
impl ::core::marker::Copy for ActivationSignalDetectorPowerState {}
impl ::core::clone::Clone for ActivationSignalDetectorPowerState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ConversationalAgentActivationKind(pub i32);
impl ConversationalAgentActivationKind {
    pub const VoiceActivationPreview: Self = Self(0i32);
    pub const Foreground: Self = Self(1i32);
}
impl ::core::marker::Copy for ConversationalAgentActivationKind {}
impl ::core::clone::Clone for ConversationalAgentActivationKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ConversationalAgentActivationResult(pub i32);
impl ConversationalAgentActivationResult {
    pub const Success: Self = Self(0i32);
    pub const AgentInactive: Self = Self(1i32);
    pub const ScreenNotAvailable: Self = Self(2i32);
    pub const AgentInterrupted: Self = Self(3i32);
}
impl ::core::marker::Copy for ConversationalAgentActivationResult {}
impl ::core::clone::Clone for ConversationalAgentActivationResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ConversationalAgentDetectorManager = *mut ::core::ffi::c_void;
pub type ConversationalAgentSession = *mut ::core::ffi::c_void;
pub type ConversationalAgentSessionInterruptedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ConversationalAgentSessionUpdateResponse(pub i32);
impl ConversationalAgentSessionUpdateResponse {
    pub const Success: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
}
impl ::core::marker::Copy for ConversationalAgentSessionUpdateResponse {}
impl ::core::clone::Clone for ConversationalAgentSessionUpdateResponse {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ConversationalAgentSignal = *mut ::core::ffi::c_void;
pub type ConversationalAgentSignalDetectedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ConversationalAgentState(pub i32);
impl ConversationalAgentState {
    pub const Inactive: Self = Self(0i32);
    pub const Detecting: Self = Self(1i32);
    pub const Listening: Self = Self(2i32);
    pub const Working: Self = Self(3i32);
    pub const Speaking: Self = Self(4i32);
    pub const ListeningAndSpeaking: Self = Self(5i32);
}
impl ::core::marker::Copy for ConversationalAgentState {}
impl ::core::clone::Clone for ConversationalAgentState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ConversationalAgentSystemStateChangeType(pub i32);
impl ConversationalAgentSystemStateChangeType {
    pub const UserAuthentication: Self = Self(0i32);
    pub const ScreenAvailability: Self = Self(1i32);
    pub const IndicatorLightAvailability: Self = Self(2i32);
    pub const VoiceActivationAvailability: Self = Self(3i32);
}
impl ::core::marker::Copy for ConversationalAgentSystemStateChangeType {}
impl ::core::clone::Clone for ConversationalAgentSystemStateChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ConversationalAgentSystemStateChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ConversationalAgentVoiceActivationPrerequisiteKind(pub i32);
impl ConversationalAgentVoiceActivationPrerequisiteKind {
    pub const MicrophonePermission: Self = Self(0i32);
    pub const KnownAgents: Self = Self(1i32);
    pub const AgentAllowed: Self = Self(2i32);
    pub const AppCapability: Self = Self(3i32);
    pub const BackgroundTaskRegistration: Self = Self(4i32);
    pub const PolicyPermission: Self = Self(5i32);
}
impl ::core::marker::Copy for ConversationalAgentVoiceActivationPrerequisiteKind {}
impl ::core::clone::Clone for ConversationalAgentVoiceActivationPrerequisiteKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct DetectionConfigurationAvailabilityChangeKind(pub i32);
impl DetectionConfigurationAvailabilityChangeKind {
    pub const SystemResourceAccess: Self = Self(0i32);
    pub const Permission: Self = Self(1i32);
    pub const LockScreenPermission: Self = Self(2i32);
}
impl ::core::marker::Copy for DetectionConfigurationAvailabilityChangeKind {}
impl ::core::clone::Clone for DetectionConfigurationAvailabilityChangeKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DetectionConfigurationAvailabilityChangedEventArgs = *mut ::core::ffi::c_void;
pub type DetectionConfigurationAvailabilityInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct DetectionConfigurationTrainingStatus(pub i32);
impl DetectionConfigurationTrainingStatus {
    pub const Success: Self = Self(0i32);
    pub const FormatNotSupported: Self = Self(1i32);
    pub const VoiceTooQuiet: Self = Self(2i32);
    pub const VoiceTooLoud: Self = Self(3i32);
    pub const VoiceTooFast: Self = Self(4i32);
    pub const VoiceTooSlow: Self = Self(5i32);
    pub const VoiceQualityProblem: Self = Self(6i32);
    pub const TrainingSystemInternalError: Self = Self(7i32);
    pub const TrainingTimedOut: Self = Self(8i32);
    pub const ConfigurationNotFound: Self = Self(9i32);
}
impl ::core::marker::Copy for DetectionConfigurationTrainingStatus {}
impl ::core::clone::Clone for DetectionConfigurationTrainingStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IActivationSignalDetectionConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    pub SignalId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ModelId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetEnabledAsync: unsafe extern "system" fn(this: *mut *mut Self, value: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEnabledAsync: usize,
    pub AvailabilityInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AvailabilityChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AvailabilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAvailabilityChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAvailabilityChanged: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetModelData: unsafe extern "system" fn(this: *mut *mut Self, datatype: ::windows_sys::core::HSTRING, data: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetModelData: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SetModelDataAsync: unsafe extern "system" fn(this: *mut *mut Self, datatype: ::windows_sys::core::HSTRING, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SetModelDataAsync: usize,
    pub GetModelDataType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetModelDataTypeAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetModelDataTypeAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetModelData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetModelData: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetModelDataAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetModelDataAsync: usize,
    pub ClearModelData: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClearModelDataAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearModelDataAsync: usize,
    pub TrainingStepsCompleted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub TrainingStepsRemaining: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub TrainingDataFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ActivationSignalDetectionTrainingDataFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ApplyTrainingData: unsafe extern "system" fn(this: *mut *mut Self, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: *mut ::core::ffi::c_void, result__: *mut DetectionConfigurationTrainingStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ApplyTrainingData: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ApplyTrainingDataAsync: unsafe extern "system" fn(this: *mut *mut Self, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ApplyTrainingDataAsync: usize,
    pub ClearTrainingData: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClearTrainingDataAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearTrainingDataAsync: usize,
}
#[repr(C)]
pub struct IActivationSignalDetectionConfiguration2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub SetModelDataWithResult: unsafe extern "system" fn(this: *mut *mut Self, datatype: ::windows_sys::core::HSTRING, data: *mut ::core::ffi::c_void, result__: *mut ActivationSignalDetectionConfigurationSetModelDataResult) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetModelDataWithResult: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SetModelDataWithResultAsync: unsafe extern "system" fn(this: *mut *mut Self, datatype: ::windows_sys::core::HSTRING, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SetModelDataWithResultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetEnabledWithResultAsync: unsafe extern "system" fn(this: *mut *mut Self, value: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEnabledWithResultAsync: usize,
    pub SetEnabledWithResult: unsafe extern "system" fn(this: *mut *mut Self, value: bool, result__: *mut ActivationSignalDetectionConfigurationStateChangeResult) -> ::windows_sys::core::HRESULT,
    pub TrainingStepCompletionMaxAllowedTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IActivationSignalDetectionConfigurationCreationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ActivationSignalDetectionConfigurationCreationStatus) -> ::windows_sys::core::HRESULT,
    pub Configuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IActivationSignalDetector {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProviderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ActivationSignalDetectorKind) -> ::windows_sys::core::HRESULT,
    pub CanCreateConfigurations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModelDataTypes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModelDataTypes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedTrainingDataFormats: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedTrainingDataFormats: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPowerStates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPowerStates: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedModelIdsForSignalId: unsafe extern "system" fn(this: *mut *mut Self, signalid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedModelIdsForSignalId: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedModelIdsForSignalIdAsync: unsafe extern "system" fn(this: *mut *mut Self, signalid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedModelIdsForSignalIdAsync: usize,
    pub CreateConfiguration: unsafe extern "system" fn(this: *mut *mut Self, signalid: ::windows_sys::core::HSTRING, modelid: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateConfigurationAsync: unsafe extern "system" fn(this: *mut *mut Self, signalid: ::windows_sys::core::HSTRING, modelid: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateConfigurationAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConfigurations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConfigurations: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConfigurationsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConfigurationsAsync: usize,
    pub GetConfiguration: unsafe extern "system" fn(this: *mut *mut Self, signalid: ::windows_sys::core::HSTRING, modelid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetConfigurationAsync: unsafe extern "system" fn(this: *mut *mut Self, signalid: ::windows_sys::core::HSTRING, modelid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetConfigurationAsync: usize,
    pub RemoveConfiguration: unsafe extern "system" fn(this: *mut *mut Self, signalid: ::windows_sys::core::HSTRING, modelid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RemoveConfigurationAsync: unsafe extern "system" fn(this: *mut *mut Self, signalid: ::windows_sys::core::HSTRING, modelid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConfigurationAsync: usize,
}
#[repr(C)]
pub struct IActivationSignalDetector2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAvailableModelIdsForSignalIdAsync: unsafe extern "system" fn(this: *mut *mut Self, signalid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAvailableModelIdsForSignalIdAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAvailableModelIdsForSignalId: unsafe extern "system" fn(this: *mut *mut Self, signalid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAvailableModelIdsForSignalId: usize,
    #[cfg(feature = "Foundation")]
    pub CreateConfigurationWithResultAsync: unsafe extern "system" fn(this: *mut *mut Self, signalid: ::windows_sys::core::HSTRING, modelid: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateConfigurationWithResultAsync: usize,
    pub CreateConfigurationWithResult: unsafe extern "system" fn(this: *mut *mut Self, signalid: ::windows_sys::core::HSTRING, modelid: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RemoveConfigurationWithResultAsync: unsafe extern "system" fn(this: *mut *mut Self, signalid: ::windows_sys::core::HSTRING, modelid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConfigurationWithResultAsync: usize,
    pub RemoveConfigurationWithResult: unsafe extern "system" fn(this: *mut *mut Self, signalid: ::windows_sys::core::HSTRING, modelid: ::windows_sys::core::HSTRING, result__: *mut ActivationSignalDetectionConfigurationRemovalResult) -> ::windows_sys::core::HRESULT,
    pub DetectorId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IConversationalAgentDetectorManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllActivationSignalDetectors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllActivationSignalDetectors: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllActivationSignalDetectorsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllActivationSignalDetectorsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetActivationSignalDetectors: unsafe extern "system" fn(this: *mut *mut Self, kind: ActivationSignalDetectorKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetActivationSignalDetectors: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetActivationSignalDetectorsAsync: unsafe extern "system" fn(this: *mut *mut Self, kind: ActivationSignalDetectorKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetActivationSignalDetectorsAsync: usize,
}
#[repr(C)]
pub struct IConversationalAgentDetectorManager2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetActivationSignalDetectorFromId: unsafe extern "system" fn(this: *mut *mut Self, detectorid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetActivationSignalDetectorFromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, detectorid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetActivationSignalDetectorFromIdAsync: usize,
}
#[repr(C)]
pub struct IConversationalAgentDetectorManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Default: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IConversationalAgentSession {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SessionInterrupted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SessionInterrupted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSessionInterrupted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSessionInterrupted: usize,
    #[cfg(feature = "Foundation")]
    pub SignalDetected: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignalDetected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSignalDetected: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSignalDetected: usize,
    #[cfg(feature = "Foundation")]
    pub SystemStateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSystemStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSystemStateChanged: usize,
    pub AgentState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ConversationalAgentState) -> ::windows_sys::core::HRESULT,
    pub Signal: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsIndicatorLightAvailable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsScreenAvailable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsUserAuthenticated: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsVoiceActivationAvailable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsInterruptible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsInterrupted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestInterruptibleAsync: unsafe extern "system" fn(this: *mut *mut Self, interruptible: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestInterruptibleAsync: usize,
    pub RequestInterruptible: unsafe extern "system" fn(this: *mut *mut Self, interruptible: bool, result__: *mut ConversationalAgentSessionUpdateResponse) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestAgentStateChangeAsync: unsafe extern "system" fn(this: *mut *mut Self, state: ConversationalAgentState, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAgentStateChangeAsync: usize,
    pub RequestAgentStateChange: unsafe extern "system" fn(this: *mut *mut Self, state: ConversationalAgentState, result__: *mut ConversationalAgentSessionUpdateResponse) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestForegroundActivationAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestForegroundActivationAsync: usize,
    pub RequestForegroundActivation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ConversationalAgentSessionUpdateResponse) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetAudioClientAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAudioClientAsync: usize,
    pub GetAudioClient: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_Audio"))]
    pub CreateAudioDeviceInputNodeAsync: unsafe extern "system" fn(this: *mut *mut Self, graph: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Audio")))]
    CreateAudioDeviceInputNodeAsync: usize,
    #[cfg(feature = "Media_Audio")]
    pub CreateAudioDeviceInputNode: unsafe extern "system" fn(this: *mut *mut Self, graph: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Audio"))]
    CreateAudioDeviceInputNode: usize,
    #[cfg(feature = "Foundation")]
    pub GetAudioCaptureDeviceIdAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAudioCaptureDeviceIdAsync: usize,
    pub GetAudioCaptureDeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetAudioRenderDeviceIdAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAudioRenderDeviceIdAsync: usize,
    pub GetAudioRenderDeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetSignalModelIdAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSignalModelIdAsync: usize,
    pub GetSignalModelId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetSignalModelIdAsync: unsafe extern "system" fn(this: *mut *mut Self, signalmodelid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSignalModelIdAsync: usize,
    pub SetSignalModelId: unsafe extern "system" fn(this: *mut *mut Self, signalmodelid: u32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedSignalModelIdsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedSignalModelIdsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedSignalModelIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedSignalModelIds: usize,
}
#[repr(C)]
pub struct IConversationalAgentSession2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestActivationAsync: unsafe extern "system" fn(this: *mut *mut Self, activationkind: ConversationalAgentActivationKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestActivationAsync: usize,
    pub RequestActivation: unsafe extern "system" fn(this: *mut *mut Self, activationkind: ConversationalAgentActivationKind, result__: *mut ConversationalAgentActivationResult) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetSupportLockScreenActivationAsync: unsafe extern "system" fn(this: *mut *mut Self, lockscreenactivationsupported: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSupportLockScreenActivationAsync: usize,
    pub SetSupportLockScreenActivation: unsafe extern "system" fn(this: *mut *mut Self, lockscreenactivationsupported: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMissingPrerequisites: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMissingPrerequisites: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMissingPrerequisitesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMissingPrerequisitesAsync: usize,
}
#[repr(C)]
pub struct IConversationalAgentSessionInterruptedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IConversationalAgentSessionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetCurrentSessionAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCurrentSessionAsync: usize,
    pub GetCurrentSessionSync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IConversationalAgentSignal {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSignalVerificationRequired: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSignalVerificationRequired: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SignalId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSignalId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SignalName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSignalName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SignalContext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSignalContext: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SignalStart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignalStart: usize,
    #[cfg(feature = "Foundation")]
    pub SetSignalStart: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSignalStart: usize,
    #[cfg(feature = "Foundation")]
    pub SignalEnd: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignalEnd: usize,
    #[cfg(feature = "Foundation")]
    pub SetSignalEnd: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSignalEnd: usize,
}
#[repr(C)]
pub struct IConversationalAgentSignal2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DetectorId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DetectorKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ActivationSignalDetectorKind) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IConversationalAgentSignalDetectedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IConversationalAgentSystemStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SystemStateChangeType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ConversationalAgentSystemStateChangeType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDetectionConfigurationAvailabilityChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DetectionConfigurationAvailabilityChangeKind) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDetectionConfigurationAvailabilityInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HasSystemResourceAccess: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HasPermission: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HasLockScreenPermission: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDetectionConfigurationAvailabilityInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub UnavailableSystemResources: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UnavailableSystemResources: usize,
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct SignalDetectorResourceKind(pub i32);
impl SignalDetectorResourceKind {
    pub const ParallelModelSupport: Self = Self(0i32);
    pub const ParallelModelSupportForAgent: Self = Self(1i32);
    pub const ParallelSignalSupport: Self = Self(2i32);
    pub const ParallelSignalSupportForAgent: Self = Self(3i32);
    pub const DisplayOffSupport: Self = Self(4i32);
    pub const PluggedInPower: Self = Self(5i32);
    pub const Detector: Self = Self(6i32);
    pub const SupportedSleepState: Self = Self(7i32);
    pub const SupportedBatterySaverState: Self = Self(8i32);
    pub const ScreenAvailability: Self = Self(9i32);
    pub const InputHardware: Self = Self(10i32);
    pub const AcousticEchoCancellation: Self = Self(11i32);
    pub const ModelIdSupport: Self = Self(12i32);
    pub const DataChannel: Self = Self(13i32);
}
impl ::core::marker::Copy for SignalDetectorResourceKind {}
impl ::core::clone::Clone for SignalDetectorResourceKind {
    fn clone(&self) -> Self {
        *self
    }
}
