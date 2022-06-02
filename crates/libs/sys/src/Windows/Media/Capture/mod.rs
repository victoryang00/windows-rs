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
#[repr(C)]
pub struct IAdvancedCapturedPhoto2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FrameBoundsRelativeToReferencePhoto: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameBoundsRelativeToReferencePhoto: usize,
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
#[repr(C)]
pub struct IAppBroadcastBackgroundServiceStreamInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReportProblemWithStream: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBroadcastCameraCaptureStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastCameraCaptureState) -> ::windows_sys::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IAppBroadcastHeartbeatRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBroadcastManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetGlobalSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ApplyGlobalSettings: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetProviderSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ApplyProviderSettings: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBroadcastMicrophoneCaptureStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastMicrophoneCaptureState) -> ::windows_sys::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IAppBroadcastPlugInManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[repr(C)]
pub struct IAppBroadcastPlugInStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub PlugInState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastPlugInState) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IAppBroadcastPreviewStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub PreviewState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastPreviewState) -> ::windows_sys::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IAppBroadcastPreviewStreamVideoFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub VideoHeader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub VideoBuffer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    VideoBuffer: usize,
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
#[repr(C)]
pub struct IAppBroadcastSignInStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SignInState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastSignInState) -> ::windows_sys::core::HRESULT,
    pub Result: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastSignInResult) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IAppBroadcastStreamAudioFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub AudioHeader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub AudioBuffer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AudioBuffer: usize,
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
#[repr(C)]
pub struct IAppBroadcastStreamStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub StreamState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppBroadcastStreamState) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IAppBroadcastTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub BackgroundService: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppBroadcastViewerCountChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ViewerCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IAppCaptureDurationGeneratedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
}
#[repr(C)]
pub struct IAppCaptureFileGeneratedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
}
#[repr(C)]
pub struct IAppCaptureManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetCurrentSettings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ApplySettings: unsafe extern "system" fn(this: *mut *mut Self, appcapturesettings: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IAppCaptureMicrophoneCaptureStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppCaptureMicrophoneCaptureState) -> ::windows_sys::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IAppCaptureRecordingStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppCaptureRecordingState) -> ::windows_sys::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IAppCaptureSettings2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsGpuConstrained: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AlternateShortcutKeys: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppCaptureSettings3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetIsMicrophoneCaptureEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsMicrophoneCaptureEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IAppCaptureSettings5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetIsEchoCancellationEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsEchoCancellationEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsCursorImageCaptureEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsCursorImageCaptureEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IAppCaptureStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppCaptureStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetAllowedAsync: unsafe extern "system" fn(this: *mut *mut Self, allowed: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAllowedAsync: usize,
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
#[repr(C)]
pub struct ICameraOptionsUIStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Show: unsafe extern "system" fn(this: *mut *mut Self, mediacapture: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICapturedFrame {
    pub base__: ::windows_sys::core::IInspectable,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct ICapturedFrameWithSoftwareBitmap {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_Imaging")]
    pub SoftwareBitmap: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SoftwareBitmap: usize,
}
#[repr(C)]
pub struct ICapturedPhoto {
    pub base__: ::windows_sys::core::IInspectable,
    pub Frame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Thumbnail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IGameBarServicesCommandEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Command: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GameBarCommand) -> ::windows_sys::core::HRESULT,
    pub Origin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GameBarCommandOrigin) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IGameBarServicesManagerGameBarServicesCreatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub GameBarServices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGameBarServicesManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGameBarServicesTargetInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AppId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TitleId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GameBarServicesDisplayMode) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IMediaCapture7 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_WindowManagement")]
    pub CreateRelativePanelWatcher: unsafe extern "system" fn(this: *mut *mut Self, capturemode: StreamingCaptureMode, displayregion: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))]
    CreateRelativePanelWatcher: usize,
}
#[repr(C)]
pub struct IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaCaptureDeviceExclusiveControlStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaCaptureFailedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Code: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaCaptureFocusChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Devices")]
    pub FocusState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Devices::MediaCaptureFocusState) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    FocusState: usize,
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
#[repr(C)]
pub struct IMediaCaptureInitializationSettings2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetMediaCategory: unsafe extern "system" fn(this: *mut *mut Self, value: MediaCategory) -> ::windows_sys::core::HRESULT,
    pub MediaCategory: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MediaCategory) -> ::windows_sys::core::HRESULT,
    pub SetAudioProcessing: unsafe extern "system" fn(this: *mut *mut Self, value: super::AudioProcessing) -> ::windows_sys::core::HRESULT,
    pub AudioProcessing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::AudioProcessing) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IMediaCaptureInitializationSettings6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AlwaysPlaySystemShutterSound: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAlwaysPlaySystemShutterSound: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IMediaCapturePauseResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub LastFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RecordDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecordDuration: usize,
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
#[repr(C)]
pub struct IMediaCaptureSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub AudioDeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoDeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub StreamingCaptureMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut StreamingCaptureMode) -> ::windows_sys::core::HRESULT,
    pub PhotoCaptureSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhotoCaptureSource) -> ::windows_sys::core::HRESULT,
    pub VideoDeviceCharacteristic: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VideoDeviceCharacteristic) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IMediaCaptureSettings3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3D11Device: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3D11Device: usize,
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
#[repr(C)]
pub struct IMediaCaptureStopResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub LastFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RecordDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecordDuration: usize,
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
#[repr(C)]
pub struct IMediaCaptureVideoProfileMediaDescription2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Subtype: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct IOptionalReferencePhotoCapturedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Frame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Context: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IPhotoConfirmationCapturedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Frame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CaptureTimeOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureTimeOffset: usize,
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
#[repr(C)]
pub struct IScreenCaptureStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISourceSuspensionChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsAudioSuspended: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsVideoSuspended: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
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
