#[doc = "*Required features: `\"Data_Text\"`*"]
#[repr(transparent)]
pub struct AlternateNormalizationFormat(pub i32);
impl AlternateNormalizationFormat {
    pub const NotNormalized: Self = Self(0i32);
    pub const Number: Self = Self(1i32);
    pub const Currency: Self = Self(3i32);
    pub const Date: Self = Self(4i32);
    pub const Time: Self = Self(5i32);
}
impl ::core::marker::Copy for AlternateNormalizationFormat {}
impl ::core::clone::Clone for AlternateNormalizationFormat {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AlternateWordForm = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAlternateWordForm {
    pub base__: ::windows_sys::core::IInspectable,
    pub SourceTextSegment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TextSegment) -> ::windows_sys::core::HRESULT,
    pub AlternateText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NormalizationFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AlternateNormalizationFormat) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISelectableWordSegment {
    pub base__: ::windows_sys::core::IInspectable,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SourceTextSegment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TextSegment) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISelectableWordsSegmenter {
    pub base__: ::windows_sys::core::IInspectable,
    pub ResolvedLanguage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetTokenAt: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, startindex: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTokens: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTokens: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Tokenize: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, startindex: u32, handler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Tokenize: usize,
}
#[repr(C)]
pub struct ISelectableWordsSegmenterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithLanguage: unsafe extern "system" fn(this: *mut *mut Self, language: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISemanticTextQuery {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Find: unsafe extern "system" fn(this: *mut *mut Self, content: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Find: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindInProperty: unsafe extern "system" fn(this: *mut *mut Self, propertycontent: ::windows_sys::core::HSTRING, propertyname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindInProperty: usize,
}
#[repr(C)]
pub struct ISemanticTextQueryFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, aqsfilter: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithLanguage: unsafe extern "system" fn(this: *mut *mut Self, aqsfilter: ::windows_sys::core::HSTRING, filterlanguage: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextConversionGenerator {
    pub base__: ::windows_sys::core::IInspectable,
    pub ResolvedLanguage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LanguageAvailableButNotInstalled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCandidatesAsync: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCandidatesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCandidatesWithMaxCountAsync: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, maxcandidates: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCandidatesWithMaxCountAsync: usize,
}
#[repr(C)]
pub struct ITextConversionGeneratorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, languagetag: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextPhoneme {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ReadingText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextPredictionGenerator {
    pub base__: ::windows_sys::core::IInspectable,
    pub ResolvedLanguage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LanguageAvailableButNotInstalled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCandidatesAsync: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCandidatesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCandidatesWithMaxCountAsync: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, maxcandidates: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCandidatesWithMaxCountAsync: usize,
}
#[repr(C)]
pub struct ITextPredictionGenerator2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCandidatesWithParametersAsync: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, maxcandidates: u32, predictionoptions: TextPredictionOptions, previousstrings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCandidatesWithParametersAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNextWordCandidatesAsync: unsafe extern "system" fn(this: *mut *mut Self, maxcandidates: u32, previousstrings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNextWordCandidatesAsync: usize,
    #[cfg(feature = "UI_Text_Core")]
    pub InputScope: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Text::Core::CoreTextInputScope) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text_Core"))]
    InputScope: usize,
    #[cfg(feature = "UI_Text_Core")]
    pub SetInputScope: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::UI::Text::Core::CoreTextInputScope) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Text_Core"))]
    SetInputScope: usize,
}
#[repr(C)]
pub struct ITextPredictionGeneratorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, languagetag: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITextReverseConversionGenerator {
    pub base__: ::windows_sys::core::IInspectable,
    pub ResolvedLanguage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LanguageAvailableButNotInstalled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ConvertBackAsync: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConvertBackAsync: usize,
}
#[repr(C)]
pub struct ITextReverseConversionGenerator2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPhonemesAsync: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPhonemesAsync: usize,
}
#[repr(C)]
pub struct ITextReverseConversionGeneratorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, languagetag: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IUnicodeCharactersStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetCodepointFromSurrogatePair: unsafe extern "system" fn(this: *mut *mut Self, highsurrogate: u32, lowsurrogate: u32, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetSurrogatePairFromCodepoint: unsafe extern "system" fn(this: *mut *mut Self, codepoint: u32, highsurrogate: *mut u16, lowsurrogate: *mut u16) -> ::windows_sys::core::HRESULT,
    pub IsHighSurrogate: unsafe extern "system" fn(this: *mut *mut Self, codepoint: u32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsLowSurrogate: unsafe extern "system" fn(this: *mut *mut Self, codepoint: u32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsSupplementary: unsafe extern "system" fn(this: *mut *mut Self, codepoint: u32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsNoncharacter: unsafe extern "system" fn(this: *mut *mut Self, codepoint: u32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsWhitespace: unsafe extern "system" fn(this: *mut *mut Self, codepoint: u32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsAlphabetic: unsafe extern "system" fn(this: *mut *mut Self, codepoint: u32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsCased: unsafe extern "system" fn(this: *mut *mut Self, codepoint: u32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsUppercase: unsafe extern "system" fn(this: *mut *mut Self, codepoint: u32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsLowercase: unsafe extern "system" fn(this: *mut *mut Self, codepoint: u32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsIdStart: unsafe extern "system" fn(this: *mut *mut Self, codepoint: u32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsIdContinue: unsafe extern "system" fn(this: *mut *mut Self, codepoint: u32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsGraphemeBase: unsafe extern "system" fn(this: *mut *mut Self, codepoint: u32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsGraphemeExtend: unsafe extern "system" fn(this: *mut *mut Self, codepoint: u32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetNumericType: unsafe extern "system" fn(this: *mut *mut Self, codepoint: u32, result__: *mut UnicodeNumericType) -> ::windows_sys::core::HRESULT,
    pub GetGeneralCategory: unsafe extern "system" fn(this: *mut *mut Self, codepoint: u32, result__: *mut UnicodeGeneralCategory) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IWordSegment {
    pub base__: ::windows_sys::core::IInspectable,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SourceTextSegment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TextSegment) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AlternateForms: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AlternateForms: usize,
}
#[repr(C)]
pub struct IWordsSegmenter {
    pub base__: ::windows_sys::core::IInspectable,
    pub ResolvedLanguage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetTokenAt: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, startindex: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTokens: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTokens: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Tokenize: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, startindex: u32, handler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Tokenize: usize,
}
#[repr(C)]
pub struct IWordsSegmenterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithLanguage: unsafe extern "system" fn(this: *mut *mut Self, language: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type SelectableWordSegment = *mut ::core::ffi::c_void;
pub type SelectableWordSegmentsTokenizingHandler = *mut ::core::ffi::c_void;
pub type SelectableWordsSegmenter = *mut ::core::ffi::c_void;
pub type SemanticTextQuery = *mut ::core::ffi::c_void;
pub type TextConversionGenerator = *mut ::core::ffi::c_void;
pub type TextPhoneme = *mut ::core::ffi::c_void;
pub type TextPredictionGenerator = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Data_Text\"`*"]
#[repr(transparent)]
pub struct TextPredictionOptions(pub u32);
impl TextPredictionOptions {
    pub const None: Self = Self(0u32);
    pub const Predictions: Self = Self(1u32);
    pub const Corrections: Self = Self(2u32);
}
impl ::core::marker::Copy for TextPredictionOptions {}
impl ::core::clone::Clone for TextPredictionOptions {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TextReverseConversionGenerator = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Data_Text\"`*"]
pub struct TextSegment {
    pub StartPosition: u32,
    pub Length: u32,
}
impl ::core::marker::Copy for TextSegment {}
impl ::core::clone::Clone for TextSegment {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Data_Text\"`*"]
#[repr(transparent)]
pub struct UnicodeGeneralCategory(pub i32);
impl UnicodeGeneralCategory {
    pub const UppercaseLetter: Self = Self(0i32);
    pub const LowercaseLetter: Self = Self(1i32);
    pub const TitlecaseLetter: Self = Self(2i32);
    pub const ModifierLetter: Self = Self(3i32);
    pub const OtherLetter: Self = Self(4i32);
    pub const NonspacingMark: Self = Self(5i32);
    pub const SpacingCombiningMark: Self = Self(6i32);
    pub const EnclosingMark: Self = Self(7i32);
    pub const DecimalDigitNumber: Self = Self(8i32);
    pub const LetterNumber: Self = Self(9i32);
    pub const OtherNumber: Self = Self(10i32);
    pub const SpaceSeparator: Self = Self(11i32);
    pub const LineSeparator: Self = Self(12i32);
    pub const ParagraphSeparator: Self = Self(13i32);
    pub const Control: Self = Self(14i32);
    pub const Format: Self = Self(15i32);
    pub const Surrogate: Self = Self(16i32);
    pub const PrivateUse: Self = Self(17i32);
    pub const ConnectorPunctuation: Self = Self(18i32);
    pub const DashPunctuation: Self = Self(19i32);
    pub const OpenPunctuation: Self = Self(20i32);
    pub const ClosePunctuation: Self = Self(21i32);
    pub const InitialQuotePunctuation: Self = Self(22i32);
    pub const FinalQuotePunctuation: Self = Self(23i32);
    pub const OtherPunctuation: Self = Self(24i32);
    pub const MathSymbol: Self = Self(25i32);
    pub const CurrencySymbol: Self = Self(26i32);
    pub const ModifierSymbol: Self = Self(27i32);
    pub const OtherSymbol: Self = Self(28i32);
    pub const NotAssigned: Self = Self(29i32);
}
impl ::core::marker::Copy for UnicodeGeneralCategory {}
impl ::core::clone::Clone for UnicodeGeneralCategory {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Data_Text\"`*"]
#[repr(transparent)]
pub struct UnicodeNumericType(pub i32);
impl UnicodeNumericType {
    pub const None: Self = Self(0i32);
    pub const Decimal: Self = Self(1i32);
    pub const Digit: Self = Self(2i32);
    pub const Numeric: Self = Self(3i32);
}
impl ::core::marker::Copy for UnicodeNumericType {}
impl ::core::clone::Clone for UnicodeNumericType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WordSegment = *mut ::core::ffi::c_void;
pub type WordSegmentsTokenizingHandler = *mut ::core::ffi::c_void;
pub type WordsSegmenter = *mut ::core::ffi::c_void;
