pub type DesignerAppExitedEventArgs = *mut ::core::ffi::c_void;
pub type DesignerAppManager = *mut ::core::ffi::c_void;
pub type DesignerAppView = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct DesignerAppViewState(pub i32);
impl DesignerAppViewState {
    pub const Visible: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
}
impl ::core::marker::Copy for DesignerAppViewState {}
impl ::core::clone::Clone for DesignerAppViewState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DesktopWindowXamlSource = *mut ::core::ffi::c_void;
pub type DesktopWindowXamlSourceGotFocusEventArgs = *mut ::core::ffi::c_void;
pub type DesktopWindowXamlSourceTakeFocusRequestedEventArgs = *mut ::core::ffi::c_void;
pub type ElementCompositionPreview = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IDesignerAppExitedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ExitCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDesignerAppManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DesignerAppExited: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesignerAppExited: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDesignerAppExited: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDesignerAppExited: usize,
    #[cfg(feature = "Foundation")]
    pub CreateNewViewAsync: unsafe extern "system" fn(this: *mut *mut Self, initialviewstate: DesignerAppViewState, initialviewsize: super::super::super::Foundation::Size, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateNewViewAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LoadObjectIntoAppAsync: unsafe extern "system" fn(this: *mut *mut Self, dllname: ::windows_sys::core::HSTRING, classid: ::windows_sys::core::GUID, initializationdata: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadObjectIntoAppAsync: usize,
}
#[repr(C)]
pub struct IDesignerAppManagerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, appusermodelid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDesignerAppView {
    pub base__: ::windows_sys::core::IInspectable,
    pub ApplicationViewId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ViewState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DesignerAppViewState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ViewSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ViewSize: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateViewAsync: unsafe extern "system" fn(this: *mut *mut Self, viewstate: DesignerAppViewState, viewsize: super::super::super::Foundation::Size, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateViewAsync: usize,
}
#[repr(C)]
pub struct IDesktopWindowXamlSource {
    pub base__: ::windows_sys::core::IInspectable,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HasFocus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TakeFocusRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TakeFocusRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTakeFocusRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTakeFocusRequested: usize,
    #[cfg(feature = "Foundation")]
    pub GotFocus: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGotFocus: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGotFocus: usize,
    pub NavigateFocus: unsafe extern "system" fn(this: *mut *mut Self, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDesktopWindowXamlSourceFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDesktopWindowXamlSourceGotFocusEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IElementCompositionPreview {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IElementCompositionPreviewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Composition")]
    pub GetElementVisual: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetElementVisual: usize,
    #[cfg(feature = "UI_Composition")]
    pub GetElementChildVisual: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetElementChildVisual: usize,
    #[cfg(feature = "UI_Composition")]
    pub SetElementChildVisual: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, visual: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    SetElementChildVisual: usize,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Controls"))]
    pub GetScrollViewerManipulationPropertySet: unsafe extern "system" fn(this: *mut *mut Self, scrollviewer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Controls")))]
    GetScrollViewerManipulationPropertySet: usize,
}
#[repr(C)]
pub struct IElementCompositionPreviewStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Composition")]
    pub SetImplicitShowAnimation: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    SetImplicitShowAnimation: usize,
    #[cfg(feature = "UI_Composition")]
    pub SetImplicitHideAnimation: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    SetImplicitHideAnimation: usize,
    pub SetIsTranslationEnabled: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub GetPointerPositionPropertySet: unsafe extern "system" fn(this: *mut *mut Self, targetelement: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetPointerPositionPropertySet: usize,
}
#[repr(C)]
pub struct IElementCompositionPreviewStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_WindowManagement")]
    pub SetAppWindowContent: unsafe extern "system" fn(this: *mut *mut Self, appwindow: *mut ::core::ffi::c_void, xamlcontent: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))]
    SetAppWindowContent: usize,
    #[cfg(feature = "UI_WindowManagement")]
    pub GetAppWindowContent: unsafe extern "system" fn(this: *mut *mut Self, appwindow: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))]
    GetAppWindowContent: usize,
}
#[repr(C)]
pub struct IWindowsXamlManager {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IWindowsXamlManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub InitializeForCurrentThread: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlSourceFocusNavigationRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut XamlSourceFocusNavigationReason) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub HintRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HintRect: usize,
    pub CorrelationId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlSourceFocusNavigationRequestFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, reason: XamlSourceFocusNavigationReason, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateInstanceWithHintRect: unsafe extern "system" fn(this: *mut *mut Self, reason: XamlSourceFocusNavigationReason, hintrect: super::super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstanceWithHintRect: usize,
    #[cfg(feature = "Foundation")]
    pub CreateInstanceWithHintRectAndCorrelationId: unsafe extern "system" fn(this: *mut *mut Self, reason: XamlSourceFocusNavigationReason, hintrect: super::super::super::Foundation::Rect, correlationid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstanceWithHintRectAndCorrelationId: usize,
}
#[repr(C)]
pub struct IXamlSourceFocusNavigationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub WasFocusMoved: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlSourceFocusNavigationResultFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, focusmoved: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlUIPresenter {
    pub base__: ::windows_sys::core::IInspectable,
    pub RootElement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRootElement: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ThemeKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetThemeKey: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ThemeResourcesXaml: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetThemeResourcesXaml: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSize: unsafe extern "system" fn(this: *mut *mut Self, width: i32, height: i32) -> ::windows_sys::core::HRESULT,
    pub Render: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Present: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlUIPresenterHost {
    pub base__: ::windows_sys::core::IInspectable,
    pub ResolveFileResource: unsafe extern "system" fn(this: *mut *mut Self, path: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlUIPresenterHost2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetGenericXamlFilePath: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlUIPresenterHost3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ResolveDictionaryResource: unsafe extern "system" fn(this: *mut *mut Self, dictionary: *mut ::core::ffi::c_void, dictionarykey: *mut ::core::ffi::c_void, suggestedvalue: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlUIPresenterStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CompleteTimelinesAutomatically: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCompleteTimelinesAutomatically: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SetHost: unsafe extern "system" fn(this: *mut *mut Self, host: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NotifyWindowSizeChanged: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IXamlUIPresenterStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives"))]
    pub GetFlyoutPlacementTargetInfo: unsafe extern "system" fn(this: *mut *mut Self, placementtarget: *mut ::core::ffi::c_void, preferredplacement: super::Controls::Primitives::FlyoutPlacementMode, targetpreferredplacement: *mut super::Controls::Primitives::FlyoutPlacementMode, allowfallbacks: *mut bool, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives")))]
    GetFlyoutPlacementTargetInfo: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives"))]
    pub GetFlyoutPlacement: unsafe extern "system" fn(this: *mut *mut Self, placementtargetbounds: super::super::super::Foundation::Rect, controlsize: super::super::super::Foundation::Size, mincontrolsize: super::super::super::Foundation::Size, containerrect: super::super::super::Foundation::Rect, targetpreferredplacement: super::Controls::Primitives::FlyoutPlacementMode, allowfallbacks: bool, chosenplacement: *mut super::Controls::Primitives::FlyoutPlacementMode, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives")))]
    GetFlyoutPlacement: usize,
}
pub type WindowsXamlManager = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct XamlSourceFocusNavigationReason(pub i32);
impl XamlSourceFocusNavigationReason {
    pub const Programmatic: Self = Self(0i32);
    pub const Restore: Self = Self(1i32);
    pub const First: Self = Self(3i32);
    pub const Last: Self = Self(4i32);
    pub const Left: Self = Self(7i32);
    pub const Up: Self = Self(8i32);
    pub const Right: Self = Self(9i32);
    pub const Down: Self = Self(10i32);
}
impl ::core::marker::Copy for XamlSourceFocusNavigationReason {}
impl ::core::clone::Clone for XamlSourceFocusNavigationReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type XamlSourceFocusNavigationRequest = *mut ::core::ffi::c_void;
pub type XamlSourceFocusNavigationResult = *mut ::core::ffi::c_void;
pub type XamlUIPresenter = *mut ::core::ffi::c_void;
