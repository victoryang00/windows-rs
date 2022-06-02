#[repr(C)]
pub struct IWebUICommandBar {
    pub base__: ::windows_sys::core::IInspectable,
    pub Visible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Opacity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Color) -> ::windows_sys::core::HRESULT,
    pub SetForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Color) -> ::windows_sys::core::HRESULT,
    pub BackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Color) -> ::windows_sys::core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Color) -> ::windows_sys::core::HRESULT,
    pub ClosedDisplayMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut WebUICommandBarClosedDisplayMode) -> ::windows_sys::core::HRESULT,
    pub SetClosedDisplayMode: unsafe extern "system" fn(this: *mut *mut Self, value: WebUICommandBarClosedDisplayMode) -> ::windows_sys::core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsOpen: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Size: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PrimaryCommands: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PrimaryCommands: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SecondaryCommands: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SecondaryCommands: usize,
    #[cfg(feature = "Foundation")]
    pub MenuOpened: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MenuOpened: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMenuOpened: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMenuOpened: usize,
    #[cfg(feature = "Foundation")]
    pub MenuClosed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MenuClosed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMenuClosed: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMenuClosed: usize,
    #[cfg(feature = "Foundation")]
    pub SizeChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SizeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSizeChanged: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSizeChanged: usize,
}
#[repr(C)]
pub struct IWebUICommandBarBitmapIcon {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
}
#[repr(C)]
pub struct IWebUICommandBarBitmapIconFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
}
#[repr(C)]
pub struct IWebUICommandBarConfirmationButton {
    pub base__: ::windows_sys::core::IInspectable,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ItemInvoked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemInvoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemInvoked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemInvoked: usize,
}
#[repr(C)]
pub struct IWebUICommandBarElement {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IWebUICommandBarIcon {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IWebUICommandBarIconButton {
    pub base__: ::windows_sys::core::IInspectable,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsToggleButton: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsToggleButton: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsChecked: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsChecked: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Icon: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetIcon: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ItemInvoked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemInvoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemInvoked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemInvoked: usize,
}
#[repr(C)]
pub struct IWebUICommandBarItemInvokedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPrimaryCommand: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebUICommandBarSizeChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Size: usize,
}
#[repr(C)]
pub struct IWebUICommandBarStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebUICommandBarSymbolIcon {
    pub base__: ::windows_sys::core::IInspectable,
    pub Symbol: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSymbol: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWebUICommandBarSymbolIconFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, symbol: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type MenuClosedEventHandler = *mut ::core::ffi::c_void;
pub type MenuOpenedEventHandler = *mut ::core::ffi::c_void;
pub type SizeChangedEventHandler = *mut ::core::ffi::c_void;
pub type WebUICommandBar = *mut ::core::ffi::c_void;
pub type WebUICommandBarBitmapIcon = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct WebUICommandBarClosedDisplayMode(pub i32);
impl WebUICommandBarClosedDisplayMode {
    pub const Default: Self = Self(0i32);
    pub const Minimal: Self = Self(1i32);
    pub const Compact: Self = Self(2i32);
}
impl ::core::marker::Copy for WebUICommandBarClosedDisplayMode {}
impl ::core::clone::Clone for WebUICommandBarClosedDisplayMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WebUICommandBarConfirmationButton = *mut ::core::ffi::c_void;
pub type WebUICommandBarIconButton = *mut ::core::ffi::c_void;
pub type WebUICommandBarItemInvokedEventArgs = *mut ::core::ffi::c_void;
pub type WebUICommandBarSizeChangedEventArgs = *mut ::core::ffi::c_void;
pub type WebUICommandBarSymbolIcon = *mut ::core::ffi::c_void;
