#[repr(C)]
pub struct ILanguageFont {
    pub base__: ::windows_sys::core::IInspectable,
    pub FontFamily: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Text")]
    pub FontWeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Text::FontWeight) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub FontStretch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Text::FontStretch) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontStretch: usize,
    #[cfg(feature = "UI_Text")]
    pub FontStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Text::FontStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontStyle: usize,
    pub ScaleFactor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILanguageFont {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2972605498, data2: 46957, data3: 17819, data4: [190, 235, 144, 17, 81, 205, 119, 209] };
}
#[repr(C)]
pub struct ILanguageFontGroup {
    pub base__: ::windows_sys::core::IInspectable,
    pub UITextFont: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UIHeadingFont: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UITitleFont: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UICaptionFont: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UINotificationHeadingFont: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TraditionalDocumentFont: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ModernDocumentFont: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DocumentHeadingFont: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FixedWidthTextFont: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DocumentAlternate1Font: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DocumentAlternate2Font: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILanguageFontGroup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4080697283, data2: 14940, data3: 19178, data4: [185, 255, 179, 159, 178, 66, 247, 246] };
}
#[repr(C)]
pub struct ILanguageFontGroupFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateLanguageFontGroup: unsafe extern "system" fn(this: *mut *mut Self, languagetag: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILanguageFontGroupFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4239305831, data2: 20087, data3: 18887, data4: [184, 86, 221, 233, 52, 252, 115, 91] };
}
pub type LanguageFont = *mut ::core::ffi::c_void;
pub type LanguageFontGroup = *mut ::core::ffi::c_void;
