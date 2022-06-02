#[doc = "*Required features: `\"Security_Authentication_OnlineId\"`*"]
#[repr(transparent)]
pub struct CredentialPromptType(pub i32);
impl CredentialPromptType {
    pub const PromptIfNeeded: Self = Self(0i32);
    pub const RetypeCredentials: Self = Self(1i32);
    pub const DoNotPrompt: Self = Self(2i32);
}
impl ::core::marker::Copy for CredentialPromptType {}
impl ::core::clone::Clone for CredentialPromptType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IOnlineIdAuthenticator {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AuthenticateUserAsync: unsafe extern "system" fn(this: *mut *mut Self, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticateUserAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AuthenticateUserAsyncAdvanced: unsafe extern "system" fn(this: *mut *mut Self, requests: *mut ::core::ffi::c_void, credentialprompttype: CredentialPromptType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AuthenticateUserAsyncAdvanced: usize,
    #[cfg(feature = "Foundation")]
    pub SignOutUserAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignOutUserAsync: usize,
    pub SetApplicationId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ApplicationId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub CanSignOut: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AuthenticatedSafeCustomerId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOnlineIdServiceTicket {
    pub base__: ::windows_sys::core::IInspectable,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOnlineIdServiceTicketRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub Service: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Policy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOnlineIdServiceTicketRequestFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateOnlineIdServiceTicketRequest: unsafe extern "system" fn(this: *mut *mut Self, service: ::windows_sys::core::HSTRING, policy: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateOnlineIdServiceTicketRequestAdvanced: unsafe extern "system" fn(this: *mut *mut Self, service: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOnlineIdSystemAuthenticatorForUser {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetTicketAsync: unsafe extern "system" fn(this: *mut *mut Self, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetTicketAsync: usize,
    pub SetApplicationId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ApplicationId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[repr(C)]
pub struct IOnlineIdSystemAuthenticatorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Default: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[repr(C)]
pub struct IOnlineIdSystemIdentity {
    pub base__: ::windows_sys::core::IInspectable,
    pub Ticket: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IOnlineIdSystemTicketResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Identity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut OnlineIdSystemTicketStatus) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUserIdentity {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Tickets: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Tickets: usize,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SafeCustomerId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SignInName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FirstName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LastName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsBetaAccount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsConfirmedPC: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
pub type OnlineIdAuthenticator = *mut ::core::ffi::c_void;
pub type OnlineIdServiceTicket = *mut ::core::ffi::c_void;
pub type OnlineIdServiceTicketRequest = *mut ::core::ffi::c_void;
pub type OnlineIdSystemAuthenticatorForUser = *mut ::core::ffi::c_void;
pub type OnlineIdSystemIdentity = *mut ::core::ffi::c_void;
pub type OnlineIdSystemTicketResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Authentication_OnlineId\"`*"]
#[repr(transparent)]
pub struct OnlineIdSystemTicketStatus(pub i32);
impl OnlineIdSystemTicketStatus {
    pub const Success: Self = Self(0i32);
    pub const Error: Self = Self(1i32);
    pub const ServiceConnectionError: Self = Self(2i32);
}
impl ::core::marker::Copy for OnlineIdSystemTicketStatus {}
impl ::core::clone::Clone for OnlineIdSystemTicketStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SignOutUserOperation = *mut ::core::ffi::c_void;
pub type UserAuthenticationOperation = *mut ::core::ffi::c_void;
pub type UserIdentity = *mut ::core::ffi::c_void;
