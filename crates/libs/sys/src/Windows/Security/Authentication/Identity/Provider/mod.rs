#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISecondaryAuthenticationFactorAuthentication {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub ServiceAuthenticationHmac: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    ServiceAuthenticationHmac: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub SessionNonce: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    SessionNonce: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub DeviceNonce: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    DeviceNonce: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub DeviceConfigurationData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    DeviceConfigurationData: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub FinishAuthenticationAsync: unsafe extern "system" fn(this: *mut *mut Self, devicehmac: *mut ::core::ffi::c_void, sessionhmac: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    FinishAuthenticationAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AbortAuthenticationAsync: unsafe extern "system" fn(this: *mut *mut Self, errorlogmessage: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AbortAuthenticationAsync: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISecondaryAuthenticationFactorAuthenticationResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SecondaryAuthenticationFactorAuthenticationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
    #[cfg(feature = "deprecated")]
    pub Authentication: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Authentication: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub StageInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    StageInfo: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISecondaryAuthenticationFactorAuthenticationStageInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Stage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SecondaryAuthenticationFactorAuthenticationStage) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Stage: usize,
    #[cfg(feature = "deprecated")]
    pub Scenario: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SecondaryAuthenticationFactorAuthenticationScenario) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Scenario: usize,
    #[cfg(feature = "deprecated")]
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceId: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISecondaryAuthenticationFactorAuthenticationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ShowNotificationMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, devicename: ::windows_sys::core::HSTRING, message: SecondaryAuthenticationFactorAuthenticationMessage, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ShowNotificationMessageAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub StartAuthenticationAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, serviceauthenticationnonce: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    StartAuthenticationAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AuthenticationStageChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AuthenticationStageChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveAuthenticationStageChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveAuthenticationStageChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetAuthenticationStageInfoAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetAuthenticationStageInfoAsync: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RegisterDevicePresenceMonitoringAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, deviceinstancepath: ::windows_sys::core::HSTRING, monitoringmode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RegisterDevicePresenceMonitoringAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub RegisterDevicePresenceMonitoringWithNewDeviceAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, deviceinstancepath: ::windows_sys::core::HSTRING, monitoringmode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode, devicefriendlyname: ::windows_sys::core::HSTRING, devicemodelnumber: ::windows_sys::core::HSTRING, deviceconfigurationdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    RegisterDevicePresenceMonitoringWithNewDeviceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub UnregisterDevicePresenceMonitoringAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    UnregisterDevicePresenceMonitoringAsync: usize,
    #[cfg(feature = "deprecated")]
    pub IsDevicePresenceMonitoringSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsDevicePresenceMonitoringSupported: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISecondaryAuthenticationFactorInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceId: usize,
    #[cfg(feature = "deprecated")]
    pub DeviceFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceFriendlyName: usize,
    #[cfg(feature = "deprecated")]
    pub DeviceModelNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceModelNumber: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub DeviceConfigurationData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    DeviceConfigurationData: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISecondaryAuthenticationFactorInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub PresenceMonitoringMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SecondaryAuthenticationFactorDevicePresenceMonitoringMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PresenceMonitoringMode: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub UpdateDevicePresenceAsync: unsafe extern "system" fn(this: *mut *mut Self, presencestate: SecondaryAuthenticationFactorDevicePresence, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    UpdateDevicePresenceAsync: usize,
    #[cfg(feature = "deprecated")]
    pub IsAuthenticationSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsAuthenticationSupported: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISecondaryAuthenticationFactorRegistration {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub FinishRegisteringDeviceAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceconfigurationdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    FinishRegisteringDeviceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AbortRegisteringDeviceAsync: unsafe extern "system" fn(this: *mut *mut Self, errorlogmessage: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AbortRegisteringDeviceAsync: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISecondaryAuthenticationFactorRegistrationResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SecondaryAuthenticationFactorRegistrationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
    #[cfg(feature = "deprecated")]
    pub Registration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Registration: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISecondaryAuthenticationFactorRegistrationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub RequestStartRegisteringDeviceAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, capabilities: SecondaryAuthenticationFactorDeviceCapabilities, devicefriendlyname: ::windows_sys::core::HSTRING, devicemodelnumber: ::windows_sys::core::HSTRING, devicekey: *mut ::core::ffi::c_void, mutualauthenticationkey: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    RequestStartRegisteringDeviceAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub FindAllRegisteredDeviceInfoAsync: unsafe extern "system" fn(this: *mut *mut Self, querytype: SecondaryAuthenticationFactorDeviceFindScope, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    FindAllRegisteredDeviceInfoAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub UnregisterDeviceAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    UnregisterDeviceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub UpdateDeviceConfigurationDataAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, deviceconfigurationdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    UpdateDeviceConfigurationDataAsync: usize,
}
pub type SecondaryAuthenticationFactorAuthentication = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationMessage(pub i32);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorAuthenticationMessage {
    pub const Invalid: Self = Self(0i32);
    pub const SwipeUpWelcome: Self = Self(1i32);
    pub const TapWelcome: Self = Self(2i32);
    pub const DeviceNeedsAttention: Self = Self(3i32);
    pub const LookingForDevice: Self = Self(4i32);
    pub const LookingForDevicePluggedin: Self = Self(5i32);
    pub const BluetoothIsDisabled: Self = Self(6i32);
    pub const NfcIsDisabled: Self = Self(7i32);
    pub const WiFiIsDisabled: Self = Self(8i32);
    pub const ExtraTapIsRequired: Self = Self(9i32);
    pub const DisabledByPolicy: Self = Self(10i32);
    pub const TapOnDeviceRequired: Self = Self(11i32);
    pub const HoldFinger: Self = Self(12i32);
    pub const ScanFinger: Self = Self(13i32);
    pub const UnauthorizedUser: Self = Self(14i32);
    pub const ReregisterRequired: Self = Self(15i32);
    pub const TryAgain: Self = Self(16i32);
    pub const SayPassphrase: Self = Self(17i32);
    pub const ReadyToSignIn: Self = Self(18i32);
    pub const UseAnotherSignInOption: Self = Self(19i32);
    pub const ConnectionRequired: Self = Self(20i32);
    pub const TimeLimitExceeded: Self = Self(21i32);
    pub const CanceledByUser: Self = Self(22i32);
    pub const CenterHand: Self = Self(23i32);
    pub const MoveHandCloser: Self = Self(24i32);
    pub const MoveHandFarther: Self = Self(25i32);
    pub const PlaceHandAbove: Self = Self(26i32);
    pub const RecognitionFailed: Self = Self(27i32);
    pub const DeviceUnavailable: Self = Self(28i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SecondaryAuthenticationFactorAuthenticationMessage {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorAuthenticationMessage {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SecondaryAuthenticationFactorAuthenticationResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationScenario(pub i32);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorAuthenticationScenario {
    pub const SignIn: Self = Self(0i32);
    pub const CredentialPrompt: Self = Self(1i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SecondaryAuthenticationFactorAuthenticationScenario {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorAuthenticationScenario {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationStage(pub i32);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorAuthenticationStage {
    pub const NotStarted: Self = Self(0i32);
    pub const WaitingForUserConfirmation: Self = Self(1i32);
    pub const CollectingCredential: Self = Self(2i32);
    pub const SuspendingAuthentication: Self = Self(3i32);
    pub const CredentialCollected: Self = Self(4i32);
    pub const CredentialAuthenticated: Self = Self(5i32);
    pub const StoppingAuthentication: Self = Self(6i32);
    pub const ReadyForLock: Self = Self(7i32);
    pub const CheckingDevicePresence: Self = Self(8i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SecondaryAuthenticationFactorAuthenticationStage {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorAuthenticationStage {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs = *mut ::core::ffi::c_void;
pub type SecondaryAuthenticationFactorAuthenticationStageInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationStatus(pub i32);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorAuthenticationStatus {
    pub const Failed: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const UnknownDevice: Self = Self(2i32);
    pub const DisabledByPolicy: Self = Self(3i32);
    pub const InvalidAuthenticationStage: Self = Self(4i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SecondaryAuthenticationFactorAuthenticationStatus {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorAuthenticationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorDeviceCapabilities(pub u32);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorDeviceCapabilities {
    pub const None: Self = Self(0u32);
    pub const SecureStorage: Self = Self(1u32);
    pub const StoreKeys: Self = Self(2u32);
    pub const ConfirmUserIntentToAuthenticate: Self = Self(4u32);
    pub const SupportSecureUserPresenceCheck: Self = Self(8u32);
    pub const TransmittedDataIsEncrypted: Self = Self(16u32);
    pub const HMacSha256: Self = Self(32u32);
    pub const CloseRangeDataTransmission: Self = Self(64u32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SecondaryAuthenticationFactorDeviceCapabilities {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorDeviceCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorDeviceFindScope(pub i32);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorDeviceFindScope {
    pub const User: Self = Self(0i32);
    pub const AllUsers: Self = Self(1i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SecondaryAuthenticationFactorDeviceFindScope {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorDeviceFindScope {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorDevicePresence(pub i32);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorDevicePresence {
    pub const Absent: Self = Self(0i32);
    pub const Present: Self = Self(1i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SecondaryAuthenticationFactorDevicePresence {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorDevicePresence {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorDevicePresenceMonitoringMode(pub i32);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorDevicePresenceMonitoringMode {
    pub const Unsupported: Self = Self(0i32);
    pub const AppManaged: Self = Self(1i32);
    pub const SystemManaged: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SecondaryAuthenticationFactorDevicePresenceMonitoringMode {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorDevicePresenceMonitoringMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus(pub i32);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus {
    pub const Unsupported: Self = Self(0i32);
    pub const Succeeded: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorFinishAuthenticationStatus(pub i32);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorFinishAuthenticationStatus {
    pub const Failed: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const NonceExpired: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SecondaryAuthenticationFactorFinishAuthenticationStatus {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorFinishAuthenticationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SecondaryAuthenticationFactorInfo = *mut ::core::ffi::c_void;
pub type SecondaryAuthenticationFactorRegistration = *mut ::core::ffi::c_void;
pub type SecondaryAuthenticationFactorRegistrationResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Authentication_Identity_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorRegistrationStatus(pub i32);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorRegistrationStatus {
    pub const Failed: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const CanceledByUser: Self = Self(2i32);
    pub const PinSetupRequired: Self = Self(3i32);
    pub const DisabledByPolicy: Self = Self(4i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SecondaryAuthenticationFactorRegistrationStatus {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorRegistrationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
