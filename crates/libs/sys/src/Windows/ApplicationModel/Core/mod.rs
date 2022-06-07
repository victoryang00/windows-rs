pub type AppListEntry = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
#[repr(transparent)]
pub struct AppRestartFailureReason(pub i32);
impl AppRestartFailureReason {
    pub const RestartPending: Self = Self(0i32);
    pub const NotInForeground: Self = Self(1i32);
    pub const InvalidUser: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
}
impl ::core::marker::Copy for AppRestartFailureReason {}
impl ::core::clone::Clone for AppRestartFailureReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CoreApplicationView = *mut ::core::ffi::c_void;
pub type CoreApplicationViewTitleBar = *mut ::core::ffi::c_void;
pub type HostedViewClosingEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAppListEntry {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LaunchAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchAsync: usize,
}
impl ::windows_sys::core::Interface for IAppListEntry {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4009816191, data2: 8456, data3: 18698, data4: [135, 122, 138, 159, 23, 194, 95, 173] };
}
#[repr(C)]
pub struct IAppListEntry2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppListEntry2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3500546221, data2: 48949, data3: 17068, data4: [172, 6, 134, 238, 235, 65, 208, 75] };
}
#[repr(C)]
pub struct IAppListEntry3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub LaunchForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    LaunchForUserAsync: usize,
}
impl ::windows_sys::core::Interface for IAppListEntry3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1620701837, data2: 64562, data3: 18186, data4: [188, 105, 75, 6, 26, 118, 239, 46] };
}
#[repr(C)]
pub struct IAppListEntry4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppListEntry4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 705896146, data2: 22261, data3: 18556, data4: [134, 151, 81, 102, 243, 179, 61, 160] };
}
#[repr(C)]
pub struct ICoreApplication {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Suspending: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Suspending: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSuspending: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSuspending: usize,
    #[cfg(feature = "Foundation")]
    pub Resuming: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Resuming: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveResuming: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveResuming: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub GetCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Run: unsafe extern "system" fn(this: *mut *mut Self, viewsource: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RunWithActivationFactories: unsafe extern "system" fn(this: *mut *mut Self, activationfactorycallback: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunWithActivationFactories: usize,
}
impl ::windows_sys::core::Interface for ICoreApplication {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 179107748, data2: 24093, data3: 18911, data4: [128, 52, 251, 106, 104, 188, 94, 209] };
}
#[repr(C)]
pub struct ICoreApplication2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub BackgroundActivated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))]
    BackgroundActivated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBackgroundActivated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBackgroundActivated: usize,
    #[cfg(feature = "Foundation")]
    pub LeavingBackground: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LeavingBackground: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLeavingBackground: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLeavingBackground: usize,
    #[cfg(feature = "Foundation")]
    pub EnteredBackground: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnteredBackground: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnteredBackground: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnteredBackground: usize,
    pub EnablePrelaunch: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreApplication2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2575729147, data2: 6838, data3: 19327, data4: [190, 74, 154, 6, 69, 34, 76, 4] };
}
#[repr(C)]
pub struct ICoreApplication3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestRestartAsync: unsafe extern "system" fn(this: *mut *mut Self, launcharguments: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestRestartAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub RequestRestartForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, launcharguments: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    RequestRestartForUserAsync: usize,
}
impl ::windows_sys::core::Interface for ICoreApplication3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4276882745, data2: 22923, data3: 17671, data4: [138, 103, 119, 38, 50, 88, 10, 87] };
}
#[repr(C)]
pub struct ICoreApplicationExit {
    pub base__: ::windows_sys::core::IInspectable,
    pub Exit: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Exiting: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Exiting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveExiting: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveExiting: usize,
}
impl ::windows_sys::core::Interface for ICoreApplicationExit {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3481683485, data2: 9758, data3: 19314, data4: [154, 205, 68, 237, 42, 206, 106, 41] };
}
#[repr(C)]
pub struct ICoreApplicationUnhandledError {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub UnhandledErrorDetected: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnhandledErrorDetected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUnhandledErrorDetected: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUnhandledErrorDetected: usize,
}
impl ::windows_sys::core::Interface for ICoreApplicationUnhandledError {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4041362096, data2: 56585, data3: 17121, data4: [176, 188, 224, 225, 49, 247, 141, 126] };
}
#[repr(C)]
pub struct ICoreApplicationUseCount {
    pub base__: ::windows_sys::core::IInspectable,
    pub IncrementApplicationUseCount: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub DecrementApplicationUseCount: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreApplicationUseCount {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1368245256, data2: 49271, data3: 18267, data4: [128, 158, 11, 192, 197, 126, 75, 116] };
}
#[repr(C)]
pub struct ICoreApplicationView {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Core")]
    pub CoreWindow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    CoreWindow: usize,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub Activated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))]
    Activated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActivated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActivated: usize,
    pub IsMain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsHosted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreApplicationView {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1670099675, data2: 17693, data3: 18017, data4: [176, 153, 65, 79, 52, 255, 185, 241] };
}
#[repr(C)]
pub struct ICoreApplicationView2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Core")]
    pub Dispatcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    Dispatcher: usize,
}
impl ::windows_sys::core::Interface for ICoreApplicationView2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1760262879, data2: 37247, data3: 18667, data4: [154, 235, 125, 229, 62, 8, 106, 177] };
}
#[repr(C)]
pub struct ICoreApplicationView3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsComponent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TitleBar: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub HostedViewClosing: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HostedViewClosing: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHostedViewClosing: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHostedViewClosing: usize,
}
impl ::windows_sys::core::Interface for ICoreApplicationView3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 132899251, data2: 42191, data3: 17744, data4: [171, 112, 176, 126, 133, 51, 11, 200] };
}
#[repr(C)]
pub struct ICoreApplicationView5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
impl ::windows_sys::core::Interface for ICoreApplicationView5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 734041512, data2: 36592, data3: 17517, data4: [158, 96, 58, 62, 4, 40, 198, 113] };
}
#[repr(C)]
pub struct ICoreApplicationView6 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
}
impl ::windows_sys::core::Interface for ICoreApplicationView6 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3239695514, data2: 1657, data3: 18874, data4: [128, 63, 183, 156, 92, 243, 76, 202] };
}
#[repr(C)]
pub struct ICoreApplicationViewTitleBar {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetExtendViewIntoTitleBar: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ExtendViewIntoTitleBar: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SystemOverlayLeftInset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SystemOverlayRightInset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LayoutMetricsChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LayoutMetricsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLayoutMetricsChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLayoutMetricsChanged: usize,
    pub IsVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsVisibleChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsVisibleChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsVisibleChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsVisibleChanged: usize,
}
impl ::windows_sys::core::Interface for ICoreApplicationViewTitleBar {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 7157219, data2: 57841, data3: 17179, data4: [149, 8, 41, 185, 105, 38, 172, 83] };
}
#[repr(C)]
pub struct ICoreImmersiveApplication {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Views: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Views: usize,
    pub CreateNewView: unsafe extern "system" fn(this: *mut *mut Self, runtimetype: ::windows_sys::core::HSTRING, entrypoint: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MainView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreImmersiveApplication {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 450498110, data2: 58530, data3: 16675, data4: [180, 81, 220, 150, 191, 128, 4, 25] };
}
#[repr(C)]
pub struct ICoreImmersiveApplication2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateNewViewFromMainView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreImmersiveApplication2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2190351926, data2: 59875, data3: 19708, data4: [155, 102, 72, 183, 142, 169, 187, 44] };
}
#[repr(C)]
pub struct ICoreImmersiveApplication3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateNewViewWithViewSource: unsafe extern "system" fn(this: *mut *mut Self, viewsource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreImmersiveApplication3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 882924335, data2: 60941, data3: 16869, data4: [131, 20, 207, 16, 201, 27, 240, 175] };
}
#[repr(C)]
pub struct IFrameworkView {
    pub base__: ::windows_sys::core::IInspectable,
    pub Initialize: unsafe extern "system" fn(this: *mut *mut Self, applicationview: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub SetWindow: unsafe extern "system" fn(this: *mut *mut Self, window: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    SetWindow: usize,
    pub Load: unsafe extern "system" fn(this: *mut *mut Self, entrypoint: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Run: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Uninitialize: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFrameworkView {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4205534416, data2: 35108, data3: 17836, data4: [173, 15, 160, 143, 174, 93, 3, 36] };
}
#[repr(C)]
pub struct IFrameworkViewSource {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFrameworkViewSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3447129620, data2: 26052, data3: 17004, data4: [148, 148, 52, 252, 67, 85, 72, 98] };
}
#[repr(C)]
pub struct IHostedViewClosingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IHostedViewClosingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3526923324, data2: 45646, data3: 18320, data4: [172, 181, 62, 66, 67, 196, 255, 135] };
}
#[repr(C)]
pub struct IUnhandledError {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Propagate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUnhandledError {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2488907558, data2: 21429, data3: 18054, data4: [158, 175, 250, 129, 98, 220, 57, 128] };
}
#[repr(C)]
pub struct IUnhandledErrorDetectedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub UnhandledError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUnhandledErrorDetectedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1738192779, data2: 45878, data3: 18466, data4: [172, 64, 13, 117, 15, 11, 122, 43] };
}
pub type UnhandledError = *mut ::core::ffi::c_void;
pub type UnhandledErrorDetectedEventArgs = *mut ::core::ffi::c_void;
