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
impl ::windows_sys::core::Interface for ICurrencyFormatter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 292752549, data2: 19200, data3: 16818, data4: [179, 50, 115, 177, 42, 73, 125, 84] };
}
#[repr(C)]
pub struct ICurrencyFormatter2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CurrencyFormatterMode) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, value: CurrencyFormatterMode) -> ::windows_sys::core::HRESULT,
    pub ApplyRoundingForCurrency: unsafe extern "system" fn(this: *mut *mut Self, roundingalgorithm: RoundingAlgorithm) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICurrencyFormatter2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 120336157, data2: 59322, data3: 16791, data4: [146, 14, 36, 124, 146, 247, 222, 166] };
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
impl ::windows_sys::core::Interface for ICurrencyFormatterFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2261209982, data2: 47416, data3: 19106, data4: [132, 176, 44, 51, 220, 91, 20, 80] };
}
#[repr(C)]
pub struct IDecimalFormatterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateDecimalFormatter: unsafe extern "system" fn(this: *mut *mut Self, languages: *mut ::core::ffi::c_void, geographicregion: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateDecimalFormatter: usize,
}
impl ::windows_sys::core::Interface for IDecimalFormatterFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 218205338, data2: 58259, data3: 18104, data4: [184, 48, 122, 105, 200, 248, 159, 187] };
}
#[repr(C)]
pub struct IIncrementNumberRounder {
    pub base__: ::windows_sys::core::IInspectable,
    pub RoundingAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RoundingAlgorithm) -> ::windows_sys::core::HRESULT,
    pub SetRoundingAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, value: RoundingAlgorithm) -> ::windows_sys::core::HRESULT,
    pub Increment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetIncrement: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIncrementNumberRounder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1889947640, data2: 26283, data3: 16725, data4: [157, 161, 115, 158, 70, 118, 69, 67] };
}
#[repr(C)]
pub struct INumberFormatter {
    pub base__: ::windows_sys::core::IInspectable,
    pub FormatInt: unsafe extern "system" fn(this: *mut *mut Self, value: i64, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FormatUInt: unsafe extern "system" fn(this: *mut *mut Self, value: u64, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FormatDouble: unsafe extern "system" fn(this: *mut *mut Self, value: f64, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for INumberFormatter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2768272457, data2: 30326, data3: 19895, data4: [134, 49, 27, 111, 242, 101, 202, 169] };
}
#[repr(C)]
pub struct INumberFormatter2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub FormatInt: unsafe extern "system" fn(this: *mut *mut Self, value: i64, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FormatUInt: unsafe extern "system" fn(this: *mut *mut Self, value: u64, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FormatDouble: unsafe extern "system" fn(this: *mut *mut Self, value: f64, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for INumberFormatter2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3567829488, data2: 32976, data3: 19213, data4: [168, 158, 136, 44, 30, 143, 131, 16] };
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
impl ::windows_sys::core::Interface for INumberFormatterOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2150837537, data2: 44769, data3: 19001, data4: [186, 162, 7, 237, 140, 150, 218, 246] };
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
impl ::windows_sys::core::Interface for INumberParser {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3865416722, data2: 18963, data3: 19027, data4: [131, 161, 57, 47, 190, 76, 255, 159] };
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
impl ::windows_sys::core::Interface for INumberRounder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1416872821, data2: 14573, data3: 17969, data4: [184, 12, 239, 52, 252, 72, 183, 245] };
}
#[repr(C)]
pub struct INumberRounderOption {
    pub base__: ::windows_sys::core::IInspectable,
    pub NumberRounder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetNumberRounder: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for INumberRounderOption {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 990413875, data2: 25711, data3: 20222, data4: [141, 72, 102, 235, 46, 73, 231, 54] };
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
impl ::windows_sys::core::Interface for INumeralSystemTranslator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 687193132, data2: 35875, data3: 16948, data4: [173, 46, 250, 90, 58, 66, 110, 155] };
}
#[repr(C)]
pub struct INumeralSystemTranslatorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, languages: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for INumeralSystemTranslatorFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2519779546, data2: 14063, data3: 19848, data4: [168, 92, 111, 13, 152, 214, 32, 166] };
}
#[repr(C)]
pub struct IPercentFormatterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreatePercentFormatter: unsafe extern "system" fn(this: *mut *mut Self, languages: *mut ::core::ffi::c_void, geographicregion: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreatePercentFormatter: usize,
}
impl ::windows_sys::core::Interface for IPercentFormatterFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3078785775, data2: 65236, data3: 16408, data4: [166, 226, 224, 153, 97, 224, 55, 101] };
}
#[repr(C)]
pub struct IPermilleFormatterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreatePermilleFormatter: unsafe extern "system" fn(this: *mut *mut Self, languages: *mut ::core::ffi::c_void, geographicregion: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreatePermilleFormatter: usize,
}
impl ::windows_sys::core::Interface for IPermilleFormatterFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 725071020, data2: 58936, data3: 20181, data4: [169, 152, 98, 246, 176, 106, 73, 174] };
}
#[repr(C)]
pub struct ISignedZeroOption {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsZeroSigned: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsZeroSigned: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISignedZeroOption {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4246527281, data2: 2620, data3: 18884, data4: [166, 66, 150, 161, 86, 79, 79, 48] };
}
#[repr(C)]
pub struct ISignificantDigitsNumberRounder {
    pub base__: ::windows_sys::core::IInspectable,
    pub RoundingAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RoundingAlgorithm) -> ::windows_sys::core::HRESULT,
    pub SetRoundingAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, value: RoundingAlgorithm) -> ::windows_sys::core::HRESULT,
    pub SignificantDigits: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetSignificantDigits: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISignificantDigitsNumberRounder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4120124362, data2: 26182, data3: 18707, data4: [140, 118, 27, 25, 31, 249, 77, 253] };
}
#[repr(C)]
pub struct ISignificantDigitsOption {
    pub base__: ::windows_sys::core::IInspectable,
    pub SignificantDigits: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSignificantDigits: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISignificantDigitsOption {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 491650269, data2: 11587, data3: 20200, data4: [187, 241, 193, 178, 106, 113, 26, 88] };
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
