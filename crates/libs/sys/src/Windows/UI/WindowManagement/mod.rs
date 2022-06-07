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
impl ::windows_sys::core::Interface for IAppWindow {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1714427046, data2: 46942, data3: 23997, data4: [153, 92, 240, 17, 127, 163, 251, 97] };
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
impl ::windows_sys::core::Interface for IAppWindowChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 501347262, data2: 42581, data3: 21933, data4: [178, 182, 235, 36, 15, 136, 3, 86] };
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
impl ::windows_sys::core::Interface for IAppWindowCloseRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3925803482, data2: 59298, data3: 22440, data4: [139, 94, 57, 196, 0, 58, 253, 187] };
}
#[repr(C)]
pub struct IAppWindowClosedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppWindowClosedReason) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppWindowClosedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3430807574, data2: 38176, data3: 23046, data4: [130, 30, 69, 106, 216, 179, 88, 170] };
}
#[repr(C)]
pub struct IAppWindowFrame {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Composition"))]
    pub DragRegionVisuals: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Composition")))]
    DragRegionVisuals: usize,
}
impl ::windows_sys::core::Interface for IAppWindowFrame {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2665620993, data2: 32349, data3: 21167, data4: [132, 107, 1, 220, 108, 41, 101, 103] };
}
#[repr(C)]
pub struct IAppWindowFrameStyle {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetFrameStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppWindowFrameStyle) -> ::windows_sys::core::HRESULT,
    pub SetFrameStyle: unsafe extern "system" fn(this: *mut *mut Self, framestyle: AppWindowFrameStyle) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppWindowFrameStyle {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2889951558, data2: 57772, data3: 21040, data4: [148, 74, 198, 8, 115, 220, 244, 169] };
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
impl ::windows_sys::core::Interface for IAppWindowPlacement {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 64782686, data2: 59305, data3: 22615, data4: [156, 3, 125, 103, 5, 148, 65, 14] };
}
#[repr(C)]
pub struct IAppWindowPresentationConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppWindowPresentationKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppWindowPresentationConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3047440099, data2: 57139, data3: 24167, data4: [189, 49, 16, 114, 69, 115, 0, 223] };
}
#[repr(C)]
pub struct IAppWindowPresentationConfigurationFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IAppWindowPresentationConfigurationFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4248176294, data2: 30837, data3: 24040, data4: [132, 255, 99, 81, 238, 19, 221, 13] };
}
#[repr(C)]
pub struct IAppWindowPresenter {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetConfiguration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsPresentationSupported: unsafe extern "system" fn(this: *mut *mut Self, presentationkind: AppWindowPresentationKind, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub RequestPresentation: unsafe extern "system" fn(this: *mut *mut Self, configuration: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub RequestPresentationByKind: unsafe extern "system" fn(this: *mut *mut Self, presentationkind: AppWindowPresentationKind, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppWindowPresenter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1525280115, data2: 57853, data3: 21271, data4: [173, 120, 90, 62, 210, 113, 187, 222] };
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
impl ::windows_sys::core::Interface for IAppWindowStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4280237731, data2: 46953, data3: 20719, data4: [152, 115, 16, 140, 208, 232, 151, 70] };
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
impl ::windows_sys::core::Interface for IAppWindowTitleBar {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1855138948, data2: 63044, data3: 21533, data4: [162, 215, 12, 38, 36, 55, 132, 45] };
}
#[repr(C)]
pub struct IAppWindowTitleBarOcclusion {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub OccludingRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OccludingRect: usize,
}
impl ::windows_sys::core::Interface for IAppWindowTitleBarOcclusion {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4272148477, data2: 11471, data3: 24515, data4: [174, 174, 248, 67, 135, 107, 243, 126] };
}
#[repr(C)]
pub struct IAppWindowTitleBarVisibility {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetPreferredVisibility: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AppWindowTitleBarVisibility) -> ::windows_sys::core::HRESULT,
    pub SetPreferredVisibility: unsafe extern "system" fn(this: *mut *mut Self, visibilitymode: AppWindowTitleBarVisibility) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAppWindowTitleBarVisibility {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2719327459, data2: 28286, data3: 22097, data4: [140, 59, 98, 72, 25, 82, 129, 84] };
}
#[repr(C)]
pub struct ICompactOverlayPresentationConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ICompactOverlayPresentationConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2816832783, data2: 22320, data3: 22214, data4: [142, 31, 214, 63, 244, 215, 152, 13] };
}
#[repr(C)]
pub struct IDefaultPresentationConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IDefaultPresentationConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3636639035, data2: 8552, data3: 22275, data4: [168, 83, 213, 37, 88, 159, 226, 185] };
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
impl ::windows_sys::core::Interface for IDisplayRegion {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3679503266, data2: 16532, data3: 24391, data4: [140, 177, 234, 1, 221, 175, 170, 148] };
}
#[repr(C)]
pub struct IFullScreenPresentationConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsExclusive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsExclusive: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFullScreenPresentationConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1137958104, data2: 53928, data3: 20541, data4: [166, 38, 21, 83, 61, 109, 95, 98] };
}
#[repr(C)]
pub struct IWindowServicesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllTopLevelWindowIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllTopLevelWindowIds: usize,
}
impl ::windows_sys::core::Interface for IWindowServicesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3488929049, data2: 20646, data3: 23652, data4: [151, 246, 194, 217, 106, 221, 127, 66] };
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
impl ::windows_sys::core::Interface for IWindowingEnvironment {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 641950656, data2: 10825, data3: 21527, data4: [179, 174, 72, 167, 28, 99, 163, 189] };
}
#[repr(C)]
pub struct IWindowingEnvironmentAddedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub WindowingEnvironment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWindowingEnvironmentAddedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4280966015, data2: 61827, data3: 23654, data4: [153, 178, 66, 144, 130, 6, 146, 153] };
}
#[repr(C)]
pub struct IWindowingEnvironmentChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IWindowingEnvironmentChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1096863686, data2: 573, data3: 24218, data4: [180, 49, 53, 14, 103, 220, 151, 138] };
}
#[repr(C)]
pub struct IWindowingEnvironmentRemovedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub WindowingEnvironment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWindowingEnvironmentRemovedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 777737331, data2: 48895, data3: 24147, data4: [147, 22, 126, 119, 95, 229, 104, 179] };
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
impl ::windows_sys::core::Interface for IWindowingEnvironmentStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2270076855, data2: 50754, data3: 21931, data4: [138, 162, 22, 47, 115, 74, 154, 114] };
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
