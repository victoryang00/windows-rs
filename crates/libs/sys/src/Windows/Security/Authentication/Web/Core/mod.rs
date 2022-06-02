pub type FindAllAccountsResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
#[repr(transparent)]
pub struct FindAllWebAccountsStatus(pub i32);
impl FindAllWebAccountsStatus {
    pub const Success: Self = Self(0i32);
    pub const NotAllowedByProvider: Self = Self(1i32);
    pub const NotSupportedByProvider: Self = Self(2i32);
    pub const ProviderError: Self = Self(3i32);
}
impl ::core::marker::Copy for FindAllWebAccountsStatus {}
impl ::core::clone::Clone for FindAllWebAccountsStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IFindAllAccountsResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub Accounts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    Accounts: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FindAllWebAccountsStatus) -> ::windows_sys::core::HRESULT,
    pub ProviderError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebAccountEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub Account: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    Account: usize,
}
#[repr(C)]
pub struct IWebAccountMonitor {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub DefaultSignInAccountChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DefaultSignInAccountChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDefaultSignInAccountChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDefaultSignInAccountChanged: usize,
}
#[repr(C)]
pub struct IWebAccountMonitor2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AccountPictureUpdated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AccountPictureUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccountPictureUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccountPictureUpdated: usize,
}
#[repr(C)]
pub struct IWebAuthenticationCoreManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetTokenSilentlyAsync: unsafe extern "system" fn(this: *mut *mut Self, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetTokenSilentlyAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub GetTokenSilentlyWithWebAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, request: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    GetTokenSilentlyWithWebAccountAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestTokenAsync: unsafe extern "system" fn(this: *mut *mut Self, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestTokenAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub RequestTokenWithWebAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, request: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    RequestTokenWithWebAccountAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub FindAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, provider: *mut ::core::ffi::c_void, webaccountid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    FindAccountAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub FindAccountProviderAsync: unsafe extern "system" fn(this: *mut *mut Self, webaccountproviderid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    FindAccountProviderAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub FindAccountProviderWithAuthorityAsync: unsafe extern "system" fn(this: *mut *mut Self, webaccountproviderid: ::windows_sys::core::HSTRING, authority: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    FindAccountProviderWithAuthorityAsync: usize,
}
#[repr(C)]
pub struct IWebAuthenticationCoreManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "System"))]
    pub FindAccountProviderWithAuthorityForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, webaccountproviderid: ::windows_sys::core::HSTRING, authority: ::windows_sys::core::HSTRING, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials", feature = "System")))]
    FindAccountProviderWithAuthorityForUserAsync: usize,
}
#[repr(C)]
pub struct IWebAuthenticationCoreManagerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub CreateWebAccountMonitor: unsafe extern "system" fn(this: *mut *mut Self, webaccounts: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    CreateWebAccountMonitor: usize,
}
#[repr(C)]
pub struct IWebAuthenticationCoreManagerStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub FindAllAccountsAsync: unsafe extern "system" fn(this: *mut *mut Self, provider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    FindAllAccountsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub FindAllAccountsWithClientIdAsync: unsafe extern "system" fn(this: *mut *mut Self, provider: *mut ::core::ffi::c_void, clientid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    FindAllAccountsWithClientIdAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub FindSystemAccountProviderAsync: unsafe extern "system" fn(this: *mut *mut Self, webaccountproviderid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    FindSystemAccountProviderAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub FindSystemAccountProviderWithAuthorityAsync: unsafe extern "system" fn(this: *mut *mut Self, webaccountproviderid: ::windows_sys::core::HSTRING, authority: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    FindSystemAccountProviderWithAuthorityAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "System"))]
    pub FindSystemAccountProviderWithAuthorityForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, webaccountproviderid: ::windows_sys::core::HSTRING, authority: ::windows_sys::core::HSTRING, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials", feature = "System")))]
    FindSystemAccountProviderWithAuthorityForUserAsync: usize,
}
#[repr(C)]
pub struct IWebProviderError {
    pub base__: ::windows_sys::core::IInspectable,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ErrorMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct IWebProviderErrorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, errorcode: u32, errormessage: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebTokenRequest {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccountProvider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccountProvider: usize,
    pub Scope: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ClientId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PromptType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WebTokenRequestPromptType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct IWebTokenRequest2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AppProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppProperties: usize,
}
#[repr(C)]
pub struct IWebTokenRequest3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CorrelationId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCorrelationId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebTokenRequestFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, provider: *mut ::core::ffi::c_void, scope: ::windows_sys::core::HSTRING, clientid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    Create: usize,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWithPromptType: unsafe extern "system" fn(this: *mut *mut Self, provider: *mut ::core::ffi::c_void, scope: ::windows_sys::core::HSTRING, clientid: ::windows_sys::core::HSTRING, prompttype: WebTokenRequestPromptType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWithPromptType: usize,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWithProvider: unsafe extern "system" fn(this: *mut *mut Self, provider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWithProvider: usize,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWithScope: unsafe extern "system" fn(this: *mut *mut Self, provider: *mut ::core::ffi::c_void, scope: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWithScope: usize,
}
#[repr(C)]
pub struct IWebTokenRequestResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ResponseData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResponseData: usize,
    pub ResponseStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WebTokenRequestStatus) -> ::windows_sys::core::HRESULT,
    pub ResponseError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InvalidateCacheAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InvalidateCacheAsync: usize,
}
#[repr(C)]
pub struct IWebTokenResponse {
    pub base__: ::windows_sys::core::IInspectable,
    pub Token: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ProviderError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct IWebTokenResponseFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithToken: unsafe extern "system" fn(this: *mut *mut Self, token: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWithTokenAndAccount: unsafe extern "system" fn(this: *mut *mut Self, token: ::windows_sys::core::HSTRING, webaccount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWithTokenAndAccount: usize,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWithTokenAccountAndError: unsafe extern "system" fn(this: *mut *mut Self, token: ::windows_sys::core::HSTRING, webaccount: *mut ::core::ffi::c_void, error: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWithTokenAccountAndError: usize,
}
pub type WebAccountEventArgs = *mut ::core::ffi::c_void;
pub type WebAccountMonitor = *mut ::core::ffi::c_void;
pub type WebProviderError = *mut ::core::ffi::c_void;
pub type WebTokenRequest = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
#[repr(transparent)]
pub struct WebTokenRequestPromptType(pub i32);
impl WebTokenRequestPromptType {
    pub const Default: Self = Self(0i32);
    pub const ForceAuthentication: Self = Self(1i32);
}
impl ::core::marker::Copy for WebTokenRequestPromptType {}
impl ::core::clone::Clone for WebTokenRequestPromptType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WebTokenRequestResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
#[repr(transparent)]
pub struct WebTokenRequestStatus(pub i32);
impl WebTokenRequestStatus {
    pub const Success: Self = Self(0i32);
    pub const UserCancel: Self = Self(1i32);
    pub const AccountSwitch: Self = Self(2i32);
    pub const UserInteractionRequired: Self = Self(3i32);
    pub const AccountProviderNotAvailable: Self = Self(4i32);
    pub const ProviderError: Self = Self(5i32);
}
impl ::core::marker::Copy for WebTokenRequestStatus {}
impl ::core::clone::Clone for WebTokenRequestStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WebTokenResponse = *mut ::core::ffi::c_void;
