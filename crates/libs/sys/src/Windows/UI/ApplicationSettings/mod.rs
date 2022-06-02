pub type AccountsSettingsPane = *mut ::core::ffi::c_void;
pub type AccountsSettingsPaneCommandsRequestedEventArgs = *mut ::core::ffi::c_void;
pub type AccountsSettingsPaneEventDeferral = *mut ::core::ffi::c_void;
pub type CredentialCommand = *mut ::core::ffi::c_void;
pub type CredentialCommandCredentialDeletedHandler = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAccountsSettingsPane {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AccountCommandsRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AccountCommandsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccountCommandsRequested: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccountCommandsRequested: usize,
}
#[repr(C)]
pub struct IAccountsSettingsPaneCommandsRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub WebAccountProviderCommands: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WebAccountProviderCommands: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub WebAccountCommands: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WebAccountCommands: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CredentialCommands: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CredentialCommands: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups"))]
    pub Commands: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Popups")))]
    Commands: usize,
    pub HeaderText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetHeaderText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAccountsSettingsPaneCommandsRequestedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[repr(C)]
pub struct IAccountsSettingsPaneEventDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAccountsSettingsPaneStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Show: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAccountsSettingsPaneStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ShowManageAccountsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowManageAccountsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAddAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAddAccountAsync: usize,
}
#[repr(C)]
pub struct IAccountsSettingsPaneStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub ShowManageAccountsForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    ShowManageAccountsForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub ShowAddAccountForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    ShowAddAccountForUserAsync: usize,
}
#[repr(C)]
pub struct ICredentialCommand {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub PasswordCredential: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    PasswordCredential: usize,
    pub CredentialDeleted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICredentialCommandFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub CreateCredentialCommand: unsafe extern "system" fn(this: *mut *mut Self, passwordcredential: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateCredentialCommand: usize,
    #[cfg(feature = "Security_Credentials")]
    pub CreateCredentialCommandWithHandler: unsafe extern "system" fn(this: *mut *mut Self, passwordcredential: *mut ::core::ffi::c_void, deleted: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateCredentialCommandWithHandler: usize,
}
#[repr(C)]
pub struct ISettingsCommandFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Popups")]
    pub CreateSettingsCommand: unsafe extern "system" fn(this: *mut *mut Self, settingscommandid: *mut ::core::ffi::c_void, label: ::windows_sys::core::HSTRING, handler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    CreateSettingsCommand: usize,
}
#[repr(C)]
pub struct ISettingsCommandStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Popups")]
    pub AccountsCommand: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    AccountsCommand: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISettingsPane {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CommandsRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CommandsRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveCommandsRequested: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveCommandsRequested: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISettingsPaneCommandsRequest {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups", feature = "deprecated"))]
    pub ApplicationCommands: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Popups", feature = "deprecated")))]
    ApplicationCommands: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISettingsPaneCommandsRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Request: usize,
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISettingsPaneStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetForCurrentView: usize,
    #[cfg(feature = "deprecated")]
    pub Show: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Show: usize,
    #[cfg(feature = "deprecated")]
    pub Edge: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SettingsEdgeLocation) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Edge: usize,
}
#[repr(C)]
pub struct IWebAccountCommand {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccount: usize,
    pub Invoked: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Actions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SupportedWebAccountActions) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebAccountCommandFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWebAccountCommand: unsafe extern "system" fn(this: *mut *mut Self, webaccount: *mut ::core::ffi::c_void, invoked: *mut ::core::ffi::c_void, actions: SupportedWebAccountActions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWebAccountCommand: usize,
}
#[repr(C)]
pub struct IWebAccountInvokedArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Action: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WebAccountAction) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebAccountProviderCommand {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccountProvider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccountProvider: usize,
    pub Invoked: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebAccountProviderCommandFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWebAccountProviderCommand: unsafe extern "system" fn(this: *mut *mut Self, webaccountprovider: *mut ::core::ffi::c_void, invoked: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWebAccountProviderCommand: usize,
}
pub type SettingsCommand = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SettingsEdgeLocation(pub i32);
#[cfg(feature = "deprecated")]
impl SettingsEdgeLocation {
    pub const Right: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SettingsEdgeLocation {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SettingsEdgeLocation {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SettingsPane = *mut ::core::ffi::c_void;
pub type SettingsPaneCommandsRequest = *mut ::core::ffi::c_void;
pub type SettingsPaneCommandsRequestedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
pub struct SupportedWebAccountActions(pub u32);
impl SupportedWebAccountActions {
    pub const None: Self = Self(0u32);
    pub const Reconnect: Self = Self(1u32);
    pub const Remove: Self = Self(2u32);
    pub const ViewDetails: Self = Self(4u32);
    pub const Manage: Self = Self(8u32);
    pub const More: Self = Self(16u32);
}
impl ::core::marker::Copy for SupportedWebAccountActions {}
impl ::core::clone::Clone for SupportedWebAccountActions {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
pub struct WebAccountAction(pub i32);
impl WebAccountAction {
    pub const Reconnect: Self = Self(0i32);
    pub const Remove: Self = Self(1i32);
    pub const ViewDetails: Self = Self(2i32);
    pub const Manage: Self = Self(3i32);
    pub const More: Self = Self(4i32);
}
impl ::core::marker::Copy for WebAccountAction {}
impl ::core::clone::Clone for WebAccountAction {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WebAccountCommand = *mut ::core::ffi::c_void;
pub type WebAccountCommandInvokedHandler = *mut ::core::ffi::c_void;
pub type WebAccountInvokedArgs = *mut ::core::ffi::c_void;
pub type WebAccountProviderCommand = *mut ::core::ffi::c_void;
pub type WebAccountProviderCommandInvokedHandler = *mut ::core::ffi::c_void;
