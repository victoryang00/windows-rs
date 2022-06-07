pub type EasClientDeviceInformation = *mut ::core::ffi::c_void;
pub type EasClientSecurityPolicy = *mut ::core::ffi::c_void;
pub type EasComplianceResults = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
pub struct EasDisallowConvenienceLogonResult(pub i32);
impl EasDisallowConvenienceLogonResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
}
impl ::core::marker::Copy for EasDisallowConvenienceLogonResult {}
impl ::core::clone::Clone for EasDisallowConvenienceLogonResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
pub struct EasEncryptionProviderType(pub i32);
impl EasEncryptionProviderType {
    pub const NotEvaluated: Self = Self(0i32);
    pub const WindowsEncryption: Self = Self(1i32);
    pub const OtherEncryption: Self = Self(2i32);
}
impl ::core::marker::Copy for EasEncryptionProviderType {}
impl ::core::clone::Clone for EasEncryptionProviderType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
pub struct EasMaxInactivityTimeLockResult(pub i32);
impl EasMaxInactivityTimeLockResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
    pub const InvalidParameter: Self = Self(4i32);
}
impl ::core::marker::Copy for EasMaxInactivityTimeLockResult {}
impl ::core::clone::Clone for EasMaxInactivityTimeLockResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
pub struct EasMaxPasswordFailedAttemptsResult(pub i32);
impl EasMaxPasswordFailedAttemptsResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
    pub const InvalidParameter: Self = Self(4i32);
}
impl ::core::marker::Copy for EasMaxPasswordFailedAttemptsResult {}
impl ::core::clone::Clone for EasMaxPasswordFailedAttemptsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
pub struct EasMinPasswordComplexCharactersResult(pub i32);
impl EasMinPasswordComplexCharactersResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
    pub const RequestedPolicyNotEnforceable: Self = Self(4i32);
    pub const InvalidParameter: Self = Self(5i32);
    pub const CurrentUserHasBlankPassword: Self = Self(6i32);
    pub const AdminsHaveBlankPassword: Self = Self(7i32);
    pub const UserCannotChangePassword: Self = Self(8i32);
    pub const AdminsCannotChangePassword: Self = Self(9i32);
    pub const LocalControlledUsersCannotChangePassword: Self = Self(10i32);
    pub const ConnectedAdminsProviderPolicyIsWeak: Self = Self(11i32);
    pub const ConnectedUserProviderPolicyIsWeak: Self = Self(12i32);
    pub const ChangeConnectedAdminsPassword: Self = Self(13i32);
    pub const ChangeConnectedUserPassword: Self = Self(14i32);
}
impl ::core::marker::Copy for EasMinPasswordComplexCharactersResult {}
impl ::core::clone::Clone for EasMinPasswordComplexCharactersResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
pub struct EasMinPasswordLengthResult(pub i32);
impl EasMinPasswordLengthResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
    pub const RequestedPolicyNotEnforceable: Self = Self(4i32);
    pub const InvalidParameter: Self = Self(5i32);
    pub const CurrentUserHasBlankPassword: Self = Self(6i32);
    pub const AdminsHaveBlankPassword: Self = Self(7i32);
    pub const UserCannotChangePassword: Self = Self(8i32);
    pub const AdminsCannotChangePassword: Self = Self(9i32);
    pub const LocalControlledUsersCannotChangePassword: Self = Self(10i32);
    pub const ConnectedAdminsProviderPolicyIsWeak: Self = Self(11i32);
    pub const ConnectedUserProviderPolicyIsWeak: Self = Self(12i32);
    pub const ChangeConnectedAdminsPassword: Self = Self(13i32);
    pub const ChangeConnectedUserPassword: Self = Self(14i32);
}
impl ::core::marker::Copy for EasMinPasswordLengthResult {}
impl ::core::clone::Clone for EasMinPasswordLengthResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
pub struct EasPasswordExpirationResult(pub i32);
impl EasPasswordExpirationResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
    pub const RequestedExpirationIncompatible: Self = Self(4i32);
    pub const InvalidParameter: Self = Self(5i32);
    pub const UserCannotChangePassword: Self = Self(6i32);
    pub const AdminsCannotChangePassword: Self = Self(7i32);
    pub const LocalControlledUsersCannotChangePassword: Self = Self(8i32);
}
impl ::core::marker::Copy for EasPasswordExpirationResult {}
impl ::core::clone::Clone for EasPasswordExpirationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
pub struct EasPasswordHistoryResult(pub i32);
impl EasPasswordHistoryResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const RequestedPolicyIsStricter: Self = Self(3i32);
    pub const InvalidParameter: Self = Self(4i32);
}
impl ::core::marker::Copy for EasPasswordHistoryResult {}
impl ::core::clone::Clone for EasPasswordHistoryResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_ExchangeActiveSyncProvisioning\"`*"]
#[repr(transparent)]
pub struct EasRequireEncryptionResult(pub i32);
impl EasRequireEncryptionResult {
    pub const NotEvaluated: Self = Self(0i32);
    pub const Compliant: Self = Self(1i32);
    pub const CanBeCompliant: Self = Self(2i32);
    pub const NotProvisionedOnAllVolumes: Self = Self(3i32);
    pub const DeFixedDataNotSupported: Self = Self(4i32);
    pub const FixedDataNotSupported: Self = Self(4i32);
    pub const DeHardwareNotCompliant: Self = Self(5i32);
    pub const HardwareNotCompliant: Self = Self(5i32);
    pub const DeWinReNotConfigured: Self = Self(6i32);
    pub const LockNotConfigured: Self = Self(6i32);
    pub const DeProtectionSuspended: Self = Self(7i32);
    pub const ProtectionSuspended: Self = Self(7i32);
    pub const DeOsVolumeNotProtected: Self = Self(8i32);
    pub const OsVolumeNotProtected: Self = Self(8i32);
    pub const DeProtectionNotYetEnabled: Self = Self(9i32);
    pub const ProtectionNotYetEnabled: Self = Self(9i32);
    pub const NoFeatureLicense: Self = Self(10i32);
    pub const OsNotProtected: Self = Self(11i32);
    pub const UnexpectedFailure: Self = Self(12i32);
}
impl ::core::marker::Copy for EasRequireEncryptionResult {}
impl ::core::clone::Clone for EasRequireEncryptionResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IEasClientDeviceInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub OperatingSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SystemManufacturer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SystemProductName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SystemSku: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEasClientDeviceInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1423956353, data2: 6504, data3: 19619, data4: [185, 88, 229, 149, 209, 101, 5, 235] };
}
#[repr(C)]
pub struct IEasClientDeviceInformation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SystemHardwareVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SystemFirmwareVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEasClientDeviceInformation2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4289943843, data2: 47910, data3: 19818, data4: [129, 188, 22, 90, 238, 10, 215, 84] };
}
#[repr(C)]
pub struct IEasClientSecurityPolicy {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequireEncryption: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetRequireEncryption: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub MinPasswordLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetMinPasswordLength: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
    pub DisallowConvenienceLogon: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetDisallowConvenienceLogon: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub MinPasswordComplexCharacters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetMinPasswordComplexCharacters: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PasswordExpiration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PasswordExpiration: usize,
    #[cfg(feature = "Foundation")]
    pub SetPasswordExpiration: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPasswordExpiration: usize,
    pub PasswordHistory: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetPasswordHistory: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub MaxPasswordFailedAttempts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub SetMaxPasswordFailedAttempts: unsafe extern "system" fn(this: *mut *mut Self, value: u8) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MaxInactivityTimeLock: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxInactivityTimeLock: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxInactivityTimeLock: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxInactivityTimeLock: usize,
    pub CheckCompliance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ApplyAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplyAsync: usize,
}
impl ::windows_sys::core::Interface for IEasClientSecurityPolicy {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1169630050, data2: 57274, data3: 19099, data4: [172, 237, 111, 226, 173, 203, 100, 32] };
}
#[repr(C)]
pub struct IEasComplianceResults {
    pub base__: ::windows_sys::core::IInspectable,
    pub Compliant: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub RequireEncryptionResult: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EasRequireEncryptionResult) -> ::windows_sys::core::HRESULT,
    pub MinPasswordLengthResult: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EasMinPasswordLengthResult) -> ::windows_sys::core::HRESULT,
    pub DisallowConvenienceLogonResult: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EasDisallowConvenienceLogonResult) -> ::windows_sys::core::HRESULT,
    pub MinPasswordComplexCharactersResult: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EasMinPasswordComplexCharactersResult) -> ::windows_sys::core::HRESULT,
    pub PasswordExpirationResult: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EasPasswordExpirationResult) -> ::windows_sys::core::HRESULT,
    pub PasswordHistoryResult: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EasPasswordHistoryResult) -> ::windows_sys::core::HRESULT,
    pub MaxPasswordFailedAttemptsResult: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EasMaxPasswordFailedAttemptsResult) -> ::windows_sys::core::HRESULT,
    pub MaxInactivityTimeLockResult: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EasMaxInactivityTimeLockResult) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEasComplianceResults {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1178347932, data2: 32537, data3: 19558, data4: [180, 3, 203, 69, 221, 87, 162, 179] };
}
#[repr(C)]
pub struct IEasComplianceResults2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub EncryptionProviderType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EasEncryptionProviderType) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEasComplianceResults2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 801005769, data2: 6824, data3: 18421, data4: [136, 187, 203, 62, 240, 191, 251, 21] };
}
