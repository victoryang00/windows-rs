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
impl ::windows_sys::core::Interface for IAlternateWordForm {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1194945566, data2: 20921, data3: 16903, data4: [145, 70, 36, 142, 99, 106, 29, 29] };
}
#[repr(C)]
pub struct ISelectableWordSegment {
    pub base__: ::windows_sys::core::IInspectable,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SourceTextSegment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TextSegment) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISelectableWordSegment {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2439662775, data2: 35495, data3: 19576, data4: [179, 116, 93, 237, 183, 82, 230, 11] };
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
impl ::windows_sys::core::Interface for ISelectableWordsSegmenter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4141625831, data2: 19219, data3: 17861, data4: [136, 151, 125, 113, 38, 158, 8, 93] };
}
#[repr(C)]
pub struct ISelectableWordsSegmenterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithLanguage: unsafe extern "system" fn(this: *mut *mut Self, language: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISelectableWordsSegmenterFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2356835912, data2: 24663, data3: 17209, data4: [188, 112, 242, 16, 1, 10, 65, 80] };
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
impl ::windows_sys::core::Interface for ISemanticTextQuery {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1780263761, data2: 8114, data3: 18697, data4: [128, 184, 53, 115, 26, 43, 62, 127] };
}
#[repr(C)]
pub struct ISemanticTextQueryFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, aqsfilter: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithLanguage: unsafe extern "system" fn(this: *mut *mut Self, aqsfilter: ::windows_sys::core::HSTRING, filterlanguage: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISemanticTextQueryFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 596378883, data2: 63893, data3: 17799, data4: [135, 119, 162, 183, 216, 10, 207, 239] };
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
impl ::windows_sys::core::Interface for ITextConversionGenerator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 56650334, data2: 10921, data3: 19126, data4: [175, 139, 165, 98, 182, 58, 137, 146] };
}
#[repr(C)]
pub struct ITextConversionGeneratorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, languagetag: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITextConversionGeneratorFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4239013761, data2: 12419, data3: 18859, data4: [190, 21, 86, 223, 187, 183, 77, 111] };
}
#[repr(C)]
pub struct ITextPhoneme {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ReadingText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITextPhoneme {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2472715274, data2: 39802, data3: 17769, data4: [148, 207, 216, 79, 47, 56, 207, 155] };
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
impl ::windows_sys::core::Interface for ITextPredictionGenerator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1588374279, data2: 44017, data3: 19638, data4: [157, 158, 50, 111, 43, 70, 135, 86] };
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
impl ::windows_sys::core::Interface for ITextPredictionGenerator2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3091669944, data2: 11383, data3: 18538, data4: [144, 10, 163, 69, 62, 237, 193, 93] };
}
#[repr(C)]
pub struct ITextPredictionGeneratorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, languagetag: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITextPredictionGeneratorFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1918350358, data2: 35746, data3: 18257, data4: [157, 48, 157, 133, 67, 86, 83, 162] };
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
impl ::windows_sys::core::Interface for ITextReverseConversionGenerator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1374156052, data2: 40017, data3: 19846, data4: [174, 27, 180, 152, 251, 173, 131, 19] };
}
#[repr(C)]
pub struct ITextReverseConversionGenerator2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPhonemesAsync: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPhonemesAsync: usize,
}
impl ::windows_sys::core::Interface for ITextReverseConversionGenerator2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 447730412, data2: 34262, data3: 18173, data4: [130, 138, 58, 72, 48, 250, 110, 24] };
}
#[repr(C)]
pub struct ITextReverseConversionGeneratorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, languagetag: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITextReverseConversionGeneratorFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1673450278, data2: 8154, data3: 16886, data4: [137, 213, 35, 221, 234, 60, 114, 154] };
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
impl ::windows_sys::core::Interface for IUnicodeCharactersStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2542837383, data2: 37521, data3: 20369, data4: [182, 200, 182, 227, 89, 215, 167, 251] };
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
impl ::windows_sys::core::Interface for IWordSegment {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3537156717, data2: 39036, data3: 19648, data4: [182, 189, 212, 154, 17, 179, 143, 154] };
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
impl ::windows_sys::core::Interface for IWordsSegmenter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2259997905, data2: 45822, data3: 20020, data4: [168, 29, 102, 100, 3, 0, 69, 79] };
}
#[repr(C)]
pub struct IWordsSegmenterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithLanguage: unsafe extern "system" fn(this: *mut *mut Self, language: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWordsSegmenterFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3868684916, data2: 64565, data3: 17756, data4: [139, 251, 109, 127, 70, 83, 202, 151] };
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
