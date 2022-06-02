#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct DataClasses(pub u32);
impl DataClasses {
    pub const None: Self = Self(0u32);
    pub const Gprs: Self = Self(1u32);
    pub const Edge: Self = Self(2u32);
    pub const Umts: Self = Self(4u32);
    pub const Hsdpa: Self = Self(8u32);
    pub const Hsupa: Self = Self(16u32);
    pub const LteAdvanced: Self = Self(32u32);
    pub const NewRadioNonStandalone: Self = Self(64u32);
    pub const NewRadioStandalone: Self = Self(128u32);
    pub const Cdma1xRtt: Self = Self(65536u32);
    pub const Cdma1xEvdo: Self = Self(131072u32);
    pub const Cdma1xEvdoRevA: Self = Self(262144u32);
    pub const Cdma1xEvdv: Self = Self(524288u32);
    pub const Cdma3xRtt: Self = Self(1048576u32);
    pub const Cdma1xEvdoRevB: Self = Self(2097152u32);
    pub const CdmaUmb: Self = Self(4194304u32);
    pub const Custom: Self = Self(2147483648u32);
}
impl ::core::marker::Copy for DataClasses {}
impl ::core::clone::Clone for DataClasses {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ESim = *mut ::core::ffi::c_void;
pub type ESimAddedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ESimAuthenticationPreference(pub i32);
impl ESimAuthenticationPreference {
    pub const OnEntry: Self = Self(0i32);
    pub const OnAction: Self = Self(1i32);
    pub const Never: Self = Self(2i32);
}
impl ::core::marker::Copy for ESimAuthenticationPreference {}
impl ::core::clone::Clone for ESimAuthenticationPreference {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ESimDiscoverEvent = *mut ::core::ffi::c_void;
pub type ESimDiscoverResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ESimDiscoverResultKind(pub i32);
impl ESimDiscoverResultKind {
    pub const None: Self = Self(0i32);
    pub const Events: Self = Self(1i32);
    pub const ProfileMetadata: Self = Self(2i32);
}
impl ::core::marker::Copy for ESimDiscoverResultKind {}
impl ::core::clone::Clone for ESimDiscoverResultKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ESimDownloadProfileMetadataResult = *mut ::core::ffi::c_void;
pub type ESimOperationResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ESimOperationStatus(pub i32);
impl ESimOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const NotAuthorized: Self = Self(1i32);
    pub const NotFound: Self = Self(2i32);
    pub const PolicyViolation: Self = Self(3i32);
    pub const InsufficientSpaceOnCard: Self = Self(4i32);
    pub const ServerFailure: Self = Self(5i32);
    pub const ServerNotReachable: Self = Self(6i32);
    pub const TimeoutWaitingForUserConsent: Self = Self(7i32);
    pub const IncorrectConfirmationCode: Self = Self(8i32);
    pub const ConfirmationCodeMaxRetriesExceeded: Self = Self(9i32);
    pub const CardRemoved: Self = Self(10i32);
    pub const CardBusy: Self = Self(11i32);
    pub const Other: Self = Self(12i32);
    pub const CardGeneralFailure: Self = Self(13i32);
    pub const ConfirmationCodeMissing: Self = Self(14i32);
    pub const InvalidMatchingId: Self = Self(15i32);
    pub const NoEligibleProfileForThisDevice: Self = Self(16i32);
    pub const OperationAborted: Self = Self(17i32);
    pub const EidMismatch: Self = Self(18i32);
    pub const ProfileNotAvailableForNewBinding: Self = Self(19i32);
    pub const ProfileNotReleasedByOperator: Self = Self(20i32);
    pub const OperationProhibitedByProfileClass: Self = Self(21i32);
    pub const ProfileNotPresent: Self = Self(22i32);
    pub const NoCorrespondingRequest: Self = Self(23i32);
    pub const TimeoutWaitingForResponse: Self = Self(24i32);
    pub const IccidAlreadyExists: Self = Self(25i32);
    pub const ProfileProcessingError: Self = Self(26i32);
    pub const ServerNotTrusted: Self = Self(27i32);
    pub const ProfileDownloadMaxRetriesExceeded: Self = Self(28i32);
}
impl ::core::marker::Copy for ESimOperationStatus {}
impl ::core::clone::Clone for ESimOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ESimPolicy = *mut ::core::ffi::c_void;
pub type ESimProfile = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ESimProfileClass(pub i32);
impl ESimProfileClass {
    pub const Operational: Self = Self(0i32);
    pub const Test: Self = Self(1i32);
    pub const Provisioning: Self = Self(2i32);
}
impl ::core::marker::Copy for ESimProfileClass {}
impl ::core::clone::Clone for ESimProfileClass {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
pub struct ESimProfileInstallProgress {
    pub TotalSizeInBytes: i32,
    pub InstalledSizeInBytes: i32,
}
impl ::core::marker::Copy for ESimProfileInstallProgress {}
impl ::core::clone::Clone for ESimProfileInstallProgress {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ESimProfileMetadata = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ESimProfileMetadataState(pub i32);
impl ESimProfileMetadataState {
    pub const Unknown: Self = Self(0i32);
    pub const WaitingForInstall: Self = Self(1i32);
    pub const Downloading: Self = Self(2i32);
    pub const Installing: Self = Self(3i32);
    pub const Expired: Self = Self(4i32);
    pub const RejectingDownload: Self = Self(5i32);
    pub const NoLongerAvailable: Self = Self(6i32);
    pub const DeniedByPolicy: Self = Self(7i32);
}
impl ::core::marker::Copy for ESimProfileMetadataState {}
impl ::core::clone::Clone for ESimProfileMetadataState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ESimProfilePolicy = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ESimProfileState(pub i32);
impl ESimProfileState {
    pub const Unknown: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
    pub const Enabled: Self = Self(2i32);
    pub const Deleted: Self = Self(3i32);
}
impl ::core::marker::Copy for ESimProfileState {}
impl ::core::clone::Clone for ESimProfileState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ESimRemovedEventArgs = *mut ::core::ffi::c_void;
pub type ESimServiceInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ESimState(pub i32);
impl ESimState {
    pub const Unknown: Self = Self(0i32);
    pub const Idle: Self = Self(1i32);
    pub const Removed: Self = Self(2i32);
    pub const Busy: Self = Self(3i32);
}
impl ::core::marker::Copy for ESimState {}
impl ::core::clone::Clone for ESimState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ESimUpdatedEventArgs = *mut ::core::ffi::c_void;
pub type ESimWatcher = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ESimWatcherStatus(pub i32);
impl ESimWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
}
impl ::core::marker::Copy for ESimWatcherStatus {}
impl ::core::clone::Clone for ESimWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HotspotAuthenticationContext = *mut ::core::ffi::c_void;
pub type HotspotAuthenticationEventDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct HotspotAuthenticationResponseCode(pub i32);
impl HotspotAuthenticationResponseCode {
    pub const NoError: Self = Self(0i32);
    pub const LoginSucceeded: Self = Self(50i32);
    pub const LoginFailed: Self = Self(100i32);
    pub const RadiusServerError: Self = Self(102i32);
    pub const NetworkAdministratorError: Self = Self(105i32);
    pub const LoginAborted: Self = Self(151i32);
    pub const AccessGatewayInternalError: Self = Self(255i32);
}
impl ::core::marker::Copy for HotspotAuthenticationResponseCode {}
impl ::core::clone::Clone for HotspotAuthenticationResponseCode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HotspotCredentialsAuthenticationResult = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IESim {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AvailableMemoryInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AvailableMemoryInBytes: usize,
    pub Eid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FirmwareVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MobileBroadbandModemDeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Policy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ESimState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetProfiles: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetProfiles: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteProfileAsync: unsafe extern "system" fn(this: *mut *mut Self, profileid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteProfileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadProfileMetadataAsync: unsafe extern "system" fn(this: *mut *mut Self, activationcode: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadProfileMetadataAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ResetAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResetAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ProfileChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProfileChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProfileChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProfileChanged: usize,
}
#[repr(C)]
pub struct IESim2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Discover: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DiscoverWithServerAddressAndMatchingId: unsafe extern "system" fn(this: *mut *mut Self, serveraddress: ::windows_sys::core::HSTRING, matchingid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DiscoverAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DiscoverAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DiscoverWithServerAddressAndMatchingIdAsync: unsafe extern "system" fn(this: *mut *mut Self, serveraddress: ::windows_sys::core::HSTRING, matchingid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DiscoverWithServerAddressAndMatchingIdAsync: usize,
}
#[repr(C)]
pub struct IESimAddedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ESim: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IESimDiscoverEvent {
    pub base__: ::windows_sys::core::IInspectable,
    pub MatchingId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RspServerAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IESimDiscoverResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Events: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Events: usize,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ESimDiscoverResultKind) -> ::windows_sys::core::HRESULT,
    pub ProfileMetadata: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Result: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IESimDownloadProfileMetadataResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Result: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ProfileMetadata: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IESimManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ServiceInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryCreateESimWatcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ServiceInfoChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServiceInfoChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveServiceInfoChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveServiceInfoChanged: usize,
}
#[repr(C)]
pub struct IESimOperationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ESimOperationStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IESimPolicy {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShouldEnableManagingUi: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IESimProfile {
    pub base__: ::windows_sys::core::IInspectable,
    pub Class: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ESimProfileClass) -> ::windows_sys::core::HRESULT,
    pub Nickname: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Policy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ProviderIcon: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ProviderIcon: usize,
    pub ProviderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ProviderName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ESimProfileState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DisableAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub EnableAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetNicknameAsync: unsafe extern "system" fn(this: *mut *mut Self, newnickname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetNicknameAsync: usize,
}
#[repr(C)]
pub struct IESimProfileMetadata {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsConfirmationCodeRequired: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Policy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ProviderIcon: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ProviderIcon: usize,
    pub ProviderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ProviderName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ESimProfileMetadataState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DenyInstallAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DenyInstallAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ConfirmInstallAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConfirmInstallAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ConfirmInstallWithConfirmationCodeAsync: unsafe extern "system" fn(this: *mut *mut Self, confirmationcode: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConfirmInstallWithConfirmationCodeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PostponeInstallAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PostponeInstallAsync: usize,
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
pub struct IESimProfilePolicy {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanDelete: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanDisable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsManagedByEnterprise: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IESimRemovedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ESim: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IESimServiceInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub AuthenticationPreference: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ESimAuthenticationPreference) -> ::windows_sys::core::HRESULT,
    pub IsESimUiEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IESimUpdatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ESim: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IESimWatcher {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ESimWatcherStatus) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
}
#[repr(C)]
pub struct IFdnAccessManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestUnlockAsync: unsafe extern "system" fn(this: *mut *mut Self, contactlistid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestUnlockAsync: usize,
}
#[repr(C)]
pub struct IHotspotAuthenticationContext {
    pub base__: ::windows_sys::core::IInspectable,
    pub WirelessNetworkId: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Networking_Connectivity")]
    pub NetworkAdapter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    NetworkAdapter: usize,
    #[cfg(feature = "Foundation")]
    pub RedirectMessageUrl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RedirectMessageUrl: usize,
    #[cfg(feature = "Data_Xml_Dom")]
    pub RedirectMessageXml: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    RedirectMessageXml: usize,
    #[cfg(feature = "Foundation")]
    pub AuthenticationUrl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticationUrl: usize,
    pub IssueCredentials: unsafe extern "system" fn(this: *mut *mut Self, username: ::windows_sys::core::HSTRING, password: ::windows_sys::core::HSTRING, extraparameters: ::windows_sys::core::HSTRING, markasmanualconnectonfailure: bool) -> ::windows_sys::core::HRESULT,
    pub AbortAuthentication: unsafe extern "system" fn(this: *mut *mut Self, markasmanual: bool) -> ::windows_sys::core::HRESULT,
    pub SkipAuthentication: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub TriggerAttentionRequired: unsafe extern "system" fn(this: *mut *mut Self, packagerelativeapplicationid: ::windows_sys::core::HSTRING, applicationparameters: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHotspotAuthenticationContext2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub IssueCredentialsAsync: unsafe extern "system" fn(this: *mut *mut Self, username: ::windows_sys::core::HSTRING, password: ::windows_sys::core::HSTRING, extraparameters: ::windows_sys::core::HSTRING, markasmanualconnectonfailure: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IssueCredentialsAsync: usize,
}
#[repr(C)]
pub struct IHotspotAuthenticationContextStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryGetAuthenticationContext: unsafe extern "system" fn(this: *mut *mut Self, eventoken: ::windows_sys::core::HSTRING, context: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHotspotAuthenticationEventDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub EventToken: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHotspotCredentialsAuthenticationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub HasNetworkErrorOccurred: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ResponseCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HotspotAuthenticationResponseCode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LogoffUrl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LogoffUrl: usize,
    #[cfg(feature = "Data_Xml_Dom")]
    pub AuthenticationReplyXml: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    AuthenticationReplyXml: usize,
}
#[repr(C)]
pub struct IKnownCSimFilePathsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub EFSpn: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFSpn: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid1: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid2: usize,
}
#[repr(C)]
pub struct IKnownRuimFilePathsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub EFSpn: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFSpn: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid1: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid2: usize,
}
#[repr(C)]
pub struct IKnownSimFilePathsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub EFOns: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFOns: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub EFSpn: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFSpn: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid1: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid2: usize,
}
#[repr(C)]
pub struct IKnownUSimFilePathsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub EFSpn: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFSpn: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub EFOpl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFOpl: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub EFPnn: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFPnn: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid1: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid2: usize,
}
#[repr(C)]
pub struct IMobileBroadbandAccount {
    pub base__: ::windows_sys::core::IInspectable,
    pub NetworkAccountId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ServiceProviderGuid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ServiceProviderName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CurrentNetwork: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CurrentDeviceInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandAccount2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking_Connectivity"))]
    pub GetConnectionProfiles: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Networking_Connectivity")))]
    GetConnectionProfiles: usize,
}
#[repr(C)]
pub struct IMobileBroadbandAccount3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AccountExperienceUrl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AccountExperienceUrl: usize,
}
#[repr(C)]
pub struct IMobileBroadbandAccountEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub NetworkAccountId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandAccountStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AvailableNetworkAccountIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AvailableNetworkAccountIds: usize,
    pub CreateFromNetworkAccountId: unsafe extern "system" fn(this: *mut *mut Self, networkaccountid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandAccountUpdatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub NetworkAccountId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HasDeviceInformationChanged: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub HasNetworkChanged: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandAccountWatcher {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AccountAdded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AccountAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccountAdded: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccountAdded: usize,
    #[cfg(feature = "Foundation")]
    pub AccountUpdated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AccountUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccountUpdated: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccountUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub AccountRemoved: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AccountRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccountRemoved: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccountRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MobileBroadbandAccountWatcherStatus) -> ::windows_sys::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandAntennaSar {
    pub base__: ::windows_sys::core::IInspectable,
    pub AntennaIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SarBackoffIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandAntennaSarFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithIndex: unsafe extern "system" fn(this: *mut *mut Self, antennaindex: i32, sarbackoffindex: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandCellCdma {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub BaseStationId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseStationId: usize,
    #[cfg(feature = "Foundation")]
    pub BaseStationPNCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseStationPNCode: usize,
    #[cfg(feature = "Foundation")]
    pub BaseStationLatitude: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseStationLatitude: usize,
    #[cfg(feature = "Foundation")]
    pub BaseStationLongitude: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseStationLongitude: usize,
    #[cfg(feature = "Foundation")]
    pub BaseStationLastBroadcastGpsTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseStationLastBroadcastGpsTime: usize,
    #[cfg(feature = "Foundation")]
    pub NetworkId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NetworkId: usize,
    #[cfg(feature = "Foundation")]
    pub PilotSignalStrengthInDB: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PilotSignalStrengthInDB: usize,
    #[cfg(feature = "Foundation")]
    pub SystemId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemId: usize,
}
#[repr(C)]
pub struct IMobileBroadbandCellGsm {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub BaseStationId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseStationId: usize,
    #[cfg(feature = "Foundation")]
    pub CellId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CellId: usize,
    #[cfg(feature = "Foundation")]
    pub ChannelNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChannelNumber: usize,
    #[cfg(feature = "Foundation")]
    pub LocationAreaCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LocationAreaCode: usize,
    pub ProviderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReceivedSignalStrengthInDBm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReceivedSignalStrengthInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub TimingAdvanceInBitPeriods: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimingAdvanceInBitPeriods: usize,
}
#[repr(C)]
pub struct IMobileBroadbandCellLte {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CellId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CellId: usize,
    #[cfg(feature = "Foundation")]
    pub ChannelNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChannelNumber: usize,
    #[cfg(feature = "Foundation")]
    pub PhysicalCellId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhysicalCellId: usize,
    pub ProviderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReferenceSignalReceivedPowerInDBm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReferenceSignalReceivedPowerInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub ReferenceSignalReceivedQualityInDBm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReferenceSignalReceivedQualityInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub TimingAdvanceInBitPeriods: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimingAdvanceInBitPeriods: usize,
    #[cfg(feature = "Foundation")]
    pub TrackingAreaCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrackingAreaCode: usize,
}
#[repr(C)]
pub struct IMobileBroadbandCellNR {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CellId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CellId: usize,
    #[cfg(feature = "Foundation")]
    pub ChannelNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChannelNumber: usize,
    #[cfg(feature = "Foundation")]
    pub PhysicalCellId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhysicalCellId: usize,
    pub ProviderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReferenceSignalReceivedPowerInDBm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReferenceSignalReceivedPowerInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub ReferenceSignalReceivedQualityInDBm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReferenceSignalReceivedQualityInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub TimingAdvanceInNanoseconds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimingAdvanceInNanoseconds: usize,
    #[cfg(feature = "Foundation")]
    pub TrackingAreaCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrackingAreaCode: usize,
    #[cfg(feature = "Foundation")]
    pub SignalToNoiseRatioInDB: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignalToNoiseRatioInDB: usize,
}
#[repr(C)]
pub struct IMobileBroadbandCellTdscdma {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CellId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CellId: usize,
    #[cfg(feature = "Foundation")]
    pub CellParameterId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CellParameterId: usize,
    #[cfg(feature = "Foundation")]
    pub ChannelNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChannelNumber: usize,
    #[cfg(feature = "Foundation")]
    pub LocationAreaCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LocationAreaCode: usize,
    #[cfg(feature = "Foundation")]
    pub PathLossInDB: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PathLossInDB: usize,
    pub ProviderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReceivedSignalCodePowerInDBm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReceivedSignalCodePowerInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub TimingAdvanceInBitPeriods: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimingAdvanceInBitPeriods: usize,
}
#[repr(C)]
pub struct IMobileBroadbandCellUmts {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CellId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CellId: usize,
    #[cfg(feature = "Foundation")]
    pub ChannelNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChannelNumber: usize,
    #[cfg(feature = "Foundation")]
    pub LocationAreaCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LocationAreaCode: usize,
    #[cfg(feature = "Foundation")]
    pub PathLossInDB: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PathLossInDB: usize,
    #[cfg(feature = "Foundation")]
    pub PrimaryScramblingCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrimaryScramblingCode: usize,
    pub ProviderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReceivedSignalCodePowerInDBm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReceivedSignalCodePowerInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub SignalToNoiseRatioInDB: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignalToNoiseRatioInDB: usize,
}
#[repr(C)]
pub struct IMobileBroadbandCellsInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub NeighboringCellsCdma: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NeighboringCellsCdma: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub NeighboringCellsGsm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NeighboringCellsGsm: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub NeighboringCellsLte: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NeighboringCellsLte: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub NeighboringCellsTdscdma: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NeighboringCellsTdscdma: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub NeighboringCellsUmts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NeighboringCellsUmts: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ServingCellsCdma: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServingCellsCdma: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ServingCellsGsm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServingCellsGsm: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ServingCellsLte: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServingCellsLte: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ServingCellsTdscdma: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServingCellsTdscdma: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ServingCellsUmts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServingCellsUmts: usize,
}
#[repr(C)]
pub struct IMobileBroadbandCellsInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub NeighboringCellsNR: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NeighboringCellsNR: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ServingCellsNR: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServingCellsNR: usize,
}
#[repr(C)]
pub struct IMobileBroadbandCurrentSlotIndexChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub CurrentSlotIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandDeviceInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub NetworkDeviceStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NetworkDeviceStatus) -> ::windows_sys::core::HRESULT,
    pub Manufacturer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Model: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FirmwareInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Sms")]
    pub CellularClass: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Devices::Sms::CellularClass) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Sms"))]
    CellularClass: usize,
    pub DataClasses: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DataClasses) -> ::windows_sys::core::HRESULT,
    pub CustomDataClass: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MobileEquipmentId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TelephoneNumbers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TelephoneNumbers: usize,
    pub SubscriberId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SimIccId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DeviceType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MobileBroadbandDeviceType) -> ::windows_sys::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CurrentRadioState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MobileBroadbandRadioState) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandDeviceInformation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PinManager: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Revision: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SerialNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandDeviceInformation3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SimSpn: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SimPnn: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SimGid1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandDeviceInformation4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SlotManager: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandDeviceService {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceServiceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCommands: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCommands: usize,
    pub OpenDataSession: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OpenCommandSession: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandDeviceServiceCommandResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub StatusCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ResponseData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ResponseData: usize,
}
#[repr(C)]
pub struct IMobileBroadbandDeviceServiceCommandSession {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SendQueryCommandAsync: unsafe extern "system" fn(this: *mut *mut Self, commandid: u32, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SendQueryCommandAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SendSetCommandAsync: unsafe extern "system" fn(this: *mut *mut Self, commandid: u32, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SendSetCommandAsync: usize,
    pub CloseSession: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandDeviceServiceDataReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub ReceivedData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ReceivedData: usize,
}
#[repr(C)]
pub struct IMobileBroadbandDeviceServiceDataSession {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub WriteDataAsync: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    WriteDataAsync: usize,
    pub CloseSession: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DataReceived: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDataReceived: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDataReceived: usize,
}
#[repr(C)]
pub struct IMobileBroadbandDeviceServiceInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceServiceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub IsDataReadSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsDataWriteSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandDeviceServiceTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DeviceServiceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ReceivedData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ReceivedData: usize,
}
#[repr(C)]
pub struct IMobileBroadbandModem {
    pub base__: ::windows_sys::core::IInspectable,
    pub CurrentAccount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DeviceInformation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MaxDeviceServiceCommandSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MaxDeviceServiceDataSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DeviceServices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeviceServices: usize,
    pub GetDeviceService: unsafe extern "system" fn(this: *mut *mut Self, deviceserviceid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsResetSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ResetAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResetAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCurrentConfigurationAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCurrentConfigurationAsync: usize,
    pub CurrentNetwork: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandModem2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetIsPassthroughEnabledAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsPassthroughEnabledAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetIsPassthroughEnabledAsync: unsafe extern "system" fn(this: *mut *mut Self, value: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIsPassthroughEnabledAsync: usize,
}
#[repr(C)]
pub struct IMobileBroadbandModem3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TryGetPcoAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetPcoAsync: usize,
    pub IsInEmergencyCallMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsInEmergencyCallModeChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsInEmergencyCallModeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsInEmergencyCallModeChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsInEmergencyCallModeChanged: usize,
}
#[repr(C)]
pub struct IMobileBroadbandModemConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    pub Uicc: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HomeProviderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HomeProviderName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandModemConfiguration2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SarManager: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandModemIsolation {
    pub base__: ::windows_sys::core::IInspectable,
    pub AddAllowedHost: unsafe extern "system" fn(this: *mut *mut Self, host: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddAllowedHostRange: unsafe extern "system" fn(this: *mut *mut Self, first: *mut ::core::ffi::c_void, last: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ApplyConfigurationAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplyConfigurationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ClearConfigurationAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearConfigurationAsync: usize,
}
#[repr(C)]
pub struct IMobileBroadbandModemIsolationFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, modemdeviceid: ::windows_sys::core::HSTRING, rulegroupid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandModemStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FromId: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandNetwork {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Networking_Connectivity")]
    pub NetworkAdapter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    NetworkAdapter: usize,
    pub NetworkRegistrationState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NetworkRegistrationState) -> ::windows_sys::core::HRESULT,
    pub RegistrationNetworkError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub PacketAttachNetworkError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ActivationNetworkError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub AccessPointName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RegisteredDataClass: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DataClasses) -> ::windows_sys::core::HRESULT,
    pub RegisteredProviderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RegisteredProviderName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ShowConnectionUI: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandNetwork2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetVoiceCallSupportAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetVoiceCallSupportAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RegistrationUiccApps: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegistrationUiccApps: usize,
}
#[repr(C)]
pub struct IMobileBroadbandNetwork3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetCellsInfoAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCellsInfoAsync: usize,
}
#[repr(C)]
pub struct IMobileBroadbandNetworkRegistrationStateChange {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Network: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub NetworkRegistrationStateChanges: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NetworkRegistrationStateChanges: usize,
}
#[repr(C)]
pub struct IMobileBroadbandPco {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    pub IsComplete: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandPcoDataChangeTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub UpdatedData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandPin {
    pub base__: ::windows_sys::core::IInspectable,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MobileBroadbandPinType) -> ::windows_sys::core::HRESULT,
    pub LockState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MobileBroadbandPinLockState) -> ::windows_sys::core::HRESULT,
    pub Format: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MobileBroadbandPinFormat) -> ::windows_sys::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MaxLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MinLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub AttemptsRemaining: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnableAsync: unsafe extern "system" fn(this: *mut *mut Self, currentpin: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisableAsync: unsafe extern "system" fn(this: *mut *mut Self, currentpin: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub EnterAsync: unsafe extern "system" fn(this: *mut *mut Self, currentpin: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnterAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ChangeAsync: unsafe extern "system" fn(this: *mut *mut Self, currentpin: ::windows_sys::core::HSTRING, newpin: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChangeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UnblockAsync: unsafe extern "system" fn(this: *mut *mut Self, pinunblockkey: ::windows_sys::core::HSTRING, newpin: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnblockAsync: usize,
}
#[repr(C)]
pub struct IMobileBroadbandPinLockStateChange {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PinType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MobileBroadbandPinType) -> ::windows_sys::core::HRESULT,
    pub PinLockState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MobileBroadbandPinLockState) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandPinLockStateChangeTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub PinLockStateChanges: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PinLockStateChanges: usize,
}
#[repr(C)]
pub struct IMobileBroadbandPinManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPins: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPins: usize,
    pub GetPin: unsafe extern "system" fn(this: *mut *mut Self, pintype: MobileBroadbandPinType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandPinOperationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSuccessful: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AttemptsRemaining: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandRadioStateChange {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RadioState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MobileBroadbandRadioState) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandRadioStateChangeTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub RadioStateChanges: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RadioStateChanges: usize,
}
#[repr(C)]
pub struct IMobileBroadbandSarManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsBackoffEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsWiFiHardwareIntegrated: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsSarControlledByHardware: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Antennas: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Antennas: usize,
    #[cfg(feature = "Foundation")]
    pub HysteresisTimerPeriod: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HysteresisTimerPeriod: usize,
    #[cfg(feature = "Foundation")]
    pub TransmissionStateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransmissionStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTransmissionStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTransmissionStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub EnableBackoffAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnableBackoffAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisableBackoffAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableBackoffAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetConfigurationAsync: unsafe extern "system" fn(this: *mut *mut Self, antennas: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetConfigurationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RevertSarToHardwareControlAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RevertSarToHardwareControlAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetTransmissionStateChangedHysteresisAsync: unsafe extern "system" fn(this: *mut *mut Self, timerperiod: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTransmissionStateChangedHysteresisAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetIsTransmittingAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsTransmittingAsync: usize,
    pub StartTransmissionStateMonitoring: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub StopTransmissionStateMonitoring: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandSlotInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub Index: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MobileBroadbandSlotState) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandSlotInfoChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SlotInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandSlotManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub SlotInfos: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SlotInfos: usize,
    pub CurrentSlotIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetCurrentSlot: unsafe extern "system" fn(this: *mut *mut Self, slotindex: i32, result__: *mut MobileBroadbandModemStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetCurrentSlotAsync: unsafe extern "system" fn(this: *mut *mut Self, slotindex: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCurrentSlotAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SlotInfoChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SlotInfoChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSlotInfoChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSlotInfoChanged: usize,
    #[cfg(feature = "Foundation")]
    pub CurrentSlotIndexChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CurrentSlotIndexChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCurrentSlotIndexChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCurrentSlotIndexChanged: usize,
}
#[repr(C)]
pub struct IMobileBroadbandTransmissionStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTransmitting: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandUicc {
    pub base__: ::windows_sys::core::IInspectable,
    pub SimIccId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetUiccAppsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetUiccAppsAsync: usize,
}
#[repr(C)]
pub struct IMobileBroadbandUiccApp {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Id: usize,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UiccAppKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRecordDetailsAsync: unsafe extern "system" fn(this: *mut *mut Self, uiccfilepath: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRecordDetailsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadRecordAsync: unsafe extern "system" fn(this: *mut *mut Self, uiccfilepath: *mut ::core::ffi::c_void, recordindex: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadRecordAsync: usize,
}
#[repr(C)]
pub struct IMobileBroadbandUiccAppReadRecordResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MobileBroadbandUiccAppOperationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
}
#[repr(C)]
pub struct IMobileBroadbandUiccAppRecordDetailsResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MobileBroadbandUiccAppOperationStatus) -> ::windows_sys::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UiccAppRecordKind) -> ::windows_sys::core::HRESULT,
    pub RecordCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RecordSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ReadAccessCondition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UiccAccessCondition) -> ::windows_sys::core::HRESULT,
    pub WriteAccessCondition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UiccAccessCondition) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMobileBroadbandUiccAppsResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MobileBroadbandUiccAppOperationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub UiccApps: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UiccApps: usize,
}
#[repr(C)]
pub struct INetworkOperatorDataUsageTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub NotificationKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NetworkOperatorDataUsageNotificationKind) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INetworkOperatorNotificationEventDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub NotificationType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NetworkOperatorEventMessageType) -> ::windows_sys::core::HRESULT,
    pub NetworkAccountId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EncodingType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RuleId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Sms")]
    pub SmsMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Sms"))]
    SmsMessage: usize,
}
#[repr(C)]
pub struct INetworkOperatorTetheringAccessPointConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    pub Ssid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSsid: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Passphrase: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPassphrase: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INetworkOperatorTetheringAccessPointConfiguration2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsBandSupported: unsafe extern "system" fn(this: *mut *mut Self, band: TetheringWiFiBand, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsBandSupportedAsync: unsafe extern "system" fn(this: *mut *mut Self, band: TetheringWiFiBand, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsBandSupportedAsync: usize,
    pub Band: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TetheringWiFiBand) -> ::windows_sys::core::HRESULT,
    pub SetBand: unsafe extern "system" fn(this: *mut *mut Self, value: TetheringWiFiBand) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INetworkOperatorTetheringClient {
    pub base__: ::windows_sys::core::IInspectable,
    pub MacAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub HostNames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    HostNames: usize,
}
#[repr(C)]
pub struct INetworkOperatorTetheringClientManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTetheringClients: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTetheringClients: usize,
}
#[repr(C)]
pub struct INetworkOperatorTetheringEntitlementCheck {
    pub base__: ::windows_sys::core::IInspectable,
    pub AuthorizeTethering: unsafe extern "system" fn(this: *mut *mut Self, allow: bool, entitlementfailurereason: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INetworkOperatorTetheringManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxClientCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ClientCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub TetheringOperationalState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TetheringOperationalState) -> ::windows_sys::core::HRESULT,
    pub GetCurrentAccessPointConfiguration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ConfigureAccessPointAsync: unsafe extern "system" fn(this: *mut *mut Self, configuration: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConfigureAccessPointAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StartTetheringAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTetheringAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopTetheringAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopTetheringAsync: usize,
}
#[repr(C)]
pub struct INetworkOperatorTetheringManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetTetheringCapability: unsafe extern "system" fn(this: *mut *mut Self, networkaccountid: ::windows_sys::core::HSTRING, result__: *mut TetheringCapability) -> ::windows_sys::core::HRESULT,
    pub CreateFromNetworkAccountId: unsafe extern "system" fn(this: *mut *mut Self, networkaccountid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INetworkOperatorTetheringManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Networking_Connectivity")]
    pub GetTetheringCapabilityFromConnectionProfile: unsafe extern "system" fn(this: *mut *mut Self, profile: *mut ::core::ffi::c_void, result__: *mut TetheringCapability) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    GetTetheringCapabilityFromConnectionProfile: usize,
    #[cfg(feature = "Networking_Connectivity")]
    pub CreateFromConnectionProfile: unsafe extern "system" fn(this: *mut *mut Self, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    CreateFromConnectionProfile: usize,
}
#[repr(C)]
pub struct INetworkOperatorTetheringManagerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Networking_Connectivity")]
    pub CreateFromConnectionProfileWithTargetAdapter: unsafe extern "system" fn(this: *mut *mut Self, profile: *mut ::core::ffi::c_void, adapter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    CreateFromConnectionProfileWithTargetAdapter: usize,
}
#[repr(C)]
pub struct INetworkOperatorTetheringManagerStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsNoConnectionsTimeoutEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub EnableNoConnectionsTimeout: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnableNoConnectionsTimeoutAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnableNoConnectionsTimeoutAsync: usize,
    pub DisableNoConnectionsTimeout: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DisableNoConnectionsTimeoutAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableNoConnectionsTimeoutAsync: usize,
}
#[repr(C)]
pub struct INetworkOperatorTetheringOperationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TetheringOperationStatus) -> ::windows_sys::core::HRESULT,
    pub AdditionalErrorMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProvisionFromXmlDocumentResults {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllElementsProvisioned: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ProvisionResultsXml: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProvisionedProfile {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Networking_Connectivity")]
    pub UpdateCost: unsafe extern "system" fn(this: *mut *mut Self, value: super::Connectivity::NetworkCostType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    UpdateCost: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateUsage: unsafe extern "system" fn(this: *mut *mut Self, value: ProfileUsage) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateUsage: usize,
}
#[repr(C)]
pub struct IProvisioningAgent {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ProvisionFromXmlDocumentAsync: unsafe extern "system" fn(this: *mut *mut Self, provisioningxmldocument: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProvisionFromXmlDocumentAsync: usize,
    pub GetProvisionedProfile: unsafe extern "system" fn(this: *mut *mut Self, mediatype: ProfileMediaType, profilename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProvisioningAgentStaticMethods {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromNetworkAccountId: unsafe extern "system" fn(this: *mut *mut Self, networkaccountid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITetheringEntitlementCheckTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub NetworkAccountId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AllowTethering: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub DenyTethering: unsafe extern "system" fn(this: *mut *mut Self, entitlementfailurereason: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUssdMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub DataCodingScheme: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetDataCodingScheme: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
    pub GetPayload: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetPayload: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const u8) -> ::windows_sys::core::HRESULT,
    pub PayloadAsText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPayloadAsText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUssdMessageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateMessage: unsafe extern "system" fn(this: *mut *mut Self, messagetext: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUssdReply {
    pub base__: ::windows_sys::core::IInspectable,
    pub ResultCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UssdResultCode) -> ::windows_sys::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUssdSession {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SendMessageAndGetReplyAsync: unsafe extern "system" fn(this: *mut *mut Self, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendMessageAndGetReplyAsync: usize,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUssdSessionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromNetworkAccountId: unsafe extern "system" fn(this: *mut *mut Self, networkaccountid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromNetworkInterfaceId: unsafe extern "system" fn(this: *mut *mut Self, networkinterfaceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type MobileBroadbandAccount = *mut ::core::ffi::c_void;
pub type MobileBroadbandAccountEventArgs = *mut ::core::ffi::c_void;
pub type MobileBroadbandAccountUpdatedEventArgs = *mut ::core::ffi::c_void;
pub type MobileBroadbandAccountWatcher = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandAccountWatcherStatus(pub i32);
impl MobileBroadbandAccountWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopped: Self = Self(3i32);
    pub const Aborted: Self = Self(4i32);
}
impl ::core::marker::Copy for MobileBroadbandAccountWatcherStatus {}
impl ::core::clone::Clone for MobileBroadbandAccountWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MobileBroadbandAntennaSar = *mut ::core::ffi::c_void;
pub type MobileBroadbandCellCdma = *mut ::core::ffi::c_void;
pub type MobileBroadbandCellGsm = *mut ::core::ffi::c_void;
pub type MobileBroadbandCellLte = *mut ::core::ffi::c_void;
pub type MobileBroadbandCellNR = *mut ::core::ffi::c_void;
pub type MobileBroadbandCellTdscdma = *mut ::core::ffi::c_void;
pub type MobileBroadbandCellUmts = *mut ::core::ffi::c_void;
pub type MobileBroadbandCellsInfo = *mut ::core::ffi::c_void;
pub type MobileBroadbandCurrentSlotIndexChangedEventArgs = *mut ::core::ffi::c_void;
pub type MobileBroadbandDeviceInformation = *mut ::core::ffi::c_void;
pub type MobileBroadbandDeviceService = *mut ::core::ffi::c_void;
pub type MobileBroadbandDeviceServiceCommandResult = *mut ::core::ffi::c_void;
pub type MobileBroadbandDeviceServiceCommandSession = *mut ::core::ffi::c_void;
pub type MobileBroadbandDeviceServiceDataReceivedEventArgs = *mut ::core::ffi::c_void;
pub type MobileBroadbandDeviceServiceDataSession = *mut ::core::ffi::c_void;
pub type MobileBroadbandDeviceServiceInformation = *mut ::core::ffi::c_void;
pub type MobileBroadbandDeviceServiceTriggerDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandDeviceType(pub i32);
impl MobileBroadbandDeviceType {
    pub const Unknown: Self = Self(0i32);
    pub const Embedded: Self = Self(1i32);
    pub const Removable: Self = Self(2i32);
    pub const Remote: Self = Self(3i32);
}
impl ::core::marker::Copy for MobileBroadbandDeviceType {}
impl ::core::clone::Clone for MobileBroadbandDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MobileBroadbandModem = *mut ::core::ffi::c_void;
pub type MobileBroadbandModemConfiguration = *mut ::core::ffi::c_void;
pub type MobileBroadbandModemIsolation = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandModemStatus(pub i32);
impl MobileBroadbandModemStatus {
    pub const Success: Self = Self(0i32);
    pub const OtherFailure: Self = Self(1i32);
    pub const Busy: Self = Self(2i32);
    pub const NoDeviceSupport: Self = Self(3i32);
}
impl ::core::marker::Copy for MobileBroadbandModemStatus {}
impl ::core::clone::Clone for MobileBroadbandModemStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MobileBroadbandNetwork = *mut ::core::ffi::c_void;
pub type MobileBroadbandNetworkRegistrationStateChange = *mut ::core::ffi::c_void;
pub type MobileBroadbandNetworkRegistrationStateChangeTriggerDetails = *mut ::core::ffi::c_void;
pub type MobileBroadbandPco = *mut ::core::ffi::c_void;
pub type MobileBroadbandPcoDataChangeTriggerDetails = *mut ::core::ffi::c_void;
pub type MobileBroadbandPin = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandPinFormat(pub i32);
impl MobileBroadbandPinFormat {
    pub const Unknown: Self = Self(0i32);
    pub const Numeric: Self = Self(1i32);
    pub const Alphanumeric: Self = Self(2i32);
}
impl ::core::marker::Copy for MobileBroadbandPinFormat {}
impl ::core::clone::Clone for MobileBroadbandPinFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandPinLockState(pub i32);
impl MobileBroadbandPinLockState {
    pub const Unknown: Self = Self(0i32);
    pub const Unlocked: Self = Self(1i32);
    pub const PinRequired: Self = Self(2i32);
    pub const PinUnblockKeyRequired: Self = Self(3i32);
}
impl ::core::marker::Copy for MobileBroadbandPinLockState {}
impl ::core::clone::Clone for MobileBroadbandPinLockState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MobileBroadbandPinLockStateChange = *mut ::core::ffi::c_void;
pub type MobileBroadbandPinLockStateChangeTriggerDetails = *mut ::core::ffi::c_void;
pub type MobileBroadbandPinManager = *mut ::core::ffi::c_void;
pub type MobileBroadbandPinOperationResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandPinType(pub i32);
impl MobileBroadbandPinType {
    pub const None: Self = Self(0i32);
    pub const Custom: Self = Self(1i32);
    pub const Pin1: Self = Self(2i32);
    pub const Pin2: Self = Self(3i32);
    pub const SimPin: Self = Self(4i32);
    pub const FirstSimPin: Self = Self(5i32);
    pub const NetworkPin: Self = Self(6i32);
    pub const NetworkSubsetPin: Self = Self(7i32);
    pub const ServiceProviderPin: Self = Self(8i32);
    pub const CorporatePin: Self = Self(9i32);
    pub const SubsidyLock: Self = Self(10i32);
}
impl ::core::marker::Copy for MobileBroadbandPinType {}
impl ::core::clone::Clone for MobileBroadbandPinType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandRadioState(pub i32);
impl MobileBroadbandRadioState {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
}
impl ::core::marker::Copy for MobileBroadbandRadioState {}
impl ::core::clone::Clone for MobileBroadbandRadioState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MobileBroadbandRadioStateChange = *mut ::core::ffi::c_void;
pub type MobileBroadbandRadioStateChangeTriggerDetails = *mut ::core::ffi::c_void;
pub type MobileBroadbandSarManager = *mut ::core::ffi::c_void;
pub type MobileBroadbandSlotInfo = *mut ::core::ffi::c_void;
pub type MobileBroadbandSlotInfoChangedEventArgs = *mut ::core::ffi::c_void;
pub type MobileBroadbandSlotManager = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandSlotState(pub i32);
impl MobileBroadbandSlotState {
    pub const Unmanaged: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const OffEmpty: Self = Self(2i32);
    pub const Off: Self = Self(3i32);
    pub const Empty: Self = Self(4i32);
    pub const NotReady: Self = Self(5i32);
    pub const Active: Self = Self(6i32);
    pub const Error: Self = Self(7i32);
    pub const ActiveEsim: Self = Self(8i32);
    pub const ActiveEsimNoProfile: Self = Self(9i32);
}
impl ::core::marker::Copy for MobileBroadbandSlotState {}
impl ::core::clone::Clone for MobileBroadbandSlotState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MobileBroadbandTransmissionStateChangedEventArgs = *mut ::core::ffi::c_void;
pub type MobileBroadbandUicc = *mut ::core::ffi::c_void;
pub type MobileBroadbandUiccApp = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandUiccAppOperationStatus(pub i32);
impl MobileBroadbandUiccAppOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const InvalidUiccFilePath: Self = Self(1i32);
    pub const AccessConditionNotHeld: Self = Self(2i32);
    pub const UiccBusy: Self = Self(3i32);
}
impl ::core::marker::Copy for MobileBroadbandUiccAppOperationStatus {}
impl ::core::clone::Clone for MobileBroadbandUiccAppOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MobileBroadbandUiccAppReadRecordResult = *mut ::core::ffi::c_void;
pub type MobileBroadbandUiccAppRecordDetailsResult = *mut ::core::ffi::c_void;
pub type MobileBroadbandUiccAppsResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct NetworkDeviceStatus(pub i32);
impl NetworkDeviceStatus {
    pub const DeviceNotReady: Self = Self(0i32);
    pub const DeviceReady: Self = Self(1i32);
    pub const SimNotInserted: Self = Self(2i32);
    pub const BadSim: Self = Self(3i32);
    pub const DeviceHardwareFailure: Self = Self(4i32);
    pub const AccountNotActivated: Self = Self(5i32);
    pub const DeviceLocked: Self = Self(6i32);
    pub const DeviceBlocked: Self = Self(7i32);
}
impl ::core::marker::Copy for NetworkDeviceStatus {}
impl ::core::clone::Clone for NetworkDeviceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct NetworkOperatorDataUsageNotificationKind(pub i32);
impl NetworkOperatorDataUsageNotificationKind {
    pub const DataUsageProgress: Self = Self(0i32);
}
impl ::core::marker::Copy for NetworkOperatorDataUsageNotificationKind {}
impl ::core::clone::Clone for NetworkOperatorDataUsageNotificationKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NetworkOperatorDataUsageTriggerDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct NetworkOperatorEventMessageType(pub i32);
impl NetworkOperatorEventMessageType {
    pub const Gsm: Self = Self(0i32);
    pub const Cdma: Self = Self(1i32);
    pub const Ussd: Self = Self(2i32);
    pub const DataPlanThresholdReached: Self = Self(3i32);
    pub const DataPlanReset: Self = Self(4i32);
    pub const DataPlanDeleted: Self = Self(5i32);
    pub const ProfileConnected: Self = Self(6i32);
    pub const ProfileDisconnected: Self = Self(7i32);
    pub const RegisteredRoaming: Self = Self(8i32);
    pub const RegisteredHome: Self = Self(9i32);
    pub const TetheringEntitlementCheck: Self = Self(10i32);
    pub const TetheringOperationalStateChanged: Self = Self(11i32);
    pub const TetheringNumberOfClientsChanged: Self = Self(12i32);
}
impl ::core::marker::Copy for NetworkOperatorEventMessageType {}
impl ::core::clone::Clone for NetworkOperatorEventMessageType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NetworkOperatorNotificationEventDetails = *mut ::core::ffi::c_void;
pub type NetworkOperatorTetheringAccessPointConfiguration = *mut ::core::ffi::c_void;
pub type NetworkOperatorTetheringClient = *mut ::core::ffi::c_void;
pub type NetworkOperatorTetheringManager = *mut ::core::ffi::c_void;
pub type NetworkOperatorTetheringOperationResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct NetworkRegistrationState(pub i32);
impl NetworkRegistrationState {
    pub const None: Self = Self(0i32);
    pub const Deregistered: Self = Self(1i32);
    pub const Searching: Self = Self(2i32);
    pub const Home: Self = Self(3i32);
    pub const Roaming: Self = Self(4i32);
    pub const Partner: Self = Self(5i32);
    pub const Denied: Self = Self(6i32);
}
impl ::core::marker::Copy for NetworkRegistrationState {}
impl ::core::clone::Clone for NetworkRegistrationState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct ProfileMediaType(pub i32);
impl ProfileMediaType {
    pub const Wlan: Self = Self(0i32);
    pub const Wwan: Self = Self(1i32);
}
impl ::core::marker::Copy for ProfileMediaType {}
impl ::core::clone::Clone for ProfileMediaType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Networking_NetworkOperators\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct ProfileUsage {
    pub UsageInMegabytes: u32,
    pub LastSyncTime: super::super::Foundation::DateTime,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for ProfileUsage {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for ProfileUsage {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ProvisionFromXmlDocumentResults = *mut ::core::ffi::c_void;
pub type ProvisionedProfile = *mut ::core::ffi::c_void;
pub type ProvisioningAgent = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct TetheringCapability(pub i32);
impl TetheringCapability {
    pub const Enabled: Self = Self(0i32);
    pub const DisabledByGroupPolicy: Self = Self(1i32);
    pub const DisabledByHardwareLimitation: Self = Self(2i32);
    pub const DisabledByOperator: Self = Self(3i32);
    pub const DisabledBySku: Self = Self(4i32);
    pub const DisabledByRequiredAppNotInstalled: Self = Self(5i32);
    pub const DisabledDueToUnknownCause: Self = Self(6i32);
    pub const DisabledBySystemCapability: Self = Self(7i32);
}
impl ::core::marker::Copy for TetheringCapability {}
impl ::core::clone::Clone for TetheringCapability {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TetheringEntitlementCheckTriggerDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct TetheringOperationStatus(pub i32);
impl TetheringOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const MobileBroadbandDeviceOff: Self = Self(2i32);
    pub const WiFiDeviceOff: Self = Self(3i32);
    pub const EntitlementCheckTimeout: Self = Self(4i32);
    pub const EntitlementCheckFailure: Self = Self(5i32);
    pub const OperationInProgress: Self = Self(6i32);
    pub const BluetoothDeviceOff: Self = Self(7i32);
    pub const NetworkLimitedConnectivity: Self = Self(8i32);
}
impl ::core::marker::Copy for TetheringOperationStatus {}
impl ::core::clone::Clone for TetheringOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct TetheringOperationalState(pub i32);
impl TetheringOperationalState {
    pub const Unknown: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Off: Self = Self(2i32);
    pub const InTransition: Self = Self(3i32);
}
impl ::core::marker::Copy for TetheringOperationalState {}
impl ::core::clone::Clone for TetheringOperationalState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct TetheringWiFiBand(pub i32);
impl TetheringWiFiBand {
    pub const Auto: Self = Self(0i32);
    pub const TwoPointFourGigahertz: Self = Self(1i32);
    pub const FiveGigahertz: Self = Self(2i32);
}
impl ::core::marker::Copy for TetheringWiFiBand {}
impl ::core::clone::Clone for TetheringWiFiBand {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct UiccAccessCondition(pub i32);
impl UiccAccessCondition {
    pub const AlwaysAllowed: Self = Self(0i32);
    pub const Pin1: Self = Self(1i32);
    pub const Pin2: Self = Self(2i32);
    pub const Pin3: Self = Self(3i32);
    pub const Pin4: Self = Self(4i32);
    pub const Administrative5: Self = Self(5i32);
    pub const Administrative6: Self = Self(6i32);
    pub const NeverAllowed: Self = Self(7i32);
}
impl ::core::marker::Copy for UiccAccessCondition {}
impl ::core::clone::Clone for UiccAccessCondition {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct UiccAppKind(pub i32);
impl UiccAppKind {
    pub const Unknown: Self = Self(0i32);
    pub const MF: Self = Self(1i32);
    pub const MFSim: Self = Self(2i32);
    pub const MFRuim: Self = Self(3i32);
    pub const USim: Self = Self(4i32);
    pub const CSim: Self = Self(5i32);
    pub const ISim: Self = Self(6i32);
}
impl ::core::marker::Copy for UiccAppKind {}
impl ::core::clone::Clone for UiccAppKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct UiccAppRecordKind(pub i32);
impl UiccAppRecordKind {
    pub const Unknown: Self = Self(0i32);
    pub const Transparent: Self = Self(1i32);
    pub const RecordOriented: Self = Self(2i32);
}
impl ::core::marker::Copy for UiccAppRecordKind {}
impl ::core::clone::Clone for UiccAppRecordKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UssdMessage = *mut ::core::ffi::c_void;
pub type UssdReply = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Networking_NetworkOperators\"`*"]
#[repr(transparent)]
pub struct UssdResultCode(pub i32);
impl UssdResultCode {
    pub const NoActionRequired: Self = Self(0i32);
    pub const ActionRequired: Self = Self(1i32);
    pub const Terminated: Self = Self(2i32);
    pub const OtherLocalClient: Self = Self(3i32);
    pub const OperationNotSupported: Self = Self(4i32);
    pub const NetworkTimeout: Self = Self(5i32);
}
impl ::core::marker::Copy for UssdResultCode {}
impl ::core::clone::Clone for UssdResultCode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UssdSession = *mut ::core::ffi::c_void;
