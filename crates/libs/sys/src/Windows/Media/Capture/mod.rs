#[cfg(feature = "Media_Capture_Core")]
pub mod Core;
#[cfg(feature = "Media_Capture_Frames")]
pub mod Frames;
pub type AdvancedCapturedPhoto = *mut ::core::ffi::c_void;
pub type AdvancedPhotoCapture = *mut ::core::ffi::c_void;
pub type AppBroadcastBackgroundService = *mut ::core::ffi::c_void;
pub type AppBroadcastBackgroundServiceSignInInfo = *mut ::core::ffi::c_void;
pub type AppBroadcastBackgroundServiceStreamInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastCameraCaptureState(pub i32);
impl AppBroadcastCameraCaptureState {
    pub const Stopped: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AppBroadcastCameraCaptureState {}
impl ::core::clone::Clone for AppBroadcastCameraCaptureState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppBroadcastCameraCaptureStateChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastCameraOverlayLocation(pub i32);
impl AppBroadcastCameraOverlayLocation {
    pub const TopLeft: Self = Self(0i32);
    pub const TopCenter: Self = Self(1i32);
    pub const TopRight: Self = Self(2i32);
    pub const MiddleLeft: Self = Self(3i32);
    pub const MiddleCenter: Self = Self(4i32);
    pub const MiddleRight: Self = Self(5i32);
    pub const BottomLeft: Self = Self(6i32);
    pub const BottomCenter: Self = Self(7i32);
    pub const BottomRight: Self = Self(8i32);
}
impl ::core::marker::Copy for AppBroadcastCameraOverlayLocation {}
impl ::core::clone::Clone for AppBroadcastCameraOverlayLocation {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastCameraOverlaySize(pub i32);
impl AppBroadcastCameraOverlaySize {
    pub const Small: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const Large: Self = Self(2i32);
}
impl ::core::marker::Copy for AppBroadcastCameraOverlaySize {}
impl ::core::clone::Clone for AppBroadcastCameraOverlaySize {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastCaptureTargetType(pub i32);
impl AppBroadcastCaptureTargetType {
    pub const AppView: Self = Self(0i32);
    pub const EntireDisplay: Self = Self(1i32);
}
impl ::core::marker::Copy for AppBroadcastCaptureTargetType {}
impl ::core::clone::Clone for AppBroadcastCaptureTargetType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastExitBroadcastModeReason(pub i32);
impl AppBroadcastExitBroadcastModeReason {
    pub const NormalExit: Self = Self(0i32);
    pub const UserCanceled: Self = Self(1i32);
    pub const AuthorizationFail: Self = Self(2i32);
    pub const ForegroundAppActivated: Self = Self(3i32);
}
impl ::core::marker::Copy for AppBroadcastExitBroadcastModeReason {}
impl ::core::clone::Clone for AppBroadcastExitBroadcastModeReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppBroadcastGlobalSettings = *mut ::core::ffi::c_void;
pub type AppBroadcastHeartbeatRequestedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastMicrophoneCaptureState(pub i32);
impl AppBroadcastMicrophoneCaptureState {
    pub const Stopped: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AppBroadcastMicrophoneCaptureState {}
impl ::core::clone::Clone for AppBroadcastMicrophoneCaptureState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppBroadcastMicrophoneCaptureStateChangedEventArgs = *mut ::core::ffi::c_void;
pub type AppBroadcastPlugIn = *mut ::core::ffi::c_void;
pub type AppBroadcastPlugInManager = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastPlugInState(pub i32);
impl AppBroadcastPlugInState {
    pub const Unknown: Self = Self(0i32);
    pub const Initialized: Self = Self(1i32);
    pub const MicrosoftSignInRequired: Self = Self(2i32);
    pub const OAuthSignInRequired: Self = Self(3i32);
    pub const ProviderSignInRequired: Self = Self(4i32);
    pub const InBandwidthTest: Self = Self(5i32);
    pub const ReadyToBroadcast: Self = Self(6i32);
}
impl ::core::marker::Copy for AppBroadcastPlugInState {}
impl ::core::clone::Clone for AppBroadcastPlugInState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppBroadcastPlugInStateChangedEventArgs = *mut ::core::ffi::c_void;
pub type AppBroadcastPreview = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastPreviewState(pub i32);
impl AppBroadcastPreviewState {
    pub const Started: Self = Self(0i32);
    pub const Stopped: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AppBroadcastPreviewState {}
impl ::core::clone::Clone for AppBroadcastPreviewState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppBroadcastPreviewStateChangedEventArgs = *mut ::core::ffi::c_void;
pub type AppBroadcastPreviewStreamReader = *mut ::core::ffi::c_void;
pub type AppBroadcastPreviewStreamVideoFrame = *mut ::core::ffi::c_void;
pub type AppBroadcastPreviewStreamVideoHeader = *mut ::core::ffi::c_void;
pub type AppBroadcastProviderSettings = *mut ::core::ffi::c_void;
pub type AppBroadcastServices = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastSignInResult(pub i32);
impl AppBroadcastSignInResult {
    pub const Success: Self = Self(0i32);
    pub const AuthenticationFailed: Self = Self(1i32);
    pub const Unauthorized: Self = Self(2i32);
    pub const ServiceUnavailable: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
}
impl ::core::marker::Copy for AppBroadcastSignInResult {}
impl ::core::clone::Clone for AppBroadcastSignInResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastSignInState(pub i32);
impl AppBroadcastSignInState {
    pub const NotSignedIn: Self = Self(0i32);
    pub const MicrosoftSignInInProgress: Self = Self(1i32);
    pub const MicrosoftSignInComplete: Self = Self(2i32);
    pub const OAuthSignInInProgress: Self = Self(3i32);
    pub const OAuthSignInComplete: Self = Self(4i32);
}
impl ::core::marker::Copy for AppBroadcastSignInState {}
impl ::core::clone::Clone for AppBroadcastSignInState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppBroadcastSignInStateChangedEventArgs = *mut ::core::ffi::c_void;
pub type AppBroadcastState = *mut ::core::ffi::c_void;
pub type AppBroadcastStreamAudioFrame = *mut ::core::ffi::c_void;
pub type AppBroadcastStreamAudioHeader = *mut ::core::ffi::c_void;
pub type AppBroadcastStreamReader = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastStreamState(pub i32);
impl AppBroadcastStreamState {
    pub const Initializing: Self = Self(0i32);
    pub const StreamReady: Self = Self(1i32);
    pub const Started: Self = Self(2i32);
    pub const Paused: Self = Self(3i32);
    pub const Terminated: Self = Self(4i32);
}
impl ::core::marker::Copy for AppBroadcastStreamState {}
impl ::core::clone::Clone for AppBroadcastStreamState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppBroadcastStreamStateChangedEventArgs = *mut ::core::ffi::c_void;
pub type AppBroadcastStreamVideoFrame = *mut ::core::ffi::c_void;
pub type AppBroadcastStreamVideoHeader = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastTerminationReason(pub i32);
impl AppBroadcastTerminationReason {
    pub const NormalTermination: Self = Self(0i32);
    pub const LostConnectionToService: Self = Self(1i32);
    pub const NoNetworkConnectivity: Self = Self(2i32);
    pub const ServiceAbort: Self = Self(3i32);
    pub const ServiceError: Self = Self(4i32);
    pub const ServiceUnavailable: Self = Self(5i32);
    pub const InternalError: Self = Self(6i32);
    pub const UnsupportedFormat: Self = Self(7i32);
    pub const BackgroundTaskTerminated: Self = Self(8i32);
    pub const BackgroundTaskUnresponsive: Self = Self(9i32);
}
impl ::core::marker::Copy for AppBroadcastTerminationReason {}
impl ::core::clone::Clone for AppBroadcastTerminationReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppBroadcastTriggerDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastVideoEncodingBitrateMode(pub i32);
impl AppBroadcastVideoEncodingBitrateMode {
    pub const Custom: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
}
impl ::core::marker::Copy for AppBroadcastVideoEncodingBitrateMode {}
impl ::core::clone::Clone for AppBroadcastVideoEncodingBitrateMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastVideoEncodingResolutionMode(pub i32);
impl AppBroadcastVideoEncodingResolutionMode {
    pub const Custom: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
}
impl ::core::marker::Copy for AppBroadcastVideoEncodingResolutionMode {}
impl ::core::clone::Clone for AppBroadcastVideoEncodingResolutionMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppBroadcastViewerCountChangedEventArgs = *mut ::core::ffi::c_void;
pub type AppCapture = *mut ::core::ffi::c_void;
pub type AppCaptureAlternateShortcutKeys = *mut ::core::ffi::c_void;
pub type AppCaptureDurationGeneratedEventArgs = *mut ::core::ffi::c_void;
pub type AppCaptureFileGeneratedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureHistoricalBufferLengthUnit(pub i32);
impl AppCaptureHistoricalBufferLengthUnit {
    pub const Megabytes: Self = Self(0i32);
    pub const Seconds: Self = Self(1i32);
}
impl ::core::marker::Copy for AppCaptureHistoricalBufferLengthUnit {}
impl ::core::clone::Clone for AppCaptureHistoricalBufferLengthUnit {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureMetadataPriority(pub i32);
impl AppCaptureMetadataPriority {
    pub const Informational: Self = Self(0i32);
    pub const Important: Self = Self(1i32);
}
impl ::core::marker::Copy for AppCaptureMetadataPriority {}
impl ::core::clone::Clone for AppCaptureMetadataPriority {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppCaptureMetadataWriter = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureMicrophoneCaptureState(pub i32);
impl AppCaptureMicrophoneCaptureState {
    pub const Stopped: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AppCaptureMicrophoneCaptureState {}
impl ::core::clone::Clone for AppCaptureMicrophoneCaptureState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppCaptureMicrophoneCaptureStateChangedEventArgs = *mut ::core::ffi::c_void;
pub type AppCaptureRecordOperation = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureRecordingState(pub i32);
impl AppCaptureRecordingState {
    pub const InProgress: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AppCaptureRecordingState {}
impl ::core::clone::Clone for AppCaptureRecordingState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppCaptureRecordingStateChangedEventArgs = *mut ::core::ffi::c_void;
pub type AppCaptureServices = *mut ::core::ffi::c_void;
pub type AppCaptureSettings = *mut ::core::ffi::c_void;
pub type AppCaptureState = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureVideoEncodingBitrateMode(pub i32);
impl AppCaptureVideoEncodingBitrateMode {
    pub const Custom: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Standard: Self = Self(2i32);
}
impl ::core::marker::Copy for AppCaptureVideoEncodingBitrateMode {}
impl ::core::clone::Clone for AppCaptureVideoEncodingBitrateMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureVideoEncodingFrameRateMode(pub i32);
impl AppCaptureVideoEncodingFrameRateMode {
    pub const Standard: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for AppCaptureVideoEncodingFrameRateMode {}
impl ::core::clone::Clone for AppCaptureVideoEncodingFrameRateMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureVideoEncodingResolutionMode(pub i32);
impl AppCaptureVideoEncodingResolutionMode {
    pub const Custom: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Standard: Self = Self(2i32);
}
impl ::core::marker::Copy for AppCaptureVideoEncodingResolutionMode {}
impl ::core::clone::Clone for AppCaptureVideoEncodingResolutionMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CameraCaptureUI = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct CameraCaptureUIMaxPhotoResolution(pub i32);
impl CameraCaptureUIMaxPhotoResolution {
    pub const HighestAvailable: Self = Self(0i32);
    pub const VerySmallQvga: Self = Self(1i32);
    pub const SmallVga: Self = Self(2i32);
    pub const MediumXga: Self = Self(3i32);
    pub const Large3M: Self = Self(4i32);
    pub const VeryLarge5M: Self = Self(5i32);
}
impl ::core::marker::Copy for CameraCaptureUIMaxPhotoResolution {}
impl ::core::clone::Clone for CameraCaptureUIMaxPhotoResolution {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct CameraCaptureUIMaxVideoResolution(pub i32);
impl CameraCaptureUIMaxVideoResolution {
    pub const HighestAvailable: Self = Self(0i32);
    pub const LowDefinition: Self = Self(1i32);
    pub const StandardDefinition: Self = Self(2i32);
    pub const HighDefinition: Self = Self(3i32);
}
impl ::core::marker::Copy for CameraCaptureUIMaxVideoResolution {}
impl ::core::clone::Clone for CameraCaptureUIMaxVideoResolution {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct CameraCaptureUIMode(pub i32);
impl CameraCaptureUIMode {
    pub const PhotoOrVideo: Self = Self(0i32);
    pub const Photo: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for CameraCaptureUIMode {}
impl ::core::clone::Clone for CameraCaptureUIMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CameraCaptureUIPhotoCaptureSettings = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct CameraCaptureUIPhotoFormat(pub i32);
impl CameraCaptureUIPhotoFormat {
    pub const Jpeg: Self = Self(0i32);
    pub const Png: Self = Self(1i32);
    pub const JpegXR: Self = Self(2i32);
}
impl ::core::marker::Copy for CameraCaptureUIPhotoFormat {}
impl ::core::clone::Clone for CameraCaptureUIPhotoFormat {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CameraCaptureUIVideoCaptureSettings = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct CameraCaptureUIVideoFormat(pub i32);
impl CameraCaptureUIVideoFormat {
    pub const Mp4: Self = Self(0i32);
    pub const Wmv: Self = Self(1i32);
}
impl ::core::marker::Copy for CameraCaptureUIVideoFormat {}
impl ::core::clone::Clone for CameraCaptureUIVideoFormat {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CapturedFrame = *mut ::core::ffi::c_void;
pub type CapturedFrameControlValues = *mut ::core::ffi::c_void;
pub type CapturedPhoto = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct ForegroundActivationArgument(pub i32);
impl ForegroundActivationArgument {
    pub const SignInRequired: Self = Self(0i32);
    pub const MoreSettings: Self = Self(1i32);
}
impl ::core::marker::Copy for ForegroundActivationArgument {}
impl ::core::clone::Clone for ForegroundActivationArgument {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct GameBarCommand(pub i32);
impl GameBarCommand {
    pub const OpenGameBar: Self = Self(0i32);
    pub const RecordHistoricalBuffer: Self = Self(1i32);
    pub const ToggleStartStopRecord: Self = Self(2i32);
    pub const StartRecord: Self = Self(3i32);
    pub const StopRecord: Self = Self(4i32);
    pub const TakeScreenshot: Self = Self(5i32);
    pub const StartBroadcast: Self = Self(6i32);
    pub const StopBroadcast: Self = Self(7i32);
    pub const PauseBroadcast: Self = Self(8i32);
    pub const ResumeBroadcast: Self = Self(9i32);
    pub const ToggleStartStopBroadcast: Self = Self(10i32);
    pub const ToggleMicrophoneCapture: Self = Self(11i32);
    pub const ToggleCameraCapture: Self = Self(12i32);
    pub const ToggleRecordingIndicator: Self = Self(13i32);
}
impl ::core::marker::Copy for GameBarCommand {}
impl ::core::clone::Clone for GameBarCommand {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct GameBarCommandOrigin(pub i32);
impl GameBarCommandOrigin {
    pub const ShortcutKey: Self = Self(0i32);
    pub const Cortana: Self = Self(1i32);
    pub const AppCommand: Self = Self(2i32);
}
impl ::core::marker::Copy for GameBarCommandOrigin {}
impl ::core::clone::Clone for GameBarCommandOrigin {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GameBarServices = *mut ::core::ffi::c_void;
pub type GameBarServicesCommandEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct GameBarServicesDisplayMode(pub i32);
impl GameBarServicesDisplayMode {
    pub const Windowed: Self = Self(0i32);
    pub const FullScreenExclusive: Self = Self(1i32);
}
impl ::core::marker::Copy for GameBarServicesDisplayMode {}
impl ::core::clone::Clone for GameBarServicesDisplayMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GameBarServicesManager = *mut ::core::ffi::c_void;
pub type GameBarServicesManagerGameBarServicesCreatedEventArgs = *mut ::core::ffi::c_void;
pub type GameBarServicesTargetInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct GameBarTargetCapturePolicy(pub i32);
impl GameBarTargetCapturePolicy {
    pub const EnabledBySystem: Self = Self(0i32);
    pub const EnabledByUser: Self = Self(1i32);
    pub const NotEnabled: Self = Self(2i32);
    pub const ProhibitedBySystem: Self = Self(3i32);
    pub const ProhibitedByPublisher: Self = Self(4i32);
}
impl ::core::marker::Copy for GameBarTargetCapturePolicy {}
impl ::core::clone::Clone for GameBarTargetCapturePolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IAdvancedCapturedPhoto {
    pub base__: ::windows_sys::core::IInspectable,
    pub Frame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_Devices")]
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Devices::AdvancedPhotoMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    Mode: usize,
    pub Context: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAdvancedCapturedPhoto {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4034032267, data2: 45714, data3: 17553, data4: [157, 65, 153, 128, 122, 85, 11, 191] };
}
#[repr(C)]
pub struct IAdvancedCapturedPhoto2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FrameBoundsRelativeToReferencePhoto: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameBoundsRelativeToReferencePhoto: usize,
}
impl ::windows_sys::core::Interface for IAdvancedCapturedPhoto2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 416247000, data2: 53246, data3: 17112, data4: [129, 4, 1, 123, 179, 24, 244, 161] };
}
#[repr(C)]
pub struct IAdvancedPhotoCapture {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CaptureAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CaptureWithContextAsync: unsafe extern "system" fn(this: *mut *mut Self, context: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureWithContextAsync: usize,
    #[cfg(feature = "Foundation")]
    pub OptionalReferencePhotoCaptured: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OptionalReferencePhotoCaptured: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOptionalReferencePhotoCaptured: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOptionalReferencePhotoCaptured: usize,
    #[cfg(feature = "Foundation")]
    pub AllPhotosCaptured: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AllPhotosCaptured: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAllPhotosCaptured: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAllPhotosCaptured: usize,
    #[cfg(feature = "Foundation")]
    pub FinishAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FinishAsync: usize,
}
impl ::windows_sys::core::Interface for IAdvancedPhotoCapture {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2214570746, data2: 26215, data3: 17628, data4: [151, 60, 166, 188, 229, 150, 170, 15] };
}
#[repr(C)]
pub struct IAppBroadcastBackgroundService {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetPlugInState: unsafe extern "system" fn(this: *mut *mut Self, value: AppBroadcastPlugInState) -> ::windows_sys::core::HRESULT,
    pub PlugInState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastPlugInState) -> ::windows_sys::core::HRESULT,
    pub SetSignInInfo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SignInInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetStreamInfo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StreamInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AppId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BroadcastTitle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetViewerCount: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub ViewerCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub TerminateBroadcast: unsafe extern "system" fn(this: *mut *mut Self, reason: AppBroadcastTerminationReason, providerspecificreason: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub HeartbeatRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HeartbeatRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHeartbeatRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHeartbeatRequested: usize,
    pub TitleId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppBroadcastBackgroundService {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3134318378, data2: 64148, data3: 18169, data4: [149, 252, 215, 21, 17, 205, 167, 11] };
}
#[repr(C)]
pub struct IAppBroadcastBackgroundService2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetBroadcastTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BroadcastLanguage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetBroadcastLanguage: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BroadcastChannel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetBroadcastChannel: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BroadcastTitleChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BroadcastTitleChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBroadcastTitleChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBroadcastTitleChanged: usize,
    #[cfg(feature = "Foundation")]
    pub BroadcastLanguageChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BroadcastLanguageChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBroadcastLanguageChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBroadcastLanguageChanged: usize,
    #[cfg(feature = "Foundation")]
    pub BroadcastChannelChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BroadcastChannelChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBroadcastChannelChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBroadcastChannelChanged: usize,
}
impl ::windows_sys::core::Interface for IAppBroadcastBackgroundService2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4237085631, data2: 21833, data3: 19335, data4: [149, 159, 35, 202, 64, 31, 212, 115] };
}
#[repr(C)]
pub struct IAppBroadcastBackgroundServiceSignInInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub SignInState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastSignInState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetOAuthRequestUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetOAuthRequestUri: usize,
    #[cfg(feature = "Foundation")]
    pub OAuthRequestUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OAuthRequestUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetOAuthCallbackUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetOAuthCallbackUri: usize,
    #[cfg(feature = "Foundation")]
    pub OAuthCallbackUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OAuthCallbackUri: usize,
    #[cfg(feature = "Security_Authentication_Web")]
    pub AuthenticationResult: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))]
    AuthenticationResult: usize,
    pub SetUserName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub UserName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SignInStateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignInStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSignInStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSignInStateChanged: usize,
}
impl ::windows_sys::core::Interface for IAppBroadcastBackgroundServiceSignInInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1584616053, data2: 35016, data3: 20170, data4: [137, 186, 72, 37, 152, 93, 184, 128] };
}
#[repr(C)]
pub struct IAppBroadcastBackgroundServiceSignInInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub UserNameChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UserNameChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUserNameChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUserNameChanged: usize,
}
impl ::windows_sys::core::Interface for IAppBroadcastBackgroundServiceSignInInfo2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2432968796, data2: 25295, data3: 19004, data4: [167, 238, 174, 181, 7, 64, 70, 69] };
}
#[repr(C)]
pub struct IAppBroadcastBackgroundServiceStreamInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub StreamState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastStreamState) -> ::windows_sys::core::HRESULT,
    pub SetDesiredVideoEncodingBitrate: unsafe extern "system" fn(this: *mut *mut Self, value: u64) -> ::windows_sys::core::HRESULT,
    pub DesiredVideoEncodingBitrate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SetBandwidthTestBitrate: unsafe extern "system" fn(this: *mut *mut Self, value: u64) -> ::windows_sys::core::HRESULT,
    pub BandwidthTestBitrate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SetAudioCodec: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AudioCodec: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BroadcastStreamReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StreamStateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StreamStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStreamStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStreamStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub VideoEncodingResolutionChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VideoEncodingResolutionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVideoEncodingResolutionChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVideoEncodingResolutionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub VideoEncodingBitrateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VideoEncodingBitrateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVideoEncodingBitrateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVideoEncodingBitrateChanged: usize,
}
impl ::windows_sys::core::Interface for IAppBroadcastBackgroundServiceStreamInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 836502204, data2: 39178, data3: 18692, data4: [170, 150, 254, 54, 67, 129, 241, 54] };
}
#[repr(C)]
pub struct IAppBroadcastBackgroundServiceStreamInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReportProblemWithStream: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppBroadcastBackgroundServiceStreamInfo2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3172900717, data2: 38108, data3: 20430, data4: [149, 65, 169, 241, 41, 89, 99, 52] };
}
#[repr(C)]
pub struct IAppBroadcastCameraCaptureStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastCameraCaptureState) -> ::windows_sys::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppBroadcastCameraCaptureStateChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 506678480, data2: 47234, data3: 19336, data4: [134, 146, 5, 153, 154, 206, 183, 15] };
}
#[repr(C)]
pub struct IAppBroadcastGlobalSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsBroadcastEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsDisabledByPolicy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsGpuConstrained: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HasHardwareEncoder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsAudioCaptureEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsAudioCaptureEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsMicrophoneCaptureEnabledByDefault: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsMicrophoneCaptureEnabledByDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsEchoCancellationEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsEchoCancellationEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSystemAudioGain: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub SystemAudioGain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMicrophoneGain: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub MicrophoneGain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetIsCameraCaptureEnabledByDefault: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsCameraCaptureEnabledByDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSelectedCameraId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SelectedCameraId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCameraOverlayLocation: unsafe extern "system" fn(this: *mut *mut Self, value: AppBroadcastCameraOverlayLocation) -> ::windows_sys::core::HRESULT,
    pub CameraOverlayLocation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastCameraOverlayLocation) -> ::windows_sys::core::HRESULT,
    pub SetCameraOverlaySize: unsafe extern "system" fn(this: *mut *mut Self, value: AppBroadcastCameraOverlaySize) -> ::windows_sys::core::HRESULT,
    pub CameraOverlaySize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastCameraOverlaySize) -> ::windows_sys::core::HRESULT,
    pub SetIsCursorImageCaptureEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsCursorImageCaptureEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppBroadcastGlobalSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2999658405, data2: 28924, data3: 19991, data4: [128, 189, 107, 160, 253, 63, 243, 160] };
}
#[repr(C)]
pub struct IAppBroadcastHeartbeatRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppBroadcastHeartbeatRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3466936963, data2: 61009, data3: 19903, data4: [148, 114, 121, 169, 237, 78, 33, 101] };
}
#[repr(C)]
pub struct IAppBroadcastManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetGlobalSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ApplyGlobalSettings: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetProviderSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ApplyProviderSettings: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppBroadcastManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 911081867, data2: 7758, data3: 16671, data4: [171, 62, 146, 149, 152, 68, 193, 86] };
}
#[repr(C)]
pub struct IAppBroadcastMicrophoneCaptureStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastMicrophoneCaptureState) -> ::windows_sys::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppBroadcastMicrophoneCaptureStateChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2825573865, data2: 37952, data3: 18696, data4: [157, 9, 101, 183, 227, 21, 215, 149] };
}
#[repr(C)]
pub struct IAppBroadcastPlugIn {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ProviderSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Logo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Logo: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppBroadcastPlugIn {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1376525926, data2: 25875, data3: 17780, data4: [172, 84, 35, 183, 151, 41, 97, 91] };
}
#[repr(C)]
pub struct IAppBroadcastPlugInManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsBroadcastProviderAvailable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PlugInList: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PlugInList: usize,
    pub DefaultPlugIn: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDefaultPlugIn: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppBroadcastPlugInManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3847281017, data2: 10145, data3: 18855, data4: [187, 244, 215, 169, 233, 208, 118, 104] };
}
#[repr(C)]
pub struct IAppBroadcastPlugInManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
impl ::windows_sys::core::Interface for IAppBroadcastPlugInManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4066663456, data2: 23670, data3: 19676, data4: [147, 100, 130, 254, 158, 182, 83, 77] };
}
#[repr(C)]
pub struct IAppBroadcastPlugInStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub PlugInState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastPlugInState) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppBroadcastPlugInStateChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1216467186, data2: 43973, data3: 20422, data4: [132, 176, 137, 55, 11, 180, 114, 18] };
}
#[repr(C)]
pub struct IAppBroadcastPreview {
    pub base__: ::windows_sys::core::IInspectable,
    pub StopPreview: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub PreviewState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastPreviewState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ErrorCode: usize,
    #[cfg(feature = "Foundation")]
    pub PreviewStateChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PreviewStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePreviewStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePreviewStateChanged: usize,
    pub PreviewStreamReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppBroadcastPreview {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 347475802, data2: 28234, data3: 19328, data4: [161, 79, 103, 238, 119, 209, 83, 231] };
}
#[repr(C)]
pub struct IAppBroadcastPreviewStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub PreviewState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastPreviewState) -> ::windows_sys::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppBroadcastPreviewStateChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1515713246, data2: 36330, data3: 20102, data4: [144, 173, 3, 252, 38, 185, 101, 60] };
}
#[repr(C)]
pub struct IAppBroadcastPreviewStreamReader {
    pub base__: ::windows_sys::core::IInspectable,
    pub VideoWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub VideoHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub VideoStride: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub VideoBitmapPixelFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    VideoBitmapPixelFormat: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub VideoBitmapAlphaMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::Imaging::BitmapAlphaMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    VideoBitmapAlphaMode: usize,
    pub TryGetNextVideoFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub VideoFrameArrived: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VideoFrameArrived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVideoFrameArrived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVideoFrameArrived: usize,
}
impl ::windows_sys::core::Interface for IAppBroadcastPreviewStreamReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2451737936, data2: 56127, data3: 16552, data4: [140, 212, 244, 227, 113, 221, 171, 55] };
}
#[repr(C)]
pub struct IAppBroadcastPreviewStreamVideoFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub VideoHeader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub VideoBuffer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    VideoBuffer: usize,
}
impl ::windows_sys::core::Interface for IAppBroadcastPreviewStreamVideoFrame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 17809057, data2: 38142, data3: 17561, data4: [184, 192, 141, 36, 66, 121, 251, 18] };
}
#[repr(C)]
pub struct IAppBroadcastPreviewStreamVideoHeader {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AbsoluteTimestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AbsoluteTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub RelativeTimestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RelativeTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub FrameId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppBroadcastPreviewStreamVideoHeader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2347720979, data2: 55940, data3: 17561, data4: [167, 171, 135, 17, 140, 180, 161, 87] };
}
#[repr(C)]
pub struct IAppBroadcastProviderSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetDefaultBroadcastTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DefaultBroadcastTitle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAudioEncodingBitrate: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub AudioEncodingBitrate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetCustomVideoEncodingBitrate: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub CustomVideoEncodingBitrate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetCustomVideoEncodingHeight: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub CustomVideoEncodingHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetCustomVideoEncodingWidth: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub CustomVideoEncodingWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetVideoEncodingBitrateMode: unsafe extern "system" fn(this: *mut *mut Self, value: AppBroadcastVideoEncodingBitrateMode) -> ::windows_sys::core::HRESULT,
    pub VideoEncodingBitrateMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastVideoEncodingBitrateMode) -> ::windows_sys::core::HRESULT,
    pub SetVideoEncodingResolutionMode: unsafe extern "system" fn(this: *mut *mut Self, value: AppBroadcastVideoEncodingResolutionMode) -> ::windows_sys::core::HRESULT,
    pub VideoEncodingResolutionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastVideoEncodingResolutionMode) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppBroadcastProviderSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3272335202, data2: 39240, data3: 17807, data4: [173, 80, 170, 6, 236, 3, 218, 8] };
}
#[repr(C)]
pub struct IAppBroadcastServices {
    pub base__: ::windows_sys::core::IInspectable,
    pub CaptureTargetType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastCaptureTargetType) -> ::windows_sys::core::HRESULT,
    pub SetCaptureTargetType: unsafe extern "system" fn(this: *mut *mut Self, value: AppBroadcastCaptureTargetType) -> ::windows_sys::core::HRESULT,
    pub BroadcastTitle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetBroadcastTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BroadcastLanguage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetBroadcastLanguage: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub UserName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CanCapture: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnterBroadcastModeAsync: unsafe extern "system" fn(this: *mut *mut Self, plugin: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnterBroadcastModeAsync: usize,
    pub ExitBroadcastMode: unsafe extern "system" fn(this: *mut *mut Self, reason: AppBroadcastExitBroadcastModeReason) -> ::windows_sys::core::HRESULT,
    pub StartBroadcast: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub PauseBroadcast: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ResumeBroadcast: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartPreview: unsafe extern "system" fn(this: *mut *mut Self, desiredsize: super::super::Foundation::Size, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartPreview: usize,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppBroadcastServices {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2254484694, data2: 38555, data3: 20028, data4: [172, 58, 139, 4, 46, 228, 238, 99] };
}
#[repr(C)]
pub struct IAppBroadcastSignInStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SignInState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastSignInState) -> ::windows_sys::core::HRESULT,
    pub Result: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastSignInResult) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppBroadcastSignInStateChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 45519524, data2: 22809, data3: 19102, data4: [141, 94, 201, 187, 13, 211, 55, 122] };
}
#[repr(C)]
pub struct IAppBroadcastState {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsCaptureTargetRunning: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ViewerCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ShouldCaptureMicrophone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShouldCaptureMicrophone: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub RestartMicrophoneCapture: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ShouldCaptureCamera: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShouldCaptureCamera: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub RestartCameraCapture: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EncodedVideoSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EncodedVideoSize: usize,
    pub MicrophoneCaptureState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastMicrophoneCaptureState) -> ::windows_sys::core::HRESULT,
    pub MicrophoneCaptureError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CameraCaptureState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastCameraCaptureState) -> ::windows_sys::core::HRESULT,
    pub CameraCaptureError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub StreamState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastStreamState) -> ::windows_sys::core::HRESULT,
    pub PlugInState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastPlugInState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OAuthRequestUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OAuthRequestUri: usize,
    #[cfg(feature = "Foundation")]
    pub OAuthCallbackUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OAuthCallbackUri: usize,
    #[cfg(feature = "Security_Authentication_Web")]
    pub AuthenticationResult: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))]
    AuthenticationResult: usize,
    #[cfg(feature = "Security_Authentication_Web")]
    pub SetAuthenticationResult: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))]
    SetAuthenticationResult: usize,
    pub SetSignInState: unsafe extern "system" fn(this: *mut *mut Self, value: AppBroadcastSignInState) -> ::windows_sys::core::HRESULT,
    pub SignInState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastSignInState) -> ::windows_sys::core::HRESULT,
    pub TerminationReason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastTerminationReason) -> ::windows_sys::core::HRESULT,
    pub TerminationReasonPlugInSpecific: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ViewerCountChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ViewerCountChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveViewerCountChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveViewerCountChanged: usize,
    #[cfg(feature = "Foundation")]
    pub MicrophoneCaptureStateChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MicrophoneCaptureStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMicrophoneCaptureStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMicrophoneCaptureStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub CameraCaptureStateChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CameraCaptureStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCameraCaptureStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCameraCaptureStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PlugInStateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlugInStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlugInStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlugInStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub StreamStateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StreamStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStreamStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStreamStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub CaptureTargetClosed: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureTargetClosed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCaptureTargetClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCaptureTargetClosed: usize,
}
impl ::windows_sys::core::Interface for IAppBroadcastState {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3993503085, data2: 32921, data3: 19933, data4: [146, 46, 197, 109, 172, 88, 171, 251] };
}
#[repr(C)]
pub struct IAppBroadcastStreamAudioFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub AudioHeader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub AudioBuffer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AudioBuffer: usize,
}
impl ::windows_sys::core::Interface for IAppBroadcastStreamAudioFrame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4020980424, data2: 8634, data3: 17727, data4: [139, 183, 94, 147, 138, 46, 154, 116] };
}
#[repr(C)]
pub struct IAppBroadcastStreamAudioHeader {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AbsoluteTimestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AbsoluteTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub RelativeTimestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RelativeTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub HasDiscontinuity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub FrameId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppBroadcastStreamAudioHeader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3206653296, data2: 27512, data3: 16918, data4: [159, 7, 90, 255, 82, 86, 241, 183] };
}
#[repr(C)]
pub struct IAppBroadcastStreamReader {
    pub base__: ::windows_sys::core::IInspectable,
    pub AudioChannels: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub AudioSampleRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub AudioAacSequence: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AudioAacSequence: usize,
    pub AudioBitrate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub TryGetNextAudioFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VideoWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub VideoHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub VideoBitrate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub TryGetNextVideoFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AudioFrameArrived: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioFrameArrived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioFrameArrived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioFrameArrived: usize,
    #[cfg(feature = "Foundation")]
    pub VideoFrameArrived: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VideoFrameArrived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVideoFrameArrived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVideoFrameArrived: usize,
}
impl ::windows_sys::core::Interface for IAppBroadcastStreamReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3006840057, data2: 13156, data3: 17504, data4: [181, 241, 60, 194, 121, 106, 138, 162] };
}
#[repr(C)]
pub struct IAppBroadcastStreamStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub StreamState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastStreamState) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppBroadcastStreamStateChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1359521587, data2: 53256, data3: 19081, data4: [147, 190, 88, 174, 217, 97, 55, 78] };
}
#[repr(C)]
pub struct IAppBroadcastStreamVideoFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub VideoHeader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub VideoBuffer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    VideoBuffer: usize,
}
impl ::windows_sys::core::Interface for IAppBroadcastStreamVideoFrame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 261607211, data2: 51684, data3: 20104, data4: [129, 148, 216, 20, 203, 213, 133, 216] };
}
#[repr(C)]
pub struct IAppBroadcastStreamVideoHeader {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AbsoluteTimestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AbsoluteTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub RelativeTimestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RelativeTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub IsKeyFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HasDiscontinuity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub FrameId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppBroadcastStreamVideoHeader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 194952910, data2: 32306, data3: 17197, data4: [140, 162, 54, 191, 16, 185, 244, 98] };
}
#[repr(C)]
pub struct IAppBroadcastTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub BackgroundService: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppBroadcastTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3739986741, data2: 60510, data3: 19855, data4: [177, 192, 93, 166, 232, 199, 86, 56] };
}
#[repr(C)]
pub struct IAppBroadcastViewerCountChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ViewerCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppBroadcastViewerCountChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3873511461, data2: 21505, data3: 19166, data4: [139, 210, 193, 78, 206, 230, 128, 125] };
}
#[repr(C)]
pub struct IAppCapture {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsCapturingAudio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsCapturingVideo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CapturingChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CapturingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCapturingChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCapturingChanged: usize,
}
impl ::windows_sys::core::Interface for IAppCapture {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2538198099, data2: 41626, data3: 17901, data4: [143, 41, 34, 208, 153, 66, 207, 247] };
}
#[repr(C)]
pub struct IAppCaptureAlternateShortcutKeys {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub SetToggleGameBarKey: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleGameBarKey: usize,
    #[cfg(feature = "System")]
    pub ToggleGameBarKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleGameBarKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleGameBarKeyModifiers: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::System::VirtualKeyModifiers) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleGameBarKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleGameBarKeyModifiers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleGameBarKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SetSaveHistoricalVideoKey: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetSaveHistoricalVideoKey: usize,
    #[cfg(feature = "System")]
    pub SaveHistoricalVideoKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SaveHistoricalVideoKey: usize,
    #[cfg(feature = "System")]
    pub SetSaveHistoricalVideoKeyModifiers: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::System::VirtualKeyModifiers) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetSaveHistoricalVideoKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SaveHistoricalVideoKeyModifiers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SaveHistoricalVideoKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SetToggleRecordingKey: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleRecordingKey: usize,
    #[cfg(feature = "System")]
    pub ToggleRecordingKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleRecordingKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleRecordingKeyModifiers: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::System::VirtualKeyModifiers) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleRecordingKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleRecordingKeyModifiers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleRecordingKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SetTakeScreenshotKey: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetTakeScreenshotKey: usize,
    #[cfg(feature = "System")]
    pub TakeScreenshotKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    TakeScreenshotKey: usize,
    #[cfg(feature = "System")]
    pub SetTakeScreenshotKeyModifiers: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::System::VirtualKeyModifiers) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetTakeScreenshotKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub TakeScreenshotKeyModifiers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    TakeScreenshotKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SetToggleRecordingIndicatorKey: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleRecordingIndicatorKey: usize,
    #[cfg(feature = "System")]
    pub ToggleRecordingIndicatorKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleRecordingIndicatorKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleRecordingIndicatorKeyModifiers: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::System::VirtualKeyModifiers) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleRecordingIndicatorKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleRecordingIndicatorKeyModifiers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleRecordingIndicatorKeyModifiers: usize,
}
impl ::windows_sys::core::Interface for IAppCaptureAlternateShortcutKeys {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 434692335, data2: 9068, data3: 16633, data4: [179, 143, 155, 125, 214, 93, 28, 204] };
}
#[repr(C)]
pub struct IAppCaptureAlternateShortcutKeys2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub SetToggleMicrophoneCaptureKey: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleMicrophoneCaptureKey: usize,
    #[cfg(feature = "System")]
    pub ToggleMicrophoneCaptureKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleMicrophoneCaptureKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleMicrophoneCaptureKeyModifiers: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::System::VirtualKeyModifiers) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleMicrophoneCaptureKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleMicrophoneCaptureKeyModifiers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleMicrophoneCaptureKeyModifiers: usize,
}
impl ::windows_sys::core::Interface for IAppCaptureAlternateShortcutKeys2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3278278800, data2: 56599, data3: 18416, data4: [149, 229, 206, 66, 40, 108, 243, 56] };
}
#[repr(C)]
pub struct IAppCaptureAlternateShortcutKeys3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub SetToggleCameraCaptureKey: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleCameraCaptureKey: usize,
    #[cfg(feature = "System")]
    pub ToggleCameraCaptureKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleCameraCaptureKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleCameraCaptureKeyModifiers: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::System::VirtualKeyModifiers) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleCameraCaptureKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleCameraCaptureKeyModifiers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleCameraCaptureKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SetToggleBroadcastKey: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleBroadcastKey: usize,
    #[cfg(feature = "System")]
    pub ToggleBroadcastKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::System::VirtualKey) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleBroadcastKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleBroadcastKeyModifiers: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::System::VirtualKeyModifiers) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleBroadcastKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleBroadcastKeyModifiers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleBroadcastKeyModifiers: usize,
}
impl ::windows_sys::core::Interface for IAppCaptureAlternateShortcutKeys3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2072069260, data2: 16782, data3: 18076, data4: [164, 154, 69, 181, 151, 200, 38, 182] };
}
#[repr(C)]
pub struct IAppCaptureDurationGeneratedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
}
impl ::windows_sys::core::Interface for IAppCaptureDurationGeneratedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3254081083, data2: 65441, data3: 17609, data4: [151, 95, 39, 251, 235, 85, 59, 53] };
}
#[repr(C)]
pub struct IAppCaptureFileGeneratedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
}
impl ::windows_sys::core::Interface for IAppCaptureFileGeneratedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1099561972, data2: 18014, data3: 17855, data4: [144, 127, 22, 91, 63, 178, 55, 88] };
}
#[repr(C)]
pub struct IAppCaptureManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetCurrentSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ApplySettings: unsafe extern "system" fn(this: *mut *mut Self, appcapturesettings: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppCaptureManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2107522727, data2: 25218, data3: 18229, data4: [141, 78, 170, 69, 249, 15, 103, 35] };
}
#[repr(C)]
pub struct IAppCaptureMetadataWriter {
    pub base__: ::windows_sys::core::IInspectable,
    pub AddStringEvent: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: ::windows_sys::core::HSTRING, priority: AppCaptureMetadataPriority) -> ::windows_sys::core::HRESULT,
    pub AddInt32Event: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: i32, priority: AppCaptureMetadataPriority) -> ::windows_sys::core::HRESULT,
    pub AddDoubleEvent: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: f64, priority: AppCaptureMetadataPriority) -> ::windows_sys::core::HRESULT,
    pub StartStringState: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: ::windows_sys::core::HSTRING, priority: AppCaptureMetadataPriority) -> ::windows_sys::core::HRESULT,
    pub StartInt32State: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: i32, priority: AppCaptureMetadataPriority) -> ::windows_sys::core::HRESULT,
    pub StartDoubleState: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: f64, priority: AppCaptureMetadataPriority) -> ::windows_sys::core::HRESULT,
    pub StopState: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub StopAllStates: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RemainingStorageBytesAvailable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MetadataPurged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MetadataPurged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMetadataPurged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMetadataPurged: usize,
}
impl ::windows_sys::core::Interface for IAppCaptureMetadataWriter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3771615351, data2: 39599, data3: 18100, data4: [173, 49, 106, 96, 180, 65, 199, 128] };
}
#[repr(C)]
pub struct IAppCaptureMicrophoneCaptureStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppCaptureMicrophoneCaptureState) -> ::windows_sys::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppCaptureMicrophoneCaptureStateChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 843916446, data2: 17852, data3: 19509, data4: [188, 53, 228, 105, 252, 122, 105, 224] };
}
#[repr(C)]
pub struct IAppCaptureRecordOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub StopRecording: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppCaptureRecordingState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ErrorCode: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
    #[cfg(feature = "Foundation")]
    pub IsFileTruncated: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsFileTruncated: usize,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub DurationGenerated: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DurationGenerated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDurationGenerated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDurationGenerated: usize,
    #[cfg(feature = "Foundation")]
    pub FileGenerated: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FileGenerated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFileGenerated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFileGenerated: usize,
}
impl ::windows_sys::core::Interface for IAppCaptureRecordOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3328188585, data2: 5432, data3: 18780, data4: [155, 187, 43, 168, 112, 236, 88, 97] };
}
#[repr(C)]
pub struct IAppCaptureRecordingStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppCaptureRecordingState) -> ::windows_sys::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppCaptureRecordingStateChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 620529426, data2: 58117, data3: 18701, data4: [180, 21, 107, 28, 144, 73, 115, 107] };
}
#[repr(C)]
pub struct IAppCaptureServices {
    pub base__: ::windows_sys::core::IInspectable,
    pub Record: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RecordTimeSpan: unsafe extern "system" fn(this: *mut *mut Self, starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecordTimeSpan: usize,
    pub CanCapture: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppCaptureServices {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1157546165, data2: 13557, data3: 20248, data4: [174, 140, 185, 18, 58, 187, 252, 13] };
}
#[repr(C)]
pub struct IAppCaptureSettings {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage")]
    pub SetAppCaptureDestinationFolder: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SetAppCaptureDestinationFolder: usize,
    #[cfg(feature = "Storage")]
    pub AppCaptureDestinationFolder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    AppCaptureDestinationFolder: usize,
    pub SetAudioEncodingBitrate: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub AudioEncodingBitrate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetIsAudioCaptureEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsAudioCaptureEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCustomVideoEncodingBitrate: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub CustomVideoEncodingBitrate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetCustomVideoEncodingHeight: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub CustomVideoEncodingHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetCustomVideoEncodingWidth: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub CustomVideoEncodingWidth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetHistoricalBufferLength: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub HistoricalBufferLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetHistoricalBufferLengthUnit: unsafe extern "system" fn(this: *mut *mut Self, value: AppCaptureHistoricalBufferLengthUnit) -> ::windows_sys::core::HRESULT,
    pub HistoricalBufferLengthUnit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppCaptureHistoricalBufferLengthUnit) -> ::windows_sys::core::HRESULT,
    pub SetIsHistoricalCaptureEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsHistoricalCaptureEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsHistoricalCaptureOnBatteryAllowed: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsHistoricalCaptureOnBatteryAllowed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsHistoricalCaptureOnWirelessDisplayAllowed: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsHistoricalCaptureOnWirelessDisplayAllowed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetMaximumRecordLength: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaximumRecordLength: usize,
    #[cfg(feature = "Foundation")]
    pub MaximumRecordLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaximumRecordLength: usize,
    #[cfg(feature = "Storage")]
    pub SetScreenshotDestinationFolder: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SetScreenshotDestinationFolder: usize,
    #[cfg(feature = "Storage")]
    pub ScreenshotDestinationFolder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    ScreenshotDestinationFolder: usize,
    pub SetVideoEncodingBitrateMode: unsafe extern "system" fn(this: *mut *mut Self, value: AppCaptureVideoEncodingBitrateMode) -> ::windows_sys::core::HRESULT,
    pub VideoEncodingBitrateMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppCaptureVideoEncodingBitrateMode) -> ::windows_sys::core::HRESULT,
    pub SetVideoEncodingResolutionMode: unsafe extern "system" fn(this: *mut *mut Self, value: AppCaptureVideoEncodingResolutionMode) -> ::windows_sys::core::HRESULT,
    pub VideoEncodingResolutionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppCaptureVideoEncodingResolutionMode) -> ::windows_sys::core::HRESULT,
    pub SetIsAppCaptureEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsAppCaptureEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsCpuConstrained: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsDisabledByPolicy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsMemoryConstrained: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HasHardwareEncoder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppCaptureSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 342375046, data2: 34823, data3: 18643, data4: [136, 58, 151, 14, 228, 83, 42, 57] };
}
#[repr(C)]
pub struct IAppCaptureSettings2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsGpuConstrained: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AlternateShortcutKeys: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppCaptureSettings2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4239970023, data2: 57963, data3: 18287, data4: [155, 26, 236, 52, 45, 42, 143, 222] };
}
#[repr(C)]
pub struct IAppCaptureSettings3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetIsMicrophoneCaptureEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsMicrophoneCaptureEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppCaptureSettings3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2838823678, data2: 35010, data3: 17110, data4: [170, 170, 64, 254, 255, 215, 90, 236] };
}
#[repr(C)]
pub struct IAppCaptureSettings4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetIsMicrophoneCaptureEnabledByDefault: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsMicrophoneCaptureEnabledByDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSystemAudioGain: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub SystemAudioGain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMicrophoneGain: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub MicrophoneGain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetVideoEncodingFrameRateMode: unsafe extern "system" fn(this: *mut *mut Self, value: AppCaptureVideoEncodingFrameRateMode) -> ::windows_sys::core::HRESULT,
    pub VideoEncodingFrameRateMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppCaptureVideoEncodingFrameRateMode) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppCaptureSettings4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 130185036, data2: 6785, data3: 18479, data4: [162, 68, 4, 157, 149, 242, 91, 11] };
}
#[repr(C)]
pub struct IAppCaptureSettings5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetIsEchoCancellationEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsEchoCancellationEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsCursorImageCaptureEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsCursorImageCaptureEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppCaptureSettings5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 411649314, data2: 45288, data3: 19360, data4: [143, 19, 62, 170, 95, 164, 1, 59] };
}
#[repr(C)]
pub struct IAppCaptureState {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTargetRunning: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsHistoricalCaptureEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ShouldCaptureMicrophone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShouldCaptureMicrophone: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub RestartMicrophoneCapture: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub MicrophoneCaptureState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppCaptureMicrophoneCaptureState) -> ::windows_sys::core::HRESULT,
    pub MicrophoneCaptureError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MicrophoneCaptureStateChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MicrophoneCaptureStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMicrophoneCaptureStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMicrophoneCaptureStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub CaptureTargetClosed: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureTargetClosed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCaptureTargetClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCaptureTargetClosed: usize,
}
impl ::windows_sys::core::Interface for IAppCaptureState {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1930642290, data2: 54507, data3: 17614, data4: [149, 56, 70, 95, 80, 106, 196, 234] };
}
#[repr(C)]
pub struct IAppCaptureStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppCaptureStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4179811692, data2: 2686, data3: 20084, data4: [139, 32, 156, 31, 144, 45, 8, 161] };
}
#[repr(C)]
pub struct IAppCaptureStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetAllowedAsync: unsafe extern "system" fn(this: *mut *mut Self, allowed: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAllowedAsync: usize,
}
impl ::windows_sys::core::Interface for IAppCaptureStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3000533460, data2: 33644, data3: 19876, data4: [175, 215, 250, 204, 4, 30, 28, 243] };
}
#[repr(C)]
pub struct ICameraCaptureUI {
    pub base__: ::windows_sys::core::IInspectable,
    pub PhotoSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VideoSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CaptureFileAsync: unsafe extern "system" fn(this: *mut *mut Self, mode: CameraCaptureUIMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CaptureFileAsync: usize,
}
impl ::windows_sys::core::Interface for ICameraCaptureUI {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1213756736, data2: 28563, data3: 19380, data4: [184, 243, 232, 158, 72, 148, 140, 145] };
}
#[repr(C)]
pub struct ICameraCaptureUIPhotoCaptureSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub Format: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CameraCaptureUIPhotoFormat) -> ::windows_sys::core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(this: *mut *mut Self, value: CameraCaptureUIPhotoFormat) -> ::windows_sys::core::HRESULT,
    pub MaxResolution: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CameraCaptureUIMaxPhotoResolution) -> ::windows_sys::core::HRESULT,
    pub SetMaxResolution: unsafe extern "system" fn(this: *mut *mut Self, value: CameraCaptureUIMaxPhotoResolution) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CroppedSizeInPixels: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CroppedSizeInPixels: usize,
    #[cfg(feature = "Foundation")]
    pub SetCroppedSizeInPixels: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCroppedSizeInPixels: usize,
    #[cfg(feature = "Foundation")]
    pub CroppedAspectRatio: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CroppedAspectRatio: usize,
    #[cfg(feature = "Foundation")]
    pub SetCroppedAspectRatio: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCroppedAspectRatio: usize,
    pub AllowCropping: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowCropping: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICameraCaptureUIPhotoCaptureSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3119890071, data2: 13426, data3: 18088, data4: [138, 158, 4, 206, 66, 204, 201, 125] };
}
#[repr(C)]
pub struct ICameraCaptureUIVideoCaptureSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub Format: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CameraCaptureUIVideoFormat) -> ::windows_sys::core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(this: *mut *mut Self, value: CameraCaptureUIVideoFormat) -> ::windows_sys::core::HRESULT,
    pub MaxResolution: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CameraCaptureUIMaxVideoResolution) -> ::windows_sys::core::HRESULT,
    pub SetMaxResolution: unsafe extern "system" fn(this: *mut *mut Self, value: CameraCaptureUIMaxVideoResolution) -> ::windows_sys::core::HRESULT,
    pub MaxDurationInSeconds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetMaxDurationInSeconds: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub AllowTrimming: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowTrimming: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICameraCaptureUIVideoCaptureSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1693003039, data2: 41613, data3: 16986, data4: [184, 79, 229, 104, 51, 95, 242, 78] };
}
#[repr(C)]
pub struct ICameraOptionsUIStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Show: unsafe extern "system" fn(this: *mut *mut Self, mediacapture: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICameraOptionsUIStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 990731828, data2: 14598, data3: 19325, data4: [148, 108, 123, 222, 132, 68, 153, 174] };
}
#[repr(C)]
pub struct ICapturedFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICapturedFrame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 500358687, data2: 22299, data3: 17624, data4: [142, 128, 160, 138, 21, 120, 118, 110] };
}
#[repr(C)]
pub struct ICapturedFrame2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ControlValues: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub BitmapProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    BitmapProperties: usize,
}
impl ::windows_sys::core::Interface for ICapturedFrame2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1413457617, data2: 48504, data3: 18534, data4: [173, 218, 36, 49, 75, 198, 93, 234] };
}
#[repr(C)]
pub struct ICapturedFrameControlValues {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Exposure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Exposure: usize,
    #[cfg(feature = "Foundation")]
    pub ExposureCompensation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExposureCompensation: usize,
    #[cfg(feature = "Foundation")]
    pub IsoSpeed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsoSpeed: usize,
    #[cfg(feature = "Foundation")]
    pub Focus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Focus: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub SceneMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))]
    SceneMode: usize,
    #[cfg(feature = "Foundation")]
    pub Flashed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Flashed: usize,
    #[cfg(feature = "Foundation")]
    pub FlashPowerPercent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FlashPowerPercent: usize,
    #[cfg(feature = "Foundation")]
    pub WhiteBalance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WhiteBalance: usize,
    #[cfg(feature = "Foundation")]
    pub ZoomFactor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ZoomFactor: usize,
}
impl ::windows_sys::core::Interface for ICapturedFrameControlValues {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2428918655, data2: 19981, data3: 19620, data4: [136, 45, 122, 20, 79, 237, 10, 144] };
}
#[repr(C)]
pub struct ICapturedFrameControlValues2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub FocusState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))]
    FocusState: usize,
    #[cfg(feature = "Foundation")]
    pub IsoDigitalGain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsoDigitalGain: usize,
    #[cfg(feature = "Foundation")]
    pub IsoAnalogGain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsoAnalogGain: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SensorFrameRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SensorFrameRate: usize,
    #[cfg(feature = "Foundation")]
    pub WhiteBalanceGain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WhiteBalanceGain: usize,
}
impl ::windows_sys::core::Interface for ICapturedFrameControlValues2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1342909320, data2: 1746, data3: 19111, data4: [167, 219, 211, 122, 247, 51, 33, 216] };
}
#[repr(C)]
pub struct ICapturedFrameWithSoftwareBitmap {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_Imaging")]
    pub SoftwareBitmap: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SoftwareBitmap: usize,
}
impl ::windows_sys::core::Interface for ICapturedFrameWithSoftwareBitmap {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3046017902, data2: 34051, data3: 18869, data4: [158, 134, 137, 125, 38, 163, 255, 61] };
}
#[repr(C)]
pub struct ICapturedPhoto {
    pub base__: ::windows_sys::core::IInspectable,
    pub Frame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Thumbnail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICapturedPhoto {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2966322778, data2: 53196, data3: 19820, data4: [138, 209, 8, 105, 32, 138, 202, 22] };
}
#[repr(C)]
pub struct IGameBarServices {
    pub base__: ::windows_sys::core::IInspectable,
    pub TargetCapturePolicy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GameBarTargetCapturePolicy) -> ::windows_sys::core::HRESULT,
    pub EnableCapture: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub DisableCapture: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub TargetInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SessionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AppBroadcastServices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AppCaptureServices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CommandReceived: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CommandReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCommandReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCommandReceived: usize,
}
impl ::windows_sys::core::Interface for IGameBarServices {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 767470935, data2: 20646, data3: 18846, data4: [140, 108, 211, 48, 167, 49, 23, 150] };
}
#[repr(C)]
pub struct IGameBarServicesCommandEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Command: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GameBarCommand) -> ::windows_sys::core::HRESULT,
    pub Origin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GameBarCommandOrigin) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGameBarServicesCommandEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2806130354, data2: 61814, data3: 20431, data4: [143, 187, 207, 105, 139, 46, 184, 224] };
}
#[repr(C)]
pub struct IGameBarServicesManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GameBarServicesCreated: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GameBarServicesCreated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGameBarServicesCreated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGameBarServicesCreated: usize,
}
impl ::windows_sys::core::Interface for IGameBarServicesManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 978033914, data2: 32651, data3: 19552, data4: [157, 187, 11, 205, 38, 45, 255, 198] };
}
#[repr(C)]
pub struct IGameBarServicesManagerGameBarServicesCreatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub GameBarServices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGameBarServicesManagerGameBarServicesCreatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3991780764, data2: 5182, data3: 18851, data4: [165, 234, 11, 25, 149, 200, 212, 110] };
}
#[repr(C)]
pub struct IGameBarServicesManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGameBarServicesManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 885110294, data2: 65317, data3: 18322, data4: [152, 242, 211, 117, 63, 21, 172, 19] };
}
#[repr(C)]
pub struct IGameBarServicesTargetInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AppId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TitleId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GameBarServicesDisplayMode) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGameBarServicesTargetInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3022008210, data2: 5649, data3: 19973, data4: [182, 239, 223, 215, 55, 174, 51, 176] };
}
#[repr(C)]
pub struct ILowLagMediaRecording {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FinishAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FinishAsync: usize,
}
impl ::windows_sys::core::Interface for ILowLagMediaRecording {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1103674103, data2: 65343, data3: 18928, data4: [164, 119, 241, 149, 227, 206, 81, 8] };
}
#[repr(C)]
pub struct ILowLagMediaRecording2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub PauseAsync: unsafe extern "system" fn(this: *mut *mut Self, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))]
    PauseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ResumeAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResumeAsync: usize,
}
impl ::windows_sys::core::Interface for ILowLagMediaRecording2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1667876696, data2: 22084, data3: 16866, data4: [151, 175, 142, 245, 106, 37, 226, 37] };
}
#[repr(C)]
pub struct ILowLagMediaRecording3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub PauseWithResultAsync: unsafe extern "system" fn(this: *mut *mut Self, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))]
    PauseWithResultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopWithResultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopWithResultAsync: usize,
}
impl ::windows_sys::core::Interface for ILowLagMediaRecording3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1546890002, data2: 18679, data3: 18394, data4: [180, 30, 144, 136, 10, 95, 224, 236] };
}
#[repr(C)]
pub struct ILowLagPhotoCapture {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CaptureAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FinishAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FinishAsync: usize,
}
impl ::windows_sys::core::Interface for ILowLagPhotoCapture {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2742178231, data2: 27460, data3: 18237, data4: [143, 36, 247, 3, 214, 192, 236, 68] };
}
#[repr(C)]
pub struct ILowLagPhotoSequenceCapture {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FinishAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FinishAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PhotoCaptured: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhotoCaptured: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePhotoCaptured: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePhotoCaptured: usize,
}
impl ::windows_sys::core::Interface for ILowLagPhotoSequenceCapture {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2093172411, data2: 47529, data3: 19601, data4: [143, 250, 40, 126, 156, 102, 134, 105] };
}
#[repr(C)]
pub struct IMediaCapture {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub InitializeAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InitializeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub InitializeWithSettingsAsync: unsafe extern "system" fn(this: *mut *mut Self, mediacaptureinitializationsettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InitializeWithSettingsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub StartRecordToStorageFileAsync: unsafe extern "system" fn(this: *mut *mut Self, encodingprofile: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage")))]
    StartRecordToStorageFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub StartRecordToStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, encodingprofile: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams")))]
    StartRecordToStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub StartRecordToCustomSinkAsync: unsafe extern "system" fn(this: *mut *mut Self, encodingprofile: *mut ::core::ffi::c_void, custommediasink: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))]
    StartRecordToCustomSinkAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub StartRecordToCustomSinkIdAsync: unsafe extern "system" fn(this: *mut *mut Self, encodingprofile: *mut ::core::ffi::c_void, customsinkactivationid: ::windows_sys::core::HSTRING, customsinksettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    StartRecordToCustomSinkIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopRecordAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopRecordAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub CapturePhotoToStorageFileAsync: unsafe extern "system" fn(this: *mut *mut Self, r#type: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage")))]
    CapturePhotoToStorageFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub CapturePhotoToStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, r#type: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams")))]
    CapturePhotoToStreamAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub AddEffectAsync: unsafe extern "system" fn(this: *mut *mut Self, mediastreamtype: MediaStreamType, effectactivationid: ::windows_sys::core::HSTRING, effectsettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    AddEffectAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ClearEffectsAsync: unsafe extern "system" fn(this: *mut *mut Self, mediastreamtype: MediaStreamType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearEffectsAsync: usize,
    pub SetEncoderProperty: unsafe extern "system" fn(this: *mut *mut Self, mediastreamtype: MediaStreamType, propertyid: ::windows_sys::core::GUID, propertyvalue: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetEncoderProperty: unsafe extern "system" fn(this: *mut *mut Self, mediastreamtype: MediaStreamType, propertyid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Failed: unsafe extern "system" fn(this: *mut *mut Self, erroreventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Failed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFailed: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RecordLimitationExceeded: unsafe extern "system" fn(this: *mut *mut Self, recordlimitationexceededeventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecordLimitationExceeded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRecordLimitationExceeded: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRecordLimitationExceeded: usize,
    pub MediaCaptureSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_Devices")]
    pub AudioDeviceController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    AudioDeviceController: usize,
    #[cfg(feature = "Media_Devices")]
    pub VideoDeviceController: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    VideoDeviceController: usize,
    pub SetPreviewMirroring: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub GetPreviewMirroring: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetPreviewRotation: unsafe extern "system" fn(this: *mut *mut Self, value: VideoRotation) -> ::windows_sys::core::HRESULT,
    pub GetPreviewRotation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VideoRotation) -> ::windows_sys::core::HRESULT,
    pub SetRecordRotation: unsafe extern "system" fn(this: *mut *mut Self, value: VideoRotation) -> ::windows_sys::core::HRESULT,
    pub GetRecordRotation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VideoRotation) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaCapture {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3323657140, data2: 64272, data3: 18996, data4: [172, 24, 202, 128, 217, 200, 231, 238] };
}
#[repr(C)]
pub struct IMediaCapture2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub PrepareLowLagRecordToStorageFileAsync: unsafe extern "system" fn(this: *mut *mut Self, encodingprofile: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage")))]
    PrepareLowLagRecordToStorageFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub PrepareLowLagRecordToStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, encodingprofile: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams")))]
    PrepareLowLagRecordToStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub PrepareLowLagRecordToCustomSinkAsync: unsafe extern "system" fn(this: *mut *mut Self, encodingprofile: *mut ::core::ffi::c_void, custommediasink: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))]
    PrepareLowLagRecordToCustomSinkAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub PrepareLowLagRecordToCustomSinkIdAsync: unsafe extern "system" fn(this: *mut *mut Self, encodingprofile: *mut ::core::ffi::c_void, customsinkactivationid: ::windows_sys::core::HSTRING, customsinksettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    PrepareLowLagRecordToCustomSinkIdAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub PrepareLowLagPhotoCaptureAsync: unsafe extern "system" fn(this: *mut *mut Self, r#type: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))]
    PrepareLowLagPhotoCaptureAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub PrepareLowLagPhotoSequenceCaptureAsync: unsafe extern "system" fn(this: *mut *mut Self, r#type: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))]
    PrepareLowLagPhotoSequenceCaptureAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub SetEncodingPropertiesAsync: unsafe extern "system" fn(this: *mut *mut Self, mediastreamtype: MediaStreamType, mediaencodingproperties: *mut ::core::ffi::c_void, encoderproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    SetEncodingPropertiesAsync: usize,
}
impl ::windows_sys::core::Interface for IMediaCapture2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2630255200, data2: 32161, data3: 16451, data4: [182, 82, 33, 184, 135, 141, 175, 249] };
}
#[repr(C)]
pub struct IMediaCapture3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Core", feature = "Media_MediaProperties"))]
    pub PrepareVariablePhotoSequenceCaptureAsync: unsafe extern "system" fn(this: *mut *mut Self, r#type: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture_Core", feature = "Media_MediaProperties")))]
    PrepareVariablePhotoSequenceCaptureAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FocusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FocusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFocusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFocusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PhotoConfirmationCaptured: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhotoConfirmationCaptured: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePhotoConfirmationCaptured: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePhotoConfirmationCaptured: usize,
}
impl ::windows_sys::core::Interface for IMediaCapture3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3558043440, data2: 5476, data3: 18030, data4: [188, 10, 175, 148, 224, 42, 176, 22] };
}
#[repr(C)]
pub struct IMediaCapture4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Media_Effects"))]
    pub AddAudioEffectAsync: unsafe extern "system" fn(this: *mut *mut Self, definition: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Effects")))]
    AddAudioEffectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Effects"))]
    pub AddVideoEffectAsync: unsafe extern "system" fn(this: *mut *mut Self, definition: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Effects")))]
    AddVideoEffectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub PauseRecordAsync: unsafe extern "system" fn(this: *mut *mut Self, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))]
    PauseRecordAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ResumeRecordAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResumeRecordAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CameraStreamStateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CameraStreamStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCameraStreamStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCameraStreamStateChanged: usize,
    #[cfg(feature = "Media_Devices")]
    pub CameraStreamState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Devices::CameraStreamState) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    CameraStreamState: usize,
    #[cfg(feature = "Foundation")]
    pub GetPreviewFrameAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPreviewFrameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetPreviewFrameCopyAsync: unsafe extern "system" fn(this: *mut *mut Self, destination: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPreviewFrameCopyAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ThermalStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ThermalStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveThermalStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveThermalStatusChanged: usize,
    pub ThermalStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaCaptureThermalStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub PrepareAdvancedPhotoCaptureAsync: unsafe extern "system" fn(this: *mut *mut Self, encodingproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))]
    PrepareAdvancedPhotoCaptureAsync: usize,
}
impl ::windows_sys::core::Interface for IMediaCapture4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3134025686, data2: 64264, data3: 18759, data4: [174, 162, 206, 20, 239, 240, 206, 19] };
}
#[repr(C)]
pub struct IMediaCapture5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RemoveEffectAsync: unsafe extern "system" fn(this: *mut *mut Self, effect: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEffectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub PauseRecordWithResultAsync: unsafe extern "system" fn(this: *mut *mut Self, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))]
    PauseRecordWithResultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopRecordWithResultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopRecordWithResultAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub FrameSources: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames")))]
    FrameSources: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Frames"))]
    pub CreateFrameReaderAsync: unsafe extern "system" fn(this: *mut *mut Self, inputsource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture_Frames")))]
    CreateFrameReaderAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Frames"))]
    pub CreateFrameReaderWithSubtypeAsync: unsafe extern "system" fn(this: *mut *mut Self, inputsource: *mut ::core::ffi::c_void, outputsubtype: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture_Frames")))]
    CreateFrameReaderWithSubtypeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Media_Capture_Frames"))]
    pub CreateFrameReaderWithSubtypeAndSizeAsync: unsafe extern "system" fn(this: *mut *mut Self, inputsource: *mut ::core::ffi::c_void, outputsubtype: ::windows_sys::core::HSTRING, outputsize: super::super::Graphics::Imaging::BitmapSize, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Media_Capture_Frames")))]
    CreateFrameReaderWithSubtypeAndSizeAsync: usize,
}
impl ::windows_sys::core::Interface for IMediaCapture5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3665329186, data2: 15003, data3: 18208, data4: [167, 30, 151, 144, 10, 49, 110, 90] };
}
#[repr(C)]
pub struct IMediaCapture6 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CaptureDeviceExclusiveControlStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureDeviceExclusiveControlStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCaptureDeviceExclusiveControlStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCaptureDeviceExclusiveControlStatusChanged: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub CreateMultiSourceFrameReaderAsync: unsafe extern "system" fn(this: *mut *mut Self, inputsources: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames")))]
    CreateMultiSourceFrameReaderAsync: usize,
}
impl ::windows_sys::core::Interface for IMediaCapture6 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 579422397, data2: 19232, data3: 19377, data4: [159, 214, 165, 131, 33, 42, 16, 18] };
}
#[repr(C)]
pub struct IMediaCapture7 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_WindowManagement")]
    pub CreateRelativePanelWatcher: unsafe extern "system" fn(this: *mut *mut Self, capturemode: StreamingCaptureMode, displayregion: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))]
    CreateRelativePanelWatcher: usize,
}
impl ::windows_sys::core::Interface for IMediaCapture7 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2439639298, data2: 34952, data3: 21530, data4: [149, 188, 36, 228, 212, 98, 84, 42] };
}
#[repr(C)]
pub struct IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaCaptureDeviceExclusiveControlStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2637140493, data2: 42376, data3: 17350, data4: [137, 214, 90, 211, 34, 175, 0, 106] };
}
#[repr(C)]
pub struct IMediaCaptureFailedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Code: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaCaptureFailedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2164122612, data2: 21700, data3: 17088, data4: [141, 25, 206, 161, 168, 124, 161, 139] };
}
#[repr(C)]
pub struct IMediaCaptureFocusChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Devices")]
    pub FocusState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Devices::MediaCaptureFocusState) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    FocusState: usize,
}
impl ::windows_sys::core::Interface for IMediaCaptureFocusChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2179054719, data2: 8823, data3: 18750, data4: [171, 238, 211, 244, 79, 249, 140, 4] };
}
#[repr(C)]
pub struct IMediaCaptureInitializationSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetAudioDeviceId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AudioDeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetVideoDeviceId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoDeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetStreamingCaptureMode: unsafe extern "system" fn(this: *mut *mut Self, value: StreamingCaptureMode) -> ::windows_sys::core::HRESULT,
    pub StreamingCaptureMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StreamingCaptureMode) -> ::windows_sys::core::HRESULT,
    pub SetPhotoCaptureSource: unsafe extern "system" fn(this: *mut *mut Self, value: PhotoCaptureSource) -> ::windows_sys::core::HRESULT,
    pub PhotoCaptureSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhotoCaptureSource) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaCaptureInitializationSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2541927024, data2: 60005, data3: 18688, data4: [147, 86, 140, 168, 135, 114, 104, 132] };
}
#[repr(C)]
pub struct IMediaCaptureInitializationSettings2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetMediaCategory: unsafe extern "system" fn(this: *mut *mut Self, value: MediaCategory) -> ::windows_sys::core::HRESULT,
    pub MediaCategory: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaCategory) -> ::windows_sys::core::HRESULT,
    pub SetAudioProcessing: unsafe extern "system" fn(this: *mut *mut Self, value: super::AudioProcessing) -> ::windows_sys::core::HRESULT,
    pub AudioProcessing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::AudioProcessing) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaCaptureInitializationSettings2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1078855206, data2: 51676, data3: 17385, data4: [174, 228, 230, 191, 27, 87, 180, 76] };
}
#[repr(C)]
pub struct IMediaCaptureInitializationSettings3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Core")]
    pub SetAudioSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    SetAudioSource: usize,
    #[cfg(feature = "Media_Core")]
    pub AudioSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    AudioSource: usize,
    #[cfg(feature = "Media_Core")]
    pub SetVideoSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    SetVideoSource: usize,
    #[cfg(feature = "Media_Core")]
    pub VideoSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    VideoSource: usize,
}
impl ::windows_sys::core::Interface for IMediaCaptureInitializationSettings3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1096831389, data2: 48712, data3: 18224, data4: [129, 4, 12, 246, 233, 233, 121, 72] };
}
#[repr(C)]
pub struct IMediaCaptureInitializationSettings4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub VideoProfile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetVideoProfile: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PreviewMediaDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPreviewMediaDescription: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RecordMediaDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRecordMediaDescription: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PhotoMediaDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPhotoMediaDescription: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaCaptureInitializationSettings4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4110591287, data2: 19639, data3: 19752, data4: [149, 237, 79, 159, 1, 46, 5, 24] };
}
#[repr(C)]
pub struct IMediaCaptureInitializationSettings5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Capture_Frames")]
    pub SourceGroup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Capture_Frames"))]
    SourceGroup: usize,
    #[cfg(feature = "Media_Capture_Frames")]
    pub SetSourceGroup: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Capture_Frames"))]
    SetSourceGroup: usize,
    pub SharingMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaCaptureSharingMode) -> ::windows_sys::core::HRESULT,
    pub SetSharingMode: unsafe extern "system" fn(this: *mut *mut Self, value: MediaCaptureSharingMode) -> ::windows_sys::core::HRESULT,
    pub MemoryPreference: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaCaptureMemoryPreference) -> ::windows_sys::core::HRESULT,
    pub SetMemoryPreference: unsafe extern "system" fn(this: *mut *mut Self, value: MediaCaptureMemoryPreference) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaCaptureInitializationSettings5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3584222136, data2: 9766, data3: 20116, data4: [183, 179, 83, 8, 160, 246, 75, 26] };
}
#[repr(C)]
pub struct IMediaCaptureInitializationSettings6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AlwaysPlaySystemShutterSound: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAlwaysPlaySystemShutterSound: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaCaptureInitializationSettings6 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3001183047, data2: 15793, data3: 19763, data4: [171, 99, 15, 250, 9, 5, 101, 133] };
}
#[repr(C)]
pub struct IMediaCaptureInitializationSettings7 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub DeviceUriPasswordCredential: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    DeviceUriPasswordCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetDeviceUriPasswordCredential: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetDeviceUriPasswordCredential: usize,
    #[cfg(feature = "Foundation")]
    pub DeviceUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeviceUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetDeviceUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDeviceUri: usize,
}
impl ::windows_sys::core::Interface for IMediaCaptureInitializationSettings7 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1096051047, data2: 62858, data3: 23938, data4: [158, 244, 237, 87, 47, 181, 227, 78] };
}
#[repr(C)]
pub struct IMediaCapturePauseResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub LastFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RecordDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecordDuration: usize,
}
impl ::windows_sys::core::Interface for IMediaCapturePauseResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2932112547, data2: 17527, data3: 19204, data4: [160, 111, 44, 28, 81, 130, 254, 157] };
}
#[repr(C)]
pub struct IMediaCaptureRelativePanelWatcher {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Devices_Enumeration")]
    pub RelativePanel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Devices::Enumeration::Panel) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    RelativePanel: usize,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Changed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChanged: usize,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaCaptureRelativePanelWatcher {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2106156390, data2: 1214, data3: 23433, data4: [179, 14, 189, 52, 169, 241, 45, 176] };
}
#[repr(C)]
pub struct IMediaCaptureSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub AudioDeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoDeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub StreamingCaptureMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StreamingCaptureMode) -> ::windows_sys::core::HRESULT,
    pub PhotoCaptureSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhotoCaptureSource) -> ::windows_sys::core::HRESULT,
    pub VideoDeviceCharacteristic: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VideoDeviceCharacteristic) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaCaptureSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 495168254, data2: 27973, data3: 17527, data4: [141, 196, 172, 91, 192, 28, 64, 145] };
}
#[repr(C)]
pub struct IMediaCaptureSettings2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ConcurrentRecordAndPhotoSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ConcurrentRecordAndPhotoSequenceSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CameraSoundRequiredForRegion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Horizontal35mmEquivalentFocalLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Horizontal35mmEquivalentFocalLength: usize,
    #[cfg(feature = "Foundation")]
    pub PitchOffsetDegrees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PitchOffsetDegrees: usize,
    #[cfg(feature = "Foundation")]
    pub Vertical35mmEquivalentFocalLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Vertical35mmEquivalentFocalLength: usize,
    pub MediaCategory: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaCategory) -> ::windows_sys::core::HRESULT,
    pub AudioProcessing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::AudioProcessing) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMediaCaptureSettings2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1872657659, data2: 64159, data3: 19219, data4: [156, 190, 90, 185, 79, 31, 52, 147] };
}
#[repr(C)]
pub struct IMediaCaptureSettings3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3D11Device: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3D11Device: usize,
}
impl ::windows_sys::core::Interface for IMediaCaptureSettings3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 809265090, data2: 32856, data3: 19227, data4: [184, 119, 140, 46, 243, 82, 132, 64] };
}
#[repr(C)]
pub struct IMediaCaptureStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsVideoProfileSupported: unsafe extern "system" fn(this: *mut *mut Self, videodeviceid: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllVideoProfiles: unsafe extern "system" fn(this: *mut *mut Self, videodeviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllVideoProfiles: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindConcurrentProfiles: unsafe extern "system" fn(this: *mut *mut Self, videodeviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindConcurrentProfiles: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindKnownVideoProfiles: unsafe extern "system" fn(this: *mut *mut Self, videodeviceid: ::windows_sys::core::HSTRING, name: KnownVideoProfile, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindKnownVideoProfiles: usize,
}
impl ::windows_sys::core::Interface for IMediaCaptureStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2901377535, data2: 39405, data3: 17989, data4: [150, 94, 25, 37, 207, 198, 56, 52] };
}
#[repr(C)]
pub struct IMediaCaptureStopResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub LastFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RecordDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecordDuration: usize,
}
impl ::windows_sys::core::Interface for IMediaCaptureStopResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4191906346, data2: 41106, data3: 19153, data4: [151, 212, 242, 1, 249, 208, 130, 219] };
}
#[repr(C)]
pub struct IMediaCaptureVideoPreview {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub StartPreviewAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartPreviewAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub StartPreviewToCustomSinkAsync: unsafe extern "system" fn(this: *mut *mut Self, encodingprofile: *mut ::core::ffi::c_void, custommediasink: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))]
    StartPreviewToCustomSinkAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub StartPreviewToCustomSinkIdAsync: unsafe extern "system" fn(this: *mut *mut Self, encodingprofile: *mut ::core::ffi::c_void, customsinkactivationid: ::windows_sys::core::HSTRING, customsinksettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    StartPreviewToCustomSinkIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopPreviewAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopPreviewAsync: usize,
}
impl ::windows_sys::core::Interface for IMediaCaptureVideoPreview {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 661811315, data2: 21662, data3: 17535, data4: [162, 10, 79, 3, 196, 121, 216, 192] };
}
#[repr(C)]
pub struct IMediaCaptureVideoProfile {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoDeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPreviewMediaDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPreviewMediaDescription: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedRecordMediaDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedRecordMediaDescription: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPhotoMediaDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPhotoMediaDescription: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConcurrency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConcurrency: usize,
}
impl ::windows_sys::core::Interface for IMediaCaptureVideoProfile {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 564163519, data2: 41966, data3: 20175, data4: [158, 246, 80, 176, 188, 78, 19, 5] };
}
#[repr(C)]
pub struct IMediaCaptureVideoProfile2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub FrameSourceInfos: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames")))]
    FrameSourceInfos: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
impl ::windows_sys::core::Interface for IMediaCaptureVideoProfile2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2547894623, data2: 38094, data3: 18063, data4: [147, 22, 252, 91, 194, 99, 143, 107] };
}
#[repr(C)]
pub struct IMediaCaptureVideoProfileMediaDescription {
    pub base__: ::windows_sys::core::IInspectable,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub FrameRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub IsVariablePhotoSequenceSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsVariablePhotoSequenceSupported: usize,
    #[cfg(feature = "deprecated")]
    pub IsHdrVideoSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsHdrVideoSupported: usize,
}
impl ::windows_sys::core::Interface for IMediaCaptureVideoProfileMediaDescription {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2148708335, data2: 46737, data3: 18943, data4: [131, 242, 193, 231, 110, 170, 234, 27] };
}
#[repr(C)]
pub struct IMediaCaptureVideoProfileMediaDescription2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Subtype: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
impl ::windows_sys::core::Interface for IMediaCaptureVideoProfileMediaDescription2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3332828947, data2: 12845, data3: 16698, data4: [184, 90, 104, 168, 142, 2, 244, 233] };
}
#[repr(C)]
pub struct IOptionalReferencePhotoCapturedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Frame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Context: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IOptionalReferencePhotoCapturedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1192200371, data2: 7789, data3: 16465, data4: [156, 139, 241, 216, 90, 240, 71, 183] };
}
#[repr(C)]
pub struct IPhotoCapturedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Frame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Thumbnail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CaptureTimeOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureTimeOffset: usize,
}
impl ::windows_sys::core::Interface for IPhotoCapturedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 926677953, data2: 38990, data3: 20464, data4: [191, 133, 28, 0, 170, 188, 90, 69] };
}
#[repr(C)]
pub struct IPhotoConfirmationCapturedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Frame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CaptureTimeOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureTimeOffset: usize,
}
impl ::windows_sys::core::Interface for IPhotoConfirmationCapturedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2873570930, data2: 49802, data3: 18471, data4: [143, 141, 54, 54, 211, 190, 181, 30] };
}
#[repr(C)]
pub struct IScreenCapture {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Core")]
    pub AudioSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    AudioSource: usize,
    #[cfg(feature = "Media_Core")]
    pub VideoSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    VideoSource: usize,
    pub IsAudioSuspended: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsVideoSuspended: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SourceSuspensionChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceSuspensionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSourceSuspensionChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSourceSuspensionChanged: usize,
}
impl ::windows_sys::core::Interface for IScreenCapture {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2300026615, data2: 52498, data3: 19982, data4: [166, 212, 91, 61, 233, 139, 46, 155] };
}
#[repr(C)]
pub struct IScreenCaptureStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IScreenCaptureStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3365454768, data2: 51365, data3: 4578, data4: [139, 139, 8, 0, 32, 12, 154, 102] };
}
#[repr(C)]
pub struct ISourceSuspensionChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsAudioSuspended: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsVideoSuspended: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISourceSuspensionChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 785283934, data2: 54427, data3: 17300, data4: [188, 50, 249, 125, 108, 237, 236, 28] };
}
#[repr(C)]
pub struct IVideoStreamConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_MediaProperties")]
    pub InputProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    InputProperties: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub OutputProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    OutputProperties: usize,
}
impl ::windows_sys::core::Interface for IVideoStreamConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3631680111, data2: 17296, data3: 19294, data4: [173, 62, 15, 138, 240, 150, 52, 144] };
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct KnownVideoProfile(pub i32);
impl KnownVideoProfile {
    pub const VideoRecording: Self = Self(0i32);
    pub const HighQualityPhoto: Self = Self(1i32);
    pub const BalancedVideoAndPhoto: Self = Self(2i32);
    pub const VideoConferencing: Self = Self(3i32);
    pub const PhotoSequence: Self = Self(4i32);
    pub const HighFrameRate: Self = Self(5i32);
    pub const VariablePhotoSequence: Self = Self(6i32);
    pub const HdrWithWcgVideo: Self = Self(7i32);
    pub const HdrWithWcgPhoto: Self = Self(8i32);
    pub const VideoHdr8: Self = Self(9i32);
    pub const CompressedCamera: Self = Self(10i32);
}
impl ::core::marker::Copy for KnownVideoProfile {}
impl ::core::clone::Clone for KnownVideoProfile {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LowLagMediaRecording = *mut ::core::ffi::c_void;
pub type LowLagPhotoCapture = *mut ::core::ffi::c_void;
pub type LowLagPhotoSequenceCapture = *mut ::core::ffi::c_void;
pub type MediaCapture = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCaptureDeviceExclusiveControlStatus(pub i32);
impl MediaCaptureDeviceExclusiveControlStatus {
    pub const ExclusiveControlAvailable: Self = Self(0i32);
    pub const SharedReadOnlyAvailable: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaCaptureDeviceExclusiveControlStatus {}
impl ::core::clone::Clone for MediaCaptureDeviceExclusiveControlStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaCaptureDeviceExclusiveControlStatusChangedEventArgs = *mut ::core::ffi::c_void;
pub type MediaCaptureFailedEventArgs = *mut ::core::ffi::c_void;
pub type MediaCaptureFailedEventHandler = *mut ::core::ffi::c_void;
pub type MediaCaptureFocusChangedEventArgs = *mut ::core::ffi::c_void;
pub type MediaCaptureInitializationSettings = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCaptureMemoryPreference(pub i32);
impl MediaCaptureMemoryPreference {
    pub const Auto: Self = Self(0i32);
    pub const Cpu: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaCaptureMemoryPreference {}
impl ::core::clone::Clone for MediaCaptureMemoryPreference {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaCapturePauseResult = *mut ::core::ffi::c_void;
pub type MediaCaptureRelativePanelWatcher = *mut ::core::ffi::c_void;
pub type MediaCaptureSettings = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCaptureSharingMode(pub i32);
impl MediaCaptureSharingMode {
    pub const ExclusiveControl: Self = Self(0i32);
    pub const SharedReadOnly: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaCaptureSharingMode {}
impl ::core::clone::Clone for MediaCaptureSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaCaptureStopResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCaptureThermalStatus(pub i32);
impl MediaCaptureThermalStatus {
    pub const Normal: Self = Self(0i32);
    pub const Overheated: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaCaptureThermalStatus {}
impl ::core::clone::Clone for MediaCaptureThermalStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MediaCaptureVideoProfile = *mut ::core::ffi::c_void;
pub type MediaCaptureVideoProfileMediaDescription = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCategory(pub i32);
impl MediaCategory {
    pub const Other: Self = Self(0i32);
    pub const Communications: Self = Self(1i32);
    pub const Media: Self = Self(2i32);
    pub const GameChat: Self = Self(3i32);
    pub const Speech: Self = Self(4i32);
    pub const FarFieldSpeech: Self = Self(5i32);
    pub const UniformSpeech: Self = Self(6i32);
    pub const VoiceTyping: Self = Self(7i32);
}
impl ::core::marker::Copy for MediaCategory {}
impl ::core::clone::Clone for MediaCategory {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaStreamType(pub i32);
impl MediaStreamType {
    pub const VideoPreview: Self = Self(0i32);
    pub const VideoRecord: Self = Self(1i32);
    pub const Audio: Self = Self(2i32);
    pub const Photo: Self = Self(3i32);
    pub const Metadata: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaStreamType {}
impl ::core::clone::Clone for MediaStreamType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type OptionalReferencePhotoCapturedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct PhotoCaptureSource(pub i32);
impl PhotoCaptureSource {
    pub const Auto: Self = Self(0i32);
    pub const VideoPreview: Self = Self(1i32);
    pub const Photo: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoCaptureSource {}
impl ::core::clone::Clone for PhotoCaptureSource {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PhotoCapturedEventArgs = *mut ::core::ffi::c_void;
pub type PhotoConfirmationCapturedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct PowerlineFrequency(pub i32);
impl PowerlineFrequency {
    pub const Disabled: Self = Self(0i32);
    pub const FiftyHertz: Self = Self(1i32);
    pub const SixtyHertz: Self = Self(2i32);
    pub const Auto: Self = Self(3i32);
}
impl ::core::marker::Copy for PowerlineFrequency {}
impl ::core::clone::Clone for PowerlineFrequency {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RecordLimitationExceededEventHandler = *mut ::core::ffi::c_void;
pub type ScreenCapture = *mut ::core::ffi::c_void;
pub type SourceSuspensionChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct StreamingCaptureMode(pub i32);
impl StreamingCaptureMode {
    pub const AudioAndVideo: Self = Self(0i32);
    pub const Audio: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for StreamingCaptureMode {}
impl ::core::clone::Clone for StreamingCaptureMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct VideoDeviceCharacteristic(pub i32);
impl VideoDeviceCharacteristic {
    pub const AllStreamsIndependent: Self = Self(0i32);
    pub const PreviewRecordStreamsIdentical: Self = Self(1i32);
    pub const PreviewPhotoStreamsIdentical: Self = Self(2i32);
    pub const RecordPhotoStreamsIdentical: Self = Self(3i32);
    pub const AllStreamsIdentical: Self = Self(4i32);
}
impl ::core::marker::Copy for VideoDeviceCharacteristic {}
impl ::core::clone::Clone for VideoDeviceCharacteristic {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct VideoRotation(pub i32);
impl VideoRotation {
    pub const None: Self = Self(0i32);
    pub const Clockwise90Degrees: Self = Self(1i32);
    pub const Clockwise180Degrees: Self = Self(2i32);
    pub const Clockwise270Degrees: Self = Self(3i32);
}
impl ::core::marker::Copy for VideoRotation {}
impl ::core::clone::Clone for VideoRotation {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VideoStreamConfiguration = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Media_Capture\"`*"]
pub struct WhiteBalanceGain {
    pub R: f64,
    pub G: f64,
    pub B: f64,
}
impl ::core::marker::Copy for WhiteBalanceGain {}
impl ::core::clone::Clone for WhiteBalanceGain {
    fn clone(&self) -> Self {
        *self
    }
}
