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
#[repr(C)]
pub struct IActivatedEventArgsDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub ActivatedOperation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IActivatedOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IWebUIBackgroundTaskInstance {
    pub base__: ::windows_sys::core::IInspectable,
    pub Succeeded: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSucceeded: unsafe extern "system" fn(this: *mut *mut Self, succeeded: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebUIBackgroundTaskInstanceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Current: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebUINavigatedDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebUINavigatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub NavigatedOperation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebUINavigatedOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
