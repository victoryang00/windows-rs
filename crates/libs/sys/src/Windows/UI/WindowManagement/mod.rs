#[cfg(feature = "UI_WindowManagement_Preview")]
pub mod Preview;
pub type AppWindow = *mut ::core::ffi::c_void;
pub type AppWindowChangedEventArgs = *mut ::core::ffi::c_void;
pub type AppWindowCloseRequestedEventArgs = *mut ::core::ffi::c_void;
pub type AppWindowClosedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
pub struct AppWindowClosedReason(pub i32);
impl AppWindowClosedReason {
    pub const Other: Self = Self(0i32);
    pub const AppInitiated: Self = Self(1i32);
    pub const UserInitiated: Self = Self(2i32);
}
impl ::core::marker::Copy for AppWindowClosedReason {}
impl ::core::clone::Clone for AppWindowClosedReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppWindowFrame = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
pub struct AppWindowFrameStyle(pub i32);
impl AppWindowFrameStyle {
    pub const Default: Self = Self(0i32);
    pub const NoFrame: Self = Self(1i32);
}
impl ::core::marker::Copy for AppWindowFrameStyle {}
impl ::core::clone::Clone for AppWindowFrameStyle {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppWindowPlacement = *mut ::core::ffi::c_void;
pub type AppWindowPresentationConfiguration = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
pub struct AppWindowPresentationKind(pub i32);
impl AppWindowPresentationKind {
    pub const Default: Self = Self(0i32);
    pub const CompactOverlay: Self = Self(1i32);
    pub const FullScreen: Self = Self(2i32);
}
impl ::core::marker::Copy for AppWindowPresentationKind {}
impl ::core::clone::Clone for AppWindowPresentationKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AppWindowPresenter = *mut ::core::ffi::c_void;
pub type AppWindowTitleBar = *mut ::core::ffi::c_void;
pub type AppWindowTitleBarOcclusion = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
pub struct AppWindowTitleBarVisibility(pub i32);
impl AppWindowTitleBarVisibility {
    pub const Default: Self = Self(0i32);
    pub const AlwaysHidden: Self = Self(1i32);
}
impl ::core::marker::Copy for AppWindowTitleBarVisibility {}
impl ::core::clone::Clone for AppWindowTitleBarVisibility {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CompactOverlayPresentationConfiguration = *mut ::core::ffi::c_void;
pub type DefaultPresentationConfiguration = *mut ::core::ffi::c_void;
pub type DisplayRegion = *mut ::core::ffi::c_void;
pub type FullScreenPresentationConfiguration = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAppWindow {
    pub base__: ::windows_sys::core::IInspectable,
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
    pub Frame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub PersistedStateId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPersistedStateId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Presenter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TitleBar: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UIContext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub WindowingEnvironment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CloseAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CloseAsync: usize,
    pub GetPlacement: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDisplayRegions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDisplayRegions: usize,
    pub RequestMoveToDisplayRegion: unsafe extern "system" fn(this: *mut *mut Self, displayregion: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RequestMoveAdjacentToCurrentView: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RequestMoveAdjacentToWindow: unsafe extern "system" fn(this: *mut *mut Self, anchorwindow: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestMoveRelativeToWindowContent: unsafe extern "system" fn(this: *mut *mut Self, anchorwindow: *mut ::core::ffi::c_void, contentoffset: super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestMoveRelativeToWindowContent: usize,
    #[cfg(feature = "Foundation")]
    pub RequestMoveRelativeToCurrentViewContent: unsafe extern "system" fn(this: *mut *mut Self, contentoffset: super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestMoveRelativeToCurrentViewContent: usize,
    #[cfg(feature = "Foundation")]
    pub RequestMoveRelativeToDisplayRegion: unsafe extern "system" fn(this: *mut *mut Self, displayregion: *mut ::core::ffi::c_void, displayregionoffset: super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestMoveRelativeToDisplayRegion: usize,
    #[cfg(feature = "Foundation")]
    pub RequestSize: unsafe extern "system" fn(this: *mut *mut Self, framesize: super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSize: usize,
    #[cfg(feature = "Foundation")]
    pub TryShowAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryShowAsync: usize,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Changed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChanged: usize,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    #[cfg(feature = "Foundation")]
    pub CloseRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CloseRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCloseRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCloseRequested: usize,
}
#[repr(C)]
pub struct IAppWindowChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub DidAvailableWindowPresentationsChange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub DidDisplayRegionsChange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub DidFrameChange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub DidSizeChange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub DidTitleBarChange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub DidVisibilityChange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub DidWindowingEnvironmentChange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub DidWindowPresentationChange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppWindowCloseRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(C)]
pub struct IAppWindowClosedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppWindowClosedReason) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppWindowFrame {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Composition"))]
    pub DragRegionVisuals: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Composition")))]
    DragRegionVisuals: usize,
}
#[repr(C)]
pub struct IAppWindowFrameStyle {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetFrameStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppWindowFrameStyle) -> ::windows_sys::core::HRESULT,
    pub SetFrameStyle: unsafe extern "system" fn(this: *mut *mut Self, framestyle: AppWindowFrameStyle) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppWindowPlacement {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayRegion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Offset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Offset: usize,
    #[cfg(feature = "Foundation")]
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Size: usize,
}
#[repr(C)]
pub struct IAppWindowPresentationConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppWindowPresentationKind) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppWindowPresentationConfigurationFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IAppWindowPresenter {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetConfiguration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsPresentationSupported: unsafe extern "system" fn(this: *mut *mut Self, presentationkind: AppWindowPresentationKind, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub RequestPresentation: unsafe extern "system" fn(this: *mut *mut Self, configuration: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub RequestPresentationByKind: unsafe extern "system" fn(this: *mut *mut Self, presentationkind: AppWindowPresentationKind, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppWindowStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TryCreateAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCreateAsync: usize,
    pub ClearAllPersistedState: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ClearPersistedState: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IAppWindowTitleBar {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonHoverBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonHoverBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonHoverBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonHoverBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonHoverForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonHoverForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonHoverForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonHoverForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonInactiveBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonInactiveBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonInactiveBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonInactiveBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonInactiveForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonInactiveForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonInactiveForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonInactiveForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonPressedBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonPressedBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonPressedBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonPressedBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonPressedForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonPressedForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonPressedForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonPressedForegroundColor: usize,
    pub ExtendsContentIntoTitleBar: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetExtendsContentIntoTitleBar: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub InactiveBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InactiveBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetInactiveBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInactiveBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub InactiveForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InactiveForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetInactiveForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInactiveForegroundColor: usize,
    pub IsVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTitleBarOcclusions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTitleBarOcclusions: usize,
}
#[repr(C)]
pub struct IAppWindowTitleBarOcclusion {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub OccludingRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OccludingRect: usize,
}
#[repr(C)]
pub struct IAppWindowTitleBarVisibility {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetPreferredVisibility: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppWindowTitleBarVisibility) -> ::windows_sys::core::HRESULT,
    pub SetPreferredVisibility: unsafe extern "system" fn(this: *mut *mut Self, visibilitymode: AppWindowTitleBarVisibility) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompactOverlayPresentationConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IDefaultPresentationConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IDisplayRegion {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayMonitorDeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub WorkAreaOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WorkAreaOffset: usize,
    #[cfg(feature = "Foundation")]
    pub WorkAreaSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WorkAreaSize: usize,
    pub WindowingEnvironment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Changed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChanged: usize,
}
#[repr(C)]
pub struct IFullScreenPresentationConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsExclusive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsExclusive: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWindowServicesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllTopLevelWindowIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllTopLevelWindowIds: usize,
}
#[repr(C)]
pub struct IWindowingEnvironment {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WindowingEnvironmentKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDisplayRegions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDisplayRegions: usize,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Changed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChanged: usize,
}
#[repr(C)]
pub struct IWindowingEnvironmentAddedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub WindowingEnvironment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWindowingEnvironmentChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IWindowingEnvironmentRemovedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub WindowingEnvironment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWindowingEnvironmentStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAll: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAll: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllWithKind: unsafe extern "system" fn(this: *mut *mut Self, kind: WindowingEnvironmentKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllWithKind: usize,
}
pub type WindowingEnvironment = *mut ::core::ffi::c_void;
pub type WindowingEnvironmentAddedEventArgs = *mut ::core::ffi::c_void;
pub type WindowingEnvironmentChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_WindowManagement\"`*"]
#[repr(transparent)]
pub struct WindowingEnvironmentKind(pub i32);
impl WindowingEnvironmentKind {
    pub const Unknown: Self = Self(0i32);
    pub const Overlapped: Self = Self(1i32);
    pub const Tiled: Self = Self(2i32);
}
impl ::core::marker::Copy for WindowingEnvironmentKind {}
impl ::core::clone::Clone for WindowingEnvironmentKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WindowingEnvironmentRemovedEventArgs = *mut ::core::ffi::c_void;
