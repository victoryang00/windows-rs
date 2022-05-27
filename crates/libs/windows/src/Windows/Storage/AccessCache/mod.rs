#[doc = "*Required features: `\"Storage_AccessCache\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AccessCacheOptions(pub u32);
impl AccessCacheOptions {
    pub const None: Self = Self(0u32);
    pub const DisallowUserInput: Self = Self(1u32);
    pub const FastLocationsOnly: Self = Self(2u32);
    pub const UseReadOnlyCachedCopy: Self = Self(4u32);
    pub const SuppressAccessTimeUpdate: Self = Self(8u32);
}
impl ::core::marker::Copy for AccessCacheOptions {}
impl ::core::clone::Clone for AccessCacheOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AccessCacheOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AccessCacheOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for AccessCacheOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccessCacheOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AccessCacheOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AccessCacheOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AccessCacheOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AccessCacheOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AccessCacheOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for AccessCacheOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Storage.AccessCache.AccessCacheOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Storage_AccessCache\"`*"]
pub struct AccessListEntry {
    pub Token: ::windows_core::HSTRING,
    pub Metadata: ::windows_core::HSTRING,
}
impl ::core::clone::Clone for AccessListEntry {
    fn clone(&self) -> Self {
        Self { Token: self.Token.clone(), Metadata: self.Metadata.clone() }
    }
}
impl ::core::fmt::Debug for AccessListEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AccessListEntry").field("Token", &self.Token).field("Metadata", &self.Metadata).finish()
    }
}
unsafe impl ::windows_core::Abi for AccessListEntry {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
unsafe impl ::windows_core::RuntimeType for AccessListEntry {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Storage.AccessCache.AccessListEntry;string;string)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(from.clone())
    }
}
impl ::core::cmp::PartialEq for AccessListEntry {
    fn eq(&self, other: &Self) -> bool {
        self.Token == other.Token && self.Metadata == other.Metadata
    }
}
impl ::core::cmp::Eq for AccessListEntry {}
impl ::core::default::Default for AccessListEntry {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct AccessListEntryView(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl AccessListEntryView {
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IIterator<AccessListEntry>> {
        let this = &::windows_core::Interface::cast::<super::super::Foundation::Collections::IIterable<AccessListEntry>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IIterator<AccessListEntry>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<AccessListEntry> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<AccessListEntry>>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<AccessListEntry>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, AccessListEntry>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [AccessListEntry]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for AccessListEntryView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for AccessListEntryView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for AccessListEntryView {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for AccessListEntryView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccessListEntryView").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::RuntimeType for AccessListEntryView {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Storage.AccessCache.AccessListEntryView;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};struct(Windows.Storage.AccessCache.AccessListEntry;string;string)))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for AccessListEntryView {
    type Vtable = super::super::Foundation::Collections::IVectorView_Vtbl<AccessListEntry>;
    const IID: ::windows_core::GUID = <super::super::Foundation::Collections::IVectorView<AccessListEntry> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for AccessListEntryView {
    const NAME: &'static str = "Windows.Storage.AccessCache.AccessListEntryView";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for AccessListEntryView {
    type Item = AccessListEntry;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &AccessListEntryView {
    type Item = AccessListEntry;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<AccessListEntryView> for ::windows_core::IUnknown {
    fn from(value: AccessListEntryView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&AccessListEntryView> for ::windows_core::IUnknown {
    fn from(value: &AccessListEntryView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AccessListEntryView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AccessListEntryView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<AccessListEntryView> for ::windows_core::IInspectable {
    fn from(value: AccessListEntryView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&AccessListEntryView> for ::windows_core::IInspectable {
    fn from(value: &AccessListEntryView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AccessListEntryView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AccessListEntryView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<AccessListEntryView> for super::super::Foundation::Collections::IIterable<AccessListEntry> {
    type Error = ::windows_core::Error;
    fn try_from(value: AccessListEntryView) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&AccessListEntryView> for super::super::Foundation::Collections::IIterable<AccessListEntry> {
    type Error = ::windows_core::Error;
    fn try_from(value: &AccessListEntryView) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<AccessListEntry>> for AccessListEntryView {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::Collections::IIterable<AccessListEntry>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IIterable<AccessListEntry>> for &AccessListEntryView {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::Collections::IIterable<AccessListEntry>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IIterable<AccessListEntry>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<AccessListEntryView> for super::super::Foundation::Collections::IVectorView<AccessListEntry> {
    type Error = ::windows_core::Error;
    fn try_from(value: AccessListEntryView) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&AccessListEntryView> for super::super::Foundation::Collections::IVectorView<AccessListEntry> {
    type Error = ::windows_core::Error;
    fn try_from(value: &AccessListEntryView) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<AccessListEntry>> for AccessListEntryView {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::Collections::IVectorView<AccessListEntry>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<AccessListEntry>> for &AccessListEntryView {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::Collections::IVectorView<AccessListEntry>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IVectorView<AccessListEntry>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IItemRemovedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IItemRemovedEventArgs {
    type Vtable = IItemRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x59677e5c_55be_4c66_ba66_5eaea79d2631);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemRemovedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RemovedEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<AccessListEntry>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageApplicationPermissionsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorageApplicationPermissionsStatics {
    type Vtable = IStorageApplicationPermissionsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4391dfaa_d033_48f9_8060_3ec847d2e3f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageApplicationPermissionsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FutureAccessList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MostRecentlyUsedList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageApplicationPermissionsStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorageApplicationPermissionsStatics2 {
    type Vtable = IStorageApplicationPermissionsStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x072716ec_aa05_4294_9a11_1a3d04519ad0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageApplicationPermissionsStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub GetFutureAccessListForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetFutureAccessListForUser: usize,
    #[cfg(feature = "System")]
    pub GetMostRecentlyUsedListForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetMostRecentlyUsedListForUser: usize,
}
#[doc = "*Required features: `\"Storage_AccessCache\"`*"]
#[repr(transparent)]
pub struct IStorageItemAccessList(::windows_core::IUnknown);
impl IStorageItemAccessList {
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn AddOverloadDefaultMetadata<'a, Param0: ::windows_core::IntoParam<'a, super::IStorageItem>>(&self, file: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AddOverloadDefaultMetadata)(::windows_core::Interface::as_raw(this), file.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn Add<'a, Param0: ::windows_core::IntoParam<'a, super::IStorageItem>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, file: Param0, metadata: Param1) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Add)(::windows_core::Interface::as_raw(this), file.into_param().abi(), metadata.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn AddOrReplaceOverloadDefaultMetadata<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::IStorageItem>>(&self, token: Param0, file: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddOrReplaceOverloadDefaultMetadata)(::windows_core::Interface::as_raw(this), token.into_param().abi(), file.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn AddOrReplace<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::IStorageItem>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0, file: Param1, metadata: Param2) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddOrReplace)(::windows_core::Interface::as_raw(this), token.into_param().abi(), file.into_param().abi(), metadata.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetItemAsync)(::windows_core::Interface::as_raw(this), token.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::IStorageItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFileAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFileAsync)(::windows_core::Interface::as_raw(this), token.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFolderAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFolderAsync)(::windows_core::Interface::as_raw(this), token.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0, options: AccessCacheOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetItemWithOptionsAsync)(::windows_core::Interface::as_raw(this), token.into_param().abi(), options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::IStorageItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFileWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0, options: AccessCacheOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFileWithOptionsAsync)(::windows_core::Interface::as_raw(this), token.into_param().abi(), options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFolderWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0, options: AccessCacheOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFolderWithOptionsAsync)(::windows_core::Interface::as_raw(this), token.into_param().abi(), options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn ContainsItem<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ContainsItem)(::windows_core::Interface::as_raw(this), token.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn CheckAccess<'a, Param0: ::windows_core::IntoParam<'a, super::IStorageItem>>(&self, file: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CheckAccess)(::windows_core::Interface::as_raw(this), file.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Entries(&self) -> ::windows_core::Result<AccessListEntryView> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Entries)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccessListEntryView>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn MaximumItemsAllowed(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaximumItemsAllowed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::convert::From<IStorageItemAccessList> for ::windows_core::IUnknown {
    fn from(value: IStorageItemAccessList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageItemAccessList> for ::windows_core::IUnknown {
    fn from(value: &IStorageItemAccessList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IStorageItemAccessList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IStorageItemAccessList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStorageItemAccessList> for ::windows_core::IInspectable {
    fn from(value: IStorageItemAccessList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageItemAccessList> for ::windows_core::IInspectable {
    fn from(value: &IStorageItemAccessList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IStorageItemAccessList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IStorageItemAccessList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStorageItemAccessList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageItemAccessList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageItemAccessList {}
impl ::core::fmt::Debug for IStorageItemAccessList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageItemAccessList").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IStorageItemAccessList {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{2caff6ad-de90-47f5-b2c3-dd36c9fdd453}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IStorageItemAccessList {
    type Vtable = IStorageItemAccessList_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2caff6ad_de90_47f5_b2c3_dd36c9fdd453);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemAccessList_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AddOverloadDefaultMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows_core::RawPtr, metadata: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AddOrReplaceOverloadDefaultMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, file: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AddOrReplace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, file: ::windows_core::RawPtr, metadata: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetItemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetItemAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetItemWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, options: AccessCacheOptions, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetItemWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFileWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, options: AccessCacheOptions, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFileWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFolderWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, options: AccessCacheOptions, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFolderWithOptionsAsync: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ContainsItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CheckAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Entries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Entries: usize,
    pub MaximumItemsAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageItemMostRecentlyUsedList(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorageItemMostRecentlyUsedList {
    type Vtable = IStorageItemMostRecentlyUsedList_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x016239d5_510d_411e_8cf1_c3d1effa4c33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemMostRecentlyUsedList_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ItemRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemRemoved: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageItemMostRecentlyUsedList2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorageItemMostRecentlyUsedList2 {
    type Vtable = IStorageItemMostRecentlyUsedList2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda481ea0_ed8d_4731_a1db_e44ee2204093);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemMostRecentlyUsedList2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AddWithMetadataAndVisibility: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows_core::RawPtr, metadata: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, visibility: RecentStorageItemVisibility, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AddOrReplaceWithMetadataAndVisibility: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, file: ::windows_core::RawPtr, metadata: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, visibility: RecentStorageItemVisibility) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Storage_AccessCache\"`*"]
#[repr(transparent)]
pub struct ItemRemovedEventArgs(::windows_core::IUnknown);
impl ItemRemovedEventArgs {
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn RemovedEntry(&self) -> ::windows_core::Result<AccessListEntry> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<AccessListEntry>>::zeroed();
            (::windows_core::Interface::vtable(this).RemovedEntry)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccessListEntry>(result__)
        }
    }
}
impl ::core::clone::Clone for ItemRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ItemRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ItemRemovedEventArgs {}
impl ::core::fmt::Debug for ItemRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ItemRemovedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ItemRemovedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Storage.AccessCache.ItemRemovedEventArgs;{59677e5c-55be-4c66-ba66-5eaea79d2631})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ItemRemovedEventArgs {
    type Vtable = IItemRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IItemRemovedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ItemRemovedEventArgs {
    const NAME: &'static str = "Windows.Storage.AccessCache.ItemRemovedEventArgs";
}
impl ::core::convert::From<ItemRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ItemRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ItemRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ItemRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ItemRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ItemRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ItemRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ItemRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ItemRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ItemRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ItemRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ItemRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Storage_AccessCache\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RecentStorageItemVisibility(pub i32);
impl RecentStorageItemVisibility {
    pub const AppOnly: Self = Self(0i32);
    pub const AppAndSystem: Self = Self(1i32);
}
impl ::core::marker::Copy for RecentStorageItemVisibility {}
impl ::core::clone::Clone for RecentStorageItemVisibility {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RecentStorageItemVisibility {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RecentStorageItemVisibility {
    type Abi = Self;
}
impl ::core::fmt::Debug for RecentStorageItemVisibility {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RecentStorageItemVisibility").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RecentStorageItemVisibility {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Storage.AccessCache.RecentStorageItemVisibility;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_AccessCache\"`*"]
pub struct StorageApplicationPermissions;
impl StorageApplicationPermissions {
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn FutureAccessList() -> ::windows_core::Result<StorageItemAccessList> {
        Self::IStorageApplicationPermissionsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FutureAccessList)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StorageItemAccessList>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn MostRecentlyUsedList() -> ::windows_core::Result<StorageItemMostRecentlyUsedList> {
        Self::IStorageApplicationPermissionsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MostRecentlyUsedList)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StorageItemMostRecentlyUsedList>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetFutureAccessListForUser<'a, Param0: ::windows_core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows_core::Result<StorageItemAccessList> {
        Self::IStorageApplicationPermissionsStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFutureAccessListForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<StorageItemAccessList>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetMostRecentlyUsedListForUser<'a, Param0: ::windows_core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows_core::Result<StorageItemMostRecentlyUsedList> {
        Self::IStorageApplicationPermissionsStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetMostRecentlyUsedListForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<StorageItemMostRecentlyUsedList>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageApplicationPermissionsStatics<R, F: FnOnce(&IStorageApplicationPermissionsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<StorageApplicationPermissions, IStorageApplicationPermissionsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IStorageApplicationPermissionsStatics2<R, F: FnOnce(&IStorageApplicationPermissionsStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<StorageApplicationPermissions, IStorageApplicationPermissionsStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for StorageApplicationPermissions {
    const NAME: &'static str = "Windows.Storage.AccessCache.StorageApplicationPermissions";
}
#[doc = "*Required features: `\"Storage_AccessCache\"`*"]
#[repr(transparent)]
pub struct StorageItemAccessList(::windows_core::IUnknown);
impl StorageItemAccessList {
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn AddOverloadDefaultMetadata<'a, Param0: ::windows_core::IntoParam<'a, super::IStorageItem>>(&self, file: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AddOverloadDefaultMetadata)(::windows_core::Interface::as_raw(this), file.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn Add<'a, Param0: ::windows_core::IntoParam<'a, super::IStorageItem>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, file: Param0, metadata: Param1) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Add)(::windows_core::Interface::as_raw(this), file.into_param().abi(), metadata.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn AddOrReplaceOverloadDefaultMetadata<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::IStorageItem>>(&self, token: Param0, file: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddOrReplaceOverloadDefaultMetadata)(::windows_core::Interface::as_raw(this), token.into_param().abi(), file.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn AddOrReplace<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::IStorageItem>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0, file: Param1, metadata: Param2) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddOrReplace)(::windows_core::Interface::as_raw(this), token.into_param().abi(), file.into_param().abi(), metadata.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetItemAsync)(::windows_core::Interface::as_raw(this), token.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::IStorageItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFileAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFileAsync)(::windows_core::Interface::as_raw(this), token.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFolderAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFolderAsync)(::windows_core::Interface::as_raw(this), token.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0, options: AccessCacheOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetItemWithOptionsAsync)(::windows_core::Interface::as_raw(this), token.into_param().abi(), options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::IStorageItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFileWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0, options: AccessCacheOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFileWithOptionsAsync)(::windows_core::Interface::as_raw(this), token.into_param().abi(), options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFolderWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0, options: AccessCacheOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFolderWithOptionsAsync)(::windows_core::Interface::as_raw(this), token.into_param().abi(), options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn ContainsItem<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ContainsItem)(::windows_core::Interface::as_raw(this), token.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn CheckAccess<'a, Param0: ::windows_core::IntoParam<'a, super::IStorageItem>>(&self, file: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CheckAccess)(::windows_core::Interface::as_raw(this), file.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Entries(&self) -> ::windows_core::Result<AccessListEntryView> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Entries)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccessListEntryView>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn MaximumItemsAllowed(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaximumItemsAllowed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for StorageItemAccessList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageItemAccessList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageItemAccessList {}
impl ::core::fmt::Debug for StorageItemAccessList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageItemAccessList").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StorageItemAccessList {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Storage.AccessCache.StorageItemAccessList;{2caff6ad-de90-47f5-b2c3-dd36c9fdd453})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StorageItemAccessList {
    type Vtable = IStorageItemAccessList_Vtbl;
    const IID: ::windows_core::GUID = <IStorageItemAccessList as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StorageItemAccessList {
    const NAME: &'static str = "Windows.Storage.AccessCache.StorageItemAccessList";
}
impl ::core::convert::From<StorageItemAccessList> for ::windows_core::IUnknown {
    fn from(value: StorageItemAccessList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageItemAccessList> for ::windows_core::IUnknown {
    fn from(value: &StorageItemAccessList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StorageItemAccessList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StorageItemAccessList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageItemAccessList> for ::windows_core::IInspectable {
    fn from(value: StorageItemAccessList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageItemAccessList> for ::windows_core::IInspectable {
    fn from(value: &StorageItemAccessList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StorageItemAccessList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StorageItemAccessList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<StorageItemAccessList> for IStorageItemAccessList {
    type Error = ::windows_core::Error;
    fn try_from(value: StorageItemAccessList) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageItemAccessList> for IStorageItemAccessList {
    type Error = ::windows_core::Error;
    fn try_from(value: &StorageItemAccessList) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IStorageItemAccessList> for StorageItemAccessList {
    fn into_param(self) -> ::windows_core::Param<'a, IStorageItemAccessList> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IStorageItemAccessList> for &StorageItemAccessList {
    fn into_param(self) -> ::windows_core::Param<'a, IStorageItemAccessList> {
        ::core::convert::TryInto::<IStorageItemAccessList>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[doc = "*Required features: `\"Storage_AccessCache\"`*"]
#[repr(transparent)]
pub struct StorageItemMostRecentlyUsedList(::windows_core::IUnknown);
impl StorageItemMostRecentlyUsedList {
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn AddOverloadDefaultMetadata<'a, Param0: ::windows_core::IntoParam<'a, super::IStorageItem>>(&self, file: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AddOverloadDefaultMetadata)(::windows_core::Interface::as_raw(this), file.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn Add<'a, Param0: ::windows_core::IntoParam<'a, super::IStorageItem>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, file: Param0, metadata: Param1) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Add)(::windows_core::Interface::as_raw(this), file.into_param().abi(), metadata.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn AddOrReplaceOverloadDefaultMetadata<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::IStorageItem>>(&self, token: Param0, file: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddOrReplaceOverloadDefaultMetadata)(::windows_core::Interface::as_raw(this), token.into_param().abi(), file.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn AddOrReplace<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::IStorageItem>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0, file: Param1, metadata: Param2) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddOrReplace)(::windows_core::Interface::as_raw(this), token.into_param().abi(), file.into_param().abi(), metadata.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = &::windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetItemAsync)(::windows_core::Interface::as_raw(this), token.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::IStorageItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFileAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFileAsync)(::windows_core::Interface::as_raw(this), token.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFolderAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFolderAsync)(::windows_core::Interface::as_raw(this), token.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0, options: AccessCacheOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = &::windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetItemWithOptionsAsync)(::windows_core::Interface::as_raw(this), token.into_param().abi(), options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::IStorageItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFileWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0, options: AccessCacheOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFileWithOptionsAsync)(::windows_core::Interface::as_raw(this), token.into_param().abi(), options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFolderWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0, options: AccessCacheOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFolderWithOptionsAsync)(::windows_core::Interface::as_raw(this), token.into_param().abi(), options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn ContainsItem<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ContainsItem)(::windows_core::Interface::as_raw(this), token.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn CheckAccess<'a, Param0: ::windows_core::IntoParam<'a, super::IStorageItem>>(&self, file: Param0) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CheckAccess)(::windows_core::Interface::as_raw(this), file.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Entries(&self) -> ::windows_core::Result<AccessListEntryView> {
        let this = &::windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Entries)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AccessListEntryView>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn MaximumItemsAllowed(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaximumItemsAllowed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ItemRemoved<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<StorageItemMostRecentlyUsedList, ItemRemovedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ItemRemoved)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemRemoved<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveItemRemoved)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn AddWithMetadataAndVisibility<'a, Param0: ::windows_core::IntoParam<'a, super::IStorageItem>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, file: Param0, metadata: Param1, visibility: RecentStorageItemVisibility) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IStorageItemMostRecentlyUsedList2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AddWithMetadataAndVisibility)(::windows_core::Interface::as_raw(this), file.into_param().abi(), metadata.into_param().abi(), visibility, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_AccessCache\"`*"]
    pub fn AddOrReplaceWithMetadataAndVisibility<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::IStorageItem>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, token: Param0, file: Param1, metadata: Param2, visibility: RecentStorageItemVisibility) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IStorageItemMostRecentlyUsedList2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddOrReplaceWithMetadataAndVisibility)(::windows_core::Interface::as_raw(this), token.into_param().abi(), file.into_param().abi(), metadata.into_param().abi(), visibility).ok() }
    }
}
impl ::core::clone::Clone for StorageItemMostRecentlyUsedList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageItemMostRecentlyUsedList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageItemMostRecentlyUsedList {}
impl ::core::fmt::Debug for StorageItemMostRecentlyUsedList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageItemMostRecentlyUsedList").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StorageItemMostRecentlyUsedList {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Storage.AccessCache.StorageItemMostRecentlyUsedList;{016239d5-510d-411e-8cf1-c3d1effa4c33})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StorageItemMostRecentlyUsedList {
    type Vtable = IStorageItemMostRecentlyUsedList_Vtbl;
    const IID: ::windows_core::GUID = <IStorageItemMostRecentlyUsedList as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StorageItemMostRecentlyUsedList {
    const NAME: &'static str = "Windows.Storage.AccessCache.StorageItemMostRecentlyUsedList";
}
impl ::core::convert::From<StorageItemMostRecentlyUsedList> for ::windows_core::IUnknown {
    fn from(value: StorageItemMostRecentlyUsedList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageItemMostRecentlyUsedList> for ::windows_core::IUnknown {
    fn from(value: &StorageItemMostRecentlyUsedList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StorageItemMostRecentlyUsedList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StorageItemMostRecentlyUsedList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageItemMostRecentlyUsedList> for ::windows_core::IInspectable {
    fn from(value: StorageItemMostRecentlyUsedList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageItemMostRecentlyUsedList> for ::windows_core::IInspectable {
    fn from(value: &StorageItemMostRecentlyUsedList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StorageItemMostRecentlyUsedList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StorageItemMostRecentlyUsedList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<StorageItemMostRecentlyUsedList> for IStorageItemAccessList {
    type Error = ::windows_core::Error;
    fn try_from(value: StorageItemMostRecentlyUsedList) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageItemMostRecentlyUsedList> for IStorageItemAccessList {
    type Error = ::windows_core::Error;
    fn try_from(value: &StorageItemMostRecentlyUsedList) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IStorageItemAccessList> for StorageItemMostRecentlyUsedList {
    fn into_param(self) -> ::windows_core::Param<'a, IStorageItemAccessList> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IStorageItemAccessList> for &StorageItemMostRecentlyUsedList {
    fn into_param(self) -> ::windows_core::Param<'a, IStorageItemAccessList> {
        ::core::convert::TryInto::<IStorageItemAccessList>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
