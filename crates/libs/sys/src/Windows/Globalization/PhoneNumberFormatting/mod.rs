#[repr(C)]
pub struct IPhoneNumberFormatter {
    pub base__: ::windows_sys::core::IInspectable,
    pub Format: unsafe extern "system" fn(this: *mut *mut Self, number: *mut ::core::ffi::c_void, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FormatWithOutputFormat: unsafe extern "system" fn(this: *mut *mut Self, number: *mut ::core::ffi::c_void, numberformat: PhoneNumberFormat, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FormatPartialString: unsafe extern "system" fn(this: *mut *mut Self, number: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FormatString: unsafe extern "system" fn(this: *mut *mut Self, number: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FormatStringWithLeftToRightMarkers: unsafe extern "system" fn(this: *mut *mut Self, number: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneNumberFormatter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 358003870, data2: 47828, data3: 19274, data4: [144, 13, 68, 7, 173, 183, 201, 129] };
}
#[repr(C)]
pub struct IPhoneNumberFormatterStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryCreate: unsafe extern "system" fn(this: *mut *mut Self, regioncode: ::windows_sys::core::HSTRING, phonenumber: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetCountryCodeForRegion: unsafe extern "system" fn(this: *mut *mut Self, regioncode: ::windows_sys::core::HSTRING, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetNationalDirectDialingPrefixForRegion: unsafe extern "system" fn(this: *mut *mut Self, regioncode: ::windows_sys::core::HSTRING, stripnondigit: bool, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WrapWithLeftToRightMarkers: unsafe extern "system" fn(this: *mut *mut Self, number: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneNumberFormatterStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1554446641, data2: 34009, data3: 16715, data4: [171, 78, 160, 85, 44, 135, 134, 2] };
}
#[repr(C)]
pub struct IPhoneNumberInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub CountryCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetLengthOfGeographicalAreaCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetNationalSignificantNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetLengthOfNationalDestinationCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub PredictNumberKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PredictedPhoneNumberKind) -> ::windows_sys::core::HRESULT,
    pub GetGeographicRegionCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CheckNumberMatch: unsafe extern "system" fn(this: *mut *mut Self, othernumber: *mut ::core::ffi::c_void, result__: *mut PhoneNumberMatchResult) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneNumberInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 477947101, data2: 51380, data3: 20131, data4: [154, 239, 179, 66, 226, 197, 180, 23] };
}
#[repr(C)]
pub struct IPhoneNumberInfoFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, number: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneNumberInfoFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2181216612, data2: 44458, data3: 19711, data4: [143, 207, 23, 231, 81, 106, 40, 255] };
}
#[repr(C)]
pub struct IPhoneNumberInfoStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, phonenumber: *mut *mut ::core::ffi::c_void, result__: *mut PhoneNumberParseResult) -> ::windows_sys::core::HRESULT,
    pub TryParseWithRegion: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, regioncode: ::windows_sys::core::HSTRING, phonenumber: *mut *mut ::core::ffi::c_void, result__: *mut PhoneNumberParseResult) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneNumberInfoStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1530875754, data2: 34473, data3: 16617, data4: [134, 73, 109, 97, 22, 25, 40, 212] };
}
#[doc = "*Required features: `\"Globalization_PhoneNumberFormatting\"`*"]
#[repr(transparent)]
pub struct PhoneNumberFormat(pub i32);
impl PhoneNumberFormat {
    pub const E164: Self = Self(0i32);
    pub const International: Self = Self(1i32);
    pub const National: Self = Self(2i32);
    pub const Rfc3966: Self = Self(3i32);
}
impl ::core::marker::Copy for PhoneNumberFormat {}
impl ::core::clone::Clone for PhoneNumberFormat {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PhoneNumberFormatter = *mut ::core::ffi::c_void;
pub type PhoneNumberInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Globalization_PhoneNumberFormatting\"`*"]
#[repr(transparent)]
pub struct PhoneNumberMatchResult(pub i32);
impl PhoneNumberMatchResult {
    pub const NoMatch: Self = Self(0i32);
    pub const ShortNationalSignificantNumberMatch: Self = Self(1i32);
    pub const NationalSignificantNumberMatch: Self = Self(2i32);
    pub const ExactMatch: Self = Self(3i32);
}
impl ::core::marker::Copy for PhoneNumberMatchResult {}
impl ::core::clone::Clone for PhoneNumberMatchResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Globalization_PhoneNumberFormatting\"`*"]
#[repr(transparent)]
pub struct PhoneNumberParseResult(pub i32);
impl PhoneNumberParseResult {
    pub const Valid: Self = Self(0i32);
    pub const NotANumber: Self = Self(1i32);
    pub const InvalidCountryCode: Self = Self(2i32);
    pub const TooShort: Self = Self(3i32);
    pub const TooLong: Self = Self(4i32);
}
impl ::core::marker::Copy for PhoneNumberParseResult {}
impl ::core::clone::Clone for PhoneNumberParseResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Globalization_PhoneNumberFormatting\"`*"]
#[repr(transparent)]
pub struct PredictedPhoneNumberKind(pub i32);
impl PredictedPhoneNumberKind {
    pub const FixedLine: Self = Self(0i32);
    pub const Mobile: Self = Self(1i32);
    pub const FixedLineOrMobile: Self = Self(2i32);
    pub const TollFree: Self = Self(3i32);
    pub const PremiumRate: Self = Self(4i32);
    pub const SharedCost: Self = Self(5i32);
    pub const Voip: Self = Self(6i32);
    pub const PersonalNumber: Self = Self(7i32);
    pub const Pager: Self = Self(8i32);
    pub const UniversalAccountNumber: Self = Self(9i32);
    pub const Voicemail: Self = Self(10i32);
    pub const Unknown: Self = Self(11i32);
}
impl ::core::marker::Copy for PredictedPhoneNumberKind {}
impl ::core::clone::Clone for PredictedPhoneNumberKind {
    fn clone(&self) -> Self {
        *self
    }
}
