#[repr(C)]
pub struct IWebAccountClientView {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ApplicationCallbackUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplicationCallbackUri: usize,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WebAccountClientViewType) -> ::windows_sys::core::HRESULT,
    pub AccountPairwiseId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebAccountClientViewFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, viewtype: WebAccountClientViewType, applicationcallbackuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
    #[cfg(feature = "Foundation")]
    pub CreateWithPairwiseId: unsafe extern "system" fn(this: *mut *mut Self, viewtype: WebAccountClientViewType, applicationcallbackuri: *mut ::core::ffi::c_void, accountpairwiseid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWithPairwiseId: usize,
}
#[repr(C)]
pub struct IWebAccountManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub UpdateWebAccountPropertiesAsync: unsafe extern "system" fn(this: *mut *mut Self, webaccount: *mut ::core::ffi::c_void, webaccountusername: ::windows_sys::core::HSTRING, additionalproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    UpdateWebAccountPropertiesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub AddWebAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, webaccountid: ::windows_sys::core::HSTRING, webaccountusername: ::windows_sys::core::HSTRING, props: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    AddWebAccountAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub DeleteWebAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, webaccount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    DeleteWebAccountAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub FindAllProviderWebAccountsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    FindAllProviderWebAccountsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Http"))]
    pub PushCookiesAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, cookies: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Web_Http")))]
    PushCookiesAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub SetViewAsync: unsafe extern "system" fn(this: *mut *mut Self, webaccount: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    SetViewAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub ClearViewAsync: unsafe extern "system" fn(this: *mut *mut Self, webaccount: *mut ::core::ffi::c_void, applicationcallbackuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    ClearViewAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub GetViewsAsync: unsafe extern "system" fn(this: *mut *mut Self, webaccount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    GetViewsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "Storage_Streams"))]
    pub SetWebAccountPictureAsync: unsafe extern "system" fn(this: *mut *mut Self, webaccount: *mut ::core::ffi::c_void, webaccountpicture: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials", feature = "Storage_Streams")))]
    SetWebAccountPictureAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub ClearWebAccountPictureAsync: unsafe extern "system" fn(this: *mut *mut Self, webaccount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    ClearWebAccountPictureAsync: usize,
}
#[repr(C)]
pub struct IWebAccountManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PullCookiesAsync: unsafe extern "system" fn(this: *mut *mut Self, uristring: ::windows_sys::core::HSTRING, callerpfn: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PullCookiesAsync: usize,
}
#[repr(C)]
pub struct IWebAccountManagerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub FindAllProviderWebAccountsForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System")))]
    FindAllProviderWebAccountsForUserAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub AddWebAccountForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, webaccountid: ::windows_sys::core::HSTRING, webaccountusername: ::windows_sys::core::HSTRING, props: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System")))]
    AddWebAccountForUserAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub AddWebAccountWithScopeForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, webaccountid: ::windows_sys::core::HSTRING, webaccountusername: ::windows_sys::core::HSTRING, props: *mut ::core::ffi::c_void, scope: WebAccountScope, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System")))]
    AddWebAccountWithScopeForUserAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub AddWebAccountWithScopeAndMapForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, webaccountid: ::windows_sys::core::HSTRING, webaccountusername: ::windows_sys::core::HSTRING, props: *mut ::core::ffi::c_void, scope: WebAccountScope, peruserwebaccountid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System")))]
    AddWebAccountWithScopeAndMapForUserAsync: usize,
}
#[repr(C)]
pub struct IWebAccountManagerStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub InvalidateAppCacheForAllAccountsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InvalidateAppCacheForAllAccountsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub InvalidateAppCacheForAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, webaccount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    InvalidateAppCacheForAccountAsync: usize,
}
#[repr(C)]
pub struct IWebAccountMapManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub AddWebAccountWithScopeAndMapAsync: unsafe extern "system" fn(this: *mut *mut Self, webaccountid: ::windows_sys::core::HSTRING, webaccountusername: ::windows_sys::core::HSTRING, props: *mut ::core::ffi::c_void, scope: WebAccountScope, peruserwebaccountid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    AddWebAccountWithScopeAndMapAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub SetPerAppToPerUserAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, perappaccount: *mut ::core::ffi::c_void, peruserwebaccountid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    SetPerAppToPerUserAccountAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub GetPerUserFromPerAppAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, perappaccount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    GetPerUserFromPerAppAccountAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub ClearPerUserFromPerAppAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, perappaccount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    ClearPerUserFromPerAppAccountAsync: usize,
}
#[repr(C)]
pub struct IWebAccountProviderAddAccountOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebAccountProviderBaseReportOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub ReportError: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Core"))]
    ReportError: usize,
}
#[repr(C)]
pub struct IWebAccountProviderDeleteAccountOperation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccount: usize,
}
#[repr(C)]
pub struct IWebAccountProviderManageAccountOperation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccount: usize,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebAccountProviderOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WebAccountProviderOperationKind) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebAccountProviderRetrieveCookiesOperation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Context: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Context: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Http"))]
    pub Cookies: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Web_Http")))]
    Cookies: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub ApplicationCallbackUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplicationCallbackUri: usize,
}
#[repr(C)]
pub struct IWebAccountProviderSignOutAccountOperation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccount: usize,
    #[cfg(feature = "Foundation")]
    pub ApplicationCallbackUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplicationCallbackUri: usize,
    pub ClientId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebAccountProviderSilentReportOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReportUserInteractionRequired: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub ReportUserInteractionRequiredWithError: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Core"))]
    ReportUserInteractionRequiredWithError: usize,
}
#[repr(C)]
pub struct IWebAccountProviderTokenObjects {
    pub base__: ::windows_sys::core::IInspectable,
    pub Operation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebAccountProviderTokenObjects2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[repr(C)]
pub struct IWebAccountProviderTokenOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProviderRequest: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ProviderResponses: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProviderResponses: usize,
    #[cfg(feature = "Foundation")]
    pub SetCacheExpirationTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCacheExpirationTime: usize,
    #[cfg(feature = "Foundation")]
    pub CacheExpirationTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CacheExpirationTime: usize,
}
#[repr(C)]
pub struct IWebAccountProviderUIReportOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReportUserCanceled: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebAccountScopeManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub AddWebAccountWithScopeAsync: unsafe extern "system" fn(this: *mut *mut Self, webaccountid: ::windows_sys::core::HSTRING, webaccountusername: ::windows_sys::core::HSTRING, props: *mut ::core::ffi::c_void, scope: WebAccountScope, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    AddWebAccountWithScopeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub SetScopeAsync: unsafe extern "system" fn(this: *mut *mut Self, webaccount: *mut ::core::ffi::c_void, scope: WebAccountScope, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    SetScopeAsync: usize,
    #[cfg(feature = "Security_Credentials")]
    pub GetScope: unsafe extern "system" fn(this: *mut *mut Self, webaccount: *mut ::core::ffi::c_void, result__: *mut WebAccountScope) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    GetScope: usize,
}
#[repr(C)]
pub struct IWebProviderTokenRequest {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub ClientRequest: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Core"))]
    ClientRequest: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub WebAccounts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    WebAccounts: usize,
    pub WebAccountSelectionOptions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WebAccountSelectionOptions) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ApplicationCallbackUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplicationCallbackUri: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Core"))]
    pub GetApplicationTokenBindingKeyAsync: unsafe extern "system" fn(this: *mut *mut Self, keytype: super::TokenBindingKeyType, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Cryptography_Core")))]
    GetApplicationTokenBindingKeyAsync: usize,
}
#[repr(C)]
pub struct IWebProviderTokenRequest2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetApplicationTokenBindingKeyIdAsync: unsafe extern "system" fn(this: *mut *mut Self, keytype: super::TokenBindingKeyType, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetApplicationTokenBindingKeyIdAsync: usize,
}
#[repr(C)]
pub struct IWebProviderTokenRequest3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ApplicationPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ApplicationProcessName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CheckApplicationForCapabilityAsync: unsafe extern "system" fn(this: *mut *mut Self, capabilityname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckApplicationForCapabilityAsync: usize,
}
#[repr(C)]
pub struct IWebProviderTokenResponse {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub ClientResponse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Core"))]
    ClientResponse: usize,
}
#[repr(C)]
pub struct IWebProviderTokenResponseFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, webtokenresponse: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Core"))]
    Create: usize,
}
pub type WebAccountClientView = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebAccountClientViewType(pub i32);
impl WebAccountClientViewType {
    pub const IdOnly: Self = Self(0i32);
    pub const IdAndProperties: Self = Self(1i32);
}
impl ::core::marker::Copy for WebAccountClientViewType {}
impl ::core::clone::Clone for WebAccountClientViewType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WebAccountProviderAddAccountOperation = *mut ::core::ffi::c_void;
pub type WebAccountProviderDeleteAccountOperation = *mut ::core::ffi::c_void;
pub type WebAccountProviderGetTokenSilentOperation = *mut ::core::ffi::c_void;
pub type WebAccountProviderManageAccountOperation = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebAccountProviderOperationKind(pub i32);
impl WebAccountProviderOperationKind {
    pub const RequestToken: Self = Self(0i32);
    pub const GetTokenSilently: Self = Self(1i32);
    pub const AddAccount: Self = Self(2i32);
    pub const ManageAccount: Self = Self(3i32);
    pub const DeleteAccount: Self = Self(4i32);
    pub const RetrieveCookies: Self = Self(5i32);
    pub const SignOutAccount: Self = Self(6i32);
}
impl ::core::marker::Copy for WebAccountProviderOperationKind {}
impl ::core::clone::Clone for WebAccountProviderOperationKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WebAccountProviderRequestTokenOperation = *mut ::core::ffi::c_void;
pub type WebAccountProviderRetrieveCookiesOperation = *mut ::core::ffi::c_void;
pub type WebAccountProviderSignOutAccountOperation = *mut ::core::ffi::c_void;
pub type WebAccountProviderTriggerDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebAccountScope(pub i32);
impl WebAccountScope {
    pub const PerUser: Self = Self(0i32);
    pub const PerApplication: Self = Self(1i32);
}
impl ::core::marker::Copy for WebAccountScope {}
impl ::core::clone::Clone for WebAccountScope {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebAccountSelectionOptions(pub u32);
impl WebAccountSelectionOptions {
    pub const Default: Self = Self(0u32);
    pub const New: Self = Self(1u32);
}
impl ::core::marker::Copy for WebAccountSelectionOptions {}
impl ::core::clone::Clone for WebAccountSelectionOptions {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WebProviderTokenRequest = *mut ::core::ffi::c_void;
pub type WebProviderTokenResponse = *mut ::core::ffi::c_void;
