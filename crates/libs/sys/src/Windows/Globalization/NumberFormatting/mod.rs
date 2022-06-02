pub type CurrencyFormatter = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Globalization_NumberFormatting\"`*"]
#[repr(transparent)]
pub struct CurrencyFormatterMode(pub i32);
impl CurrencyFormatterMode {
    pub const UseSymbol: Self = Self(0i32);
    pub const UseCurrencyCode: Self = Self(1i32);
}
impl ::core::marker::Copy for CurrencyFormatterMode {}
impl ::core::clone::Clone for CurrencyFormatterMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DecimalFormatter = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct ICurrencyFormatter {
    pub base__: ::windows_sys::core::IInspectable,
    pub Currency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub SetCurrency: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetCurrency: usize,
}
#[repr(C)]
pub struct ICurrencyFormatter2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CurrencyFormatterMode) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, value: CurrencyFormatterMode) -> ::windows_sys::core::HRESULT,
    pub ApplyRoundingForCurrency: unsafe extern "system" fn(this: *mut *mut Self, roundingalgorithm: RoundingAlgorithm) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICurrencyFormatterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateCurrencyFormatterCode: unsafe extern "system" fn(this: *mut *mut Self, currencycode: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCurrencyFormatterCodeContext: unsafe extern "system" fn(this: *mut *mut Self, currencycode: ::windows_sys::core::HSTRING, languages: *mut ::core::ffi::c_void, geographicregion: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCurrencyFormatterCodeContext: usize,
}
#[repr(C)]
pub struct IDecimalFormatterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateDecimalFormatter: unsafe extern "system" fn(this: *mut *mut Self, languages: *mut ::core::ffi::c_void, geographicregion: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateDecimalFormatter: usize,
}
#[repr(C)]
pub struct IIncrementNumberRounder {
    pub base__: ::windows_sys::core::IInspectable,
    pub RoundingAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RoundingAlgorithm) -> ::windows_sys::core::HRESULT,
    pub SetRoundingAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, value: RoundingAlgorithm) -> ::windows_sys::core::HRESULT,
    pub Increment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetIncrement: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INumberFormatter {
    pub base__: ::windows_sys::core::IInspectable,
    pub FormatInt: unsafe extern "system" fn(this: *mut *mut Self, value: i64, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FormatUInt: unsafe extern "system" fn(this: *mut *mut Self, value: u64, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FormatDouble: unsafe extern "system" fn(this: *mut *mut Self, value: f64, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INumberFormatter2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub FormatInt: unsafe extern "system" fn(this: *mut *mut Self, value: i64, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FormatUInt: unsafe extern "system" fn(this: *mut *mut Self, value: u64, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FormatDouble: unsafe extern "system" fn(this: *mut *mut Self, value: f64, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INumberFormatterOptions {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Languages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Languages: usize,
    pub GeographicRegion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IntegerDigits: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetIntegerDigits: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub FractionDigits: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetFractionDigits: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub IsGrouped: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsGrouped: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsDecimalPointAlwaysDisplayed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsDecimalPointAlwaysDisplayed: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub NumeralSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetNumeralSystem: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ResolvedLanguage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ResolvedGeographicRegion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INumberParser {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ParseInt: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ParseInt: usize,
    #[cfg(feature = "Foundation")]
    pub ParseUInt: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ParseUInt: usize,
    #[cfg(feature = "Foundation")]
    pub ParseDouble: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ParseDouble: usize,
}
#[repr(C)]
pub struct INumberRounder {
    pub base__: ::windows_sys::core::IInspectable,
    pub RoundInt32: unsafe extern "system" fn(this: *mut *mut Self, value: i32, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub RoundUInt32: unsafe extern "system" fn(this: *mut *mut Self, value: u32, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub RoundInt64: unsafe extern "system" fn(this: *mut *mut Self, value: i64, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    pub RoundUInt64: unsafe extern "system" fn(this: *mut *mut Self, value: u64, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub RoundSingle: unsafe extern "system" fn(this: *mut *mut Self, value: f32, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub RoundDouble: unsafe extern "system" fn(this: *mut *mut Self, value: f64, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INumberRounderOption {
    pub base__: ::windows_sys::core::IInspectable,
    pub NumberRounder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetNumberRounder: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INumeralSystemTranslator {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Languages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Languages: usize,
    pub ResolvedLanguage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NumeralSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetNumeralSystem: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TranslateNumerals: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INumeralSystemTranslatorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, languages: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
}
#[repr(C)]
pub struct IPercentFormatterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreatePercentFormatter: unsafe extern "system" fn(this: *mut *mut Self, languages: *mut ::core::ffi::c_void, geographicregion: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreatePercentFormatter: usize,
}
#[repr(C)]
pub struct IPermilleFormatterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreatePermilleFormatter: unsafe extern "system" fn(this: *mut *mut Self, languages: *mut ::core::ffi::c_void, geographicregion: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreatePermilleFormatter: usize,
}
#[repr(C)]
pub struct ISignedZeroOption {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsZeroSigned: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsZeroSigned: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISignificantDigitsNumberRounder {
    pub base__: ::windows_sys::core::IInspectable,
    pub RoundingAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RoundingAlgorithm) -> ::windows_sys::core::HRESULT,
    pub SetRoundingAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, value: RoundingAlgorithm) -> ::windows_sys::core::HRESULT,
    pub SignificantDigits: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetSignificantDigits: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISignificantDigitsOption {
    pub base__: ::windows_sys::core::IInspectable,
    pub SignificantDigits: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSignificantDigits: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
}
pub type IncrementNumberRounder = *mut ::core::ffi::c_void;
pub type NumeralSystemTranslator = *mut ::core::ffi::c_void;
pub type PercentFormatter = *mut ::core::ffi::c_void;
pub type PermilleFormatter = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Globalization_NumberFormatting\"`*"]
#[repr(transparent)]
pub struct RoundingAlgorithm(pub i32);
impl RoundingAlgorithm {
    pub const None: Self = Self(0i32);
    pub const RoundDown: Self = Self(1i32);
    pub const RoundUp: Self = Self(2i32);
    pub const RoundTowardsZero: Self = Self(3i32);
    pub const RoundAwayFromZero: Self = Self(4i32);
    pub const RoundHalfDown: Self = Self(5i32);
    pub const RoundHalfUp: Self = Self(6i32);
    pub const RoundHalfTowardsZero: Self = Self(7i32);
    pub const RoundHalfAwayFromZero: Self = Self(8i32);
    pub const RoundHalfToEven: Self = Self(9i32);
    pub const RoundHalfToOdd: Self = Self(10i32);
}
impl ::core::marker::Copy for RoundingAlgorithm {}
impl ::core::clone::Clone for RoundingAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SignificantDigitsNumberRounder = *mut ::core::ffi::c_void;
