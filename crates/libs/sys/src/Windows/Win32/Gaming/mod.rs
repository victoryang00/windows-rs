#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckGamingPrivilegeSilently(privilegeid: u32, scope: ::windows_sys::core::HSTRING, policy: ::windows_sys::core::HSTRING, hasprivilege: *mut super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckGamingPrivilegeSilentlyForUser(user: *mut *mut ::windows_sys::core::IInspectable, privilegeid: u32, scope: ::windows_sys::core::HSTRING, policy: ::windows_sys::core::HSTRING, hasprivilege: *mut super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub fn CheckGamingPrivilegeWithUI(privilegeid: u32, scope: ::windows_sys::core::HSTRING, policy: ::windows_sys::core::HSTRING, friendlymessage: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub fn CheckGamingPrivilegeWithUIForUser(user: *mut *mut ::windows_sys::core::IInspectable, privilegeid: u32, scope: ::windows_sys::core::HSTRING, policy: ::windows_sys::core::HSTRING, friendlymessage: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub fn GetExpandedResourceExclusiveCpuCount(exclusivecpucount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub fn GetGamingDeviceModelInformation(information: *mut GAMING_DEVICE_MODEL_INFORMATION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HasExpandedResources(hasexpandedresources: *mut super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ProcessPendingGameUI(waitforcompletion: super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub fn ReleaseExclusiveCpuSets() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub fn ShowChangeFriendRelationshipUI(targetuserxuid: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub fn ShowChangeFriendRelationshipUIForUser(user: *mut *mut ::windows_sys::core::IInspectable, targetuserxuid: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub fn ShowCustomizeUserProfileUI(completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub fn ShowCustomizeUserProfileUIForUser(user: *mut *mut ::windows_sys::core::IInspectable, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub fn ShowFindFriendsUI(completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub fn ShowFindFriendsUIForUser(user: *mut *mut ::windows_sys::core::IInspectable, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub fn ShowGameInfoUI(titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub fn ShowGameInfoUIForUser(user: *mut *mut ::windows_sys::core::IInspectable, titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub fn ShowGameInviteUI(serviceconfigurationid: ::windows_sys::core::HSTRING, sessiontemplatename: ::windows_sys::core::HSTRING, sessionid: ::windows_sys::core::HSTRING, invitationdisplaytext: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub fn ShowGameInviteUIForUser(user: *mut *mut ::windows_sys::core::IInspectable, serviceconfigurationid: ::windows_sys::core::HSTRING, sessiontemplatename: ::windows_sys::core::HSTRING, sessionid: ::windows_sys::core::HSTRING, invitationdisplaytext: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub fn ShowGameInviteUIWithContext(serviceconfigurationid: ::windows_sys::core::HSTRING, sessiontemplatename: ::windows_sys::core::HSTRING, sessionid: ::windows_sys::core::HSTRING, invitationdisplaytext: ::windows_sys::core::HSTRING, customactivationcontext: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub fn ShowGameInviteUIWithContextForUser(user: *mut *mut ::windows_sys::core::IInspectable, serviceconfigurationid: ::windows_sys::core::HSTRING, sessiontemplatename: ::windows_sys::core::HSTRING, sessionid: ::windows_sys::core::HSTRING, invitationdisplaytext: ::windows_sys::core::HSTRING, customactivationcontext: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub fn ShowPlayerPickerUI(promptdisplaytext: ::windows_sys::core::HSTRING, xuids: *const ::windows_sys::core::HSTRING, xuidscount: usize, preselectedxuids: *const ::windows_sys::core::HSTRING, preselectedxuidscount: usize, minselectioncount: usize, maxselectioncount: usize, completionroutine: PlayerPickerUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub fn ShowPlayerPickerUIForUser(user: *mut *mut ::windows_sys::core::IInspectable, promptdisplaytext: ::windows_sys::core::HSTRING, xuids: *const ::windows_sys::core::HSTRING, xuidscount: usize, preselectedxuids: *const ::windows_sys::core::HSTRING, preselectedxuidscount: usize, minselectioncount: usize, maxselectioncount: usize, completionroutine: PlayerPickerUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub fn ShowProfileCardUI(targetuserxuid: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub fn ShowProfileCardUIForUser(user: *mut *mut ::windows_sys::core::IInspectable, targetuserxuid: ::windows_sys::core::HSTRING, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub fn ShowTitleAchievementsUI(titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub fn ShowTitleAchievementsUIForUser(user: *mut *mut ::windows_sys::core::IInspectable, titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub fn ShowUserSettingsUI(completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`*"]
    pub fn ShowUserSettingsUIForUser(user: *mut *mut ::windows_sys::core::IInspectable, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Gaming\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TryCancelPendingGameUI() -> super::Foundation::BOOL;
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub type GAMESTATS_OPEN_RESULT = i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMESTATS_OPEN_CREATED: GAMESTATS_OPEN_RESULT = 0i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMESTATS_OPEN_OPENED: GAMESTATS_OPEN_RESULT = 1i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub type GAMESTATS_OPEN_TYPE = i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMESTATS_OPEN_OPENORCREATE: GAMESTATS_OPEN_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMESTATS_OPEN_OPENONLY: GAMESTATS_OPEN_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub type GAME_INSTALL_SCOPE = i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GIS_NOT_INSTALLED: GAME_INSTALL_SCOPE = 1i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GIS_CURRENT_USER: GAME_INSTALL_SCOPE = 2i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GIS_ALL_USERS: GAME_INSTALL_SCOPE = 3i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub type GAMING_DEVICE_DEVICE_ID = i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMING_DEVICE_DEVICE_ID_NONE: GAMING_DEVICE_DEVICE_ID = 0i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE: GAMING_DEVICE_DEVICE_ID = 1988865574i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_S: GAMING_DEVICE_DEVICE_ID = 712204761i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_X: GAMING_DEVICE_DEVICE_ID = 1523980231i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_X_DEVKIT: GAMING_DEVICE_DEVICE_ID = 284675555i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub struct GAMING_DEVICE_MODEL_INFORMATION {
    pub vendorId: GAMING_DEVICE_VENDOR_ID,
    pub deviceId: GAMING_DEVICE_DEVICE_ID,
}
impl ::core::marker::Copy for GAMING_DEVICE_MODEL_INFORMATION {}
impl ::core::clone::Clone for GAMING_DEVICE_MODEL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub type GAMING_DEVICE_VENDOR_ID = i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMING_DEVICE_VENDOR_ID_NONE: GAMING_DEVICE_VENDOR_ID = 0i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const GAMING_DEVICE_VENDOR_ID_MICROSOFT: GAMING_DEVICE_VENDOR_ID = -1024700366i32;
pub const GameExplorer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2589895056, data2: 12340, data3: 19823, data4: [145, 40, 1, 243, 198, 16, 34, 188] };
pub const GameStatistics: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3687340588, data2: 49372, data3: 18785, data4: [182, 226, 210, 139, 98, 193, 26, 212] };
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub type GameUICompletionRoutine = ::core::option::Option<unsafe extern "system" fn(returncode: ::windows_sys::core::HRESULT, context: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const ID_GDF_THUMBNAIL_STR: &str = "__GDF_THUMBNAIL";
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const ID_GDF_XML_STR: &str = "__GDF_XML";
#[repr(C)]
pub struct IGameExplorer {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub AddGame: unsafe extern "system" fn(this: *mut *mut Self, bstrgdfbinarypath: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, bstrgameinstalldirectory: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddGame: usize,
    pub RemoveGame: unsafe extern "system" fn(this: *mut *mut Self, guidinstanceid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub UpdateGame: unsafe extern "system" fn(this: *mut *mut Self, guidinstanceid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub VerifyAccess: unsafe extern "system" fn(this: *mut *mut Self, bstrgdfbinarypath: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, pfhasaccess: *mut super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    VerifyAccess: usize,
}
impl ::windows_sys::core::Interface for IGameExplorer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3887266674, data2: 55080, data3: 18867, data4: [165, 242, 24, 235, 245, 241, 52, 158] };
}
#[repr(C)]
pub struct IGameExplorer2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub InstallGame: unsafe extern "system" fn(this: *mut *mut Self, binarygdfpath: ::windows_sys::core::PCWSTR, installdirectory: ::windows_sys::core::PCWSTR, installscope: GAME_INSTALL_SCOPE) -> ::windows_sys::core::HRESULT,
    pub UninstallGame: unsafe extern "system" fn(this: *mut *mut Self, binarygdfpath: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CheckAccess: unsafe extern "system" fn(this: *mut *mut Self, binarygdfpath: ::windows_sys::core::PCWSTR, phasaccess: *mut super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CheckAccess: usize,
}
impl ::windows_sys::core::Interface for IGameExplorer2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2257013415, data2: 41453, data3: 17677, data4: [167, 235, 184, 158, 32, 178, 255, 243] };
}
#[repr(C)]
pub struct IGameStatistics {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetMaxCategoryLength: unsafe extern "system" fn(this: *mut *mut Self, cch: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetMaxNameLength: unsafe extern "system" fn(this: *mut *mut Self, cch: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetMaxValueLength: unsafe extern "system" fn(this: *mut *mut Self, cch: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetMaxCategories: unsafe extern "system" fn(this: *mut *mut Self, pmax: *mut u16) -> ::windows_sys::core::HRESULT,
    pub GetMaxStatsPerCategory: unsafe extern "system" fn(this: *mut *mut Self, pmax: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetCategoryTitle: unsafe extern "system" fn(this: *mut *mut Self, categoryindex: u16, title: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetCategoryTitle: unsafe extern "system" fn(this: *mut *mut Self, categoryindex: u16, ptitle: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetStatistic: unsafe extern "system" fn(this: *mut *mut Self, categoryindex: u16, statindex: u16, pname: *mut ::windows_sys::core::PWSTR, pvalue: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetStatistic: unsafe extern "system" fn(this: *mut *mut Self, categoryindex: u16, statindex: u16, name: ::windows_sys::core::PCWSTR, value: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut *mut Self, trackchanges: super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub SetLastPlayedCategory: unsafe extern "system" fn(this: *mut *mut Self, categoryindex: u32) -> ::windows_sys::core::HRESULT,
    pub GetLastPlayedCategory: unsafe extern "system" fn(this: *mut *mut Self, pcategoryindex: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGameStatistics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 948423114, data2: 1184, data3: 17070, data4: [188, 76, 95, 166, 199, 114, 17, 69] };
}
#[repr(C)]
pub struct IGameStatisticsMgr {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetGameStatistics: unsafe extern "system" fn(this: *mut *mut Self, gdfbinarypath: ::windows_sys::core::PCWSTR, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveGameStatistics: unsafe extern "system" fn(this: *mut *mut Self, gdfbinarypath: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGameStatisticsMgr {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2951997969, data2: 59150, data3: 16509, data4: [149, 221, 53, 230, 18, 196, 28, 226] };
}
#[repr(C)]
pub struct IXblIdpAuthManager {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetGamerAccount: unsafe extern "system" fn(this: *mut *mut Self, msaaccountid: ::windows_sys::core::PCWSTR, xuid: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetGamerAccount: unsafe extern "system" fn(this: *mut *mut Self, msaaccountid: *mut ::windows_sys::core::PWSTR, xuid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub SetAppViewInitialized: unsafe extern "system" fn(this: *mut *mut Self, appsid: ::windows_sys::core::PCWSTR, msaaccountid: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub GetEnvironment: unsafe extern "system" fn(this: *mut *mut Self, environment: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetSandbox: unsafe extern "system" fn(this: *mut *mut Self, sandbox: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTokenAndSignatureWithTokenResult: unsafe extern "system" fn(this: *mut *mut Self, msaaccountid: ::windows_sys::core::PCWSTR, appsid: ::windows_sys::core::PCWSTR, msatarget: ::windows_sys::core::PCWSTR, msapolicy: ::windows_sys::core::PCWSTR, httpmethod: ::windows_sys::core::PCWSTR, uri: ::windows_sys::core::PCWSTR, headers: ::windows_sys::core::PCWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL, result: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTokenAndSignatureWithTokenResult: usize,
}
impl ::windows_sys::core::Interface for IXblIdpAuthManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3948796680, data2: 35775, data3: 17563, data4: [172, 33, 176, 45, 222, 179, 177, 54] };
}
#[repr(C)]
pub struct IXblIdpAuthTokenResult {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetStatus: unsafe extern "system" fn(this: *mut *mut Self, status: *mut XBL_IDP_AUTH_TOKEN_STATUS) -> ::windows_sys::core::HRESULT,
    pub GetErrorCode: unsafe extern "system" fn(this: *mut *mut Self, errorcode: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub GetToken: unsafe extern "system" fn(this: *mut *mut Self, token: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetSignature: unsafe extern "system" fn(this: *mut *mut Self, signature: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetSandbox: unsafe extern "system" fn(this: *mut *mut Self, sandbox: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetEnvironment: unsafe extern "system" fn(this: *mut *mut Self, environment: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetMsaAccountId: unsafe extern "system" fn(this: *mut *mut Self, msaaccountid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetXuid: unsafe extern "system" fn(this: *mut *mut Self, xuid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetGamertag: unsafe extern "system" fn(this: *mut *mut Self, gamertag: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetAgeGroup: unsafe extern "system" fn(this: *mut *mut Self, agegroup: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetPrivileges: unsafe extern "system" fn(this: *mut *mut Self, privileges: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetMsaTarget: unsafe extern "system" fn(this: *mut *mut Self, msatarget: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetMsaPolicy: unsafe extern "system" fn(this: *mut *mut Self, msapolicy: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetMsaAppId: unsafe extern "system" fn(this: *mut *mut Self, msaappid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetRedirect: unsafe extern "system" fn(this: *mut *mut Self, redirect: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetMessage: unsafe extern "system" fn(this: *mut *mut Self, message: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetHelpId: unsafe extern "system" fn(this: *mut *mut Self, helpid: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetEnforcementBans: unsafe extern "system" fn(this: *mut *mut Self, enforcementbans: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetRestrictions: unsafe extern "system" fn(this: *mut *mut Self, restrictions: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetTitleRestrictions: unsafe extern "system" fn(this: *mut *mut Self, titlerestrictions: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXblIdpAuthTokenResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1187906085, data2: 62055, data3: 19816, data4: [178, 153, 178, 118, 37, 82, 222, 193] };
}
#[repr(C)]
pub struct IXblIdpAuthTokenResult2 {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetModernGamertag: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetModernGamertagSuffix: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    pub GetUniqueModernGamertag: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IXblIdpAuthTokenResult2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1977049264, data2: 24761, data3: 16685, data4: [153, 79, 38, 178, 205, 95, 120, 18] };
}
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub type KnownGamingPrivileges = i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_BROADCAST: KnownGamingPrivileges = 190i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_VIEW_FRIENDS_LIST: KnownGamingPrivileges = 197i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_GAME_DVR: KnownGamingPrivileges = 198i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_SHARE_KINECT_CONTENT: KnownGamingPrivileges = 199i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_MULTIPLAYER_PARTIES: KnownGamingPrivileges = 203i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_COMMUNICATION_VOICE_INGAME: KnownGamingPrivileges = 205i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_COMMUNICATION_VOICE_SKYPE: KnownGamingPrivileges = 206i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_CLOUD_GAMING_MANAGE_SESSION: KnownGamingPrivileges = 207i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_CLOUD_GAMING_JOIN_SESSION: KnownGamingPrivileges = 208i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_CLOUD_SAVED_GAMES: KnownGamingPrivileges = 209i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_SHARE_CONTENT: KnownGamingPrivileges = 211i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_PREMIUM_CONTENT: KnownGamingPrivileges = 214i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_SUBSCRIPTION_CONTENT: KnownGamingPrivileges = 219i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_SOCIAL_NETWORK_SHARING: KnownGamingPrivileges = 220i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_PREMIUM_VIDEO: KnownGamingPrivileges = 224i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_VIDEO_COMMUNICATIONS: KnownGamingPrivileges = 235i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_PURCHASE_CONTENT: KnownGamingPrivileges = 245i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_USER_CREATED_CONTENT: KnownGamingPrivileges = 247i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_PROFILE_VIEWING: KnownGamingPrivileges = 249i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_COMMUNICATIONS: KnownGamingPrivileges = 252i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_MULTIPLAYER_SESSIONS: KnownGamingPrivileges = 254i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XPRIVILEGE_ADD_FRIEND: KnownGamingPrivileges = 255i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub type PlayerPickerUICompletionRoutine = ::core::option::Option<unsafe extern "system" fn(returncode: ::windows_sys::core::HRESULT, context: *const ::core::ffi::c_void, selectedxuids: *const ::windows_sys::core::HSTRING, selectedxuidscount: usize)>;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub type XBL_IDP_AUTH_TOKEN_STATUS = i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_SUCCESS: XBL_IDP_AUTH_TOKEN_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_OFFLINE_SUCCESS: XBL_IDP_AUTH_TOKEN_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_NO_ACCOUNT_SET: XBL_IDP_AUTH_TOKEN_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_LOAD_MSA_ACCOUNT_FAILED: XBL_IDP_AUTH_TOKEN_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_XBOX_VETO: XBL_IDP_AUTH_TOKEN_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_MSA_INTERRUPT: XBL_IDP_AUTH_TOKEN_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_OFFLINE_NO_CONSENT: XBL_IDP_AUTH_TOKEN_STATUS = 6i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_VIEW_NOT_SET: XBL_IDP_AUTH_TOKEN_STATUS = 7i32;
#[doc = "*Required features: `\"Win32_Gaming\"`*"]
pub const XBL_IDP_AUTH_TOKEN_STATUS_UNKNOWN: XBL_IDP_AUTH_TOKEN_STATUS = -1i32;
pub const XblIdpAuthManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3458421579, data2: 22232, data3: 18808, data4: [134, 162, 126, 229, 112, 100, 4, 104] };
pub const XblIdpAuthTokenResult: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2672374849, data2: 29770, data3: 16652, data4: [174, 43, 154, 34, 247, 199, 115, 31] };
