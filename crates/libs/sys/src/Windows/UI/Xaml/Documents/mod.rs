pub type Block = *mut ::core::ffi::c_void;
pub type BlockCollection = *mut ::core::ffi::c_void;
pub type Bold = *mut ::core::ffi::c_void;
pub type ContactContentLinkProvider = *mut ::core::ffi::c_void;
pub type ContentLink = *mut ::core::ffi::c_void;
pub type ContentLinkInvokedEventArgs = *mut ::core::ffi::c_void;
pub type ContentLinkProvider = *mut ::core::ffi::c_void;
pub type ContentLinkProviderCollection = *mut ::core::ffi::c_void;
pub type Glyphs = *mut ::core::ffi::c_void;
pub type Hyperlink = *mut ::core::ffi::c_void;
pub type HyperlinkClickEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IBlock {
    pub base__: ::windows_sys::core::IInspectable,
    pub TextAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::TextAlignment) -> ::windows_sys::core::HRESULT,
    pub SetTextAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::TextAlignment) -> ::windows_sys::core::HRESULT,
    pub LineHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetLineHeight: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub LineStackingStrategy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::LineStackingStrategy) -> ::windows_sys::core::HRESULT,
    pub SetLineStackingStrategy: unsafe extern "system" fn(this: *mut *mut Self, value: super::LineStackingStrategy) -> ::windows_sys::core::HRESULT,
    pub Margin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Thickness) -> ::windows_sys::core::HRESULT,
    pub SetMargin: unsafe extern "system" fn(this: *mut *mut Self, value: super::Thickness) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBlock2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HorizontalTextAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::TextAlignment) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalTextAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: super::TextAlignment) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBlockFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBlockStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TextAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LineHeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LineStackingStrategyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MarginProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBlockStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HorizontalTextAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBold {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IContactContentLinkProvider {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IContentLink {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Text")]
    pub Info: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    Info: usize,
    #[cfg(feature = "UI_Text")]
    pub SetInfo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetInfo: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Background: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Background: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetBackground: usize,
    #[cfg(feature = "UI_Core")]
    pub Cursor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Core::CoreCursorType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    Cursor: usize,
    #[cfg(feature = "UI_Core")]
    pub SetCursor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Core::CoreCursorType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    SetCursor: usize,
    pub XYFocusLeft: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetXYFocusLeft: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusRight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetXYFocusRight: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusUp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetXYFocusUp: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusDown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetXYFocusDown: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ElementSoundMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::ElementSoundMode) -> ::windows_sys::core::HRESULT,
    pub SetElementSoundMode: unsafe extern "system" fn(this: *mut *mut Self, value: super::ElementSoundMode) -> ::windows_sys::core::HRESULT,
    pub FocusState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::FocusState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusUpNavigationStrategy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusUpNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusUpNavigationStrategy: unsafe extern "system" fn(this: *mut *mut Self, value: super::Input::XYFocusNavigationStrategy) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusUpNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusDownNavigationStrategy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusDownNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusDownNavigationStrategy: unsafe extern "system" fn(this: *mut *mut Self, value: super::Input::XYFocusNavigationStrategy) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusDownNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusLeftNavigationStrategy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusLeftNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusLeftNavigationStrategy: unsafe extern "system" fn(this: *mut *mut Self, value: super::Input::XYFocusNavigationStrategy) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusLeftNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusRightNavigationStrategy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusRightNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusRightNavigationStrategy: unsafe extern "system" fn(this: *mut *mut Self, value: super::Input::XYFocusNavigationStrategy) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusRightNavigationStrategy: usize,
    pub IsTabStop: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsTabStop: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub TabIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetTabIndex: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Invoked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Invoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInvoked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInvoked: usize,
    #[cfg(feature = "Foundation")]
    pub GotFocus: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGotFocus: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub LostFocus: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LostFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLostFocus: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLostFocus: usize,
    pub Focus: unsafe extern "system" fn(this: *mut *mut Self, value: super::FocusState, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentLinkInvokedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Text")]
    pub ContentLinkInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    ContentLinkInfo: usize,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentLinkProvider {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IContentLinkProviderCollection {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IContentLinkProviderFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IContentLinkStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub BackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CursorProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusLeftProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusRightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusUpProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusDownProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ElementSoundModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FocusStateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusUpNavigationStrategyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusDownNavigationStrategyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusLeftNavigationStrategyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusRightNavigationStrategyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsTabStopProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TabIndexProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGlyphs {
    pub base__: ::windows_sys::core::IInspectable,
    pub UnicodeString: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetUnicodeString: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Indices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetIndices: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FontUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FontUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetFontUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetFontUri: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub StyleSimulations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Media::StyleSimulations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    StyleSimulations: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetStyleSimulations: unsafe extern "system" fn(this: *mut *mut Self, value: super::Media::StyleSimulations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetStyleSimulations: usize,
    pub FontRenderingEmSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFontRenderingEmSize: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub OriginX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetOriginX: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub OriginY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetOriginY: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Fill: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Fill: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFill: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFill: usize,
}
#[repr(C)]
pub struct IGlyphs2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsColorFontEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsColorFontEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ColorFontPaletteIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetColorFontPaletteIndex: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGlyphsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub UnicodeStringProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IndicesProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontUriProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StyleSimulationsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontRenderingEmSizeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OriginXProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OriginYProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FillProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGlyphsStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsColorFontEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ColorFontPaletteIndexProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHyperlink {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub NavigateUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NavigateUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetNavigateUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetNavigateUri: usize,
    #[cfg(feature = "Foundation")]
    pub Click: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Click: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClick: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClick: usize,
}
#[repr(C)]
pub struct IHyperlink2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub UnderlineStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UnderlineStyle) -> ::windows_sys::core::HRESULT,
    pub SetUnderlineStyle: unsafe extern "system" fn(this: *mut *mut Self, value: UnderlineStyle) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHyperlink3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub XYFocusLeft: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetXYFocusLeft: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusRight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetXYFocusRight: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusUp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetXYFocusUp: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusDown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetXYFocusDown: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ElementSoundMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::ElementSoundMode) -> ::windows_sys::core::HRESULT,
    pub SetElementSoundMode: unsafe extern "system" fn(this: *mut *mut Self, value: super::ElementSoundMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHyperlink4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub FocusState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::FocusState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusUpNavigationStrategy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusUpNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusUpNavigationStrategy: unsafe extern "system" fn(this: *mut *mut Self, value: super::Input::XYFocusNavigationStrategy) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusUpNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusDownNavigationStrategy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusDownNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusDownNavigationStrategy: unsafe extern "system" fn(this: *mut *mut Self, value: super::Input::XYFocusNavigationStrategy) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusDownNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusLeftNavigationStrategy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusLeftNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusLeftNavigationStrategy: unsafe extern "system" fn(this: *mut *mut Self, value: super::Input::XYFocusNavigationStrategy) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusLeftNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusRightNavigationStrategy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Input::XYFocusNavigationStrategy) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusRightNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusRightNavigationStrategy: unsafe extern "system" fn(this: *mut *mut Self, value: super::Input::XYFocusNavigationStrategy) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusRightNavigationStrategy: usize,
    #[cfg(feature = "Foundation")]
    pub GotFocus: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGotFocus: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub LostFocus: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LostFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLostFocus: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLostFocus: usize,
    pub Focus: unsafe extern "system" fn(this: *mut *mut Self, value: super::FocusState, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHyperlink5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTabStop: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsTabStop: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub TabIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetTabIndex: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHyperlinkClickEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IHyperlinkStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub NavigateUriProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHyperlinkStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub UnderlineStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHyperlinkStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub XYFocusLeftProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusRightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusUpProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusDownProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ElementSoundModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHyperlinkStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub FocusStateProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusUpNavigationStrategyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusDownNavigationStrategyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusLeftNavigationStrategyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub XYFocusRightNavigationStrategyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IHyperlinkStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTabStopProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TabIndexProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInline {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IInlineFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IInlineUIContainer {
    pub base__: ::windows_sys::core::IInspectable,
    pub Child: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetChild: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IItalic {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ILineBreak {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IParagraph {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Inlines: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Inlines: usize,
    pub TextIndent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetTextIndent: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IParagraphStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TextIndentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPlaceContentLinkProvider {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IRun {
    pub base__: ::windows_sys::core::IInspectable,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FlowDirection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::FlowDirection) -> ::windows_sys::core::HRESULT,
    pub SetFlowDirection: unsafe extern "system" fn(this: *mut *mut Self, value: super::FlowDirection) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRunStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FlowDirectionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpan {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Inlines: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Inlines: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetInlines: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetInlines: usize,
}
#[repr(C)]
pub struct ISpanFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextElement {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FontSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetFontSize: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FontFamily: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FontFamily: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFontFamily: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFontFamily: usize,
    #[cfg(feature = "UI_Text")]
    pub FontWeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontWeight: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub FontStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontStyle: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub FontStretch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::FontStretch) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontStretch: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontStretch: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::FontStretch) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontStretch: usize,
    pub CharacterSpacing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetCharacterSpacing: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Foreground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Foreground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetForeground: usize,
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ContentStart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ContentEnd: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ElementStart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ElementEnd: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FindName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextElement2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTextScaleFactorEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsTextScaleFactorEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextElement3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllowFocusOnInteraction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowFocusOnInteraction: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AccessKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAccessKey: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ExitDisplayModeOnAccessKeyInvoked: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetExitDisplayModeOnAccessKeyInvoked: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextElement4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Text")]
    pub TextDecorations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Text::TextDecorations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    TextDecorations: usize,
    #[cfg(feature = "UI_Text")]
    pub SetTextDecorations: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Text::TextDecorations) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetTextDecorations: usize,
    pub IsAccessKeyScope: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsAccessKeyScope: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AccessKeyScopeOwner: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAccessKeyScopeOwner: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub KeyTipPlacementMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Input::KeyTipPlacementMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    KeyTipPlacementMode: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetKeyTipPlacementMode: unsafe extern "system" fn(this: *mut *mut Self, value: super::Input::KeyTipPlacementMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetKeyTipPlacementMode: usize,
    pub KeyTipHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetKeyTipHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub KeyTipVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetKeyTipVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub AccessKeyDisplayRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    AccessKeyDisplayRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccessKeyDisplayRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccessKeyDisplayRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub AccessKeyDisplayDismissed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    AccessKeyDisplayDismissed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccessKeyDisplayDismissed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccessKeyDisplayDismissed: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input"))]
    pub AccessKeyInvoked: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Xaml_Input")))]
    AccessKeyInvoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccessKeyInvoked: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccessKeyInvoked: usize,
}
#[repr(C)]
pub struct ITextElement5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub XamlRoot: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetXamlRoot: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextElementFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITextElementOverrides {
    pub base__: ::windows_sys::core::IInspectable,
    pub OnDisconnectVisualChildren: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextElementStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FontSizeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontFamilyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontWeightProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FontStretchProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CharacterSpacingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LanguageProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextElementStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTextScaleFactorEnabledProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextElementStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllowFocusOnInteractionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AccessKeyProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ExitDisplayModeOnAccessKeyInvokedProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextElementStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TextDecorationsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsAccessKeyScopeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AccessKeyScopeOwnerProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub KeyTipPlacementModeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub KeyTipHorizontalOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub KeyTipVerticalOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextHighlighter {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Ranges: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Ranges: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Foreground: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Foreground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetForeground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Background: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Background: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetBackground: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetBackground: usize,
}
#[repr(C)]
pub struct ITextHighlighterBase {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITextHighlighterBaseFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITextHighlighterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextHighlighterStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ForegroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BackgroundProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextPointer {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VisualParent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LogicalDirection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LogicalDirection) -> ::windows_sys::core::HRESULT,
    pub Offset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetCharacterRect: unsafe extern "system" fn(this: *mut *mut Self, direction: LogicalDirection, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCharacterRect: usize,
    pub GetPositionAtOffset: unsafe extern "system" fn(this: *mut *mut Self, offset: i32, direction: LogicalDirection, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITypography {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITypographyStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub AnnotationAlternatesProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetAnnotationAlternates: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAnnotationAlternates: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: i32) -> ::windows_sys::core::HRESULT,
    pub EastAsianExpertFormsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetEastAsianExpertForms: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetEastAsianExpertForms: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub EastAsianLanguageProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetEastAsianLanguage: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut super::FontEastAsianLanguage) -> ::windows_sys::core::HRESULT,
    pub SetEastAsianLanguage: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: super::FontEastAsianLanguage) -> ::windows_sys::core::HRESULT,
    pub EastAsianWidthsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetEastAsianWidths: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut super::FontEastAsianWidths) -> ::windows_sys::core::HRESULT,
    pub SetEastAsianWidths: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: super::FontEastAsianWidths) -> ::windows_sys::core::HRESULT,
    pub StandardLigaturesProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStandardLigatures: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStandardLigatures: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub ContextualLigaturesProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetContextualLigatures: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetContextualLigatures: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub DiscretionaryLigaturesProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDiscretionaryLigatures: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetDiscretionaryLigatures: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub HistoricalLigaturesProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetHistoricalLigatures: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHistoricalLigatures: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub StandardSwashesProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStandardSwashes: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetStandardSwashes: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: i32) -> ::windows_sys::core::HRESULT,
    pub ContextualSwashesProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetContextualSwashes: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetContextualSwashes: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: i32) -> ::windows_sys::core::HRESULT,
    pub ContextualAlternatesProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetContextualAlternates: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetContextualAlternates: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub StylisticAlternatesProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStylisticAlternates: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetStylisticAlternates: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: i32) -> ::windows_sys::core::HRESULT,
    pub StylisticSet1Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStylisticSet1: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStylisticSet1: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub StylisticSet2Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStylisticSet2: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStylisticSet2: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub StylisticSet3Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStylisticSet3: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStylisticSet3: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub StylisticSet4Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStylisticSet4: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStylisticSet4: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub StylisticSet5Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStylisticSet5: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStylisticSet5: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub StylisticSet6Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStylisticSet6: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStylisticSet6: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub StylisticSet7Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStylisticSet7: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStylisticSet7: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub StylisticSet8Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStylisticSet8: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStylisticSet8: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub StylisticSet9Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStylisticSet9: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStylisticSet9: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub StylisticSet10Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStylisticSet10: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStylisticSet10: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub StylisticSet11Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStylisticSet11: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStylisticSet11: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub StylisticSet12Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStylisticSet12: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStylisticSet12: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub StylisticSet13Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStylisticSet13: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStylisticSet13: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub StylisticSet14Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStylisticSet14: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStylisticSet14: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub StylisticSet15Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStylisticSet15: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStylisticSet15: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub StylisticSet16Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStylisticSet16: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStylisticSet16: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub StylisticSet17Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStylisticSet17: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStylisticSet17: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub StylisticSet18Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStylisticSet18: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStylisticSet18: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub StylisticSet19Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStylisticSet19: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStylisticSet19: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub StylisticSet20Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStylisticSet20: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetStylisticSet20: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub CapitalsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCapitals: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut super::FontCapitals) -> ::windows_sys::core::HRESULT,
    pub SetCapitals: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: super::FontCapitals) -> ::windows_sys::core::HRESULT,
    pub CapitalSpacingProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCapitalSpacing: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCapitalSpacing: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub KerningProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetKerning: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetKerning: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub CaseSensitiveFormsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCaseSensitiveForms: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCaseSensitiveForms: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub HistoricalFormsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetHistoricalForms: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHistoricalForms: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub FractionProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetFraction: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut super::FontFraction) -> ::windows_sys::core::HRESULT,
    pub SetFraction: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: super::FontFraction) -> ::windows_sys::core::HRESULT,
    pub NumeralStyleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNumeralStyle: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut super::FontNumeralStyle) -> ::windows_sys::core::HRESULT,
    pub SetNumeralStyle: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: super::FontNumeralStyle) -> ::windows_sys::core::HRESULT,
    pub NumeralAlignmentProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNumeralAlignment: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut super::FontNumeralAlignment) -> ::windows_sys::core::HRESULT,
    pub SetNumeralAlignment: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: super::FontNumeralAlignment) -> ::windows_sys::core::HRESULT,
    pub SlashedZeroProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetSlashedZero: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSlashedZero: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub MathematicalGreekProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetMathematicalGreek: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetMathematicalGreek: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: bool) -> ::windows_sys::core::HRESULT,
    pub VariantsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetVariants: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, result__: *mut super::FontVariants) -> ::windows_sys::core::HRESULT,
    pub SetVariants: unsafe extern "system" fn(this: *mut *mut Self, element: *mut ::core::ffi::c_void, value: super::FontVariants) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUnderline {
    pub base__: ::windows_sys::core::IInspectable,
}
pub type Inline = *mut ::core::ffi::c_void;
pub type InlineCollection = *mut ::core::ffi::c_void;
pub type InlineUIContainer = *mut ::core::ffi::c_void;
pub type Italic = *mut ::core::ffi::c_void;
pub type LineBreak = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct LogicalDirection(pub i32);
impl LogicalDirection {
    pub const Backward: Self = Self(0i32);
    pub const Forward: Self = Self(1i32);
}
impl ::core::marker::Copy for LogicalDirection {}
impl ::core::clone::Clone for LogicalDirection {
    fn clone(&self) -> Self {
        *self
    }
}
pub type Paragraph = *mut ::core::ffi::c_void;
pub type PlaceContentLinkProvider = *mut ::core::ffi::c_void;
pub type Run = *mut ::core::ffi::c_void;
pub type Span = *mut ::core::ffi::c_void;
pub type TextElement = *mut ::core::ffi::c_void;
pub type TextHighlighter = *mut ::core::ffi::c_void;
pub type TextHighlighterBase = *mut ::core::ffi::c_void;
pub type TextPointer = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
pub struct TextRange {
    pub StartIndex: i32,
    pub Length: i32,
}
impl ::core::marker::Copy for TextRange {}
impl ::core::clone::Clone for TextRange {
    fn clone(&self) -> Self {
        *self
    }
}
pub type Typography = *mut ::core::ffi::c_void;
pub type Underline = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct UnderlineStyle(pub i32);
impl UnderlineStyle {
    pub const None: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
}
impl ::core::marker::Copy for UnderlineStyle {}
impl ::core::clone::Clone for UnderlineStyle {
    fn clone(&self) -> Self {
        *self
    }
}
