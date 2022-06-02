#[cfg(feature = "ApplicationModel_UserActivities_Core")]
pub mod Core;
#[repr(C)]
pub struct IUserActivity {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UserActivityState) -> ::windows_sys::core::HRESULT,
    pub ActivityId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VisualElements: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContentUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetContentUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetContentUri: usize,
    pub ContentType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetContentType: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FallbackUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FallbackUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetFallbackUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetFallbackUri: usize,
    #[cfg(feature = "Foundation")]
    pub ActivationUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ActivationUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetActivationUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetActivationUri: usize,
    pub ContentInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContentInfo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
    pub CreateSession: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUserActivity2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ToJson: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUserActivity3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsRoamable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsRoamable: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUserActivityAttribution {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub IconUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IconUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetIconUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIconUri: usize,
    pub AlternateText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAlternateText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AddImageQuery: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAddImageQuery: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUserActivityAttributionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateWithUri: unsafe extern "system" fn(this: *mut *mut Self, iconuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWithUri: usize,
}
#[repr(C)]
pub struct IUserActivityChannel {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetOrCreateUserActivityAsync: unsafe extern "system" fn(this: *mut *mut Self, activityid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetOrCreateUserActivityAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteActivityAsync: unsafe extern "system" fn(this: *mut *mut Self, activityid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteActivityAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAllActivitiesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAllActivitiesAsync: usize,
}
#[repr(C)]
pub struct IUserActivityChannel2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRecentUserActivitiesAsync: unsafe extern "system" fn(this: *mut *mut Self, maxuniqueactivities: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRecentUserActivitiesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSessionHistoryItemsForUserActivityAsync: unsafe extern "system" fn(this: *mut *mut Self, activityid: ::windows_sys::core::HSTRING, starttime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSessionHistoryItemsForUserActivityAsync: usize,
}
#[repr(C)]
pub struct IUserActivityChannelStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUserActivityChannelStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisableAutoSessionCreation: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub TryGetForWebAccount: unsafe extern "system" fn(this: *mut *mut Self, account: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    TryGetForWebAccount: usize,
}
#[repr(C)]
pub struct IUserActivityChannelStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[repr(C)]
pub struct IUserActivityContentInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub ToJson: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUserActivityContentInfoStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromJson: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUserActivityFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithActivityId: unsafe extern "system" fn(this: *mut *mut Self, activityid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUserActivityRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetUserActivity: unsafe extern "system" fn(this: *mut *mut Self, activity: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUserActivityRequestManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub UserActivityRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UserActivityRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUserActivityRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUserActivityRequested: usize,
}
#[repr(C)]
pub struct IUserActivityRequestManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUserActivityRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(C)]
pub struct IUserActivitySession {
    pub base__: ::windows_sys::core::IInspectable,
    pub ActivityId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUserActivitySessionHistoryItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub UserActivity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub EndTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndTime: usize,
}
#[repr(C)]
pub struct IUserActivityStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryParseFromJson: unsafe extern "system" fn(this: *mut *mut Self, json: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TryParseFromJsonArray: unsafe extern "system" fn(this: *mut *mut Self, json: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryParseFromJsonArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ToJsonArray: unsafe extern "system" fn(this: *mut *mut Self, activities: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ToJsonArray: usize,
}
#[repr(C)]
pub struct IUserActivityVisualElements {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI")]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    BackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetBackgroundColor: usize,
    pub Attribution: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAttribution: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Shell")]
    pub SetContent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Shell"))]
    SetContent: usize,
    #[cfg(feature = "UI_Shell")]
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Shell"))]
    Content: usize,
}
#[repr(C)]
pub struct IUserActivityVisualElements2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AttributionDisplayText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAttributionDisplayText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
pub type UserActivity = *mut ::core::ffi::c_void;
pub type UserActivityAttribution = *mut ::core::ffi::c_void;
pub type UserActivityChannel = *mut ::core::ffi::c_void;
pub type UserActivityContentInfo = *mut ::core::ffi::c_void;
pub type UserActivityRequest = *mut ::core::ffi::c_void;
pub type UserActivityRequestManager = *mut ::core::ffi::c_void;
pub type UserActivityRequestedEventArgs = *mut ::core::ffi::c_void;
pub type UserActivitySession = *mut ::core::ffi::c_void;
pub type UserActivitySessionHistoryItem = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_UserActivities\"`*"]
#[repr(transparent)]
pub struct UserActivityState(pub i32);
impl UserActivityState {
    pub const New: Self = Self(0i32);
    pub const Published: Self = Self(1i32);
}
impl ::core::marker::Copy for UserActivityState {}
impl ::core::clone::Clone for UserActivityState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UserActivityVisualElements = *mut ::core::ffi::c_void;
