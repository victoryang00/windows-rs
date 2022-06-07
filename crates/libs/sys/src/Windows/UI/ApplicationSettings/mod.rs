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
impl ::windows_sys::core::Interface for IAccountsSettingsPane {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2179634220, data2: 20233, data3: 17414, data4: [165, 56, 131, 141, 155, 20, 183, 230] };
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
impl ::windows_sys::core::Interface for IAccountsSettingsPaneCommandsRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 996720793, data2: 56089, data3: 17872, data4: [154, 191, 149, 211, 119, 60, 147, 48] };
}
#[repr(C)]
pub struct IAccountsSettingsPaneCommandsRequestedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
impl ::windows_sys::core::Interface for IAccountsSettingsPaneCommandsRequestedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 909081517, data2: 20023, data3: 18791, data4: [140, 64, 231, 142, 231, 161, 229, 187] };
}
#[repr(C)]
pub struct IAccountsSettingsPaneEventDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAccountsSettingsPaneEventDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3421658431, data2: 58810, data3: 16623, data4: [147, 218, 101, 224, 150, 229, 251, 4] };
}
#[repr(C)]
pub struct IAccountsSettingsPaneStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Show: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAccountsSettingsPaneStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1444907872, data2: 45292, data3: 16720, data4: [168, 220, 32, 142, 228, 75, 6, 138] };
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
impl ::windows_sys::core::Interface for IAccountsSettingsPaneStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3525179330, data2: 52749, data3: 18511, data4: [184, 232, 232, 35, 194, 21, 118, 94] };
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
impl ::windows_sys::core::Interface for IAccountsSettingsPaneStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 138478680, data2: 41658, data3: 19567, data4: [180, 172, 72, 245, 20, 51, 18, 22] };
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
impl ::windows_sys::core::Interface for ICredentialCommand {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2784388582, data2: 24899, data3: 19066, data4: [169, 113, 176, 23, 186, 151, 140, 226] };
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
impl ::windows_sys::core::Interface for ICredentialCommandFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 669551639, data2: 48190, data3: 19328, data4: [148, 149, 78, 215, 32, 228, 138, 145] };
}
#[repr(C)]
pub struct ISettingsCommandFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Popups")]
    pub CreateSettingsCommand: unsafe extern "system" fn(this: *mut *mut Self, settingscommandid: *mut ::core::ffi::c_void, label: ::windows_sys::core::HSTRING, handler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    CreateSettingsCommand: usize,
}
impl ::windows_sys::core::Interface for ISettingsCommandFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1759599411, data2: 7299, data3: 17210, data4: [170, 90, 206, 238, 165, 189, 71, 100] };
}
#[repr(C)]
pub struct ISettingsCommandStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Popups")]
    pub AccountsCommand: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    AccountsCommand: usize,
}
impl ::windows_sys::core::Interface for ISettingsCommandStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1956309332, data2: 12137, data3: 19223, data4: [138, 186, 208, 92, 229, 119, 142, 70] };
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
impl ::windows_sys::core::Interface for ISettingsPane {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2983004466, data2: 17776, data3: 19561, data4: [141, 56, 137, 68, 101, 97, 172, 224] };
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
impl ::windows_sys::core::Interface for ISettingsPaneCommandsRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1155474350, data2: 23918, data3: 16488, data4: [161, 104, 244, 118, 67, 24, 33, 20] };
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
impl ::windows_sys::core::Interface for ISettingsPaneCommandsRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 543120676, data2: 6984, data3: 17961, data4: [166, 202, 47, 223, 237, 175, 183, 93] };
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
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ISettingsPaneStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 476730053, data2: 65305, data3: 18203, data4: [186, 107, 248, 243, 86, 148, 173, 154] };
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
impl ::windows_sys::core::Interface for IWebAccountCommand {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3399717784, data2: 40186, data3: 16966, data4: [176, 196, 169, 19, 163, 137, 101, 65] };
}
#[repr(C)]
pub struct IWebAccountCommandFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWebAccountCommand: unsafe extern "system" fn(this: *mut *mut Self, webaccount: *mut ::core::ffi::c_void, invoked: *mut ::core::ffi::c_void, actions: SupportedWebAccountActions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWebAccountCommand: usize,
}
impl ::windows_sys::core::Interface for IWebAccountCommandFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3215379967, data2: 12077, data3: 17141, data4: [129, 222, 29, 86, 186, 252, 73, 109] };
}
#[repr(C)]
pub struct IWebAccountInvokedArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Action: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WebAccountAction) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebAccountInvokedArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3886795840, data2: 41432, data3: 19549, data4: [154, 127, 29, 52, 178, 249, 10, 210] };
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
impl ::windows_sys::core::Interface for IWebAccountProviderCommand {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3600539034, data2: 41126, data3: 20123, data4: [136, 220, 199, 30, 117, 122, 53, 1] };
}
#[repr(C)]
pub struct IWebAccountProviderCommandFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWebAccountProviderCommand: unsafe extern "system" fn(this: *mut *mut Self, webaccountprovider: *mut ::core::ffi::c_void, invoked: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWebAccountProviderCommand: usize,
}
impl ::windows_sys::core::Interface for IWebAccountProviderCommandFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3580201499, data2: 45430, data3: 18294, data4: [132, 105, 169, 211, 255, 11, 63, 89] };
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
