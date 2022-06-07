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
impl ::windows_sys::core::Interface for IUserActivity {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4228923038, data2: 11435, data3: 19766, data4: [174, 162, 180, 187, 85, 108, 239, 15] };
}
#[repr(C)]
pub struct IUserActivity2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ToJson: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserActivity2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2646871138, data2: 2244, data3: 18348, data4: [170, 156, 43, 178, 34, 28, 85, 253] };
}
#[repr(C)]
pub struct IUserActivity3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsRoamable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsRoamable: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserActivity3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3882448708, data2: 57762, data3: 20807, data4: [142, 6, 85, 241, 238, 239, 39, 28] };
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
impl ::windows_sys::core::Interface for IUserActivityAttribution {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 883280053, data2: 34525, data3: 19180, data4: [164, 145, 106, 79, 174, 165, 210, 46] };
}
#[repr(C)]
pub struct IUserActivityAttributionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateWithUri: unsafe extern "system" fn(this: *mut *mut Self, iconuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWithUri: usize,
}
impl ::windows_sys::core::Interface for IUserActivityAttributionFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3861631570, data2: 50534, data3: 20290, data4: [153, 116, 145, 108, 77, 118, 55, 126] };
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
impl ::windows_sys::core::Interface for IUserActivityChannel {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3133208760, data2: 41188, data3: 18491, data4: [185, 72, 156, 186, 189, 6, 7, 12] };
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
impl ::windows_sys::core::Interface for IUserActivityChannel2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 379118427, data2: 60286, data3: 20128, data4: [191, 23, 164, 89, 232, 190, 112, 108] };
}
#[repr(C)]
pub struct IUserActivityChannelStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserActivityChannelStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3368027563, data2: 6541, data3: 19840, data4: [171, 178, 201, 119, 94, 196, 167, 41] };
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
impl ::windows_sys::core::Interface for IUserActivityChannelStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2391268912, data2: 43599, data3: 17956, data4: [154, 208, 212, 15, 59, 160, 49, 124] };
}
#[repr(C)]
pub struct IUserActivityChannelStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
impl ::windows_sys::core::Interface for IUserActivityChannelStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1404849627, data2: 48095, data3: 22916, data4: [128, 42, 83, 5, 135, 78, 32, 92] };
}
#[repr(C)]
pub struct IUserActivityContentInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub ToJson: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserActivityContentInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3013207469, data2: 4991, data3: 16541, data4: [130, 45, 225, 175, 39, 206, 8, 220] };
}
#[repr(C)]
pub struct IUserActivityContentInfoStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromJson: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserActivityContentInfoStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2575876939, data2: 902, data3: 19401, data4: [150, 138, 130, 0, 176, 4, 20, 79] };
}
#[repr(C)]
pub struct IUserActivityFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithActivityId: unsafe extern "system" fn(this: *mut *mut Self, activityid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserActivityFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2084067160, data2: 13853, data3: 19047, data4: [138, 59, 52, 202, 41, 120, 249, 163] };
}
#[repr(C)]
pub struct IUserActivityRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetUserActivity: unsafe extern "system" fn(this: *mut *mut Self, activity: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserActivityRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2700043093, data2: 53045, data3: 20464, data4: [136, 51, 80, 203, 75, 114, 224, 109] };
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
impl ::windows_sys::core::Interface for IUserActivityRequestManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 204521038, data2: 36925, data3: 18646, data4: [130, 212, 64, 67, 237, 87, 121, 27] };
}
#[repr(C)]
pub struct IUserActivityRequestManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserActivityRequestManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3224972785, data2: 8778, data3: 17196, data4: [129, 229, 12, 118, 180, 196, 206, 250] };
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
impl ::windows_sys::core::Interface for IUserActivityRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2764864076, data2: 33321, data3: 19709, data4: [163, 188, 198, 29, 49, 133, 117, 164] };
}
#[repr(C)]
pub struct IUserActivitySession {
    pub base__: ::windows_sys::core::IInspectable,
    pub ActivityId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserActivitySession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2923646328, data2: 9466, data3: 17571, data4: [173, 72, 110, 218, 97, 170, 25, 36] };
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
impl ::windows_sys::core::Interface for IUserActivitySessionHistoryItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3906313171, data2: 15965, data3: 18941, data4: [152, 215, 109, 169, 117, 33, 226, 85] };
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
impl ::windows_sys::core::Interface for IUserActivityStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2358235955, data2: 3593, data3: 18422, data4: [154, 199, 149, 207, 92, 57, 54, 123] };
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
impl ::windows_sys::core::Interface for IUserActivityVisualElements {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2490725651, data2: 9775, data3: 18927, data4: [187, 191, 155, 117, 210, 232, 82, 80] };
}
#[repr(C)]
pub struct IUserActivityVisualElements2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AttributionDisplayText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAttributionDisplayText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserActivityVisualElements2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3400433607, data2: 16111, data3: 17241, data4: [130, 92, 157, 81, 185, 34, 13, 227] };
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
