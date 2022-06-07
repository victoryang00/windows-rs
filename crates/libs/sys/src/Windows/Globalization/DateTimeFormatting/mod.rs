pub type DateTimeFormatter = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
#[repr(transparent)]
pub struct DayFormat(pub i32);
impl DayFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
}
impl ::core::marker::Copy for DayFormat {}
impl ::core::clone::Clone for DayFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
#[repr(transparent)]
pub struct DayOfWeekFormat(pub i32);
impl DayOfWeekFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
    pub const Abbreviated: Self = Self(2i32);
    pub const Full: Self = Self(3i32);
}
impl ::core::marker::Copy for DayOfWeekFormat {}
impl ::core::clone::Clone for DayOfWeekFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
#[repr(transparent)]
pub struct HourFormat(pub i32);
impl HourFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
}
impl ::core::marker::Copy for HourFormat {}
impl ::core::clone::Clone for HourFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IDateTimeFormatter {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Languages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Languages: usize,
    pub GeographicRegion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Calendar: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Clock: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NumeralSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetNumeralSystem: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Patterns: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Patterns: usize,
    pub Template: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Format: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Format: usize,
    pub IncludeYear: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut YearFormat) -> ::windows_sys::core::HRESULT,
    pub IncludeMonth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MonthFormat) -> ::windows_sys::core::HRESULT,
    pub IncludeDayOfWeek: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DayOfWeekFormat) -> ::windows_sys::core::HRESULT,
    pub IncludeDay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DayFormat) -> ::windows_sys::core::HRESULT,
    pub IncludeHour: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HourFormat) -> ::windows_sys::core::HRESULT,
    pub IncludeMinute: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MinuteFormat) -> ::windows_sys::core::HRESULT,
    pub IncludeSecond: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SecondFormat) -> ::windows_sys::core::HRESULT,
    pub ResolvedLanguage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ResolvedGeographicRegion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDateTimeFormatter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2515454480, data2: 29664, data3: 20043, data4: [161, 131, 61, 106, 208, 186, 53, 236] };
}
#[repr(C)]
pub struct IDateTimeFormatter2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FormatUsingTimeZone: unsafe extern "system" fn(this: *mut *mut Self, datetime: super::super::Foundation::DateTime, timezoneid: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FormatUsingTimeZone: usize,
}
impl ::windows_sys::core::Interface for IDateTimeFormatter2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 667490950, data2: 48554, data3: 20432, data4: [158, 54, 103, 29, 90, 165, 238, 3] };
}
#[repr(C)]
pub struct IDateTimeFormatterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateDateTimeFormatter: unsafe extern "system" fn(this: *mut *mut Self, formattemplate: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateDateTimeFormatterLanguages: unsafe extern "system" fn(this: *mut *mut Self, formattemplate: ::windows_sys::core::HSTRING, languages: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateDateTimeFormatterLanguages: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateDateTimeFormatterContext: unsafe extern "system" fn(this: *mut *mut Self, formattemplate: ::windows_sys::core::HSTRING, languages: *mut ::core::ffi::c_void, geographicregion: ::windows_sys::core::HSTRING, calendar: ::windows_sys::core::HSTRING, clock: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateDateTimeFormatterContext: usize,
    pub CreateDateTimeFormatterDate: unsafe extern "system" fn(this: *mut *mut Self, yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateDateTimeFormatterTime: unsafe extern "system" fn(this: *mut *mut Self, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateDateTimeFormatterDateTimeLanguages: unsafe extern "system" fn(this: *mut *mut Self, yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, languages: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateDateTimeFormatterDateTimeLanguages: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateDateTimeFormatterDateTimeContext: unsafe extern "system" fn(this: *mut *mut Self, yearformat: YearFormat, monthformat: MonthFormat, dayformat: DayFormat, dayofweekformat: DayOfWeekFormat, hourformat: HourFormat, minuteformat: MinuteFormat, secondformat: SecondFormat, languages: *mut ::core::ffi::c_void, geographicregion: ::windows_sys::core::HSTRING, calendar: ::windows_sys::core::HSTRING, clock: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateDateTimeFormatterDateTimeContext: usize,
}
impl ::windows_sys::core::Interface for IDateTimeFormatterFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3968698963, data2: 6702, data3: 16685, data4: [136, 21, 59, 116, 95, 177, 162, 160] };
}
#[repr(C)]
pub struct IDateTimeFormatterStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub LongDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LongTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ShortDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ShortTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDateTimeFormatterStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3217942464, data2: 57164, data3: 18990, data4: [144, 18, 244, 125, 175, 63, 18, 18] };
}
#[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
#[repr(transparent)]
pub struct MinuteFormat(pub i32);
impl MinuteFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
}
impl ::core::marker::Copy for MinuteFormat {}
impl ::core::clone::Clone for MinuteFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
#[repr(transparent)]
pub struct MonthFormat(pub i32);
impl MonthFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
    pub const Abbreviated: Self = Self(2i32);
    pub const Full: Self = Self(3i32);
    pub const Numeric: Self = Self(4i32);
}
impl ::core::marker::Copy for MonthFormat {}
impl ::core::clone::Clone for MonthFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
#[repr(transparent)]
pub struct SecondFormat(pub i32);
impl SecondFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
}
impl ::core::marker::Copy for SecondFormat {}
impl ::core::clone::Clone for SecondFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Globalization_DateTimeFormatting\"`*"]
#[repr(transparent)]
pub struct YearFormat(pub i32);
impl YearFormat {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
    pub const Abbreviated: Self = Self(2i32);
    pub const Full: Self = Self(3i32);
}
impl ::core::marker::Copy for YearFormat {}
impl ::core::clone::Clone for YearFormat {
    fn clone(&self) -> Self {
        *self
    }
}
