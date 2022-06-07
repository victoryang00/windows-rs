#[cfg(feature = "ApplicationModel_SocialInfo_Provider")]
pub mod Provider;
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISocialFeedChildItem {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Author: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Author: usize,
    #[cfg(feature = "deprecated")]
    pub PrimaryContent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PrimaryContent: usize,
    #[cfg(feature = "deprecated")]
    pub SecondaryContent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SecondaryContent: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Timestamp: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetTimestamp: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetTimestamp: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TargetUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TargetUri: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetTargetUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetTargetUri: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Thumbnails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Thumbnails: usize,
    #[cfg(feature = "deprecated")]
    pub SharedItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SharedItem: usize,
    #[cfg(feature = "deprecated")]
    pub SetSharedItem: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetSharedItem: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ISocialFeedChildItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 191535194, data2: 54685, data3: 16574, data4: [152, 12, 72, 138, 42, 179, 10, 131] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISocialFeedContent {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Title: usize,
    #[cfg(feature = "deprecated")]
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetTitle: usize,
    #[cfg(feature = "deprecated")]
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Message: usize,
    #[cfg(feature = "deprecated")]
    pub SetMessage: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetMessage: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TargetUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TargetUri: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetTargetUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetTargetUri: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ISocialFeedContent {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2721375273, data2: 15929, data3: 18765, data4: [163, 124, 244, 98, 162, 73, 69, 20] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISocialFeedItem {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Author: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Author: usize,
    #[cfg(feature = "deprecated")]
    pub PrimaryContent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PrimaryContent: usize,
    #[cfg(feature = "deprecated")]
    pub SecondaryContent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SecondaryContent: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Timestamp: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetTimestamp: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetTimestamp: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TargetUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TargetUri: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetTargetUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetTargetUri: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Thumbnails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Thumbnails: usize,
    #[cfg(feature = "deprecated")]
    pub SharedItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SharedItem: usize,
    #[cfg(feature = "deprecated")]
    pub SetSharedItem: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetSharedItem: usize,
    #[cfg(feature = "deprecated")]
    pub BadgeStyle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SocialItemBadgeStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BadgeStyle: usize,
    #[cfg(feature = "deprecated")]
    pub SetBadgeStyle: unsafe extern "system" fn(this: *mut *mut Self, value: SocialItemBadgeStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetBadgeStyle: usize,
    #[cfg(feature = "deprecated")]
    pub BadgeCountValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BadgeCountValue: usize,
    #[cfg(feature = "deprecated")]
    pub SetBadgeCountValue: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetBadgeCountValue: usize,
    #[cfg(feature = "deprecated")]
    pub RemoteId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoteId: usize,
    #[cfg(feature = "deprecated")]
    pub SetRemoteId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetRemoteId: usize,
    #[cfg(feature = "deprecated")]
    pub ChildItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ChildItem: usize,
    #[cfg(feature = "deprecated")]
    pub SetChildItem: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetChildItem: usize,
    #[cfg(feature = "deprecated")]
    pub Style: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SocialFeedItemStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Style: usize,
    #[cfg(feature = "deprecated")]
    pub SetStyle: unsafe extern "system" fn(this: *mut *mut Self, value: SocialFeedItemStyle) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetStyle: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ISocialFeedItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1326682795, data2: 8050, data3: 19763, data4: [182, 149, 222, 62, 29, 182, 3, 23] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISocialFeedSharedItem {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub OriginalSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    OriginalSource: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetOriginalSource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetOriginalSource: usize,
    #[cfg(feature = "deprecated")]
    pub Content: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Content: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Timestamp: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetTimestamp: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetTimestamp: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TargetUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TargetUri: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetTargetUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetTargetUri: usize,
    #[cfg(feature = "deprecated")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetThumbnail: usize,
    #[cfg(feature = "deprecated")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Thumbnail: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ISocialFeedSharedItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2080087616, data2: 42666, data3: 17831, data4: [159, 246, 84, 196, 33, 5, 221, 31] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISocialItemThumbnail {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TargetUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TargetUri: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetTargetUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetTargetUri: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ImageUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ImageUri: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetImageUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetImageUri: usize,
    #[cfg(all(feature = "Graphics_Imaging", feature = "deprecated"))]
    pub BitmapSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Imaging", feature = "deprecated")))]
    BitmapSize: usize,
    #[cfg(all(feature = "Graphics_Imaging", feature = "deprecated"))]
    pub SetBitmapSize: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Imaging", feature = "deprecated")))]
    SetBitmapSize: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub SetImageAsync: unsafe extern "system" fn(this: *mut *mut Self, image: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    SetImageAsync: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ISocialItemThumbnail {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1556054810, data2: 16136, data3: 18815, data4: [145, 127, 87, 224, 157, 132, 177, 65] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISocialUserInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DisplayName: usize,
    #[cfg(feature = "deprecated")]
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetDisplayName: usize,
    #[cfg(feature = "deprecated")]
    pub UserName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UserName: usize,
    #[cfg(feature = "deprecated")]
    pub SetUserName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetUserName: usize,
    #[cfg(feature = "deprecated")]
    pub RemoteId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoteId: usize,
    #[cfg(feature = "deprecated")]
    pub SetRemoteId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetRemoteId: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TargetUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TargetUri: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetTargetUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetTargetUri: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ISocialUserInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2656967633, data2: 37072, data3: 19997, data4: [149, 84, 132, 77, 70, 96, 127, 97] };
}
pub type SocialFeedChildItem = *mut ::core::ffi::c_void;
pub type SocialFeedContent = *mut ::core::ffi::c_void;
pub type SocialFeedItem = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_SocialInfo\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SocialFeedItemStyle(pub i32);
#[cfg(feature = "deprecated")]
impl SocialFeedItemStyle {
    pub const Default: Self = Self(0i32);
    pub const Photo: Self = Self(1i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SocialFeedItemStyle {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SocialFeedItemStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_SocialInfo\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SocialFeedKind(pub i32);
#[cfg(feature = "deprecated")]
impl SocialFeedKind {
    pub const HomeFeed: Self = Self(0i32);
    pub const ContactFeed: Self = Self(1i32);
    pub const Dashboard: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SocialFeedKind {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SocialFeedKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SocialFeedSharedItem = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_SocialInfo\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SocialFeedUpdateMode(pub i32);
#[cfg(feature = "deprecated")]
impl SocialFeedUpdateMode {
    pub const Append: Self = Self(0i32);
    pub const Replace: Self = Self(1i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SocialFeedUpdateMode {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SocialFeedUpdateMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_SocialInfo\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SocialItemBadgeStyle(pub i32);
#[cfg(feature = "deprecated")]
impl SocialItemBadgeStyle {
    pub const Hidden: Self = Self(0i32);
    pub const Visible: Self = Self(1i32);
    pub const VisibleWithCount: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SocialItemBadgeStyle {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SocialItemBadgeStyle {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SocialItemThumbnail = *mut ::core::ffi::c_void;
pub type SocialUserInfo = *mut ::core::ffi::c_void;
