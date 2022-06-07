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
impl ::windows_sys::core::Interface for IWebUICommandBar {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2767978518, data2: 56293, data3: 16813, data4: [141, 123, 20, 105, 139, 214, 145, 29] };
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
impl ::windows_sys::core::Interface for IWebUICommandBarBitmapIcon {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2240761669, data2: 2264, data3: 19014, data4: [129, 236, 0, 1, 91, 11, 28, 108] };
}
#[repr(C)]
pub struct IWebUICommandBarBitmapIconFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IWebUICommandBarBitmapIconFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4093106058, data2: 30323, data3: 17482, data4: [190, 98, 172, 18, 211, 28, 34, 49] };
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
impl ::windows_sys::core::Interface for IWebUICommandBarConfirmationButton {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2263319114, data2: 58325, data3: 20150, data4: [178, 255, 143, 1, 138, 23, 33, 5] };
}
#[repr(C)]
pub struct IWebUICommandBarElement {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IWebUICommandBarElement {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3372654274, data2: 10314, data3: 17971, data4: [138, 173, 99, 122, 39, 226, 130, 195] };
}
#[repr(C)]
pub struct IWebUICommandBarIcon {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IWebUICommandBarIcon {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3582420317, data2: 8212, data3: 17086, data4: [150, 154, 125, 20, 202, 108, 138, 73] };
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
impl ::windows_sys::core::Interface for IWebUICommandBarIconButton {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2400962874, data2: 14972, data3: 18498, data4: [160, 207, 175, 246, 234, 48, 133, 134] };
}
#[repr(C)]
pub struct IWebUICommandBarItemInvokedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPrimaryCommand: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebUICommandBarItemInvokedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 810474461, data2: 59201, data3: 16879, data4: [189, 196, 164, 92, 234, 42, 79, 112] };
}
#[repr(C)]
pub struct IWebUICommandBarSizeChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Size: usize,
}
impl ::windows_sys::core::Interface for IWebUICommandBarSizeChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4226933494, data2: 12329, data3: 18201, data4: [131, 120, 146, 248, 43, 135, 175, 30] };
}
#[repr(C)]
pub struct IWebUICommandBarStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebUICommandBarStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 340381113, data2: 42246, data3: 17854, data4: [143, 66, 178, 131, 126, 47, 224, 201] };
}
#[repr(C)]
pub struct IWebUICommandBarSymbolIcon {
    pub base__: ::windows_sys::core::IInspectable,
    pub Symbol: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSymbol: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebUICommandBarSymbolIcon {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3566425207, data2: 64806, data3: 18157, data4: [134, 88, 26, 63, 68, 0, 231, 179] };
}
#[repr(C)]
pub struct IWebUICommandBarSymbolIconFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, symbol: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWebUICommandBarSymbolIconFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1371413023, data2: 14128, data3: 17054, data4: [182, 34, 20, 226, 183, 191, 106, 7] };
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
