#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub type ACCOUNT_STATE = i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const NOT_CONNECTED: ACCOUNT_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const CONNECTING: ACCOUNT_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const CONNECT_COMPLETED: ACCOUNT_STATE = 2i32;
#[repr(C)]
pub struct AsyncIAssociatedIdentityProvider {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Begin_AssociateIdentity: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Begin_AssociateIdentity: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Finish_AssociateIdentity: unsafe extern "system" fn(this: *mut *mut Self, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Finish_AssociateIdentity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Begin_DisassociateIdentity: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Begin_DisassociateIdentity: usize,
    pub Finish_DisassociateIdentity: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Begin_ChangeCredential: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Begin_ChangeCredential: usize,
    pub Finish_ChangeCredential: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct AsyncIConnectedIdentityProvider {
    pub base__: ::windows_sys::core::IUnknown,
    pub Begin_ConnectIdentity: unsafe extern "system" fn(this: *mut *mut Self, authbuffer: *const u8, authbuffersize: u32) -> ::windows_sys::core::HRESULT,
    pub Finish_ConnectIdentity: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Begin_DisconnectIdentity: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Finish_DisconnectIdentity: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Begin_IsConnected: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Finish_IsConnected: unsafe extern "system" fn(this: *mut *mut Self, connected: *mut super::super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Finish_IsConnected: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Begin_GetUrl: unsafe extern "system" fn(this: *mut *mut Self, identifier: IDENTITY_URL, context: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Begin_GetUrl: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Finish_GetUrl: unsafe extern "system" fn(this: *mut *mut Self, postdata: *mut super::super::super::super::System::Com::VARIANT, url: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Finish_GetUrl: usize,
    pub Begin_GetAccountState: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Finish_GetAccountState: unsafe extern "system" fn(this: *mut *mut Self, pstate: *mut ACCOUNT_STATE) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct AsyncIIdentityAdvise {
    pub base__: ::windows_sys::core::IUnknown,
    pub Begin_IdentityUpdated: unsafe extern "system" fn(this: *mut *mut Self, dwidentityupdateevents: u32, lpszuniqueid: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub Finish_IdentityUpdated: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct AsyncIIdentityAuthentication {
    pub base__: ::windows_sys::core::IUnknown,
    pub Begin_SetIdentityCredential: unsafe extern "system" fn(this: *mut *mut Self, credbuffer: *const u8, credbufferlength: u32) -> ::windows_sys::core::HRESULT,
    pub Finish_SetIdentityCredential: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Begin_ValidateIdentityCredential: unsafe extern "system" fn(this: *mut *mut Self, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Begin_ValidateIdentityCredential: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Finish_ValidateIdentityCredential: unsafe extern "system" fn(this: *mut *mut Self, ppidentityproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Finish_ValidateIdentityCredential: usize,
}
#[repr(C)]
pub struct AsyncIIdentityProvider {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub Begin_GetIdentityEnum: unsafe extern "system" fn(this: *mut *mut Self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    Begin_GetIdentityEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Finish_GetIdentityEnum: unsafe extern "system" fn(this: *mut *mut Self, ppidentityenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Finish_GetIdentityEnum: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Begin_Create: unsafe extern "system" fn(this: *mut *mut Self, lpszusername: ::windows_sys::core::PCWSTR, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Begin_Create: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Finish_Create: unsafe extern "system" fn(this: *mut *mut Self, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Finish_Create: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Begin_Import: unsafe extern "system" fn(this: *mut *mut Self, ppropertystore: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Begin_Import: usize,
    pub Finish_Import: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Begin_Delete: unsafe extern "system" fn(this: *mut *mut Self, lpszuniqueid: ::windows_sys::core::PCWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Begin_Delete: usize,
    pub Finish_Delete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Begin_FindByUniqueID: unsafe extern "system" fn(this: *mut *mut Self, lpszuniqueid: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Finish_FindByUniqueID: unsafe extern "system" fn(this: *mut *mut Self, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Finish_FindByUniqueID: usize,
    pub Begin_GetProviderPropertyStore: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Finish_GetProviderPropertyStore: unsafe extern "system" fn(this: *mut *mut Self, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Finish_GetProviderPropertyStore: usize,
    pub Begin_Advise: unsafe extern "system" fn(this: *mut *mut Self, pidentityadvise: *mut ::core::ffi::c_void, dwidentityupdateevents: u32) -> ::windows_sys::core::HRESULT,
    pub Finish_Advise: unsafe extern "system" fn(this: *mut *mut Self, pdwcookie: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Begin_UnAdvise: unsafe extern "system" fn(this: *mut *mut Self, dwcookie: u32) -> ::windows_sys::core::HRESULT,
    pub Finish_UnAdvise: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct AsyncIIdentityStore {
    pub base__: ::windows_sys::core::IUnknown,
    pub Begin_GetCount: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Finish_GetCount: unsafe extern "system" fn(this: *mut *mut Self, pdwproviders: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Begin_GetAt: unsafe extern "system" fn(this: *mut *mut Self, dwprovider: u32, pprovguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Finish_GetAt: unsafe extern "system" fn(this: *mut *mut Self, pprovguid: *mut ::windows_sys::core::GUID, ppidentityprovider: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Begin_AddToCache: unsafe extern "system" fn(this: *mut *mut Self, lpszuniqueid: ::windows_sys::core::PCWSTR, providerguid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Finish_AddToCache: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Begin_ConvertToSid: unsafe extern "system" fn(this: *mut *mut Self, lpszuniqueid: ::windows_sys::core::PCWSTR, providerguid: *const ::windows_sys::core::GUID, cbsid: u16, psid: *mut u8) -> ::windows_sys::core::HRESULT,
    pub Finish_ConvertToSid: unsafe extern "system" fn(this: *mut *mut Self, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub Begin_EnumerateIdentities: unsafe extern "system" fn(this: *mut *mut Self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    Begin_EnumerateIdentities: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Finish_EnumerateIdentities: unsafe extern "system" fn(this: *mut *mut Self, ppidentityenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Finish_EnumerateIdentities: usize,
    pub Begin_Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Finish_Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct AsyncIIdentityStoreEx {
    pub base__: ::windows_sys::core::IUnknown,
    pub Begin_CreateConnectedIdentity: unsafe extern "system" fn(this: *mut *mut Self, localname: ::windows_sys::core::PCWSTR, connectedname: ::windows_sys::core::PCWSTR, providerguid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Finish_CreateConnectedIdentity: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Begin_DeleteConnectedIdentity: unsafe extern "system" fn(this: *mut *mut Self, connectedname: ::windows_sys::core::PCWSTR, providerguid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Finish_DeleteConnectedIdentity: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
pub const CIdentityProfileHandler: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3975528262, data2: 58294, data3: 17562, data4: [181, 107, 67, 245, 143, 134, 120, 20] };
pub const CoClassIdentityStore: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 819237446, data2: 53783, data3: 18015, data4: [176, 11, 172, 157, 221, 101, 46, 183] };
#[repr(C)]
pub struct IAssociatedIdentityProvider {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub AssociateIdentity: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::super::super::Foundation::HWND, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))]
    AssociateIdentity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisassociateIdentity: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisassociateIdentity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ChangeCredential: unsafe extern "system" fn(this: *mut *mut Self, hwndparent: super::super::super::super::Foundation::HWND, lpszuniqueid: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ChangeCredential: usize,
}
#[repr(C)]
pub struct IConnectedIdentityProvider {
    pub base__: ::windows_sys::core::IUnknown,
    pub ConnectIdentity: unsafe extern "system" fn(this: *mut *mut Self, authbuffer: *const u8, authbuffersize: u32) -> ::windows_sys::core::HRESULT,
    pub DisconnectIdentity: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsConnected: unsafe extern "system" fn(this: *mut *mut Self, connected: *mut super::super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsConnected: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetUrl: unsafe extern "system" fn(this: *mut *mut Self, identifier: IDENTITY_URL, context: *mut ::core::ffi::c_void, postdata: *mut super::super::super::super::System::Com::VARIANT, url: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetUrl: usize,
    pub GetAccountState: unsafe extern "system" fn(this: *mut *mut Self, pstate: *mut ACCOUNT_STATE) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_KEYWORD_ASSOCIATED: &str = "associated";
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_KEYWORD_CONNECTED: &str = "connected";
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_KEYWORD_HOMEGROUP: &str = "homegroup";
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_KEYWORD_LOCAL: &str = "local";
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub type IDENTITY_TYPE = i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITIES_ALL: IDENTITY_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITIES_ME_ONLY: IDENTITY_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub type IDENTITY_URL = i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_CREATE_ACCOUNT_WIZARD: IDENTITY_URL = 0i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_SIGN_IN_WIZARD: IDENTITY_URL = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_CHANGE_PASSWORD_WIZARD: IDENTITY_URL = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_IFEXISTS_WIZARD: IDENTITY_URL = 3i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_ACCOUNT_SETTINGS: IDENTITY_URL = 4i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_RESTORE_WIZARD: IDENTITY_URL = 5i32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_URL_CONNECT_WIZARD: IDENTITY_URL = 6i32;
#[repr(C)]
pub struct IIdentityAdvise {
    pub base__: ::windows_sys::core::IUnknown,
    pub IdentityUpdated: unsafe extern "system" fn(this: *mut *mut Self, dwidentityupdateevents: IdentityUpdateEvent, lpszuniqueid: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IIdentityAuthentication {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetIdentityCredential: unsafe extern "system" fn(this: *mut *mut Self, credbuffer: *const u8, credbufferlength: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub ValidateIdentityCredential: unsafe extern "system" fn(this: *mut *mut Self, credbuffer: *const u8, credbufferlength: u32, ppidentityproperties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    ValidateIdentityCredential: usize,
}
#[repr(C)]
pub struct IIdentityProvider {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetIdentityEnum: unsafe extern "system" fn(this: *mut *mut Self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppidentityenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetIdentityEnum: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, lpszusername: ::windows_sys::core::PCWSTR, pppropertystore: *mut *mut ::core::ffi::c_void, pkeywordstoadd: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    Create: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Import: unsafe extern "system" fn(this: *mut *mut Self, ppropertystore: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Import: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self, lpszuniqueid: ::windows_sys::core::PCWSTR, pkeywordstodelete: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Delete: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub FindByUniqueID: unsafe extern "system" fn(this: *mut *mut Self, lpszuniqueid: ::windows_sys::core::PCWSTR, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    FindByUniqueID: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetProviderPropertyStore: unsafe extern "system" fn(this: *mut *mut Self, pppropertystore: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetProviderPropertyStore: usize,
    pub Advise: unsafe extern "system" fn(this: *mut *mut Self, pidentityadvise: *mut ::core::ffi::c_void, dwidentityupdateevents: IdentityUpdateEvent, pdwcookie: *mut u32) -> ::windows_sys::core::HRESULT,
    pub UnAdvise: unsafe extern "system" fn(this: *mut *mut Self, dwcookie: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IIdentityStore {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetCount: unsafe extern "system" fn(this: *mut *mut Self, pdwproviders: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, dwprovider: u32, pprovguid: *mut ::windows_sys::core::GUID, ppidentityprovider: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddToCache: unsafe extern "system" fn(this: *mut *mut Self, lpszuniqueid: ::windows_sys::core::PCWSTR, providerguid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ConvertToSid: unsafe extern "system" fn(this: *mut *mut Self, lpszuniqueid: ::windows_sys::core::PCWSTR, providerguid: *const ::windows_sys::core::GUID, cbsid: u16, psid: *mut u8, pcbrequiredsid: *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub EnumerateIdentities: unsafe extern "system" fn(this: *mut *mut Self, eidentitytype: IDENTITY_TYPE, pfilterkey: *const super::super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfilterpropvarvalue: *const super::super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppidentityenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    EnumerateIdentities: usize,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IIdentityStoreEx {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateConnectedIdentity: unsafe extern "system" fn(this: *mut *mut Self, localname: ::windows_sys::core::PCWSTR, connectedname: ::windows_sys::core::PCWSTR, providerguid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub DeleteConnectedIdentity: unsafe extern "system" fn(this: *mut *mut Self, connectedname: ::windows_sys::core::PCWSTR, providerguid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub type IdentityUpdateEvent = u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_ASSOCIATED: IdentityUpdateEvent = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_DISASSOCIATED: IdentityUpdateEvent = 2u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_CREATED: IdentityUpdateEvent = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_IMPORTED: IdentityUpdateEvent = 8u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_DELETED: IdentityUpdateEvent = 16u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_PROPCHANGED: IdentityUpdateEvent = 32u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_CONNECTED: IdentityUpdateEvent = 64u32;
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const IDENTITY_DISCONNECTED: IdentityUpdateEvent = 128u32;
pub const OID_OAssociatedIdentityProviderObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2563089373, data2: 56168, data3: 20250, data4: [141, 43, 144, 121, 205, 254, 175, 97] };
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_COMPLETE_ACCOUNT: &str = "CompleteAccount";
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_MODERN_SETTINGS_ADD_USER: &str = "ModernSettingsAddUser";
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_NTH_USER_FIRST_AUTH: &str = "NthUserFirstAuth";
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_OUT_OF_BOX_EXPERIENCE: &str = "OutOfBoxExperience";
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_OUT_OF_BOX_UPGRADE_EXPERIENCE: &str = "OutOfBoxUpgradeExperience";
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_PROPERTY_STORE: &str = "PropertyStore";
#[doc = "*Required features: `\"Win32_Security_Authentication_Identity_Provider\"`*"]
pub const STR_USER_NAME: &str = "Username";
