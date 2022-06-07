#[cfg(feature = "UI_WebUI_Core")]
pub mod Core;
pub type ActivatedDeferral = *mut ::core::ffi::c_void;
pub type ActivatedEventHandler = *mut ::core::ffi::c_void;
pub type ActivatedOperation = *mut ::core::ffi::c_void;
pub type BackgroundActivatedEventArgs = *mut ::core::ffi::c_void;
pub type BackgroundActivatedEventHandler = *mut ::core::ffi::c_void;
pub type EnteredBackgroundEventArgs = *mut ::core::ffi::c_void;
pub type EnteredBackgroundEventHandler = *mut ::core::ffi::c_void;
pub type HtmlPrintDocumentSource = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IActivatedDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IActivatedDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3283949944, data2: 42033, data3: 18904, data4: [167, 106, 57, 90, 78, 3, 220, 243] };
}
#[repr(C)]
pub struct IActivatedEventArgsDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub ActivatedOperation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IActivatedEventArgsDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3396165492, data2: 25538, data3: 17574, data4: [185, 123, 217, 160, 60, 32, 188, 155] };
}
#[repr(C)]
pub struct IActivatedOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IActivatedOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3063985340, data2: 50890, data3: 17149, data4: [152, 24, 113, 144, 78, 69, 254, 215] };
}
#[repr(C)]
pub struct IHtmlPrintDocumentSource {
    pub base__: ::windows_sys::core::IInspectable,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PrintContent) -> ::windows_sys::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut *mut Self, value: PrintContent) -> ::windows_sys::core::HRESULT,
    pub LeftMargin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetLeftMargin: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub TopMargin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetTopMargin: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub RightMargin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRightMargin: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub BottomMargin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetBottomMargin: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub EnableHeaderFooter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetEnableHeaderFooter: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ShrinkToFit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShrinkToFit: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub PercentScale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetPercentScale: unsafe extern "system" fn(this: *mut *mut Self, scalepercent: f32) -> ::windows_sys::core::HRESULT,
    pub PageRange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TrySetPageRange: unsafe extern "system" fn(this: *mut *mut Self, strpagerange: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IHtmlPrintDocumentSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3467003546, data2: 3589, data3: 18042, data4: [171, 201, 54, 236, 29, 76, 220, 182] };
}
#[repr(C)]
pub struct INewWebUIViewCreatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub WebUIView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub ActivatedEventArgs: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    ActivatedEventArgs: usize,
    pub HasPendingNavigate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for INewWebUIViewCreatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3907105302, data2: 48683, data3: 19614, data4: [133, 231, 8, 49, 67, 236, 75, 231] };
}
#[repr(C)]
pub struct IWebUIActivationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub Activated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))]
    Activated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActivated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActivated: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub Suspending: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
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
    #[cfg(feature = "Foundation")]
    pub Navigated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Navigated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNavigated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNavigated: usize,
}
impl ::windows_sys::core::Interface for IWebUIActivationStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 890996413, data2: 17331, data3: 18475, data4: [133, 219, 53, 216, 123, 81, 122, 217] };
}
#[repr(C)]
pub struct IWebUIActivationStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub LeavingBackground: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    LeavingBackground: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLeavingBackground: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLeavingBackground: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub EnteredBackground: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    EnteredBackground: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnteredBackground: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnteredBackground: usize,
    pub EnablePrelaunch: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebUIActivationStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3370682006, data2: 19832, data3: 19108, data4: [143, 6, 42, 158, 173, 198, 196, 10] };
}
#[repr(C)]
pub struct IWebUIActivationStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub RequestRestartAsync: unsafe extern "system" fn(this: *mut *mut Self, launcharguments: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))]
    RequestRestartAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "System"))]
    pub RequestRestartForUserAsync: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, launcharguments: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "System")))]
    RequestRestartForUserAsync: usize,
}
impl ::windows_sys::core::Interface for IWebUIActivationStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2443949702, data2: 6901, data3: 17477, data4: [180, 159, 148, 89, 244, 15, 200, 222] };
}
#[repr(C)]
pub struct IWebUIActivationStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub NewWebUIViewCreated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NewWebUIViewCreated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNewWebUIViewCreated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNewWebUIViewCreated: usize,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub BackgroundActivated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))]
    BackgroundActivated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBackgroundActivated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBackgroundActivated: usize,
}
impl ::windows_sys::core::Interface for IWebUIActivationStatics4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1580799017, data2: 6207, data3: 18317, data4: [138, 37, 103, 248, 13, 3, 147, 91] };
}
#[repr(C)]
pub struct IWebUIBackgroundTaskInstance {
    pub base__: ::windows_sys::core::IInspectable,
    pub Succeeded: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSucceeded: unsafe extern "system" fn(this: *mut *mut Self, succeeded: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebUIBackgroundTaskInstance {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 603008037, data2: 58103, data3: 18241, data4: [188, 156, 57, 69, 149, 222, 36, 220] };
}
#[repr(C)]
pub struct IWebUIBackgroundTaskInstanceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Current: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebUIBackgroundTaskInstanceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2625262225, data2: 6574, data3: 19619, data4: [185, 75, 254, 78, 199, 68, 167, 64] };
}
#[repr(C)]
pub struct IWebUINavigatedDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebUINavigatedDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3624149069, data2: 33567, data3: 18146, data4: [180, 50, 58, 252, 226, 17, 249, 98] };
}
#[repr(C)]
pub struct IWebUINavigatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub NavigatedOperation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebUINavigatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2807579064, data2: 9369, data3: 16432, data4: [166, 157, 21, 210, 217, 207, 229, 36] };
}
#[repr(C)]
pub struct IWebUINavigatedOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebUINavigatedOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2056675080, data2: 33154, data3: 19081, data4: [171, 103, 132, 146, 232, 117, 13, 75] };
}
#[repr(C)]
pub struct IWebUIView {
    pub base__: ::windows_sys::core::IInspectable,
    pub ApplicationViewId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub Activated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))]
    Activated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActivated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActivated: usize,
    pub IgnoreApplicationContentUriRulesNavigationRestrictions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIgnoreApplicationContentUriRulesNavigationRestrictions: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebUIView {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1736701519, data2: 21210, data3: 20439, data4: [190, 105, 142, 246, 40, 75, 66, 60] };
}
#[repr(C)]
pub struct IWebUIViewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateWithUriAsync: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWithUriAsync: usize,
}
impl ::windows_sys::core::Interface for IWebUIViewStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3046237800, data2: 36441, data3: 17657, data4: [136, 3, 27, 36, 201, 20, 157, 48] };
}
pub type LeavingBackgroundEventArgs = *mut ::core::ffi::c_void;
pub type LeavingBackgroundEventHandler = *mut ::core::ffi::c_void;
pub type NavigatedEventHandler = *mut ::core::ffi::c_void;
pub type NewWebUIViewCreatedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_WebUI\"`*"]
#[repr(transparent)]
pub struct PrintContent(pub i32);
impl PrintContent {
    pub const AllPages: Self = Self(0i32);
    pub const CurrentPage: Self = Self(1i32);
    pub const CustomPageRange: Self = Self(2i32);
    pub const CurrentSelection: Self = Self(3i32);
}
impl ::core::marker::Copy for PrintContent {}
impl ::core::clone::Clone for PrintContent {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ResumingEventHandler = *mut ::core::ffi::c_void;
pub type SuspendingDeferral = *mut ::core::ffi::c_void;
pub type SuspendingEventArgs = *mut ::core::ffi::c_void;
pub type SuspendingEventHandler = *mut ::core::ffi::c_void;
pub type SuspendingOperation = *mut ::core::ffi::c_void;
pub type WebUIAppointmentsProviderAddAppointmentActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIBackgroundTaskInstanceRuntimeClass = *mut ::core::ffi::c_void;
pub type WebUIBarcodeScannerPreviewActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUICachedFileUpdaterActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUICameraSettingsActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUICommandLineActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIContactCallActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIContactMapActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIContactMessageActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIContactPanelActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIContactPickerActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIContactPostActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIContactVideoCallActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIDeviceActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIDevicePairingActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIDialReceiverActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIFileActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIFileOpenPickerActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIFileOpenPickerContinuationEventArgs = *mut ::core::ffi::c_void;
pub type WebUIFileSavePickerActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIFileSavePickerContinuationEventArgs = *mut ::core::ffi::c_void;
pub type WebUIFolderPickerContinuationEventArgs = *mut ::core::ffi::c_void;
pub type WebUILaunchActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUILockScreenActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUILockScreenCallActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUILockScreenComponentActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUINavigatedDeferral = *mut ::core::ffi::c_void;
pub type WebUINavigatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUINavigatedOperation = *mut ::core::ffi::c_void;
pub type WebUIPhoneCallActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIPrint3DWorkflowActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIPrintTaskSettingsActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIPrintWorkflowForegroundTaskActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIProtocolActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIProtocolForResultsActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIRestrictedLaunchActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUISearchActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIShareTargetActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIStartupTaskActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIToastNotificationActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIUserDataAccountProviderActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIView = *mut ::core::ffi::c_void;
pub type WebUIVoiceCommandActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIWalletActionActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIWebAccountProviderActivatedEventArgs = *mut ::core::ffi::c_void;
pub type WebUIWebAuthenticationBrokerContinuationEventArgs = *mut ::core::ffi::c_void;
