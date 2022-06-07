#[doc = "*Required features: `\"Media_ContentRestrictions\"`*"]
#[repr(transparent)]
pub struct ContentAccessRestrictionLevel(pub i32);
impl ContentAccessRestrictionLevel {
    pub const Allow: Self = Self(0i32);
    pub const Warn: Self = Self(1i32);
    pub const Block: Self = Self(2i32);
    pub const Hide: Self = Self(3i32);
}
impl ::core::marker::Copy for ContentAccessRestrictionLevel {}
impl ::core::clone::Clone for ContentAccessRestrictionLevel {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContentRestrictionsBrowsePolicy = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IContentRestrictionsBrowsePolicy {
    pub base__: ::windows_sys::core::IInspectable,
    pub GeographicRegion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MaxBrowsableAgeRating: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxBrowsableAgeRating: usize,
    #[cfg(feature = "Foundation")]
    pub PreferredAgeRating: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PreferredAgeRating: usize,
}
impl ::windows_sys::core::Interface for IContentRestrictionsBrowsePolicy {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2348888996, data2: 17454, data3: 17946, data4: [135, 87, 250, 210, 245, 189, 55, 228] };
}
#[repr(C)]
pub struct IRatedContentDescription {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Image: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Image: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetImage: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetImage: usize,
    pub Category: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RatedContentCategory) -> ::windows_sys::core::HRESULT,
    pub SetCategory: unsafe extern "system" fn(this: *mut *mut Self, value: RatedContentCategory) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Ratings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Ratings: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetRatings: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetRatings: usize,
}
impl ::windows_sys::core::Interface for IRatedContentDescription {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1766352607, data2: 26290, data3: 19907, data4: [150, 177, 240, 144, 238, 222, 226, 85] };
}
#[repr(C)]
pub struct IRatedContentDescriptionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, title: ::windows_sys::core::HSTRING, category: RatedContentCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRatedContentDescriptionFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 775479138, data2: 39824, data3: 20390, data4: [137, 193, 75, 141, 47, 251, 53, 115] };
}
#[repr(C)]
pub struct IRatedContentRestrictions {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetBrowsePolicyAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetBrowsePolicyAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetRestrictionLevelAsync: unsafe extern "system" fn(this: *mut *mut Self, ratedcontentdescription: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetRestrictionLevelAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestContentAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, ratedcontentdescription: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestContentAccessAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RestrictionsChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RestrictionsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRestrictionsChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRestrictionsChanged: usize,
}
impl ::windows_sys::core::Interface for IRatedContentRestrictions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1065296843, data2: 47623, data3: 17409, data4: [164, 157, 139, 146, 34, 32, 87, 35] };
}
#[repr(C)]
pub struct IRatedContentRestrictionsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithMaxAgeRating: unsafe extern "system" fn(this: *mut *mut Self, maxagerating: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRatedContentRestrictionsFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4216007062, data2: 50109, data3: 18704, data4: [150, 25, 151, 207, 208, 105, 77, 86] };
}
#[doc = "*Required features: `\"Media_ContentRestrictions\"`*"]
#[repr(transparent)]
pub struct RatedContentCategory(pub i32);
impl RatedContentCategory {
    pub const General: Self = Self(0i32);
    pub const Application: Self = Self(1i32);
    pub const Game: Self = Self(2i32);
    pub const Movie: Self = Self(3i32);
    pub const Television: Self = Self(4i32);
    pub const Music: Self = Self(5i32);
}
impl ::core::marker::Copy for RatedContentCategory {}
impl ::core::clone::Clone for RatedContentCategory {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RatedContentDescription = *mut ::core::ffi::c_void;
pub type RatedContentRestrictions = *mut ::core::ffi::c_void;
