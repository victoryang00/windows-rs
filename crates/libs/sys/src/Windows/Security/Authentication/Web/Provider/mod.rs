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
impl ::windows_sys::core::Interface for IWebAccountClientView {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3887949498, data2: 3015, data3: 19558, data4: [191, 212, 101, 211, 8, 44, 188, 168] };
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
impl ::windows_sys::core::Interface for IWebAccountClientViewFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1634539172, data2: 56866, data3: 18517, data4: [163, 38, 6, 206, 191, 42, 63, 35] };
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
impl ::windows_sys::core::Interface for IWebAccountManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3001606566, data2: 54426, data3: 16434, data4: [132, 191, 26, 40, 71, 116, 123, 241] };
}
#[repr(C)]
pub struct IWebAccountManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PullCookiesAsync: unsafe extern "system" fn(this: *mut *mut Self, uristring: ::windows_sys::core::HSTRING, callerpfn: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PullCookiesAsync: usize,
}
impl ::windows_sys::core::Interface for IWebAccountManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1755818025, data2: 11615, data3: 18003, data4: [139, 176, 189, 47, 166, 189, 45, 135] };
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
impl ::windows_sys::core::Interface for IWebAccountManagerStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3712295846, data2: 35407, data3: 19106, data4: [177, 94, 3, 245, 80, 175, 19, 89] };
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
impl ::windows_sys::core::Interface for IWebAccountManagerStatics4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1508623058, data2: 63451, data3: 16687, data4: [188, 63, 242, 254, 160, 68, 48, 180] };
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
impl ::windows_sys::core::Interface for IWebAccountMapManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3908715631, data2: 14875, data3: 18596, data4: [142, 144, 30, 89, 202, 111, 84, 219] };
}
#[repr(C)]
pub struct IWebAccountProviderAddAccountOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebAccountProviderAddAccountOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1944837327, data2: 17272, data3: 19577, data4: [147, 53, 165, 215, 171, 129, 89, 78] };
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
impl ::windows_sys::core::Interface for IWebAccountProviderBaseReportOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3148131515, data2: 39227, data3: 19799, data4: [187, 228, 20, 33, 227, 102, 139, 76] };
}
#[repr(C)]
pub struct IWebAccountProviderDeleteAccountOperation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccount: usize,
}
impl ::windows_sys::core::Interface for IWebAccountProviderDeleteAccountOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 180046008, data2: 40449, data3: 18889, data4: [163, 85, 125, 72, 202, 247, 214, 202] };
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
impl ::windows_sys::core::Interface for IWebAccountProviderManageAccountOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3978353756, data2: 53787, data3: 17982, data4: [169, 183, 193, 253, 14, 218, 233, 120] };
}
#[repr(C)]
pub struct IWebAccountProviderOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WebAccountProviderOperationKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebAccountProviderOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1834820646, data2: 4273, data3: 16794, data4: [164, 78, 249, 197, 22, 21, 116, 230] };
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
impl ::windows_sys::core::Interface for IWebAccountProviderRetrieveCookiesOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1510212673, data2: 4003, data3: 19121, data4: [160, 28, 32, 177, 16, 53, 133, 148] };
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
impl ::windows_sys::core::Interface for IWebAccountProviderSignOutAccountOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3096502813, data2: 3157, data3: 18364, data4: [140, 114, 4, 166, 252, 124, 172, 7] };
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
impl ::windows_sys::core::Interface for IWebAccountProviderSilentReportOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3769976312, data2: 15119, data3: 17626, data4: [146, 76, 123, 24, 186, 170, 98, 169] };
}
#[repr(C)]
pub struct IWebAccountProviderTokenObjects {
    pub base__: ::windows_sys::core::IInspectable,
    pub Operation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebAccountProviderTokenObjects {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1083123787, data2: 4904, data3: 17115, data4: [137, 164, 11, 206, 122, 113, 125, 142] };
}
#[repr(C)]
pub struct IWebAccountProviderTokenObjects2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
impl ::windows_sys::core::Interface for IWebAccountProviderTokenObjects2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 270579859, data2: 23717, data3: 20479, data4: [149, 251, 184, 32, 39, 63, 195, 149] };
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
impl ::windows_sys::core::Interface for IWebAccountProviderTokenOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2512786366, data2: 8244, data3: 19512, data4: [148, 52, 210, 108, 20, 178, 180, 178] };
}
#[repr(C)]
pub struct IWebAccountProviderUIReportOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReportUserCanceled: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebAccountProviderUIReportOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 687837907, data2: 36736, data3: 17147, data4: [148, 79, 178, 16, 123, 189, 66, 230] };
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
impl ::windows_sys::core::Interface for IWebAccountScopeManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1550639996, data2: 4786, data3: 16954, data4: [191, 61, 133, 184, 215, 229, 54, 86] };
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
impl ::windows_sys::core::Interface for IWebProviderTokenRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 504919947, data2: 34821, data3: 17739, data4: [159, 17, 70, 141, 42, 241, 9, 90] };
}
#[repr(C)]
pub struct IWebProviderTokenRequest2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetApplicationTokenBindingKeyIdAsync: unsafe extern "system" fn(this: *mut *mut Self, keytype: super::TokenBindingKeyType, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetApplicationTokenBindingKeyIdAsync: usize,
}
impl ::windows_sys::core::Interface for IWebProviderTokenRequest2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3050778188, data2: 4273, data3: 19110, data4: [136, 177, 11, 108, 158, 12, 30, 70] };
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
impl ::windows_sys::core::Interface for IWebProviderTokenRequest3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 455546538, data2: 17033, data3: 17518, data4: [146, 86, 218, 251, 111, 102, 165, 30] };
}
#[repr(C)]
pub struct IWebProviderTokenResponse {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub ClientResponse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Core"))]
    ClientResponse: usize,
}
impl ::windows_sys::core::Interface for IWebProviderTokenResponse {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4011931539, data2: 61269, data3: 16774, data4: [183, 206, 140, 178, 231, 249, 132, 158] };
}
#[repr(C)]
pub struct IWebProviderTokenResponseFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, webtokenresponse: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Core"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IWebProviderTokenResponseFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4199143834, data2: 9658, data3: 16503, data4: [156, 250, 157, 180, 222, 167, 183, 26] };
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
