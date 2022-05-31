#[cfg(feature = "Provider")]
pub mod Provider;
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISocialFeedChildItem(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISocialFeedChildItem {
    type Vtable = ISocialFeedChildItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0b6a985a_d59d_40be_980c_488a2ab30a83);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISocialFeedChildItem_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Author: usize,
    #[cfg(feature = "winrt-")]
    pub PrimaryContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    PrimaryContent: usize,
    #[cfg(feature = "winrt-")]
    pub SecondaryContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SecondaryContent: usize,
    #[cfg(feature = "winrt-")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Timestamp: usize,
    #[cfg(feature = "winrt-")]
    pub SetTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetTimestamp: usize,
    #[cfg(feature = "winrt-")]
    pub TargetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TargetUri: usize,
    #[cfg(feature = "winrt-")]
    pub SetTargetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetTargetUri: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub Thumbnails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    Thumbnails: usize,
    #[cfg(feature = "winrt-")]
    pub SharedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SharedItem: usize,
    #[cfg(feature = "winrt-")]
    pub SetSharedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetSharedItem: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISocialFeedContent(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISocialFeedContent {
    type Vtable = ISocialFeedContent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa234e429_3e39_494d_a37c_f462a2494514);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISocialFeedContent_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Title: usize,
    #[cfg(feature = "winrt-")]
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetTitle: usize,
    #[cfg(feature = "winrt-")]
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Message: usize,
    #[cfg(feature = "winrt-")]
    pub SetMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetMessage: usize,
    #[cfg(feature = "winrt-")]
    pub TargetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TargetUri: usize,
    #[cfg(feature = "winrt-")]
    pub SetTargetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetTargetUri: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISocialFeedItem(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISocialFeedItem {
    type Vtable = ISocialFeedItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4f1392ab_1f72_4d33_b695_de3e1db60317);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISocialFeedItem_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Author: usize,
    #[cfg(feature = "winrt-")]
    pub PrimaryContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    PrimaryContent: usize,
    #[cfg(feature = "winrt-")]
    pub SecondaryContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SecondaryContent: usize,
    #[cfg(feature = "winrt-")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Timestamp: usize,
    #[cfg(feature = "winrt-")]
    pub SetTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetTimestamp: usize,
    #[cfg(feature = "winrt-")]
    pub TargetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TargetUri: usize,
    #[cfg(feature = "winrt-")]
    pub SetTargetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetTargetUri: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub Thumbnails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    Thumbnails: usize,
    #[cfg(feature = "winrt-")]
    pub SharedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SharedItem: usize,
    #[cfg(feature = "winrt-")]
    pub SetSharedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetSharedItem: usize,
    #[cfg(feature = "winrt-")]
    pub BadgeStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocialItemBadgeStyle) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    BadgeStyle: usize,
    #[cfg(feature = "winrt-")]
    pub SetBadgeStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SocialItemBadgeStyle) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetBadgeStyle: usize,
    #[cfg(feature = "winrt-")]
    pub BadgeCountValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    BadgeCountValue: usize,
    #[cfg(feature = "winrt-")]
    pub SetBadgeCountValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetBadgeCountValue: usize,
    #[cfg(feature = "winrt-")]
    pub RemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoteId: usize,
    #[cfg(feature = "winrt-")]
    pub SetRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetRemoteId: usize,
    #[cfg(feature = "winrt-")]
    pub ChildItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ChildItem: usize,
    #[cfg(feature = "winrt-")]
    pub SetChildItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetChildItem: usize,
    #[cfg(feature = "winrt-")]
    pub Style: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SocialFeedItemStyle) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Style: usize,
    #[cfg(feature = "winrt-")]
    pub SetStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SocialFeedItemStyle) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetStyle: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISocialFeedSharedItem(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISocialFeedSharedItem {
    type Vtable = ISocialFeedSharedItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7bfb9e40_a6aa_45a7_9ff6_54c42105dd1f);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISocialFeedSharedItem_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub OriginalSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    OriginalSource: usize,
    #[cfg(feature = "winrt-")]
    pub SetOriginalSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetOriginalSource: usize,
    #[cfg(feature = "winrt-")]
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Content: usize,
    #[cfg(feature = "winrt-")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Timestamp: usize,
    #[cfg(feature = "winrt-")]
    pub SetTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetTimestamp: usize,
    #[cfg(feature = "winrt-")]
    pub TargetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TargetUri: usize,
    #[cfg(feature = "winrt-")]
    pub SetTargetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetTargetUri: usize,
    #[cfg(feature = "winrt-")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetThumbnail: usize,
    #[cfg(feature = "winrt-")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Thumbnail: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISocialItemThumbnail(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISocialItemThumbnail {
    type Vtable = ISocialItemThumbnail_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5cbf831a_3f08_497f_917f_57e09d84b141);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISocialItemThumbnail_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub TargetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TargetUri: usize,
    #[cfg(feature = "winrt-")]
    pub SetTargetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetTargetUri: usize,
    #[cfg(feature = "winrt-")]
    pub ImageUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ImageUri: usize,
    #[cfg(feature = "winrt-")]
    pub SetImageUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetImageUri: usize,
    #[cfg(all(feature = "winrt-graphics", feature = "winrt-"))]
    pub BitmapSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_graphics::Imaging::BitmapSize) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-graphics", feature = "winrt-")))]
    BitmapSize: usize,
    #[cfg(all(feature = "winrt-graphics", feature = "winrt-"))]
    pub SetBitmapSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_graphics::Imaging::BitmapSize) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-graphics", feature = "winrt-")))]
    SetBitmapSize: usize,
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub SetImageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-storage", feature = "winrt-")))]
    SetImageAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISocialUserInfo(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISocialUserInfo {
    type Vtable = ISocialUserInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e5e1bd1_90d0_4e1d_9554_844d46607f61);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISocialUserInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    DisplayName: usize,
    #[cfg(feature = "winrt-")]
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetDisplayName: usize,
    #[cfg(feature = "winrt-")]
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    UserName: usize,
    #[cfg(feature = "winrt-")]
    pub SetUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetUserName: usize,
    #[cfg(feature = "winrt-")]
    pub RemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    RemoteId: usize,
    #[cfg(feature = "winrt-")]
    pub SetRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetRemoteId: usize,
    #[cfg(feature = "winrt-")]
    pub TargetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    TargetUri: usize,
    #[cfg(feature = "winrt-")]
    pub SetTargetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetTargetUri: usize,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SocialFeedChildItem(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SocialFeedChildItem {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SocialFeedChildItem, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-")]
    pub fn Author(&self) -> ::windows_core::Result<SocialUserInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Author)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SocialUserInfo>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn PrimaryContent(&self) -> ::windows_core::Result<SocialFeedContent> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrimaryContent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SocialFeedContent>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SecondaryContent(&self) -> ::windows_core::Result<SocialFeedContent> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SecondaryContent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SocialFeedContent>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Timestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetTimestamp<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTimestamp)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn TargetUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetTargetUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn Thumbnails(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SocialItemThumbnail>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnails)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SocialItemThumbnail>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SharedItem(&self) -> ::windows_core::Result<SocialFeedSharedItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SharedItem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SocialFeedSharedItem>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetSharedItem<'a, Param0: ::windows_core::IntoParam<'a, SocialFeedSharedItem>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSharedItem)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SocialFeedChildItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SocialFeedChildItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SocialFeedChildItem {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SocialFeedChildItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocialFeedChildItem").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SocialFeedChildItem {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SocialInfo.SocialFeedChildItem;{0b6a985a-d59d-40be-980c-488a2ab30a83})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SocialFeedChildItem {
    type Vtable = ISocialFeedChildItem_Vtbl;
    const IID: ::windows_core::GUID = <ISocialFeedChildItem as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for SocialFeedChildItem {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.SocialFeedChildItem";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SocialFeedChildItem> for ::windows_core::IUnknown {
    fn from(value: SocialFeedChildItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SocialFeedChildItem> for ::windows_core::IUnknown {
    fn from(value: &SocialFeedChildItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SocialFeedChildItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SocialFeedChildItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SocialFeedChildItem> for ::windows_core::IInspectable {
    fn from(value: SocialFeedChildItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SocialFeedChildItem> for ::windows_core::IInspectable {
    fn from(value: &SocialFeedChildItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SocialFeedChildItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SocialFeedChildItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for SocialFeedChildItem {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for SocialFeedChildItem {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SocialFeedContent(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SocialFeedContent {
    #[cfg(feature = "winrt-")]
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetTitle<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTitle)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Message(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Message)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetMessage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMessage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn TargetUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetTargetUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SocialFeedContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SocialFeedContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SocialFeedContent {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SocialFeedContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocialFeedContent").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SocialFeedContent {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SocialInfo.SocialFeedContent;{a234e429-3e39-494d-a37c-f462a2494514})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SocialFeedContent {
    type Vtable = ISocialFeedContent_Vtbl;
    const IID: ::windows_core::GUID = <ISocialFeedContent as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for SocialFeedContent {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.SocialFeedContent";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SocialFeedContent> for ::windows_core::IUnknown {
    fn from(value: SocialFeedContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SocialFeedContent> for ::windows_core::IUnknown {
    fn from(value: &SocialFeedContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SocialFeedContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SocialFeedContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SocialFeedContent> for ::windows_core::IInspectable {
    fn from(value: SocialFeedContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SocialFeedContent> for ::windows_core::IInspectable {
    fn from(value: &SocialFeedContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SocialFeedContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SocialFeedContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for SocialFeedContent {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for SocialFeedContent {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SocialFeedItem(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SocialFeedItem {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SocialFeedItem, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-")]
    pub fn Author(&self) -> ::windows_core::Result<SocialUserInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Author)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SocialUserInfo>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn PrimaryContent(&self) -> ::windows_core::Result<SocialFeedContent> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrimaryContent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SocialFeedContent>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SecondaryContent(&self) -> ::windows_core::Result<SocialFeedContent> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SecondaryContent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SocialFeedContent>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Timestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetTimestamp<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTimestamp)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn TargetUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetTargetUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn Thumbnails(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SocialItemThumbnail>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnails)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SocialItemThumbnail>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SharedItem(&self) -> ::windows_core::Result<SocialFeedSharedItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SharedItem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SocialFeedSharedItem>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetSharedItem<'a, Param0: ::windows_core::IntoParam<'a, SocialFeedSharedItem>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSharedItem)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn BadgeStyle(&self) -> ::windows_core::Result<SocialItemBadgeStyle> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SocialItemBadgeStyle>::zeroed();
            (::windows_core::Interface::vtable(this).BadgeStyle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SocialItemBadgeStyle>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetBadgeStyle(&self, value: SocialItemBadgeStyle) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBadgeStyle)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn BadgeCountValue(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).BadgeCountValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetBadgeCountValue(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBadgeCountValue)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoteId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetRemoteId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRemoteId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn ChildItem(&self) -> ::windows_core::Result<SocialFeedChildItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChildItem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SocialFeedChildItem>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetChildItem<'a, Param0: ::windows_core::IntoParam<'a, SocialFeedChildItem>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetChildItem)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Style(&self) -> ::windows_core::Result<SocialFeedItemStyle> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SocialFeedItemStyle>::zeroed();
            (::windows_core::Interface::vtable(this).Style)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SocialFeedItemStyle>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetStyle(&self, value: SocialFeedItemStyle) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStyle)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SocialFeedItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SocialFeedItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SocialFeedItem {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SocialFeedItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocialFeedItem").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SocialFeedItem {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SocialInfo.SocialFeedItem;{4f1392ab-1f72-4d33-b695-de3e1db60317})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SocialFeedItem {
    type Vtable = ISocialFeedItem_Vtbl;
    const IID: ::windows_core::GUID = <ISocialFeedItem as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for SocialFeedItem {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.SocialFeedItem";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SocialFeedItem> for ::windows_core::IUnknown {
    fn from(value: SocialFeedItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SocialFeedItem> for ::windows_core::IUnknown {
    fn from(value: &SocialFeedItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SocialFeedItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SocialFeedItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SocialFeedItem> for ::windows_core::IInspectable {
    fn from(value: SocialFeedItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SocialFeedItem> for ::windows_core::IInspectable {
    fn from(value: &SocialFeedItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SocialFeedItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SocialFeedItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for SocialFeedItem {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for SocialFeedItem {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SocialFeedItemStyle(pub i32);
#[cfg(feature = "winrt-")]
impl SocialFeedItemStyle {
    pub const Default: Self = Self(0i32);
    pub const Photo: Self = Self(1i32);
}
#[cfg(feature = "winrt-")]
impl ::core::marker::Copy for SocialFeedItemStyle {}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SocialFeedItemStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "winrt-")]
impl ::core::default::Default for SocialFeedItemStyle {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Abi for SocialFeedItemStyle {
    type Abi = Self;
}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SocialFeedItemStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocialFeedItemStyle").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SocialFeedItemStyle {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.SocialInfo.SocialFeedItemStyle;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SocialFeedKind(pub i32);
#[cfg(feature = "winrt-")]
impl SocialFeedKind {
    pub const HomeFeed: Self = Self(0i32);
    pub const ContactFeed: Self = Self(1i32);
    pub const Dashboard: Self = Self(2i32);
}
#[cfg(feature = "winrt-")]
impl ::core::marker::Copy for SocialFeedKind {}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SocialFeedKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "winrt-")]
impl ::core::default::Default for SocialFeedKind {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Abi for SocialFeedKind {
    type Abi = Self;
}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SocialFeedKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocialFeedKind").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SocialFeedKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.SocialInfo.SocialFeedKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SocialFeedSharedItem(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SocialFeedSharedItem {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SocialFeedSharedItem, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-")]
    pub fn OriginalSource(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OriginalSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetOriginalSource<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOriginalSource)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Content(&self) -> ::windows_core::Result<SocialFeedContent> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SocialFeedContent>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Timestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetTimestamp<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTimestamp)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn TargetUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetTargetUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetThumbnail<'a, Param0: ::windows_core::IntoParam<'a, SocialItemThumbnail>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetThumbnail)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Thumbnail(&self) -> ::windows_core::Result<SocialItemThumbnail> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SocialItemThumbnail>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SocialFeedSharedItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SocialFeedSharedItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SocialFeedSharedItem {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SocialFeedSharedItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocialFeedSharedItem").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SocialFeedSharedItem {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SocialInfo.SocialFeedSharedItem;{7bfb9e40-a6aa-45a7-9ff6-54c42105dd1f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SocialFeedSharedItem {
    type Vtable = ISocialFeedSharedItem_Vtbl;
    const IID: ::windows_core::GUID = <ISocialFeedSharedItem as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for SocialFeedSharedItem {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.SocialFeedSharedItem";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SocialFeedSharedItem> for ::windows_core::IUnknown {
    fn from(value: SocialFeedSharedItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SocialFeedSharedItem> for ::windows_core::IUnknown {
    fn from(value: &SocialFeedSharedItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SocialFeedSharedItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SocialFeedSharedItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SocialFeedSharedItem> for ::windows_core::IInspectable {
    fn from(value: SocialFeedSharedItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SocialFeedSharedItem> for ::windows_core::IInspectable {
    fn from(value: &SocialFeedSharedItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SocialFeedSharedItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SocialFeedSharedItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for SocialFeedSharedItem {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for SocialFeedSharedItem {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SocialFeedUpdateMode(pub i32);
#[cfg(feature = "winrt-")]
impl SocialFeedUpdateMode {
    pub const Append: Self = Self(0i32);
    pub const Replace: Self = Self(1i32);
}
#[cfg(feature = "winrt-")]
impl ::core::marker::Copy for SocialFeedUpdateMode {}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SocialFeedUpdateMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "winrt-")]
impl ::core::default::Default for SocialFeedUpdateMode {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Abi for SocialFeedUpdateMode {
    type Abi = Self;
}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SocialFeedUpdateMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocialFeedUpdateMode").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SocialFeedUpdateMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.SocialInfo.SocialFeedUpdateMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SocialItemBadgeStyle(pub i32);
#[cfg(feature = "winrt-")]
impl SocialItemBadgeStyle {
    pub const Hidden: Self = Self(0i32);
    pub const Visible: Self = Self(1i32);
    pub const VisibleWithCount: Self = Self(2i32);
}
#[cfg(feature = "winrt-")]
impl ::core::marker::Copy for SocialItemBadgeStyle {}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SocialItemBadgeStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "winrt-")]
impl ::core::default::Default for SocialItemBadgeStyle {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Abi for SocialItemBadgeStyle {
    type Abi = Self;
}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SocialItemBadgeStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocialItemBadgeStyle").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SocialItemBadgeStyle {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.SocialInfo.SocialItemBadgeStyle;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SocialItemThumbnail(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SocialItemThumbnail {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SocialItemThumbnail, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-")]
    pub fn TargetUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetTargetUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn ImageUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImageUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetImageUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetImageUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "winrt-graphics", feature = "winrt-"))]
    pub fn BitmapSize(&self) -> ::windows_core::Result<::winrt_graphics::Imaging::BitmapSize> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_graphics::Imaging::BitmapSize>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::Imaging::BitmapSize>(result__)
        }
    }
    #[cfg(all(feature = "winrt-graphics", feature = "winrt-"))]
    pub fn SetBitmapSize<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_graphics::Imaging::BitmapSize>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBitmapSize)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub fn SetImageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream>>(&self, image: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetImageAsync)(::windows_core::Interface::as_raw(this), image.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SocialItemThumbnail {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SocialItemThumbnail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SocialItemThumbnail {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SocialItemThumbnail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocialItemThumbnail").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SocialItemThumbnail {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SocialInfo.SocialItemThumbnail;{5cbf831a-3f08-497f-917f-57e09d84b141})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SocialItemThumbnail {
    type Vtable = ISocialItemThumbnail_Vtbl;
    const IID: ::windows_core::GUID = <ISocialItemThumbnail as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for SocialItemThumbnail {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.SocialItemThumbnail";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SocialItemThumbnail> for ::windows_core::IUnknown {
    fn from(value: SocialItemThumbnail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SocialItemThumbnail> for ::windows_core::IUnknown {
    fn from(value: &SocialItemThumbnail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SocialItemThumbnail {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SocialItemThumbnail {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SocialItemThumbnail> for ::windows_core::IInspectable {
    fn from(value: SocialItemThumbnail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SocialItemThumbnail> for ::windows_core::IInspectable {
    fn from(value: &SocialItemThumbnail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SocialItemThumbnail {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SocialItemThumbnail {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for SocialItemThumbnail {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for SocialItemThumbnail {}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SocialUserInfo(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SocialUserInfo {
    #[cfg(feature = "winrt-")]
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn UserName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).UserName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetUserName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUserName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn RemoteId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetRemoteId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRemoteId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn TargetUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetTargetUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SocialUserInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SocialUserInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SocialUserInfo {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SocialUserInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocialUserInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SocialUserInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SocialInfo.SocialUserInfo;{9e5e1bd1-90d0-4e1d-9554-844d46607f61})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SocialUserInfo {
    type Vtable = ISocialUserInfo_Vtbl;
    const IID: ::windows_core::GUID = <ISocialUserInfo as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for SocialUserInfo {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.SocialUserInfo";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SocialUserInfo> for ::windows_core::IUnknown {
    fn from(value: SocialUserInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SocialUserInfo> for ::windows_core::IUnknown {
    fn from(value: &SocialUserInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SocialUserInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SocialUserInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SocialUserInfo> for ::windows_core::IInspectable {
    fn from(value: SocialUserInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SocialUserInfo> for ::windows_core::IInspectable {
    fn from(value: &SocialUserInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SocialUserInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SocialUserInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Send for SocialUserInfo {}
#[cfg(feature = "winrt-")]
unsafe impl ::core::marker::Sync for SocialUserInfo {}
