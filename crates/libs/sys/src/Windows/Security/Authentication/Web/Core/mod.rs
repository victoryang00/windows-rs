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
impl ::windows_sys::core::Interface for IFindAllAccountsResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2776705885, data2: 46894, data3: 16908, data4: [134, 171, 170, 192, 215, 183, 38, 31] };
}
#[repr(C)]
pub struct IWebAccountEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub Account: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    Account: usize,
}
impl ::windows_sys::core::Interface for IWebAccountEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1874264957, data2: 16974, data3: 17644, data4: [151, 124, 239, 36, 21, 70, 42, 90] };
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
impl ::windows_sys::core::Interface for IWebAccountMonitor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1950742013, data2: 43677, data3: 17945, data4: [141, 93, 193, 56, 164, 237, 227, 229] };
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
impl ::windows_sys::core::Interface for IWebAccountMonitor2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2813182456, data2: 9400, data3: 20225, data4: [154, 229, 36, 84, 94, 113, 35, 58] };
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
impl ::windows_sys::core::Interface for IWebAuthenticationCoreManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1791655058, data2: 42369, data3: 17529, data4: [156, 16, 117, 46, 255, 68, 253, 52] };
}
#[repr(C)]
pub struct IWebAuthenticationCoreManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "System"))]
    pub FindAccountProviderWithAuthorityForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, webaccountproviderid: ::windows_sys::core::HSTRING, authority: ::windows_sys::core::HSTRING, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials", feature = "System")))]
    FindAccountProviderWithAuthorityForUserAsync: usize,
}
impl ::windows_sys::core::Interface for IWebAuthenticationCoreManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4119074890, data2: 35671, data3: 18464, data4: [182, 164, 112, 165, 182, 252, 244, 74] };
}
#[repr(C)]
pub struct IWebAuthenticationCoreManagerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub CreateWebAccountMonitor: unsafe extern "system" fn(this: *mut *mut Self, webaccounts: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    CreateWebAccountMonitor: usize,
}
impl ::windows_sys::core::Interface for IWebAuthenticationCoreManagerStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 604303026, data2: 35108, data3: 19859, data4: [171, 58, 153, 104, 139, 65, 157, 86] };
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
impl ::windows_sys::core::Interface for IWebAuthenticationCoreManagerStatics4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1424372734, data2: 38624, data3: 16872, data4: [152, 50, 18, 152, 137, 124, 42, 175] };
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
impl ::windows_sys::core::Interface for IWebProviderError {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3675855793, data2: 20677, data3: 18441, data4: [141, 202, 9, 201, 148, 16, 36, 92] };
}
#[repr(C)]
pub struct IWebProviderErrorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, errorcode: u32, errormessage: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebProviderErrorFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3821275693, data2: 35311, data3: 20023, data4: [132, 127, 168, 185, 213, 163, 41, 16] };
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
impl ::windows_sys::core::Interface for IWebTokenRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3078311272, data2: 44491, data3: 18035, data4: [179, 100, 12, 247, 179, 92, 175, 151] };
}
#[repr(C)]
pub struct IWebTokenRequest2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AppProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppProperties: usize,
}
impl ::windows_sys::core::Interface for IWebTokenRequest2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3607150713, data2: 12488, data3: 17303, data4: [150, 84, 150, 28, 59, 232, 184, 85] };
}
#[repr(C)]
pub struct IWebTokenRequest3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CorrelationId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCorrelationId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebTokenRequest3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1517640529, data2: 15281, data3: 16805, data4: [166, 61, 144, 188, 50, 199, 219, 154] };
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
impl ::windows_sys::core::Interface for IWebTokenRequestFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1827804188, data2: 4080, data3: 19559, data4: [184, 79, 153, 221, 190, 74, 114, 201] };
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
impl ::windows_sys::core::Interface for IWebTokenRequestResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3240788741, data2: 53752, data3: 17539, data4: [141, 84, 56, 254, 41, 39, 132, 255] };
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
impl ::windows_sys::core::Interface for IWebTokenResponse {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1739048394, data2: 33782, data3: 17606, data4: [163, 177, 14, 182, 158, 65, 250, 138] };
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
impl ::windows_sys::core::Interface for IWebTokenResponseFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2875979768, data2: 21584, data3: 20214, data4: [151, 247, 5, 43, 4, 49, 192, 240] };
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
