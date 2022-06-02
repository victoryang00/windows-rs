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
