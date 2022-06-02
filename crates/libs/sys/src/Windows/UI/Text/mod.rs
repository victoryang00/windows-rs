#[cfg(feature = "UI_Text_Core")]
pub mod Core;
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct CaretType(pub i32);
impl CaretType {
    pub const Normal: Self = Self(0i32);
    pub const Null: Self = Self(1i32);
}
impl ::core::marker::Copy for CaretType {}
impl ::core::clone::Clone for CaretType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContentLinkInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct FindOptions(pub u32);
impl FindOptions {
    pub const None: Self = Self(0u32);
    pub const Word: Self = Self(2u32);
    pub const Case: Self = Self(4u32);
}
impl ::core::marker::Copy for FindOptions {}
impl ::core::clone::Clone for FindOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct FontStretch(pub i32);
impl FontStretch {
    pub const Undefined: Self = Self(0i32);
    pub const UltraCondensed: Self = Self(1i32);
    pub const ExtraCondensed: Self = Self(2i32);
    pub const Condensed: Self = Self(3i32);
    pub const SemiCondensed: Self = Self(4i32);
    pub const Normal: Self = Self(5i32);
    pub const SemiExpanded: Self = Self(6i32);
    pub const Expanded: Self = Self(7i32);
    pub const ExtraExpanded: Self = Self(8i32);
    pub const UltraExpanded: Self = Self(9i32);
}
impl ::core::marker::Copy for FontStretch {}
impl ::core::clone::Clone for FontStretch {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct FontStyle(pub i32);
impl FontStyle {
    pub const Normal: Self = Self(0i32);
    pub const Oblique: Self = Self(1i32);
    pub const Italic: Self = Self(2i32);
}
impl ::core::marker::Copy for FontStyle {}
impl ::core::clone::Clone for FontStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"UI_Text\"`*"]
pub struct FontWeight {
    pub Weight: u16,
}
impl ::core::marker::Copy for FontWeight {}
impl ::core::clone::Clone for FontWeight {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FontWeights = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct FormatEffect(pub i32);
impl FormatEffect {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Toggle: Self = Self(2i32);
    pub const Undefined: Self = Self(3i32);
}
impl ::core::marker::Copy for FormatEffect {}
impl ::core::clone::Clone for FormatEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct HorizontalCharacterAlignment(pub i32);
impl HorizontalCharacterAlignment {
    pub const Left: Self = Self(0i32);
    pub const Right: Self = Self(1i32);
    pub const Center: Self = Self(2i32);
}
impl ::core::marker::Copy for HorizontalCharacterAlignment {}
impl ::core::clone::Clone for HorizontalCharacterAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IContentLinkInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub DisplayText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SecondaryText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSecondaryText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
    pub LinkContentKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLinkContentKind: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IFontWeights {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IFontWeightsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Black: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FontWeight) -> ::windows_sys::core::HRESULT,
    pub Bold: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FontWeight) -> ::windows_sys::core::HRESULT,
    pub ExtraBlack: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FontWeight) -> ::windows_sys::core::HRESULT,
    pub ExtraBold: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FontWeight) -> ::windows_sys::core::HRESULT,
    pub ExtraLight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FontWeight) -> ::windows_sys::core::HRESULT,
    pub Light: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FontWeight) -> ::windows_sys::core::HRESULT,
    pub Medium: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FontWeight) -> ::windows_sys::core::HRESULT,
    pub Normal: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FontWeight) -> ::windows_sys::core::HRESULT,
    pub SemiBold: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FontWeight) -> ::windows_sys::core::HRESULT,
    pub SemiLight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FontWeight) -> ::windows_sys::core::HRESULT,
    pub Thin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FontWeight) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRichEditTextRange {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContentLinkInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetContentLinkInfo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextCharacterFormat {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllCaps: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FormatEffect) -> ::windows_sys::core::HRESULT,
    pub SetAllCaps: unsafe extern "system" fn(this: *mut *mut Self, value: FormatEffect) -> ::windows_sys::core::HRESULT,
    pub BackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Color) -> ::windows_sys::core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::Color) -> ::windows_sys::core::HRESULT,
    pub Bold: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FormatEffect) -> ::windows_sys::core::HRESULT,
    pub SetBold: unsafe extern "system" fn(this: *mut *mut Self, value: FormatEffect) -> ::windows_sys::core::HRESULT,
    pub FontStretch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FontStretch) -> ::windows_sys::core::HRESULT,
    pub SetFontStretch: unsafe extern "system" fn(this: *mut *mut Self, value: FontStretch) -> ::windows_sys::core::HRESULT,
    pub FontStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FontStyle) -> ::windows_sys::core::HRESULT,
    pub SetFontStyle: unsafe extern "system" fn(this: *mut *mut Self, value: FontStyle) -> ::windows_sys::core::HRESULT,
    pub ForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Color) -> ::windows_sys::core::HRESULT,
    pub SetForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::Color) -> ::windows_sys::core::HRESULT,
    pub Hidden: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FormatEffect) -> ::windows_sys::core::HRESULT,
    pub SetHidden: unsafe extern "system" fn(this: *mut *mut Self, value: FormatEffect) -> ::windows_sys::core::HRESULT,
    pub Italic: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FormatEffect) -> ::windows_sys::core::HRESULT,
    pub SetItalic: unsafe extern "system" fn(this: *mut *mut Self, value: FormatEffect) -> ::windows_sys::core::HRESULT,
    pub Kerning: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetKerning: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub LanguageTag: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLanguageTag: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LinkType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LinkType) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Outline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FormatEffect) -> ::windows_sys::core::HRESULT,
    pub SetOutline: unsafe extern "system" fn(this: *mut *mut Self, value: FormatEffect) -> ::windows_sys::core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub ProtectedText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FormatEffect) -> ::windows_sys::core::HRESULT,
    pub SetProtectedText: unsafe extern "system" fn(this: *mut *mut Self, value: FormatEffect) -> ::windows_sys::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetSize: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub SmallCaps: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FormatEffect) -> ::windows_sys::core::HRESULT,
    pub SetSmallCaps: unsafe extern "system" fn(this: *mut *mut Self, value: FormatEffect) -> ::windows_sys::core::HRESULT,
    pub Spacing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetSpacing: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub Strikethrough: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FormatEffect) -> ::windows_sys::core::HRESULT,
    pub SetStrikethrough: unsafe extern "system" fn(this: *mut *mut Self, value: FormatEffect) -> ::windows_sys::core::HRESULT,
    pub Subscript: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FormatEffect) -> ::windows_sys::core::HRESULT,
    pub SetSubscript: unsafe extern "system" fn(this: *mut *mut Self, value: FormatEffect) -> ::windows_sys::core::HRESULT,
    pub Superscript: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FormatEffect) -> ::windows_sys::core::HRESULT,
    pub SetSuperscript: unsafe extern "system" fn(this: *mut *mut Self, value: FormatEffect) -> ::windows_sys::core::HRESULT,
    pub TextScript: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TextScript) -> ::windows_sys::core::HRESULT,
    pub SetTextScript: unsafe extern "system" fn(this: *mut *mut Self, value: TextScript) -> ::windows_sys::core::HRESULT,
    pub Underline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UnderlineType) -> ::windows_sys::core::HRESULT,
    pub SetUnderline: unsafe extern "system" fn(this: *mut *mut Self, value: UnderlineType) -> ::windows_sys::core::HRESULT,
    pub Weight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetWeight: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub SetClone: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetClone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(this: *mut *mut Self, format: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextConstantsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub AutoColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Color) -> ::windows_sys::core::HRESULT,
    pub MinUnitCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MaxUnitCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub UndefinedColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Color) -> ::windows_sys::core::HRESULT,
    pub UndefinedFloatValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub UndefinedInt32Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub UndefinedFontStretch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FontStretch) -> ::windows_sys::core::HRESULT,
    pub UndefinedFontStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FontStyle) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextDocument {
    pub base__: ::windows_sys::core::IInspectable,
    pub CaretType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CaretType) -> ::windows_sys::core::HRESULT,
    pub SetCaretType: unsafe extern "system" fn(this: *mut *mut Self, value: CaretType) -> ::windows_sys::core::HRESULT,
    pub DefaultTabStop: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetDefaultTabStop: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub Selection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UndoLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetUndoLimit: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub CanCopy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanPaste: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanRedo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanUndo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ApplyDisplayUpdates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub BatchDisplayUpdates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub BeginUndoGroup: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EndUndoGroup: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetDefaultCharacterFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDefaultParagraphFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetRange: unsafe extern "system" fn(this: *mut *mut Self, startposition: i32, endposition: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetRangeFromPoint: unsafe extern "system" fn(this: *mut *mut Self, point: super::super::Foundation::Point, options: PointOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetRangeFromPoint: usize,
    pub GetText: unsafe extern "system" fn(this: *mut *mut Self, options: TextGetOptions, value: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub LoadFromStream: unsafe extern "system" fn(this: *mut *mut Self, options: TextSetOptions, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadFromStream: usize,
    pub Redo: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SaveToStream: unsafe extern "system" fn(this: *mut *mut Self, options: TextGetOptions, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SaveToStream: usize,
    pub SetDefaultCharacterFormat: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDefaultParagraphFormat: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, options: TextSetOptions, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Undo: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextDocument2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AlignmentIncludesTrailingWhitespace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAlignmentIncludesTrailingWhitespace: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IgnoreTrailingCharacterSpacing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIgnoreTrailingCharacterSpacing: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextDocument3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ClearUndoRedoHistory: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextDocument4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetMath: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetMath: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetMathMode: unsafe extern "system" fn(this: *mut *mut Self, mode: RichEditMathMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextParagraphFormat {
    pub base__: ::windows_sys::core::IInspectable,
    pub Alignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ParagraphAlignment) -> ::windows_sys::core::HRESULT,
    pub SetAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: ParagraphAlignment) -> ::windows_sys::core::HRESULT,
    pub FirstLineIndent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub KeepTogether: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FormatEffect) -> ::windows_sys::core::HRESULT,
    pub SetKeepTogether: unsafe extern "system" fn(this: *mut *mut Self, value: FormatEffect) -> ::windows_sys::core::HRESULT,
    pub KeepWithNext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FormatEffect) -> ::windows_sys::core::HRESULT,
    pub SetKeepWithNext: unsafe extern "system" fn(this: *mut *mut Self, value: FormatEffect) -> ::windows_sys::core::HRESULT,
    pub LeftIndent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub LineSpacing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub LineSpacingRule: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LineSpacingRule) -> ::windows_sys::core::HRESULT,
    pub ListAlignment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MarkerAlignment) -> ::windows_sys::core::HRESULT,
    pub SetListAlignment: unsafe extern "system" fn(this: *mut *mut Self, value: MarkerAlignment) -> ::windows_sys::core::HRESULT,
    pub ListLevelIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetListLevelIndex: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub ListStart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetListStart: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub ListStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MarkerStyle) -> ::windows_sys::core::HRESULT,
    pub SetListStyle: unsafe extern "system" fn(this: *mut *mut Self, value: MarkerStyle) -> ::windows_sys::core::HRESULT,
    pub ListTab: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetListTab: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub ListType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MarkerType) -> ::windows_sys::core::HRESULT,
    pub SetListType: unsafe extern "system" fn(this: *mut *mut Self, value: MarkerType) -> ::windows_sys::core::HRESULT,
    pub NoLineNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FormatEffect) -> ::windows_sys::core::HRESULT,
    pub SetNoLineNumber: unsafe extern "system" fn(this: *mut *mut Self, value: FormatEffect) -> ::windows_sys::core::HRESULT,
    pub PageBreakBefore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FormatEffect) -> ::windows_sys::core::HRESULT,
    pub SetPageBreakBefore: unsafe extern "system" fn(this: *mut *mut Self, value: FormatEffect) -> ::windows_sys::core::HRESULT,
    pub RightIndent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRightIndent: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub RightToLeft: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FormatEffect) -> ::windows_sys::core::HRESULT,
    pub SetRightToLeft: unsafe extern "system" fn(this: *mut *mut Self, value: FormatEffect) -> ::windows_sys::core::HRESULT,
    pub Style: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ParagraphStyle) -> ::windows_sys::core::HRESULT,
    pub SetStyle: unsafe extern "system" fn(this: *mut *mut Self, value: ParagraphStyle) -> ::windows_sys::core::HRESULT,
    pub SpaceAfter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetSpaceAfter: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub SpaceBefore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetSpaceBefore: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub WidowControl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FormatEffect) -> ::windows_sys::core::HRESULT,
    pub SetWidowControl: unsafe extern "system" fn(this: *mut *mut Self, value: FormatEffect) -> ::windows_sys::core::HRESULT,
    pub TabCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub AddTab: unsafe extern "system" fn(this: *mut *mut Self, position: f32, align: TabAlignment, leader: TabLeader) -> ::windows_sys::core::HRESULT,
    pub ClearAllTabs: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub DeleteTab: unsafe extern "system" fn(this: *mut *mut Self, position: f32) -> ::windows_sys::core::HRESULT,
    pub GetClone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetTab: unsafe extern "system" fn(this: *mut *mut Self, index: i32, position: *mut f32, align: *mut TabAlignment, leader: *mut TabLeader) -> ::windows_sys::core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(this: *mut *mut Self, format: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetClone: unsafe extern "system" fn(this: *mut *mut Self, format: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetIndents: unsafe extern "system" fn(this: *mut *mut Self, start: f32, left: f32, right: f32) -> ::windows_sys::core::HRESULT,
    pub SetLineSpacing: unsafe extern "system" fn(this: *mut *mut Self, rule: LineSpacingRule, spacing: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextRange {
    pub base__: ::windows_sys::core::IInspectable,
    pub Character: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetCharacter: unsafe extern "system" fn(this: *mut *mut Self, value: u16) -> ::windows_sys::core::HRESULT,
    pub CharacterFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCharacterFormat: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FormattedText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetFormattedText: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EndPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetEndPosition: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub Gravity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RangeGravity) -> ::windows_sys::core::HRESULT,
    pub SetGravity: unsafe extern "system" fn(this: *mut *mut Self, value: RangeGravity) -> ::windows_sys::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Link: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLink: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ParagraphFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetParagraphFormat: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StartPosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetStartPosition: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub StoryLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CanPaste: unsafe extern "system" fn(this: *mut *mut Self, format: i32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ChangeCase: unsafe extern "system" fn(this: *mut *mut Self, value: LetterCase) -> ::windows_sys::core::HRESULT,
    pub Collapse: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Copy: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Cut: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut *mut Self, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub EndOf: unsafe extern "system" fn(this: *mut *mut Self, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Expand: unsafe extern "system" fn(this: *mut *mut Self, unit: TextRangeUnit, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub FindText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, scanlength: i32, options: FindOptions, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetCharacterUtf32: unsafe extern "system" fn(this: *mut *mut Self, value: *mut u32, offset: i32) -> ::windows_sys::core::HRESULT,
    pub GetClone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetIndex: unsafe extern "system" fn(this: *mut *mut Self, unit: TextRangeUnit, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPoint: unsafe extern "system" fn(this: *mut *mut Self, horizontalalign: HorizontalCharacterAlignment, verticalalign: VerticalCharacterAlignment, options: PointOptions, point: *mut super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPoint: usize,
    #[cfg(feature = "Foundation")]
    pub GetRect: unsafe extern "system" fn(this: *mut *mut Self, options: PointOptions, rect: *mut super::super::Foundation::Rect, hit: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetRect: usize,
    pub GetText: unsafe extern "system" fn(this: *mut *mut Self, options: TextGetOptions, value: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetTextViaStream: unsafe extern "system" fn(this: *mut *mut Self, options: TextGetOptions, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetTextViaStream: usize,
    pub InRange: unsafe extern "system" fn(this: *mut *mut Self, range: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub InsertImage: unsafe extern "system" fn(this: *mut *mut Self, width: i32, height: i32, ascent: i32, verticalalign: VerticalCharacterAlignment, alternatetext: ::windows_sys::core::HSTRING, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InsertImage: usize,
    pub InStory: unsafe extern "system" fn(this: *mut *mut Self, range: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(this: *mut *mut Self, range: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut *mut Self, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MoveEnd: unsafe extern "system" fn(this: *mut *mut Self, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MoveStart: unsafe extern "system" fn(this: *mut *mut Self, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Paste: unsafe extern "system" fn(this: *mut *mut Self, format: i32) -> ::windows_sys::core::HRESULT,
    pub ScrollIntoView: unsafe extern "system" fn(this: *mut *mut Self, value: PointOptions) -> ::windows_sys::core::HRESULT,
    pub MatchSelection: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetIndex: unsafe extern "system" fn(this: *mut *mut Self, unit: TextRangeUnit, index: i32, extend: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetPoint: unsafe extern "system" fn(this: *mut *mut Self, point: super::super::Foundation::Point, options: PointOptions, extend: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPoint: usize,
    pub SetRange: unsafe extern "system" fn(this: *mut *mut Self, startposition: i32, endposition: i32) -> ::windows_sys::core::HRESULT,
    pub SetText2: unsafe extern "system" fn(this: *mut *mut Self, options: TextSetOptions, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetTextViaStream: unsafe extern "system" fn(this: *mut *mut Self, options: TextSetOptions, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetTextViaStream: usize,
    pub StartOf: unsafe extern "system" fn(this: *mut *mut Self, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextSelection {
    pub base__: ::windows_sys::core::IInspectable,
    pub Options: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SelectionOptions) -> ::windows_sys::core::HRESULT,
    pub SetOptions: unsafe extern "system" fn(this: *mut *mut Self, value: SelectionOptions) -> ::windows_sys::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SelectionType) -> ::windows_sys::core::HRESULT,
    pub EndKey: unsafe extern "system" fn(this: *mut *mut Self, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub HomeKey: unsafe extern "system" fn(this: *mut *mut Self, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MoveDown: unsafe extern "system" fn(this: *mut *mut Self, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MoveLeft: unsafe extern "system" fn(this: *mut *mut Self, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MoveRight: unsafe extern "system" fn(this: *mut *mut Self, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MoveUp: unsafe extern "system" fn(this: *mut *mut Self, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub TypeText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct LetterCase(pub i32);
impl LetterCase {
    pub const Lower: Self = Self(0i32);
    pub const Upper: Self = Self(1i32);
}
impl ::core::marker::Copy for LetterCase {}
impl ::core::clone::Clone for LetterCase {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct LineSpacingRule(pub i32);
impl LineSpacingRule {
    pub const Undefined: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const OneAndHalf: Self = Self(2i32);
    pub const Double: Self = Self(3i32);
    pub const AtLeast: Self = Self(4i32);
    pub const Exactly: Self = Self(5i32);
    pub const Multiple: Self = Self(6i32);
    pub const Percent: Self = Self(7i32);
}
impl ::core::marker::Copy for LineSpacingRule {}
impl ::core::clone::Clone for LineSpacingRule {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct LinkType(pub i32);
impl LinkType {
    pub const Undefined: Self = Self(0i32);
    pub const NotALink: Self = Self(1i32);
    pub const ClientLink: Self = Self(2i32);
    pub const FriendlyLinkName: Self = Self(3i32);
    pub const FriendlyLinkAddress: Self = Self(4i32);
    pub const AutoLink: Self = Self(5i32);
    pub const AutoLinkEmail: Self = Self(6i32);
    pub const AutoLinkPhone: Self = Self(7i32);
    pub const AutoLinkPath: Self = Self(8i32);
}
impl ::core::marker::Copy for LinkType {}
impl ::core::clone::Clone for LinkType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct MarkerAlignment(pub i32);
impl MarkerAlignment {
    pub const Undefined: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
    pub const Center: Self = Self(2i32);
    pub const Right: Self = Self(3i32);
}
impl ::core::marker::Copy for MarkerAlignment {}
impl ::core::clone::Clone for MarkerAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct MarkerStyle(pub i32);
impl MarkerStyle {
    pub const Undefined: Self = Self(0i32);
    pub const Parenthesis: Self = Self(1i32);
    pub const Parentheses: Self = Self(2i32);
    pub const Period: Self = Self(3i32);
    pub const Plain: Self = Self(4i32);
    pub const Minus: Self = Self(5i32);
    pub const NoNumber: Self = Self(6i32);
}
impl ::core::marker::Copy for MarkerStyle {}
impl ::core::clone::Clone for MarkerStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct MarkerType(pub i32);
impl MarkerType {
    pub const Undefined: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Bullet: Self = Self(2i32);
    pub const Arabic: Self = Self(3i32);
    pub const LowercaseEnglishLetter: Self = Self(4i32);
    pub const UppercaseEnglishLetter: Self = Self(5i32);
    pub const LowercaseRoman: Self = Self(6i32);
    pub const UppercaseRoman: Self = Self(7i32);
    pub const UnicodeSequence: Self = Self(8i32);
    pub const CircledNumber: Self = Self(9i32);
    pub const BlackCircleWingding: Self = Self(10i32);
    pub const WhiteCircleWingding: Self = Self(11i32);
    pub const ArabicWide: Self = Self(12i32);
    pub const SimplifiedChinese: Self = Self(13i32);
    pub const TraditionalChinese: Self = Self(14i32);
    pub const JapanSimplifiedChinese: Self = Self(15i32);
    pub const JapanKorea: Self = Self(16i32);
    pub const ArabicDictionary: Self = Self(17i32);
    pub const ArabicAbjad: Self = Self(18i32);
    pub const Hebrew: Self = Self(19i32);
    pub const ThaiAlphabetic: Self = Self(20i32);
    pub const ThaiNumeric: Self = Self(21i32);
    pub const DevanagariVowel: Self = Self(22i32);
    pub const DevanagariConsonant: Self = Self(23i32);
    pub const DevanagariNumeric: Self = Self(24i32);
}
impl ::core::marker::Copy for MarkerType {}
impl ::core::clone::Clone for MarkerType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct ParagraphAlignment(pub i32);
impl ParagraphAlignment {
    pub const Undefined: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
    pub const Center: Self = Self(2i32);
    pub const Right: Self = Self(3i32);
    pub const Justify: Self = Self(4i32);
}
impl ::core::marker::Copy for ParagraphAlignment {}
impl ::core::clone::Clone for ParagraphAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct ParagraphStyle(pub i32);
impl ParagraphStyle {
    pub const Undefined: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Normal: Self = Self(2i32);
    pub const Heading1: Self = Self(3i32);
    pub const Heading2: Self = Self(4i32);
    pub const Heading3: Self = Self(5i32);
    pub const Heading4: Self = Self(6i32);
    pub const Heading5: Self = Self(7i32);
    pub const Heading6: Self = Self(8i32);
    pub const Heading7: Self = Self(9i32);
    pub const Heading8: Self = Self(10i32);
    pub const Heading9: Self = Self(11i32);
}
impl ::core::marker::Copy for ParagraphStyle {}
impl ::core::clone::Clone for ParagraphStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct PointOptions(pub u32);
impl PointOptions {
    pub const None: Self = Self(0u32);
    pub const IncludeInset: Self = Self(1u32);
    pub const Start: Self = Self(32u32);
    pub const ClientCoordinates: Self = Self(256u32);
    pub const AllowOffClient: Self = Self(512u32);
    pub const Transform: Self = Self(1024u32);
    pub const NoHorizontalScroll: Self = Self(65536u32);
    pub const NoVerticalScroll: Self = Self(262144u32);
}
impl ::core::marker::Copy for PointOptions {}
impl ::core::clone::Clone for PointOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct RangeGravity(pub i32);
impl RangeGravity {
    pub const UIBehavior: Self = Self(0i32);
    pub const Backward: Self = Self(1i32);
    pub const Forward: Self = Self(2i32);
    pub const Inward: Self = Self(3i32);
    pub const Outward: Self = Self(4i32);
}
impl ::core::marker::Copy for RangeGravity {}
impl ::core::clone::Clone for RangeGravity {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct RichEditMathMode(pub i32);
impl RichEditMathMode {
    pub const NoMath: Self = Self(0i32);
    pub const MathOnly: Self = Self(1i32);
}
impl ::core::marker::Copy for RichEditMathMode {}
impl ::core::clone::Clone for RichEditMathMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RichEditTextDocument = *mut ::core::ffi::c_void;
pub type RichEditTextRange = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct SelectionOptions(pub u32);
impl SelectionOptions {
    pub const StartActive: Self = Self(1u32);
    pub const AtEndOfLine: Self = Self(2u32);
    pub const Overtype: Self = Self(4u32);
    pub const Active: Self = Self(8u32);
    pub const Replace: Self = Self(16u32);
}
impl ::core::marker::Copy for SelectionOptions {}
impl ::core::clone::Clone for SelectionOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct SelectionType(pub i32);
impl SelectionType {
    pub const None: Self = Self(0i32);
    pub const InsertionPoint: Self = Self(1i32);
    pub const Normal: Self = Self(2i32);
    pub const InlineShape: Self = Self(7i32);
    pub const Shape: Self = Self(8i32);
}
impl ::core::marker::Copy for SelectionType {}
impl ::core::clone::Clone for SelectionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct TabAlignment(pub i32);
impl TabAlignment {
    pub const Left: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
    pub const Decimal: Self = Self(3i32);
    pub const Bar: Self = Self(4i32);
}
impl ::core::marker::Copy for TabAlignment {}
impl ::core::clone::Clone for TabAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct TabLeader(pub i32);
impl TabLeader {
    pub const Spaces: Self = Self(0i32);
    pub const Dots: Self = Self(1i32);
    pub const Dashes: Self = Self(2i32);
    pub const Lines: Self = Self(3i32);
    pub const ThickLines: Self = Self(4i32);
    pub const Equals: Self = Self(5i32);
}
impl ::core::marker::Copy for TabLeader {}
impl ::core::clone::Clone for TabLeader {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct TextDecorations(pub u32);
impl TextDecorations {
    pub const None: Self = Self(0u32);
    pub const Underline: Self = Self(1u32);
    pub const Strikethrough: Self = Self(2u32);
}
impl ::core::marker::Copy for TextDecorations {}
impl ::core::clone::Clone for TextDecorations {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct TextGetOptions(pub u32);
impl TextGetOptions {
    pub const None: Self = Self(0u32);
    pub const AdjustCrlf: Self = Self(1u32);
    pub const UseCrlf: Self = Self(2u32);
    pub const UseObjectText: Self = Self(4u32);
    pub const AllowFinalEop: Self = Self(8u32);
    pub const NoHidden: Self = Self(32u32);
    pub const IncludeNumbering: Self = Self(64u32);
    pub const FormatRtf: Self = Self(8192u32);
    pub const UseLf: Self = Self(16777216u32);
}
impl ::core::marker::Copy for TextGetOptions {}
impl ::core::clone::Clone for TextGetOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct TextRangeUnit(pub i32);
impl TextRangeUnit {
    pub const Character: Self = Self(0i32);
    pub const Word: Self = Self(1i32);
    pub const Sentence: Self = Self(2i32);
    pub const Paragraph: Self = Self(3i32);
    pub const Line: Self = Self(4i32);
    pub const Story: Self = Self(5i32);
    pub const Screen: Self = Self(6i32);
    pub const Section: Self = Self(7i32);
    pub const Window: Self = Self(8i32);
    pub const CharacterFormat: Self = Self(9i32);
    pub const ParagraphFormat: Self = Self(10i32);
    pub const Object: Self = Self(11i32);
    pub const HardParagraph: Self = Self(12i32);
    pub const Cluster: Self = Self(13i32);
    pub const Bold: Self = Self(14i32);
    pub const Italic: Self = Self(15i32);
    pub const Underline: Self = Self(16i32);
    pub const Strikethrough: Self = Self(17i32);
    pub const ProtectedText: Self = Self(18i32);
    pub const Link: Self = Self(19i32);
    pub const SmallCaps: Self = Self(20i32);
    pub const AllCaps: Self = Self(21i32);
    pub const Hidden: Self = Self(22i32);
    pub const Outline: Self = Self(23i32);
    pub const Shadow: Self = Self(24i32);
    pub const Imprint: Self = Self(25i32);
    pub const Disabled: Self = Self(26i32);
    pub const Revised: Self = Self(27i32);
    pub const Subscript: Self = Self(28i32);
    pub const Superscript: Self = Self(29i32);
    pub const FontBound: Self = Self(30i32);
    pub const LinkProtected: Self = Self(31i32);
    pub const ContentLink: Self = Self(32i32);
}
impl ::core::marker::Copy for TextRangeUnit {}
impl ::core::clone::Clone for TextRangeUnit {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct TextScript(pub i32);
impl TextScript {
    pub const Undefined: Self = Self(0i32);
    pub const Ansi: Self = Self(1i32);
    pub const EastEurope: Self = Self(2i32);
    pub const Cyrillic: Self = Self(3i32);
    pub const Greek: Self = Self(4i32);
    pub const Turkish: Self = Self(5i32);
    pub const Hebrew: Self = Self(6i32);
    pub const Arabic: Self = Self(7i32);
    pub const Baltic: Self = Self(8i32);
    pub const Vietnamese: Self = Self(9i32);
    pub const Default: Self = Self(10i32);
    pub const Symbol: Self = Self(11i32);
    pub const Thai: Self = Self(12i32);
    pub const ShiftJis: Self = Self(13i32);
    pub const GB2312: Self = Self(14i32);
    pub const Hangul: Self = Self(15i32);
    pub const Big5: Self = Self(16i32);
    pub const PC437: Self = Self(17i32);
    pub const Oem: Self = Self(18i32);
    pub const Mac: Self = Self(19i32);
    pub const Armenian: Self = Self(20i32);
    pub const Syriac: Self = Self(21i32);
    pub const Thaana: Self = Self(22i32);
    pub const Devanagari: Self = Self(23i32);
    pub const Bengali: Self = Self(24i32);
    pub const Gurmukhi: Self = Self(25i32);
    pub const Gujarati: Self = Self(26i32);
    pub const Oriya: Self = Self(27i32);
    pub const Tamil: Self = Self(28i32);
    pub const Telugu: Self = Self(29i32);
    pub const Kannada: Self = Self(30i32);
    pub const Malayalam: Self = Self(31i32);
    pub const Sinhala: Self = Self(32i32);
    pub const Lao: Self = Self(33i32);
    pub const Tibetan: Self = Self(34i32);
    pub const Myanmar: Self = Self(35i32);
    pub const Georgian: Self = Self(36i32);
    pub const Jamo: Self = Self(37i32);
    pub const Ethiopic: Self = Self(38i32);
    pub const Cherokee: Self = Self(39i32);
    pub const Aboriginal: Self = Self(40i32);
    pub const Ogham: Self = Self(41i32);
    pub const Runic: Self = Self(42i32);
    pub const Khmer: Self = Self(43i32);
    pub const Mongolian: Self = Self(44i32);
    pub const Braille: Self = Self(45i32);
    pub const Yi: Self = Self(46i32);
    pub const Limbu: Self = Self(47i32);
    pub const TaiLe: Self = Self(48i32);
    pub const NewTaiLue: Self = Self(49i32);
    pub const SylotiNagri: Self = Self(50i32);
    pub const Kharoshthi: Self = Self(51i32);
    pub const Kayahli: Self = Self(52i32);
    pub const UnicodeSymbol: Self = Self(53i32);
    pub const Emoji: Self = Self(54i32);
    pub const Glagolitic: Self = Self(55i32);
    pub const Lisu: Self = Self(56i32);
    pub const Vai: Self = Self(57i32);
    pub const NKo: Self = Self(58i32);
    pub const Osmanya: Self = Self(59i32);
    pub const PhagsPa: Self = Self(60i32);
    pub const Gothic: Self = Self(61i32);
    pub const Deseret: Self = Self(62i32);
    pub const Tifinagh: Self = Self(63i32);
}
impl ::core::marker::Copy for TextScript {}
impl ::core::clone::Clone for TextScript {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct TextSetOptions(pub u32);
impl TextSetOptions {
    pub const None: Self = Self(0u32);
    pub const UnicodeBidi: Self = Self(1u32);
    pub const Unlink: Self = Self(8u32);
    pub const Unhide: Self = Self(16u32);
    pub const CheckTextLimit: Self = Self(32u32);
    pub const FormatRtf: Self = Self(8192u32);
    pub const ApplyRtfDocumentDefaults: Self = Self(16384u32);
}
impl ::core::marker::Copy for TextSetOptions {}
impl ::core::clone::Clone for TextSetOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct UnderlineType(pub i32);
impl UnderlineType {
    pub const Undefined: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Single: Self = Self(2i32);
    pub const Words: Self = Self(3i32);
    pub const Double: Self = Self(4i32);
    pub const Dotted: Self = Self(5i32);
    pub const Dash: Self = Self(6i32);
    pub const DashDot: Self = Self(7i32);
    pub const DashDotDot: Self = Self(8i32);
    pub const Wave: Self = Self(9i32);
    pub const Thick: Self = Self(10i32);
    pub const Thin: Self = Self(11i32);
    pub const DoubleWave: Self = Self(12i32);
    pub const HeavyWave: Self = Self(13i32);
    pub const LongDash: Self = Self(14i32);
    pub const ThickDash: Self = Self(15i32);
    pub const ThickDashDot: Self = Self(16i32);
    pub const ThickDashDotDot: Self = Self(17i32);
    pub const ThickDotted: Self = Self(18i32);
    pub const ThickLongDash: Self = Self(19i32);
}
impl ::core::marker::Copy for UnderlineType {}
impl ::core::clone::Clone for UnderlineType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Text\"`*"]
#[repr(transparent)]
pub struct VerticalCharacterAlignment(pub i32);
impl VerticalCharacterAlignment {
    pub const Top: Self = Self(0i32);
    pub const Baseline: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
}
impl ::core::marker::Copy for VerticalCharacterAlignment {}
impl ::core::clone::Clone for VerticalCharacterAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
