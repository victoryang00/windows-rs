#[repr(C)]
pub struct IMicrosoftAccountMultiFactorAuthenticationManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetOneTimePassCodeAsync: unsafe extern "system" fn(this: *mut *mut Self, useraccountid: ::windows_sys::core::HSTRING, codelength: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetOneTimePassCodeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AddDeviceAsync: unsafe extern "system" fn(this: *mut *mut Self, useraccountid: ::windows_sys::core::HSTRING, authenticationtoken: ::windows_sys::core::HSTRING, wnschannelid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddDeviceAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDeviceAsync: unsafe extern "system" fn(this: *mut *mut Self, useraccountid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDeviceAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateWnsChannelAsync: unsafe extern "system" fn(this: *mut *mut Self, useraccountid: ::windows_sys::core::HSTRING, channeluri: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateWnsChannelAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSessionsAsync: unsafe extern "system" fn(this: *mut *mut Self, useraccountidlist: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSessionsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSessionsAndUnregisteredAccountsAsync: unsafe extern "system" fn(this: *mut *mut Self, useraccountidlist: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSessionsAndUnregisteredAccountsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ApproveSessionUsingAuthSessionInfoAsync: unsafe extern "system" fn(this: *mut *mut Self, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, authenticationsessioninfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApproveSessionUsingAuthSessionInfoAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ApproveSessionAsync: unsafe extern "system" fn(this: *mut *mut Self, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, useraccountid: ::windows_sys::core::HSTRING, sessionid: ::windows_sys::core::HSTRING, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApproveSessionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DenySessionUsingAuthSessionInfoAsync: unsafe extern "system" fn(this: *mut *mut Self, authenticationsessioninfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DenySessionUsingAuthSessionInfoAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DenySessionAsync: unsafe extern "system" fn(this: *mut *mut Self, useraccountid: ::windows_sys::core::HSTRING, sessionid: ::windows_sys::core::HSTRING, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DenySessionAsync: usize,
}
#[repr(C)]
pub struct IMicrosoftAccountMultiFactorAuthenticatorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Current: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMicrosoftAccountMultiFactorGetSessionsResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Sessions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Sessions: usize,
    pub ServiceResponse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MicrosoftAccountMultiFactorServiceResponse) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMicrosoftAccountMultiFactorOneTimeCodedInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub Code: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TimeInterval: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeInterval: usize,
    #[cfg(feature = "Foundation")]
    pub TimeToLive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeToLive: usize,
    pub ServiceResponse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MicrosoftAccountMultiFactorServiceResponse) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMicrosoftAccountMultiFactorSessionInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub UserAccountId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SessionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplaySessionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ApprovalStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MicrosoftAccountMultiFactorSessionApprovalStatus) -> ::windows_sys::core::HRESULT,
    pub AuthenticationType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MicrosoftAccountMultiFactorAuthenticationType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestTime: usize,
    #[cfg(feature = "Foundation")]
    pub ExpirationTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationTime: usize,
}
#[repr(C)]
pub struct IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Sessions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Sessions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UnregisteredAccounts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UnregisteredAccounts: usize,
    pub ServiceResponse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MicrosoftAccountMultiFactorServiceResponse) -> ::windows_sys::core::HRESULT,
}
pub type MicrosoftAccountMultiFactorAuthenticationManager = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Authentication_Identity_Core\"`*"]
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorAuthenticationType(pub i32);
impl MicrosoftAccountMultiFactorAuthenticationType {
    pub const User: Self = Self(0i32);
    pub const Device: Self = Self(1i32);
}
impl ::core::marker::Copy for MicrosoftAccountMultiFactorAuthenticationType {}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorAuthenticationType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MicrosoftAccountMultiFactorGetSessionsResult = *mut ::core::ffi::c_void;
pub type MicrosoftAccountMultiFactorOneTimeCodedInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Authentication_Identity_Core\"`*"]
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorServiceResponse(pub i32);
impl MicrosoftAccountMultiFactorServiceResponse {
    pub const Success: Self = Self(0i32);
    pub const Error: Self = Self(1i32);
    pub const NoNetworkConnection: Self = Self(2i32);
    pub const ServiceUnavailable: Self = Self(3i32);
    pub const TotpSetupDenied: Self = Self(4i32);
    pub const NgcNotSetup: Self = Self(5i32);
    pub const SessionAlreadyDenied: Self = Self(6i32);
    pub const SessionAlreadyApproved: Self = Self(7i32);
    pub const SessionExpired: Self = Self(8i32);
    pub const NgcNonceExpired: Self = Self(9i32);
    pub const InvalidSessionId: Self = Self(10i32);
    pub const InvalidSessionType: Self = Self(11i32);
    pub const InvalidOperation: Self = Self(12i32);
    pub const InvalidStateTransition: Self = Self(13i32);
    pub const DeviceNotFound: Self = Self(14i32);
    pub const FlowDisabled: Self = Self(15i32);
    pub const SessionNotApproved: Self = Self(16i32);
    pub const OperationCanceledByUser: Self = Self(17i32);
    pub const NgcDisabledByServer: Self = Self(18i32);
    pub const NgcKeyNotFoundOnServer: Self = Self(19i32);
    pub const UIRequired: Self = Self(20i32);
    pub const DeviceIdChanged: Self = Self(21i32);
}
impl ::core::marker::Copy for MicrosoftAccountMultiFactorServiceResponse {}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorServiceResponse {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_Authentication_Identity_Core\"`*"]
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorSessionApprovalStatus(pub i32);
impl MicrosoftAccountMultiFactorSessionApprovalStatus {
    pub const Pending: Self = Self(0i32);
    pub const Approved: Self = Self(1i32);
    pub const Denied: Self = Self(2i32);
}
impl ::core::marker::Copy for MicrosoftAccountMultiFactorSessionApprovalStatus {}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorSessionApprovalStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_Authentication_Identity_Core\"`*"]
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorSessionAuthenticationStatus(pub i32);
impl MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    pub const Authenticated: Self = Self(0i32);
    pub const Unauthenticated: Self = Self(1i32);
}
impl ::core::marker::Copy for MicrosoftAccountMultiFactorSessionAuthenticationStatus {}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MicrosoftAccountMultiFactorSessionInfo = *mut ::core::ffi::c_void;
pub type MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo = *mut ::core::ffi::c_void;
