#[cfg(feature = "Web_UI_Interop")]
pub mod Interop;
#[repr(C)]
pub struct IWebViewControl {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Source: usize,
    #[cfg(feature = "Foundation")]
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSource: usize,
    pub DocumentTitle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CanGoBack: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanGoForward: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI")]
    pub SetDefaultBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetDefaultBackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub DefaultBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    DefaultBackgroundColor: usize,
    pub ContainsFullScreenElement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Settings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DeferredPermissionRequests: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeferredPermissionRequests: usize,
    pub GoForward: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GoBack: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Navigate: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Navigate: usize,
    pub NavigateToString: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NavigateToLocalStreamUri: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, streamresolver: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NavigateToLocalStreamUri: usize,
    #[cfg(feature = "Web_Http")]
    pub NavigateWithHttpRequestMessage: unsafe extern "system" fn(this: *mut *mut Self, requestmessage: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    NavigateWithHttpRequestMessage: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InvokeScriptAsync: unsafe extern "system" fn(this: *mut *mut Self, scriptname: ::windows_sys::core::HSTRING, arguments: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InvokeScriptAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CapturePreviewToStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CapturePreviewToStreamAsync: usize,
    #[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation"))]
    pub CaptureSelectedContentToDataPackageAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation")))]
    CaptureSelectedContentToDataPackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub BuildLocalStreamUri: unsafe extern "system" fn(this: *mut *mut Self, contentidentifier: ::windows_sys::core::HSTRING, relativepath: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BuildLocalStreamUri: usize,
    pub GetDeferredPermissionRequestById: unsafe extern "system" fn(this: *mut *mut Self, id: u32, result: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NavigationStarting: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NavigationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNavigationStarting: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNavigationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub ContentLoading: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentLoading: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContentLoading: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContentLoading: usize,
    #[cfg(feature = "Foundation")]
    pub DOMContentLoaded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DOMContentLoaded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDOMContentLoaded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDOMContentLoaded: usize,
    #[cfg(feature = "Foundation")]
    pub NavigationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NavigationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNavigationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNavigationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub FrameNavigationStarting: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameNavigationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFrameNavigationStarting: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFrameNavigationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub FrameContentLoading: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameContentLoading: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFrameContentLoading: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFrameContentLoading: usize,
    #[cfg(feature = "Foundation")]
    pub FrameDOMContentLoaded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameDOMContentLoaded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFrameDOMContentLoaded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFrameDOMContentLoaded: usize,
    #[cfg(feature = "Foundation")]
    pub FrameNavigationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameNavigationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFrameNavigationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFrameNavigationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub ScriptNotify: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScriptNotify: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveScriptNotify: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveScriptNotify: usize,
    #[cfg(feature = "Foundation")]
    pub LongRunningScriptDetected: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LongRunningScriptDetected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLongRunningScriptDetected: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLongRunningScriptDetected: usize,
    #[cfg(feature = "Foundation")]
    pub UnsafeContentWarningDisplaying: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnsafeContentWarningDisplaying: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUnsafeContentWarningDisplaying: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUnsafeContentWarningDisplaying: usize,
    #[cfg(feature = "Foundation")]
    pub UnviewableContentIdentified: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnviewableContentIdentified: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUnviewableContentIdentified: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUnviewableContentIdentified: usize,
    #[cfg(feature = "Foundation")]
    pub PermissionRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PermissionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePermissionRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePermissionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub UnsupportedUriSchemeIdentified: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnsupportedUriSchemeIdentified: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUnsupportedUriSchemeIdentified: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUnsupportedUriSchemeIdentified: usize,
    #[cfg(feature = "Foundation")]
    pub NewWindowRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NewWindowRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNewWindowRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNewWindowRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ContainsFullScreenElementChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContainsFullScreenElementChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContainsFullScreenElementChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContainsFullScreenElementChanged: usize,
    #[cfg(feature = "Foundation")]
    pub WebResourceRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WebResourceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveWebResourceRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveWebResourceRequested: usize,
}
impl ::windows_sys::core::Interface for IWebViewControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1066537750, data2: 48240, data3: 19418, data4: [145, 54, 201, 67, 112, 137, 159, 171] };
}
#[repr(C)]
pub struct IWebViewControl2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AddInitializeScript: unsafe extern "system" fn(this: *mut *mut Self, script: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebViewControl2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295779577, data2: 51423, data3: 16844, data4: [139, 213, 42, 148, 123, 32, 69, 3] };
}
#[repr(C)]
pub struct IWebViewControlContentLoadingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
}
impl ::windows_sys::core::Interface for IWebViewControlContentLoadingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2587872434, data2: 47547, data3: 16459, data4: [162, 43, 102, 220, 205, 18, 80, 198] };
}
#[repr(C)]
pub struct IWebViewControlDOMContentLoadedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
}
impl ::windows_sys::core::Interface for IWebViewControlDOMContentLoadedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3196829704, data2: 38209, data3: 17733, data4: [159, 242, 45, 245, 133, 178, 159, 125] };
}
#[repr(C)]
pub struct IWebViewControlDeferredPermissionRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub PermissionType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WebViewControlPermissionType) -> ::windows_sys::core::HRESULT,
    pub Allow: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Deny: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebViewControlDeferredPermissionRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 753093088, data2: 55129, data3: 17500, data4: [153, 38, 137, 149, 41, 143, 21, 43] };
}
#[repr(C)]
pub struct IWebViewControlLongRunningScriptDetectedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ExecutionTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExecutionTime: usize,
    pub StopPageScriptExecution: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStopPageScriptExecution: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebViewControlLongRunningScriptDetectedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 711875514, data2: 39092, data3: 17852, data4: [187, 235, 15, 105, 206, 73, 197, 153] };
}
#[repr(C)]
pub struct IWebViewControlNavigationCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub IsSuccess: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub WebErrorStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::WebErrorStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebViewControlNavigationCompletedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 541104408, data2: 18965, data3: 19526, data4: [165, 93, 247, 158, 219, 11, 222, 139] };
}
#[repr(C)]
pub struct IWebViewControlNavigationStartingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebViewControlNavigationStartingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 210786245, data2: 2568, data3: 16839, data4: [134, 59, 113, 227, 169, 84, 145, 55] };
}
#[repr(C)]
pub struct IWebViewControlNewWindowRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub Referrer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Referrer: usize,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebViewControlNewWindowRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1039420347, data2: 41252, data3: 18133, data4: [160, 131, 208, 44, 172, 223, 245, 173] };
}
#[repr(C)]
pub struct IWebViewControlNewWindowRequestedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub NewWindow: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetNewWindow: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IWebViewControlNewWindowRequestedEventArgs2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3040631974, data2: 10926, data3: 19452, data4: [146, 185, 195, 14, 146, 180, 128, 152] };
}
#[repr(C)]
pub struct IWebViewControlPermissionRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub PermissionType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WebViewControlPermissionType) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WebViewControlPermissionState) -> ::windows_sys::core::HRESULT,
    pub Defer: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Allow: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Deny: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebViewControlPermissionRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3854336876, data2: 61999, data3: 16610, data4: [149, 178, 119, 41, 248, 64, 235, 127] };
}
#[repr(C)]
pub struct IWebViewControlPermissionRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub PermissionRequest: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebViewControlPermissionRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 656428369, data2: 9352, data3: 19653, data4: [150, 142, 10, 119, 30, 89, 193, 71] };
}
#[repr(C)]
pub struct IWebViewControlScriptNotifyEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebViewControlScriptNotifyEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1226696059, data2: 28489, data3: 16827, data4: [181, 145, 81, 184, 91, 129, 112, 55] };
}
#[repr(C)]
pub struct IWebViewControlSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetIsJavaScriptEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsJavaScriptEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsIndexedDBEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsIndexedDBEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsScriptNotifyAllowed: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsScriptNotifyAllowed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebViewControlSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3382083519, data2: 24216, data3: 19709, data4: [140, 206, 39, 176, 145, 30, 61, 232] };
}
#[repr(C)]
pub struct IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3820493124, data2: 58620, data3: 17372, data4: [148, 202, 249, 128, 243, 11, 197, 29] };
}
#[repr(C)]
pub struct IWebViewControlUnviewableContentIdentifiedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub Referrer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Referrer: usize,
    pub MediaType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebViewControlUnviewableContentIdentifiedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1251377371, data2: 35058, data3: 20000, data4: [182, 147, 180, 226, 223, 74, 165, 129] };
}
#[repr(C)]
pub struct IWebViewControlWebResourceRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
    #[cfg(feature = "Web_Http")]
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    Request: usize,
    #[cfg(feature = "Web_Http")]
    pub SetResponse: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    SetResponse: usize,
    #[cfg(feature = "Web_Http")]
    pub Response: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    Response: usize,
}
impl ::windows_sys::core::Interface for IWebViewControlWebResourceRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1154896461, data2: 21924, data3: 19851, data4: [137, 28, 147, 29, 142, 37, 212, 46] };
}
pub type WebViewControlContentLoadingEventArgs = *mut ::core::ffi::c_void;
pub type WebViewControlDOMContentLoadedEventArgs = *mut ::core::ffi::c_void;
pub type WebViewControlDeferredPermissionRequest = *mut ::core::ffi::c_void;
pub type WebViewControlLongRunningScriptDetectedEventArgs = *mut ::core::ffi::c_void;
pub type WebViewControlNavigationCompletedEventArgs = *mut ::core::ffi::c_void;
pub type WebViewControlNavigationStartingEventArgs = *mut ::core::ffi::c_void;
pub type WebViewControlNewWindowRequestedEventArgs = *mut ::core::ffi::c_void;
pub type WebViewControlPermissionRequest = *mut ::core::ffi::c_void;
pub type WebViewControlPermissionRequestedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlPermissionState(pub i32);
impl WebViewControlPermissionState {
    pub const Unknown: Self = Self(0i32);
    pub const Defer: Self = Self(1i32);
    pub const Allow: Self = Self(2i32);
    pub const Deny: Self = Self(3i32);
}
impl ::core::marker::Copy for WebViewControlPermissionState {}
impl ::core::clone::Clone for WebViewControlPermissionState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlPermissionType(pub i32);
impl WebViewControlPermissionType {
    pub const Geolocation: Self = Self(0i32);
    pub const UnlimitedIndexedDBQuota: Self = Self(1i32);
    pub const Media: Self = Self(2i32);
    pub const PointerLock: Self = Self(3i32);
    pub const WebNotifications: Self = Self(4i32);
    pub const Screen: Self = Self(5i32);
    pub const ImmersiveView: Self = Self(6i32);
}
impl ::core::marker::Copy for WebViewControlPermissionType {}
impl ::core::clone::Clone for WebViewControlPermissionType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WebViewControlScriptNotifyEventArgs = *mut ::core::ffi::c_void;
pub type WebViewControlSettings = *mut ::core::ffi::c_void;
pub type WebViewControlUnsupportedUriSchemeIdentifiedEventArgs = *mut ::core::ffi::c_void;
pub type WebViewControlUnviewableContentIdentifiedEventArgs = *mut ::core::ffi::c_void;
pub type WebViewControlWebResourceRequestedEventArgs = *mut ::core::ffi::c_void;
