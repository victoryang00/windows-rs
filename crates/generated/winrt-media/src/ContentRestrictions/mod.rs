#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for ContentAccessRestrictionLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ContentAccessRestrictionLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContentAccessRestrictionLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentAccessRestrictionLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ContentAccessRestrictionLevel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.ContentRestrictions.ContentAccessRestrictionLevel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ContentRestrictionsBrowsePolicy(::windows_core::IUnknown);
impl ContentRestrictionsBrowsePolicy {
    pub fn GeographicRegion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GeographicRegion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn MaxBrowsableAgeRating(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MaxBrowsableAgeRating)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u32>>(result__)
        }
    }
    pub fn PreferredAgeRating(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreferredAgeRating)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u32>>(result__)
        }
    }
}
impl ::core::clone::Clone for ContentRestrictionsBrowsePolicy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContentRestrictionsBrowsePolicy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContentRestrictionsBrowsePolicy {}
impl ::core::fmt::Debug for ContentRestrictionsBrowsePolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentRestrictionsBrowsePolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ContentRestrictionsBrowsePolicy {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.ContentRestrictions.ContentRestrictionsBrowsePolicy;{8c0133a4-442e-461a-8757-fad2f5bd37e4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ContentRestrictionsBrowsePolicy {
    type Vtable = IContentRestrictionsBrowsePolicy_Vtbl;
    const IID: ::windows_core::GUID = <IContentRestrictionsBrowsePolicy as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ContentRestrictionsBrowsePolicy {
    const NAME: &'static str = "Windows.Media.ContentRestrictions.ContentRestrictionsBrowsePolicy";
}
impl ::core::convert::From<ContentRestrictionsBrowsePolicy> for ::windows_core::IUnknown {
    fn from(value: ContentRestrictionsBrowsePolicy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentRestrictionsBrowsePolicy> for ::windows_core::IUnknown {
    fn from(value: &ContentRestrictionsBrowsePolicy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ContentRestrictionsBrowsePolicy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ContentRestrictionsBrowsePolicy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContentRestrictionsBrowsePolicy> for ::windows_core::IInspectable {
    fn from(value: ContentRestrictionsBrowsePolicy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentRestrictionsBrowsePolicy> for ::windows_core::IInspectable {
    fn from(value: &ContentRestrictionsBrowsePolicy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ContentRestrictionsBrowsePolicy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ContentRestrictionsBrowsePolicy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ContentRestrictionsBrowsePolicy {}
unsafe impl ::core::marker::Sync for ContentRestrictionsBrowsePolicy {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentRestrictionsBrowsePolicy(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentRestrictionsBrowsePolicy {
    type Vtable = IContentRestrictionsBrowsePolicy_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c0133a4_442e_461a_8757_fad2f5bd37e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentRestrictionsBrowsePolicy_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GeographicRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MaxBrowsableAgeRating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PreferredAgeRating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRatedContentDescription(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRatedContentDescription {
    type Vtable = IRatedContentDescription_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x694866df_66b2_4dc3_96b1_f090eedee255);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRatedContentDescription_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub Image: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Image: usize,
    #[cfg(feature = "winrt-storage")]
    pub SetImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    SetImage: usize,
    pub Category: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RatedContentCategory) -> ::windows_core::HRESULT,
    pub SetCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: RatedContentCategory) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Ratings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Ratings: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetRatings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetRatings: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRatedContentDescriptionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRatedContentDescriptionFactory {
    type Vtable = IRatedContentDescriptionFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e38df62_9b90_4fa6_89c1_4b8d2ffb3573);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRatedContentDescriptionFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, title: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, category: RatedContentCategory, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRatedContentRestrictions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRatedContentRestrictions {
    type Vtable = IRatedContentRestrictions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3f7f23cb_ba07_4401_a49d_8b9222205723);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRatedContentRestrictions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetBrowsePolicyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetRestrictionLevelAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ratedcontentdescription: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestContentAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ratedcontentdescription: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RestrictionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRestrictionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRatedContentRestrictionsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRatedContentRestrictionsFactory {
    type Vtable = IRatedContentRestrictionsFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb4b2996_c3bd_4910_9619_97cfd0694d56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRatedContentRestrictionsFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateWithMaxAgeRating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxagerating: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for RatedContentCategory {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RatedContentCategory {
    type Abi = Self;
}
impl ::core::fmt::Debug for RatedContentCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RatedContentCategory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RatedContentCategory {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.ContentRestrictions.RatedContentCategory;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct RatedContentDescription(::windows_core::IUnknown);
impl RatedContentDescription {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTitle<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTitle)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Image(&self) -> ::windows_core::Result<::winrt_storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Image)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn SetImage<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStreamReference>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetImage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Category(&self) -> ::windows_core::Result<RatedContentCategory> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<RatedContentCategory>::zeroed();
            (::windows_core::Interface::vtable(this).Category)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RatedContentCategory>(result__)
        }
    }
    pub fn SetCategory(&self, value: RatedContentCategory) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCategory)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Ratings(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Ratings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetRatings<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRatings)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(id: Param0, title: Param1, category: RatedContentCategory) -> ::windows_core::Result<RatedContentDescription> {
        Self::IRatedContentDescriptionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), id.into_param().abi(), title.into_param().abi(), category, result__.as_mut_ptr()).from_abi::<RatedContentDescription>(result__)
        })
    }
    pub fn IRatedContentDescriptionFactory<R, F: FnOnce(&IRatedContentDescriptionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RatedContentDescription, IRatedContentDescriptionFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RatedContentDescription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RatedContentDescription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RatedContentDescription {}
impl ::core::fmt::Debug for RatedContentDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RatedContentDescription").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RatedContentDescription {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.ContentRestrictions.RatedContentDescription;{694866df-66b2-4dc3-96b1-f090eedee255})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RatedContentDescription {
    type Vtable = IRatedContentDescription_Vtbl;
    const IID: ::windows_core::GUID = <IRatedContentDescription as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RatedContentDescription {
    const NAME: &'static str = "Windows.Media.ContentRestrictions.RatedContentDescription";
}
impl ::core::convert::From<RatedContentDescription> for ::windows_core::IUnknown {
    fn from(value: RatedContentDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RatedContentDescription> for ::windows_core::IUnknown {
    fn from(value: &RatedContentDescription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RatedContentDescription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RatedContentDescription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RatedContentDescription> for ::windows_core::IInspectable {
    fn from(value: RatedContentDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RatedContentDescription> for ::windows_core::IInspectable {
    fn from(value: &RatedContentDescription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RatedContentDescription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RatedContentDescription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RatedContentDescription {}
unsafe impl ::core::marker::Sync for RatedContentDescription {}
#[repr(transparent)]
pub struct RatedContentRestrictions(::windows_core::IUnknown);
impl RatedContentRestrictions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RatedContentRestrictions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn GetBrowsePolicyAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ContentRestrictionsBrowsePolicy>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetBrowsePolicyAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ContentRestrictionsBrowsePolicy>>(result__)
        }
    }
    pub fn GetRestrictionLevelAsync<'a, Param0: ::windows_core::IntoParam<'a, RatedContentDescription>>(&self, ratedcontentdescription: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ContentAccessRestrictionLevel>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetRestrictionLevelAsync)(::windows_core::Interface::as_raw(this), ratedcontentdescription.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ContentAccessRestrictionLevel>>(result__)
        }
    }
    pub fn RequestContentAccessAsync<'a, Param0: ::windows_core::IntoParam<'a, RatedContentDescription>>(&self, ratedcontentdescription: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestContentAccessAsync)(::windows_core::Interface::as_raw(this), ratedcontentdescription.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn RestrictionsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RestrictionsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveRestrictionsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRestrictionsChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CreateWithMaxAgeRating(maxagerating: u32) -> ::windows_core::Result<RatedContentRestrictions> {
        Self::IRatedContentRestrictionsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithMaxAgeRating)(::windows_core::Interface::as_raw(this), maxagerating, result__.as_mut_ptr()).from_abi::<RatedContentRestrictions>(result__)
        })
    }
    pub fn IRatedContentRestrictionsFactory<R, F: FnOnce(&IRatedContentRestrictionsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RatedContentRestrictions, IRatedContentRestrictionsFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RatedContentRestrictions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RatedContentRestrictions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RatedContentRestrictions {}
impl ::core::fmt::Debug for RatedContentRestrictions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RatedContentRestrictions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RatedContentRestrictions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.ContentRestrictions.RatedContentRestrictions;{3f7f23cb-ba07-4401-a49d-8b9222205723})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RatedContentRestrictions {
    type Vtable = IRatedContentRestrictions_Vtbl;
    const IID: ::windows_core::GUID = <IRatedContentRestrictions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RatedContentRestrictions {
    const NAME: &'static str = "Windows.Media.ContentRestrictions.RatedContentRestrictions";
}
impl ::core::convert::From<RatedContentRestrictions> for ::windows_core::IUnknown {
    fn from(value: RatedContentRestrictions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RatedContentRestrictions> for ::windows_core::IUnknown {
    fn from(value: &RatedContentRestrictions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RatedContentRestrictions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RatedContentRestrictions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RatedContentRestrictions> for ::windows_core::IInspectable {
    fn from(value: RatedContentRestrictions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RatedContentRestrictions> for ::windows_core::IInspectable {
    fn from(value: &RatedContentRestrictions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RatedContentRestrictions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RatedContentRestrictions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RatedContentRestrictions {}
unsafe impl ::core::marker::Sync for RatedContentRestrictions {}
