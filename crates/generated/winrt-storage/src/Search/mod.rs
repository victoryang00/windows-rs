#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CommonFileQuery(pub i32);
impl CommonFileQuery {
    pub const DefaultQuery: Self = Self(0i32);
    pub const OrderByName: Self = Self(1i32);
    pub const OrderByTitle: Self = Self(2i32);
    pub const OrderByMusicProperties: Self = Self(3i32);
    pub const OrderBySearchRank: Self = Self(4i32);
    pub const OrderByDate: Self = Self(5i32);
}
impl ::core::marker::Copy for CommonFileQuery {}
impl ::core::clone::Clone for CommonFileQuery {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CommonFileQuery {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CommonFileQuery {
    type Abi = Self;
}
impl ::core::fmt::Debug for CommonFileQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CommonFileQuery").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CommonFileQuery {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Storage.Search.CommonFileQuery;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CommonFolderQuery(pub i32);
impl CommonFolderQuery {
    pub const DefaultQuery: Self = Self(0i32);
    pub const GroupByYear: Self = Self(100i32);
    pub const GroupByMonth: Self = Self(101i32);
    pub const GroupByArtist: Self = Self(102i32);
    pub const GroupByAlbum: Self = Self(103i32);
    pub const GroupByAlbumArtist: Self = Self(104i32);
    pub const GroupByComposer: Self = Self(105i32);
    pub const GroupByGenre: Self = Self(106i32);
    pub const GroupByPublishedYear: Self = Self(107i32);
    pub const GroupByRating: Self = Self(108i32);
    pub const GroupByTag: Self = Self(109i32);
    pub const GroupByAuthor: Self = Self(110i32);
    pub const GroupByType: Self = Self(111i32);
}
impl ::core::marker::Copy for CommonFolderQuery {}
impl ::core::clone::Clone for CommonFolderQuery {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CommonFolderQuery {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CommonFolderQuery {
    type Abi = Self;
}
impl ::core::fmt::Debug for CommonFolderQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CommonFolderQuery").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CommonFolderQuery {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Storage.Search.CommonFolderQuery;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ContentIndexer(::windows_core::IUnknown);
impl ContentIndexer {
    pub fn AddAsync<'a, Param0: ::windows_core::IntoParam<'a, IIndexableContent>>(&self, indexablecontent: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AddAsync)(::windows_core::Interface::as_raw(this), indexablecontent.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn UpdateAsync<'a, Param0: ::windows_core::IntoParam<'a, IIndexableContent>>(&self, indexablecontent: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateAsync)(::windows_core::Interface::as_raw(this), indexablecontent.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn DeleteAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, contentid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteAsync)(::windows_core::Interface::as_raw(this), contentid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeleteMultipleAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, contentids: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteMultipleAsync)(::windows_core::Interface::as_raw(this), contentids.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn DeleteAllAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteAllAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RetrievePropertiesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, contentid: Param0, propertiestoretrieve: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RetrievePropertiesAsync)(::windows_core::Interface::as_raw(this), contentid.into_param().abi(), propertiestoretrieve.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>>(result__)
        }
    }
    pub fn Revision(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).Revision)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateQueryWithSortOrderAndLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<SortEntry>>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, searchfilter: Param0, propertiestoretrieve: Param1, sortorder: Param2, searchfilterlanguage: Param3) -> ::windows_core::Result<ContentIndexerQuery> {
        let this = &::windows_core::Interface::cast::<IContentIndexerQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateQueryWithSortOrderAndLanguage)(::windows_core::Interface::as_raw(this), searchfilter.into_param().abi(), propertiestoretrieve.into_param().abi(), sortorder.into_param().abi(), searchfilterlanguage.into_param().abi(), result__.as_mut_ptr()).from_abi::<ContentIndexerQuery>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateQueryWithSortOrder<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<SortEntry>>>(&self, searchfilter: Param0, propertiestoretrieve: Param1, sortorder: Param2) -> ::windows_core::Result<ContentIndexerQuery> {
        let this = &::windows_core::Interface::cast::<IContentIndexerQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateQueryWithSortOrder)(::windows_core::Interface::as_raw(this), searchfilter.into_param().abi(), propertiestoretrieve.into_param().abi(), sortorder.into_param().abi(), result__.as_mut_ptr()).from_abi::<ContentIndexerQuery>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateQuery<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, searchfilter: Param0, propertiestoretrieve: Param1) -> ::windows_core::Result<ContentIndexerQuery> {
        let this = &::windows_core::Interface::cast::<IContentIndexerQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateQuery)(::windows_core::Interface::as_raw(this), searchfilter.into_param().abi(), propertiestoretrieve.into_param().abi(), result__.as_mut_ptr()).from_abi::<ContentIndexerQuery>(result__)
        }
    }
    pub fn GetIndexerWithName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(indexname: Param0) -> ::windows_core::Result<ContentIndexer> {
        Self::IContentIndexerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetIndexerWithName)(::windows_core::Interface::as_raw(this), indexname.into_param().abi(), result__.as_mut_ptr()).from_abi::<ContentIndexer>(result__)
        })
    }
    pub fn GetIndexer() -> ::windows_core::Result<ContentIndexer> {
        Self::IContentIndexerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetIndexer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ContentIndexer>(result__)
        })
    }
    pub fn IContentIndexerStatics<R, F: FnOnce(&IContentIndexerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ContentIndexer, IContentIndexerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ContentIndexer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContentIndexer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContentIndexer {}
impl ::core::fmt::Debug for ContentIndexer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentIndexer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ContentIndexer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.ContentIndexer;{b1767f8d-f698-4982-b05f-3a6e8cab01a2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ContentIndexer {
    type Vtable = IContentIndexer_Vtbl;
    const IID: ::windows_core::GUID = <IContentIndexer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ContentIndexer {
    const NAME: &'static str = "Windows.Storage.Search.ContentIndexer";
}
impl ::core::convert::From<ContentIndexer> for ::windows_core::IUnknown {
    fn from(value: ContentIndexer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentIndexer> for ::windows_core::IUnknown {
    fn from(value: &ContentIndexer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ContentIndexer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ContentIndexer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContentIndexer> for ::windows_core::IInspectable {
    fn from(value: ContentIndexer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentIndexer> for ::windows_core::IInspectable {
    fn from(value: &ContentIndexer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ContentIndexer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ContentIndexer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ContentIndexer {}
unsafe impl ::core::marker::Sync for ContentIndexer {}
#[repr(transparent)]
pub struct ContentIndexerQuery(::windows_core::IUnknown);
impl ContentIndexerQuery {
    pub fn GetCountAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCountAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPropertiesAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPropertiesAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPropertiesRangeAsync(&self, startindex: u32, maxitems: u32) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPropertiesRangeAsync)(::windows_core::Interface::as_raw(this), startindex, maxitems, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<IIndexableContent>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<IIndexableContent>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRangeAsync(&self, startindex: u32, maxitems: u32) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<IIndexableContent>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetRangeAsync)(::windows_core::Interface::as_raw(this), startindex, maxitems, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<IIndexableContent>>>(result__)
        }
    }
    pub fn QueryFolder(&self) -> ::windows_core::Result<super::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).QueryFolder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::StorageFolder>(result__)
        }
    }
}
impl ::core::clone::Clone for ContentIndexerQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContentIndexerQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContentIndexerQuery {}
impl ::core::fmt::Debug for ContentIndexerQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentIndexerQuery").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ContentIndexerQuery {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.ContentIndexerQuery;{70e3b0f8-4bfc-428a-8889-cc51da9a7b9d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ContentIndexerQuery {
    type Vtable = IContentIndexerQuery_Vtbl;
    const IID: ::windows_core::GUID = <IContentIndexerQuery as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ContentIndexerQuery {
    const NAME: &'static str = "Windows.Storage.Search.ContentIndexerQuery";
}
impl ::core::convert::From<ContentIndexerQuery> for ::windows_core::IUnknown {
    fn from(value: ContentIndexerQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentIndexerQuery> for ::windows_core::IUnknown {
    fn from(value: &ContentIndexerQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ContentIndexerQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ContentIndexerQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContentIndexerQuery> for ::windows_core::IInspectable {
    fn from(value: ContentIndexerQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentIndexerQuery> for ::windows_core::IInspectable {
    fn from(value: &ContentIndexerQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ContentIndexerQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ContentIndexerQuery {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ContentIndexerQuery {}
unsafe impl ::core::marker::Sync for ContentIndexerQuery {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DateStackOption(pub i32);
impl DateStackOption {
    pub const None: Self = Self(0i32);
    pub const Year: Self = Self(1i32);
    pub const Month: Self = Self(2i32);
}
impl ::core::marker::Copy for DateStackOption {}
impl ::core::clone::Clone for DateStackOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DateStackOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DateStackOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for DateStackOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DateStackOption").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DateStackOption {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Storage.Search.DateStackOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FolderDepth(pub i32);
impl FolderDepth {
    pub const Shallow: Self = Self(0i32);
    pub const Deep: Self = Self(1i32);
}
impl ::core::marker::Copy for FolderDepth {}
impl ::core::clone::Clone for FolderDepth {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FolderDepth {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FolderDepth {
    type Abi = Self;
}
impl ::core::fmt::Debug for FolderDepth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FolderDepth").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FolderDepth {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Storage.Search.FolderDepth;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentIndexer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentIndexer {
    type Vtable = IContentIndexer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb1767f8d_f698_4982_b05f_3a6e8cab01a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIndexer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AddAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexablecontent: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UpdateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexablecontent: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DeleteMultipleAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentids: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeleteMultipleAsync: usize,
    pub DeleteAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RetrievePropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, propertiestoretrieve: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RetrievePropertiesAsync: usize,
    pub Revision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentIndexerQuery(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentIndexerQuery {
    type Vtable = IContentIndexerQuery_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70e3b0f8_4bfc_428a_8889_cc51da9a7b9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIndexerQuery_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPropertiesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPropertiesRangeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxitems: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPropertiesRangeAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRangeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxitems: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRangeAsync: usize,
    pub QueryFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentIndexerQueryOperations(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentIndexerQueryOperations {
    type Vtable = IContentIndexerQueryOperations_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x28823e10_4786_42f1_9730_792b3566b150);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIndexerQueryOperations_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateQueryWithSortOrderAndLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchfilter: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, propertiestoretrieve: ::windows_core::RawPtr, sortorder: ::windows_core::RawPtr, searchfilterlanguage: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateQueryWithSortOrderAndLanguage: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateQueryWithSortOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchfilter: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, propertiestoretrieve: ::windows_core::RawPtr, sortorder: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateQueryWithSortOrder: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchfilter: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, propertiestoretrieve: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateQuery: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentIndexerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentIndexerStatics {
    type Vtable = IContentIndexerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c488375_b37e_4c60_9ba8_b760fda3e59d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIndexerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetIndexerWithName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetIndexer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IIndexableContent(::windows_core::IUnknown);
impl IIndexableContent {
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
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Stream(&self) -> ::windows_core::Result<super::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Stream)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Streams::IRandomAccessStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStream<'a, Param0: ::windows_core::IntoParam<'a, super::Streams::IRandomAccessStream>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStream)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StreamContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).StreamContentType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetStreamContentType<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStreamContentType)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IIndexableContent> for ::windows_core::IUnknown {
    fn from(value: IIndexableContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IIndexableContent> for ::windows_core::IUnknown {
    fn from(value: &IIndexableContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IIndexableContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IIndexableContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IIndexableContent> for ::windows_core::IInspectable {
    fn from(value: IIndexableContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IIndexableContent> for ::windows_core::IInspectable {
    fn from(value: &IIndexableContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IIndexableContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IIndexableContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IIndexableContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IIndexableContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IIndexableContent {}
impl ::core::fmt::Debug for IIndexableContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIndexableContent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IIndexableContent {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{ccf1a05f-d4b5-483a-b06e-e0db1ec420e4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IIndexableContent {
    type Vtable = IIndexableContent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xccf1a05f_d4b5_483a_b06e_e0db1ec420e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIndexableContent_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Stream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Stream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStream: usize,
    pub StreamContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetStreamContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IQueryOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IQueryOptions {
    type Vtable = IQueryOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1e5e46ee_0f45_4838_a8e9_d0479d446c30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FileTypeFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FileTypeFilter: usize,
    pub FolderDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FolderDepth) -> ::windows_core::HRESULT,
    pub SetFolderDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FolderDepth) -> ::windows_core::HRESULT,
    pub ApplicationSearchFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetApplicationSearchFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UserSearchFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetUserSearchFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IndexerOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IndexerOption) -> ::windows_core::HRESULT,
    pub SetIndexerOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: IndexerOption) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SortOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SortOrder: usize,
    pub GroupPropertyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DateStackOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DateStackOption) -> ::windows_core::HRESULT,
    pub SaveToString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LoadFromString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_FileProperties")]
    pub SetThumbnailPrefetch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: super::FileProperties::ThumbnailMode, requestedsize: u32, options: super::FileProperties::ThumbnailOptions) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    SetThumbnailPrefetch: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_FileProperties"))]
    pub SetPropertyPrefetch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: super::FileProperties::PropertyPrefetchOptions, propertiestoretrieve: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_FileProperties")))]
    SetPropertyPrefetch: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IQueryOptionsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IQueryOptionsFactory {
    type Vtable = IQueryOptionsFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x032e1f8c_a9c1_4e71_8011_0dee9d4811a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryOptionsFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCommonFileQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFileQuery, filetypefilter: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCommonFileQuery: usize,
    pub CreateCommonFolderQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IQueryOptionsWithProviderFilter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IQueryOptionsWithProviderFilter {
    type Vtable = IQueryOptionsWithProviderFilter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b9d1026_15c4_44dd_b89a_47a59b7d7c4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryOptionsWithProviderFilter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub StorageProviderIdFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StorageProviderIdFilter: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageFileQueryResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorageFileQueryResult {
    type Vtable = IStorageFileQueryResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x52fda447_2baa_412c_b29f_d4b1778efa1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFileQueryResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxnumberofitems: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsyncDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsyncDefaultStartAndCount: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageFileQueryResult2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorageFileQueryResult2 {
    type Vtable = IStorageFileQueryResult2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e5db9dd_7141_46c4_8be3_e9dc9e27275c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFileQueryResult2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Data_Text", feature = "Foundation_Collections"))]
    pub GetMatchingPropertiesWithRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Data_Text", feature = "Foundation_Collections")))]
    GetMatchingPropertiesWithRanges: usize,
}
#[repr(transparent)]
pub struct IStorageFolderQueryOperations(::windows_core::IUnknown);
impl IStorageFolderQueryOperations {
    pub fn GetIndexedStateAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<IndexedState>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetIndexedStateAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<IndexedState>>(result__)
        }
    }
    pub fn CreateFileQueryOverloadDefault(&self) -> ::windows_core::Result<StorageFileQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFileQueryOverloadDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StorageFileQueryResult>(result__)
        }
    }
    pub fn CreateFileQuery(&self, query: CommonFileQuery) -> ::windows_core::Result<StorageFileQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFileQuery)(::windows_core::Interface::as_raw(this), query, result__.as_mut_ptr()).from_abi::<StorageFileQueryResult>(result__)
        }
    }
    pub fn CreateFileQueryWithOptions<'a, Param0: ::windows_core::IntoParam<'a, QueryOptions>>(&self, queryoptions: Param0) -> ::windows_core::Result<StorageFileQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFileQueryWithOptions)(::windows_core::Interface::as_raw(this), queryoptions.into_param().abi(), result__.as_mut_ptr()).from_abi::<StorageFileQueryResult>(result__)
        }
    }
    pub fn CreateFolderQueryOverloadDefault(&self) -> ::windows_core::Result<StorageFolderQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFolderQueryOverloadDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StorageFolderQueryResult>(result__)
        }
    }
    pub fn CreateFolderQuery(&self, query: CommonFolderQuery) -> ::windows_core::Result<StorageFolderQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFolderQuery)(::windows_core::Interface::as_raw(this), query, result__.as_mut_ptr()).from_abi::<StorageFolderQueryResult>(result__)
        }
    }
    pub fn CreateFolderQueryWithOptions<'a, Param0: ::windows_core::IntoParam<'a, QueryOptions>>(&self, queryoptions: Param0) -> ::windows_core::Result<StorageFolderQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFolderQueryWithOptions)(::windows_core::Interface::as_raw(this), queryoptions.into_param().abi(), result__.as_mut_ptr()).from_abi::<StorageFolderQueryResult>(result__)
        }
    }
    pub fn CreateItemQuery(&self) -> ::windows_core::Result<StorageItemQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateItemQuery)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StorageItemQueryResult>(result__)
        }
    }
    pub fn CreateItemQueryWithOptions<'a, Param0: ::windows_core::IntoParam<'a, QueryOptions>>(&self, queryoptions: Param0) -> ::windows_core::Result<StorageItemQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateItemQueryWithOptions)(::windows_core::Interface::as_raw(this), queryoptions.into_param().abi(), result__.as_mut_ptr()).from_abi::<StorageItemQueryResult>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsync(&self, query: CommonFileQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFilesAsync)(::windows_core::Interface::as_raw(this), query, startindex, maxitemstoretrieve, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::StorageFile>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsyncOverloadDefaultStartAndCount(&self, query: CommonFileQuery) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFilesAsyncOverloadDefaultStartAndCount)(::windows_core::Interface::as_raw(this), query, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::StorageFile>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsync(&self, query: CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFoldersAsync)(::windows_core::Interface::as_raw(this), query, startindex, maxitemstoretrieve, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::StorageFolder>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsyncOverloadDefaultStartAndCount(&self, query: CommonFolderQuery) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFoldersAsyncOverloadDefaultStartAndCount)(::windows_core::Interface::as_raw(this), query, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::StorageFolder>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::IStorageItem>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetItemsAsync)(::windows_core::Interface::as_raw(this), startindex, maxitemstoretrieve, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::IStorageItem>>>(result__)
        }
    }
    pub fn AreQueryOptionsSupported<'a, Param0: ::windows_core::IntoParam<'a, QueryOptions>>(&self, queryoptions: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AreQueryOptionsSupported)(::windows_core::Interface::as_raw(this), queryoptions.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsCommonFolderQuerySupported(&self, query: CommonFolderQuery) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCommonFolderQuerySupported)(::windows_core::Interface::as_raw(this), query, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsCommonFileQuerySupported(&self, query: CommonFileQuery) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCommonFileQuerySupported)(::windows_core::Interface::as_raw(this), query, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<IStorageFolderQueryOperations> for ::windows_core::IUnknown {
    fn from(value: IStorageFolderQueryOperations) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageFolderQueryOperations> for ::windows_core::IUnknown {
    fn from(value: &IStorageFolderQueryOperations) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IStorageFolderQueryOperations {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IStorageFolderQueryOperations {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStorageFolderQueryOperations> for ::windows_core::IInspectable {
    fn from(value: IStorageFolderQueryOperations) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageFolderQueryOperations> for ::windows_core::IInspectable {
    fn from(value: &IStorageFolderQueryOperations) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IStorageFolderQueryOperations {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IStorageFolderQueryOperations {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStorageFolderQueryOperations {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageFolderQueryOperations {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageFolderQueryOperations {}
impl ::core::fmt::Debug for IStorageFolderQueryOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageFolderQueryOperations").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IStorageFolderQueryOperations {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{cb43ccc9-446b-4a4f-be97-757771be5203}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IStorageFolderQueryOperations {
    type Vtable = IStorageFolderQueryOperations_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb43ccc9_446b_4a4f_be97_757771be5203);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolderQueryOperations_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetIndexedStateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFileQueryOverloadDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFileQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFileQuery, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFileQueryWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryoptions: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFolderQueryOverloadDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFolderQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFolderQueryWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryoptions: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateItemQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateItemQueryWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryoptions: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFileQuery, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsyncOverloadDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFileQuery, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsyncOverloadDefaultStartAndCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsyncOverloadDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsyncOverloadDefaultStartAndCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsAsync: usize,
    pub AreQueryOptionsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryoptions: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsCommonFolderQuerySupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsCommonFileQuerySupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFileQuery, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageFolderQueryResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorageFolderQueryResult {
    type Vtable = IStorageFolderQueryResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6654c911_7d66_46fa_aecf_e4a4baa93ab8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolderQueryResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxnumberofitems: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsyncDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsyncDefaultStartAndCount: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageItemQueryResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorageItemQueryResult {
    type Vtable = IStorageItemQueryResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8948079_9d58_47b8_b2b2_41b07f4795f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemQueryResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxnumberofitems: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsAsyncDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsAsyncDefaultStartAndCount: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageLibraryChangeTrackerTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorageLibraryChangeTrackerTriggerDetails {
    type Vtable = IStorageLibraryChangeTrackerTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dc7a369_b7a3_4df2_9d61_eba85a0343d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryChangeTrackerTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Folder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ChangeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageLibraryContentChangedTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorageLibraryContentChangedTriggerDetails {
    type Vtable = IStorageLibraryContentChangedTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a371977_abbf_4e1d_8aa5_6385d8884799);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryContentChangedTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Folder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateModifiedSinceQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastquerytime: ::winrt_foundation::DateTime, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IStorageQueryResultBase(::windows_core::IUnknown);
impl IStorageQueryResultBase {
    pub fn GetItemCountAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetItemCountAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<u32>>(result__)
        }
    }
    pub fn Folder(&self) -> ::windows_core::Result<super::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Folder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::StorageFolder>(result__)
        }
    }
    pub fn ContentsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IStorageQueryResultBase, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ContentsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveContentsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveContentsChanged)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    pub fn OptionsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IStorageQueryResultBase, ::windows_core::IInspectable>>>(&self, changedhandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).OptionsChanged)(::windows_core::Interface::as_raw(this), changedhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveOptionsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOptionsChanged)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    pub fn FindStartIndexAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindStartIndexAsync)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<u32>>(result__)
        }
    }
    pub fn GetCurrentQueryOptions(&self) -> ::windows_core::Result<QueryOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentQueryOptions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<QueryOptions>(result__)
        }
    }
    pub fn ApplyNewQueryOptions<'a, Param0: ::windows_core::IntoParam<'a, QueryOptions>>(&self, newqueryoptions: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ApplyNewQueryOptions)(::windows_core::Interface::as_raw(this), newqueryoptions.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IStorageQueryResultBase> for ::windows_core::IUnknown {
    fn from(value: IStorageQueryResultBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageQueryResultBase> for ::windows_core::IUnknown {
    fn from(value: &IStorageQueryResultBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IStorageQueryResultBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IStorageQueryResultBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStorageQueryResultBase> for ::windows_core::IInspectable {
    fn from(value: IStorageQueryResultBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageQueryResultBase> for ::windows_core::IInspectable {
    fn from(value: &IStorageQueryResultBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IStorageQueryResultBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IStorageQueryResultBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStorageQueryResultBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageQueryResultBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageQueryResultBase {}
impl ::core::fmt::Debug for IStorageQueryResultBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageQueryResultBase").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IStorageQueryResultBase {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{c297d70d-7353-47ab-ba58-8c61425dc54b}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IStorageQueryResultBase {
    type Vtable = IStorageQueryResultBase_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc297d70d_7353_47ab_ba58_8c61425dc54b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageQueryResultBase_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetItemCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Folder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContentsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveContentsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub OptionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changedhandler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveOptionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub FindStartIndexAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetCurrentQueryOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ApplyNewQueryOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newqueryoptions: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IValueAndLanguage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IValueAndLanguage {
    type Vtable = IValueAndLanguage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9914881_a1ee_4bc4_92a5_466968e30436);
}
#[repr(C)]
#[doc(hidden)]
pub struct IValueAndLanguage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IndexableContent(::windows_core::IUnknown);
impl IndexableContent {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<IndexableContent, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
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
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Stream(&self) -> ::windows_core::Result<super::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Stream)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Streams::IRandomAccessStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStream<'a, Param0: ::windows_core::IntoParam<'a, super::Streams::IRandomAccessStream>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStream)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StreamContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).StreamContentType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetStreamContentType<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStreamContentType)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for IndexableContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IndexableContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IndexableContent {}
impl ::core::fmt::Debug for IndexableContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IndexableContent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IndexableContent {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.IndexableContent;{ccf1a05f-d4b5-483a-b06e-e0db1ec420e4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IndexableContent {
    type Vtable = IIndexableContent_Vtbl;
    const IID: ::windows_core::GUID = <IIndexableContent as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IndexableContent {
    const NAME: &'static str = "Windows.Storage.Search.IndexableContent";
}
impl ::core::convert::From<IndexableContent> for ::windows_core::IUnknown {
    fn from(value: IndexableContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IndexableContent> for ::windows_core::IUnknown {
    fn from(value: &IndexableContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IndexableContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IndexableContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IndexableContent> for ::windows_core::IInspectable {
    fn from(value: IndexableContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IndexableContent> for ::windows_core::IInspectable {
    fn from(value: &IndexableContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IndexableContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IndexableContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IndexableContent> for IIndexableContent {
    type Error = ::windows_core::Error;
    fn try_from(value: IndexableContent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IndexableContent> for IIndexableContent {
    type Error = ::windows_core::Error;
    fn try_from(value: &IndexableContent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IIndexableContent> for IndexableContent {
    fn into_param(self) -> ::windows_core::Param<'a, IIndexableContent> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IIndexableContent> for &IndexableContent {
    fn into_param(self) -> ::windows_core::Param<'a, IIndexableContent> {
        ::core::convert::TryInto::<IIndexableContent>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for IndexableContent {}
unsafe impl ::core::marker::Sync for IndexableContent {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IndexedState(pub i32);
impl IndexedState {
    pub const Unknown: Self = Self(0i32);
    pub const NotIndexed: Self = Self(1i32);
    pub const PartiallyIndexed: Self = Self(2i32);
    pub const FullyIndexed: Self = Self(3i32);
}
impl ::core::marker::Copy for IndexedState {}
impl ::core::clone::Clone for IndexedState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IndexedState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IndexedState {
    type Abi = Self;
}
impl ::core::fmt::Debug for IndexedState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IndexedState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IndexedState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Storage.Search.IndexedState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IndexerOption(pub i32);
impl IndexerOption {
    pub const UseIndexerWhenAvailable: Self = Self(0i32);
    pub const OnlyUseIndexer: Self = Self(1i32);
    pub const DoNotUseIndexer: Self = Self(2i32);
    pub const OnlyUseIndexerAndOptimizeForIndexedProperties: Self = Self(3i32);
}
impl ::core::marker::Copy for IndexerOption {}
impl ::core::clone::Clone for IndexerOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IndexerOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IndexerOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for IndexerOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IndexerOption").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IndexerOption {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Storage.Search.IndexerOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct QueryOptions(::windows_core::IUnknown);
impl QueryOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<QueryOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FileTypeFilter(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FileTypeFilter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn FolderDepth(&self) -> ::windows_core::Result<FolderDepth> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<FolderDepth>::zeroed();
            (::windows_core::Interface::vtable(this).FolderDepth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FolderDepth>(result__)
        }
    }
    pub fn SetFolderDepth(&self, value: FolderDepth) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFolderDepth)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ApplicationSearchFilter(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ApplicationSearchFilter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetApplicationSearchFilter<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetApplicationSearchFilter)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn UserSearchFilter(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).UserSearchFilter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetUserSearchFilter<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUserSearchFilter)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IndexerOption(&self) -> ::windows_core::Result<IndexerOption> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<IndexerOption>::zeroed();
            (::windows_core::Interface::vtable(this).IndexerOption)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IndexerOption>(result__)
        }
    }
    pub fn SetIndexerOption(&self, value: IndexerOption) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIndexerOption)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SortOrder(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SortEntry>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SortOrder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SortEntry>>(result__)
        }
    }
    pub fn GroupPropertyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GroupPropertyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DateStackOption(&self) -> ::windows_core::Result<DateStackOption> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DateStackOption>::zeroed();
            (::windows_core::Interface::vtable(this).DateStackOption)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DateStackOption>(result__)
        }
    }
    pub fn SaveToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SaveToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn LoadFromString<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LoadFromString)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn SetThumbnailPrefetch(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32, options: super::FileProperties::ThumbnailOptions) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetThumbnailPrefetch)(::windows_core::Interface::as_raw(this), mode, requestedsize, options).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_FileProperties"))]
    pub fn SetPropertyPrefetch<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, options: super::FileProperties::PropertyPrefetchOptions, propertiestoretrieve: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPropertyPrefetch)(::windows_core::Interface::as_raw(this), options, propertiestoretrieve.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCommonFileQuery<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(query: CommonFileQuery, filetypefilter: Param1) -> ::windows_core::Result<QueryOptions> {
        Self::IQueryOptionsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateCommonFileQuery)(::windows_core::Interface::as_raw(this), query, filetypefilter.into_param().abi(), result__.as_mut_ptr()).from_abi::<QueryOptions>(result__)
        })
    }
    pub fn CreateCommonFolderQuery(query: CommonFolderQuery) -> ::windows_core::Result<QueryOptions> {
        Self::IQueryOptionsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateCommonFolderQuery)(::windows_core::Interface::as_raw(this), query, result__.as_mut_ptr()).from_abi::<QueryOptions>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn StorageProviderIdFilter(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<IQueryOptionsWithProviderFilter>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StorageProviderIdFilter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn IQueryOptionsFactory<R, F: FnOnce(&IQueryOptionsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<QueryOptions, IQueryOptionsFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for QueryOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for QueryOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for QueryOptions {}
impl ::core::fmt::Debug for QueryOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QueryOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for QueryOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.QueryOptions;{1e5e46ee-0f45-4838-a8e9-d0479d446c30})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for QueryOptions {
    type Vtable = IQueryOptions_Vtbl;
    const IID: ::windows_core::GUID = <IQueryOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for QueryOptions {
    const NAME: &'static str = "Windows.Storage.Search.QueryOptions";
}
impl ::core::convert::From<QueryOptions> for ::windows_core::IUnknown {
    fn from(value: QueryOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&QueryOptions> for ::windows_core::IUnknown {
    fn from(value: &QueryOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for QueryOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a QueryOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<QueryOptions> for ::windows_core::IInspectable {
    fn from(value: QueryOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&QueryOptions> for ::windows_core::IInspectable {
    fn from(value: &QueryOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for QueryOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a QueryOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for QueryOptions {}
unsafe impl ::core::marker::Sync for QueryOptions {}
#[repr(C)]
pub struct SortEntry {
    pub PropertyName: ::windows_core::HSTRING,
    pub AscendingOrder: bool,
}
impl ::core::clone::Clone for SortEntry {
    fn clone(&self) -> Self {
        Self { PropertyName: self.PropertyName.clone(), AscendingOrder: self.AscendingOrder }
    }
}
impl ::core::fmt::Debug for SortEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SortEntry").field("PropertyName", &self.PropertyName).field("AscendingOrder", &self.AscendingOrder).finish()
    }
}
unsafe impl ::windows_core::Abi for SortEntry {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
unsafe impl ::windows_core::RuntimeType for SortEntry {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Storage.Search.SortEntry;string;b1)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(from.clone())
    }
}
impl ::core::cmp::PartialEq for SortEntry {
    fn eq(&self, other: &Self) -> bool {
        self.PropertyName == other.PropertyName && self.AscendingOrder == other.AscendingOrder
    }
}
impl ::core::cmp::Eq for SortEntry {}
impl ::core::default::Default for SortEntry {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct SortEntryVector(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl SortEntryVector {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<SortEntry>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<SortEntry>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<SortEntry>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<SortEntry> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<SortEntry>>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<SortEntry>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<SortEntry>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<SortEntry>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, SortEntry>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, SortEntry>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, SortEntry>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, SortEntry>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [SortEntry]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[SortEntry]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for SortEntryVector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for SortEntryVector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for SortEntryVector {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for SortEntryVector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SortEntryVector").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::RuntimeType for SortEntryVector {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.SortEntryVector;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};struct(Windows.Storage.Search.SortEntry;string;b1)))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for SortEntryVector {
    type Vtable = ::winrt_foundation::Collections::IVector_Vtbl<SortEntry>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVector<SortEntry> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for SortEntryVector {
    const NAME: &'static str = "Windows.Storage.Search.SortEntryVector";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for SortEntryVector {
    type Item = SortEntry;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &SortEntryVector {
    type Item = SortEntry;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<SortEntryVector> for ::windows_core::IUnknown {
    fn from(value: SortEntryVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&SortEntryVector> for ::windows_core::IUnknown {
    fn from(value: &SortEntryVector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SortEntryVector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SortEntryVector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<SortEntryVector> for ::windows_core::IInspectable {
    fn from(value: SortEntryVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&SortEntryVector> for ::windows_core::IInspectable {
    fn from(value: &SortEntryVector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SortEntryVector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SortEntryVector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<SortEntryVector> for ::winrt_foundation::Collections::IIterable<SortEntry> {
    type Error = ::windows_core::Error;
    fn try_from(value: SortEntryVector) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&SortEntryVector> for ::winrt_foundation::Collections::IIterable<SortEntry> {
    type Error = ::windows_core::Error;
    fn try_from(value: &SortEntryVector) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<SortEntry>> for SortEntryVector {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<SortEntry>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<SortEntry>> for &SortEntryVector {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<SortEntry>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<SortEntry>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<SortEntryVector> for ::winrt_foundation::Collections::IVector<SortEntry> {
    type Error = ::windows_core::Error;
    fn try_from(value: SortEntryVector) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&SortEntryVector> for ::winrt_foundation::Collections::IVector<SortEntry> {
    type Error = ::windows_core::Error;
    fn try_from(value: &SortEntryVector) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<SortEntry>> for SortEntryVector {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<SortEntry>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<SortEntry>> for &SortEntryVector {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<SortEntry>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<SortEntry>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
pub struct StorageFileQueryResult(::windows_core::IUnknown);
impl StorageFileQueryResult {
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFilesAsync)(::windows_core::Interface::as_raw(this), startindex, maxnumberofitems, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::StorageFile>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsyncDefaultStartAndCount(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFilesAsyncDefaultStartAndCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::StorageFile>>>(result__)
        }
    }
    #[cfg(all(feature = "Data_Text", feature = "Foundation_Collections"))]
    pub fn GetMatchingPropertiesWithRanges<'a, Param0: ::windows_core::IntoParam<'a, super::StorageFile>>(&self, file: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::winrt_foundation::Collections::IVectorView<::winrt_data::Text::TextSegment>>> {
        let this = &::windows_core::Interface::cast::<IStorageFileQueryResult2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetMatchingPropertiesWithRanges)(::windows_core::Interface::as_raw(this), file.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::winrt_foundation::Collections::IVectorView<::winrt_data::Text::TextSegment>>>(result__)
        }
    }
    pub fn GetItemCountAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<u32>> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetItemCountAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<u32>>(result__)
        }
    }
    pub fn Folder(&self) -> ::windows_core::Result<super::StorageFolder> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Folder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::StorageFolder>(result__)
        }
    }
    pub fn ContentsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IStorageQueryResultBase, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ContentsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveContentsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveContentsChanged)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    pub fn OptionsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IStorageQueryResultBase, ::windows_core::IInspectable>>>(&self, changedhandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).OptionsChanged)(::windows_core::Interface::as_raw(this), changedhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveOptionsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOptionsChanged)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    pub fn FindStartIndexAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<u32>> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindStartIndexAsync)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<u32>>(result__)
        }
    }
    pub fn GetCurrentQueryOptions(&self) -> ::windows_core::Result<QueryOptions> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentQueryOptions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<QueryOptions>(result__)
        }
    }
    pub fn ApplyNewQueryOptions<'a, Param0: ::windows_core::IntoParam<'a, QueryOptions>>(&self, newqueryoptions: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ApplyNewQueryOptions)(::windows_core::Interface::as_raw(this), newqueryoptions.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for StorageFileQueryResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageFileQueryResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageFileQueryResult {}
impl ::core::fmt::Debug for StorageFileQueryResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageFileQueryResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StorageFileQueryResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.StorageFileQueryResult;{52fda447-2baa-412c-b29f-d4b1778efa1e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StorageFileQueryResult {
    type Vtable = IStorageFileQueryResult_Vtbl;
    const IID: ::windows_core::GUID = <IStorageFileQueryResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StorageFileQueryResult {
    const NAME: &'static str = "Windows.Storage.Search.StorageFileQueryResult";
}
impl ::core::convert::From<StorageFileQueryResult> for ::windows_core::IUnknown {
    fn from(value: StorageFileQueryResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageFileQueryResult> for ::windows_core::IUnknown {
    fn from(value: &StorageFileQueryResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StorageFileQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StorageFileQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageFileQueryResult> for ::windows_core::IInspectable {
    fn from(value: StorageFileQueryResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageFileQueryResult> for ::windows_core::IInspectable {
    fn from(value: &StorageFileQueryResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StorageFileQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StorageFileQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<StorageFileQueryResult> for IStorageQueryResultBase {
    type Error = ::windows_core::Error;
    fn try_from(value: StorageFileQueryResult) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageFileQueryResult> for IStorageQueryResultBase {
    type Error = ::windows_core::Error;
    fn try_from(value: &StorageFileQueryResult) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IStorageQueryResultBase> for StorageFileQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, IStorageQueryResultBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IStorageQueryResultBase> for &StorageFileQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, IStorageQueryResultBase> {
        ::core::convert::TryInto::<IStorageQueryResultBase>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
pub struct StorageFolderQueryResult(::windows_core::IUnknown);
impl StorageFolderQueryResult {
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFoldersAsync)(::windows_core::Interface::as_raw(this), startindex, maxnumberofitems, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::StorageFolder>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsyncDefaultStartAndCount(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFoldersAsyncDefaultStartAndCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::StorageFolder>>>(result__)
        }
    }
    pub fn GetItemCountAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<u32>> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetItemCountAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<u32>>(result__)
        }
    }
    pub fn Folder(&self) -> ::windows_core::Result<super::StorageFolder> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Folder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::StorageFolder>(result__)
        }
    }
    pub fn ContentsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IStorageQueryResultBase, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ContentsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveContentsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveContentsChanged)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    pub fn OptionsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IStorageQueryResultBase, ::windows_core::IInspectable>>>(&self, changedhandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).OptionsChanged)(::windows_core::Interface::as_raw(this), changedhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveOptionsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOptionsChanged)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    pub fn FindStartIndexAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<u32>> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindStartIndexAsync)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<u32>>(result__)
        }
    }
    pub fn GetCurrentQueryOptions(&self) -> ::windows_core::Result<QueryOptions> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentQueryOptions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<QueryOptions>(result__)
        }
    }
    pub fn ApplyNewQueryOptions<'a, Param0: ::windows_core::IntoParam<'a, QueryOptions>>(&self, newqueryoptions: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ApplyNewQueryOptions)(::windows_core::Interface::as_raw(this), newqueryoptions.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for StorageFolderQueryResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageFolderQueryResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageFolderQueryResult {}
impl ::core::fmt::Debug for StorageFolderQueryResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageFolderQueryResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StorageFolderQueryResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.StorageFolderQueryResult;{6654c911-7d66-46fa-aecf-e4a4baa93ab8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StorageFolderQueryResult {
    type Vtable = IStorageFolderQueryResult_Vtbl;
    const IID: ::windows_core::GUID = <IStorageFolderQueryResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StorageFolderQueryResult {
    const NAME: &'static str = "Windows.Storage.Search.StorageFolderQueryResult";
}
impl ::core::convert::From<StorageFolderQueryResult> for ::windows_core::IUnknown {
    fn from(value: StorageFolderQueryResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageFolderQueryResult> for ::windows_core::IUnknown {
    fn from(value: &StorageFolderQueryResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StorageFolderQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StorageFolderQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageFolderQueryResult> for ::windows_core::IInspectable {
    fn from(value: StorageFolderQueryResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageFolderQueryResult> for ::windows_core::IInspectable {
    fn from(value: &StorageFolderQueryResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StorageFolderQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StorageFolderQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<StorageFolderQueryResult> for IStorageQueryResultBase {
    type Error = ::windows_core::Error;
    fn try_from(value: StorageFolderQueryResult) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageFolderQueryResult> for IStorageQueryResultBase {
    type Error = ::windows_core::Error;
    fn try_from(value: &StorageFolderQueryResult) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IStorageQueryResultBase> for StorageFolderQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, IStorageQueryResultBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IStorageQueryResultBase> for &StorageFolderQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, IStorageQueryResultBase> {
        ::core::convert::TryInto::<IStorageQueryResultBase>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
pub struct StorageItemQueryResult(::windows_core::IUnknown);
impl StorageItemQueryResult {
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemsAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::IStorageItem>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetItemsAsync)(::windows_core::Interface::as_raw(this), startindex, maxnumberofitems, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::IStorageItem>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemsAsyncDefaultStartAndCount(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::IStorageItem>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetItemsAsyncDefaultStartAndCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<super::IStorageItem>>>(result__)
        }
    }
    pub fn GetItemCountAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<u32>> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetItemCountAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<u32>>(result__)
        }
    }
    pub fn Folder(&self) -> ::windows_core::Result<super::StorageFolder> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Folder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::StorageFolder>(result__)
        }
    }
    pub fn ContentsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IStorageQueryResultBase, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ContentsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveContentsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveContentsChanged)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    pub fn OptionsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IStorageQueryResultBase, ::windows_core::IInspectable>>>(&self, changedhandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).OptionsChanged)(::windows_core::Interface::as_raw(this), changedhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveOptionsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOptionsChanged)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    pub fn FindStartIndexAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<u32>> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindStartIndexAsync)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<u32>>(result__)
        }
    }
    pub fn GetCurrentQueryOptions(&self) -> ::windows_core::Result<QueryOptions> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentQueryOptions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<QueryOptions>(result__)
        }
    }
    pub fn ApplyNewQueryOptions<'a, Param0: ::windows_core::IntoParam<'a, QueryOptions>>(&self, newqueryoptions: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ApplyNewQueryOptions)(::windows_core::Interface::as_raw(this), newqueryoptions.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for StorageItemQueryResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageItemQueryResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageItemQueryResult {}
impl ::core::fmt::Debug for StorageItemQueryResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageItemQueryResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StorageItemQueryResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.StorageItemQueryResult;{e8948079-9d58-47b8-b2b2-41b07f4795f9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StorageItemQueryResult {
    type Vtable = IStorageItemQueryResult_Vtbl;
    const IID: ::windows_core::GUID = <IStorageItemQueryResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StorageItemQueryResult {
    const NAME: &'static str = "Windows.Storage.Search.StorageItemQueryResult";
}
impl ::core::convert::From<StorageItemQueryResult> for ::windows_core::IUnknown {
    fn from(value: StorageItemQueryResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageItemQueryResult> for ::windows_core::IUnknown {
    fn from(value: &StorageItemQueryResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StorageItemQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StorageItemQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageItemQueryResult> for ::windows_core::IInspectable {
    fn from(value: StorageItemQueryResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageItemQueryResult> for ::windows_core::IInspectable {
    fn from(value: &StorageItemQueryResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StorageItemQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StorageItemQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<StorageItemQueryResult> for IStorageQueryResultBase {
    type Error = ::windows_core::Error;
    fn try_from(value: StorageItemQueryResult) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageItemQueryResult> for IStorageQueryResultBase {
    type Error = ::windows_core::Error;
    fn try_from(value: &StorageItemQueryResult) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IStorageQueryResultBase> for StorageItemQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, IStorageQueryResultBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IStorageQueryResultBase> for &StorageItemQueryResult {
    fn into_param(self) -> ::windows_core::Param<'a, IStorageQueryResultBase> {
        ::core::convert::TryInto::<IStorageQueryResultBase>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
pub struct StorageLibraryChangeTrackerTriggerDetails(::windows_core::IUnknown);
impl StorageLibraryChangeTrackerTriggerDetails {
    pub fn Folder(&self) -> ::windows_core::Result<super::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Folder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::StorageFolder>(result__)
        }
    }
    pub fn ChangeTracker(&self) -> ::windows_core::Result<super::StorageLibraryChangeTracker> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChangeTracker)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::StorageLibraryChangeTracker>(result__)
        }
    }
}
impl ::core::clone::Clone for StorageLibraryChangeTrackerTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageLibraryChangeTrackerTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageLibraryChangeTrackerTriggerDetails {}
impl ::core::fmt::Debug for StorageLibraryChangeTrackerTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageLibraryChangeTrackerTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StorageLibraryChangeTrackerTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.StorageLibraryChangeTrackerTriggerDetails;{1dc7a369-b7a3-4df2-9d61-eba85a0343d2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StorageLibraryChangeTrackerTriggerDetails {
    type Vtable = IStorageLibraryChangeTrackerTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IStorageLibraryChangeTrackerTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StorageLibraryChangeTrackerTriggerDetails {
    const NAME: &'static str = "Windows.Storage.Search.StorageLibraryChangeTrackerTriggerDetails";
}
impl ::core::convert::From<StorageLibraryChangeTrackerTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: StorageLibraryChangeTrackerTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageLibraryChangeTrackerTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &StorageLibraryChangeTrackerTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StorageLibraryChangeTrackerTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StorageLibraryChangeTrackerTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageLibraryChangeTrackerTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: StorageLibraryChangeTrackerTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageLibraryChangeTrackerTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &StorageLibraryChangeTrackerTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StorageLibraryChangeTrackerTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StorageLibraryChangeTrackerTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct StorageLibraryContentChangedTriggerDetails(::windows_core::IUnknown);
impl StorageLibraryContentChangedTriggerDetails {
    pub fn Folder(&self) -> ::windows_core::Result<super::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Folder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::StorageFolder>(result__)
        }
    }
    pub fn CreateModifiedSinceQuery<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, lastquerytime: Param0) -> ::windows_core::Result<StorageItemQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateModifiedSinceQuery)(::windows_core::Interface::as_raw(this), lastquerytime.into_param().abi(), result__.as_mut_ptr()).from_abi::<StorageItemQueryResult>(result__)
        }
    }
}
impl ::core::clone::Clone for StorageLibraryContentChangedTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageLibraryContentChangedTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageLibraryContentChangedTriggerDetails {}
impl ::core::fmt::Debug for StorageLibraryContentChangedTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageLibraryContentChangedTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StorageLibraryContentChangedTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.StorageLibraryContentChangedTriggerDetails;{2a371977-abbf-4e1d-8aa5-6385d8884799})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StorageLibraryContentChangedTriggerDetails {
    type Vtable = IStorageLibraryContentChangedTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IStorageLibraryContentChangedTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StorageLibraryContentChangedTriggerDetails {
    const NAME: &'static str = "Windows.Storage.Search.StorageLibraryContentChangedTriggerDetails";
}
impl ::core::convert::From<StorageLibraryContentChangedTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: StorageLibraryContentChangedTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageLibraryContentChangedTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &StorageLibraryContentChangedTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StorageLibraryContentChangedTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StorageLibraryContentChangedTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageLibraryContentChangedTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: StorageLibraryContentChangedTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageLibraryContentChangedTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &StorageLibraryContentChangedTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StorageLibraryContentChangedTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StorageLibraryContentChangedTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct ValueAndLanguage(::windows_core::IUnknown);
impl ValueAndLanguage {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ValueAndLanguage, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for ValueAndLanguage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ValueAndLanguage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ValueAndLanguage {}
impl ::core::fmt::Debug for ValueAndLanguage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ValueAndLanguage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ValueAndLanguage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.ValueAndLanguage;{b9914881-a1ee-4bc4-92a5-466968e30436})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ValueAndLanguage {
    type Vtable = IValueAndLanguage_Vtbl;
    const IID: ::windows_core::GUID = <IValueAndLanguage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ValueAndLanguage {
    const NAME: &'static str = "Windows.Storage.Search.ValueAndLanguage";
}
impl ::core::convert::From<ValueAndLanguage> for ::windows_core::IUnknown {
    fn from(value: ValueAndLanguage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ValueAndLanguage> for ::windows_core::IUnknown {
    fn from(value: &ValueAndLanguage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ValueAndLanguage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ValueAndLanguage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ValueAndLanguage> for ::windows_core::IInspectable {
    fn from(value: ValueAndLanguage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ValueAndLanguage> for ::windows_core::IInspectable {
    fn from(value: &ValueAndLanguage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ValueAndLanguage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ValueAndLanguage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ValueAndLanguage {}
unsafe impl ::core::marker::Sync for ValueAndLanguage {}
