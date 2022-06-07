#[repr(C)]
pub struct ILocalCategoriesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub BankAndCreditUnions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EatDrink: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Hospitals: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HotelsAndMotels: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub All: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Parking: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SeeDo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Shop: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILocalCategoriesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4103313909, data2: 33377, data3: 17185, data4: [153, 116, 239, 146, 212, 154, 141, 202] };
}
#[repr(C)]
pub struct ILocalLocation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Address: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Identifier: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub Point: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Point: usize,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DataAttribution: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILocalLocation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3138382251, data2: 17666, data3: 20268, data4: [148, 169, 13, 96, 222, 14, 33, 99] };
}
#[repr(C)]
pub struct ILocalLocation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Category: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RatingInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub HoursOfOperation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    HoursOfOperation: usize,
}
impl ::windows_sys::core::Interface for ILocalLocation2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1855860860, data2: 60597, data3: 20476, data4: [187, 140, 186, 80, 186, 140, 45, 198] };
}
#[repr(C)]
pub struct ILocalLocationFinderResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub LocalLocations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LocalLocations: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LocalLocationFinderStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILocalLocationFinderResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3499846854, data2: 62264, data3: 16785, data4: [159, 216, 84, 64, 185, 166, 143, 82] };
}
#[repr(C)]
pub struct ILocalLocationFinderStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindLocalLocationsAsync: unsafe extern "system" fn(this: *mut *mut Self, searchterm: ::windows_sys::core::HSTRING, searcharea: *mut ::core::ffi::c_void, localcategory: ::windows_sys::core::HSTRING, maxresults: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindLocalLocationsAsync: usize,
}
impl ::windows_sys::core::Interface for ILocalLocationFinderStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3538907972, data2: 41182, data3: 18634, data4: [129, 168, 7, 199, 220, 253, 55, 171] };
}
#[repr(C)]
pub struct ILocalLocationHoursOfOperationItem {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Globalization")]
    pub Day: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Globalization::DayOfWeek) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    Day: usize,
    #[cfg(feature = "Foundation")]
    pub Start: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Start: usize,
    #[cfg(feature = "Foundation")]
    pub Span: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Span: usize,
}
impl ::windows_sys::core::Interface for ILocalLocationHoursOfOperationItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 592743538, data2: 41415, data3: 17393, data4: [164, 240, 16, 145, 195, 158, 198, 64] };
}
#[repr(C)]
pub struct ILocalLocationRatingInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AggregateRating: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AggregateRating: usize,
    #[cfg(feature = "Foundation")]
    pub RatingCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RatingCount: usize,
    pub ProviderIdentifier: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILocalLocationRatingInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3407719254, data2: 13140, data3: 17169, data4: [139, 192, 162, 212, 213, 235, 128, 110] };
}
#[repr(C)]
pub struct IPlaceInfoHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromLocalLocation: unsafe extern "system" fn(this: *mut *mut Self, location: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlaceInfoHelperStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3709643175, data2: 43462, data3: 18715, data4: [188, 9, 232, 15, 206, 164, 142, 230] };
}
pub type LocalLocation = *mut ::core::ffi::c_void;
pub type LocalLocationFinderResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
#[repr(transparent)]
pub struct LocalLocationFinderStatus(pub i32);
impl LocalLocationFinderStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const InvalidCredentials: Self = Self(2i32);
    pub const InvalidCategory: Self = Self(3i32);
    pub const InvalidSearchTerm: Self = Self(4i32);
    pub const InvalidSearchArea: Self = Self(5i32);
    pub const NetworkFailure: Self = Self(6i32);
    pub const NotSupported: Self = Self(7i32);
}
impl ::core::marker::Copy for LocalLocationFinderStatus {}
impl ::core::clone::Clone for LocalLocationFinderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LocalLocationHoursOfOperationItem = *mut ::core::ffi::c_void;
pub type LocalLocationRatingInfo = *mut ::core::ffi::c_void;
