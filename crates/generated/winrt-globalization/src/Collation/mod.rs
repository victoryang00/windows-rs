#[repr(transparent)]
pub struct CharacterGrouping(::windows_core::IUnknown);
impl CharacterGrouping {
    pub fn First(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for CharacterGrouping {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CharacterGrouping {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CharacterGrouping {}
impl ::core::fmt::Debug for CharacterGrouping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CharacterGrouping").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CharacterGrouping {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Globalization.Collation.CharacterGrouping;{fae761bb-805d-4bb0-95bb-c1f7c3e8eb8e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CharacterGrouping {
    type Vtable = ICharacterGrouping_Vtbl;
    const IID: ::windows_core::GUID = <ICharacterGrouping as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CharacterGrouping {
    const NAME: &'static str = "Windows.Globalization.Collation.CharacterGrouping";
}
impl ::core::convert::From<CharacterGrouping> for ::windows_core::IUnknown {
    fn from(value: CharacterGrouping) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CharacterGrouping> for ::windows_core::IUnknown {
    fn from(value: &CharacterGrouping) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CharacterGrouping {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CharacterGrouping {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CharacterGrouping> for ::windows_core::IInspectable {
    fn from(value: CharacterGrouping) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CharacterGrouping> for ::windows_core::IInspectable {
    fn from(value: &CharacterGrouping) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CharacterGrouping {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CharacterGrouping {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CharacterGrouping {}
unsafe impl ::core::marker::Sync for CharacterGrouping {}
#[repr(transparent)]
pub struct CharacterGroupings(::windows_core::IUnknown);
impl CharacterGroupings {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CharacterGroupings, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Lookup<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, text: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), text.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(language: Param0) -> ::windows_core::Result<CharacterGroupings> {
        Self::ICharacterGroupingsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), language.into_param().abi(), result__.as_mut_ptr()).from_abi::<CharacterGroupings>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<CharacterGrouping>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<CharacterGrouping>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<CharacterGrouping>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<CharacterGrouping> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVectorView<CharacterGrouping>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<CharacterGrouping>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVectorView<CharacterGrouping>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, CharacterGrouping>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVectorView<CharacterGrouping>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<CharacterGrouping>]) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVectorView<CharacterGrouping>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ICharacterGroupingsFactory<R, F: FnOnce(&ICharacterGroupingsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CharacterGroupings, ICharacterGroupingsFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CharacterGroupings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CharacterGroupings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CharacterGroupings {}
impl ::core::fmt::Debug for CharacterGroupings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CharacterGroupings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CharacterGroupings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Globalization.Collation.CharacterGroupings;{b8d20a75-d4cf-4055-80e5-ce169c226496})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CharacterGroupings {
    type Vtable = ICharacterGroupings_Vtbl;
    const IID: ::windows_core::GUID = <ICharacterGroupings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CharacterGroupings {
    const NAME: &'static str = "Windows.Globalization.Collation.CharacterGroupings";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for CharacterGroupings {
    type Item = CharacterGrouping;
    type IntoIter = ::winrt_foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &CharacterGroupings {
    type Item = CharacterGrouping;
    type IntoIter = ::winrt_foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<CharacterGroupings> for ::windows_core::IUnknown {
    fn from(value: CharacterGroupings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CharacterGroupings> for ::windows_core::IUnknown {
    fn from(value: &CharacterGroupings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CharacterGroupings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CharacterGroupings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CharacterGroupings> for ::windows_core::IInspectable {
    fn from(value: CharacterGroupings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CharacterGroupings> for ::windows_core::IInspectable {
    fn from(value: &CharacterGroupings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CharacterGroupings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CharacterGroupings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<CharacterGroupings> for ::winrt_foundation::Collections::IIterable<CharacterGrouping> {
    type Error = ::windows_core::Error;
    fn try_from(value: CharacterGroupings) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&CharacterGroupings> for ::winrt_foundation::Collections::IIterable<CharacterGrouping> {
    type Error = ::windows_core::Error;
    fn try_from(value: &CharacterGroupings) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<CharacterGrouping>> for CharacterGroupings {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<CharacterGrouping>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<CharacterGrouping>> for &CharacterGroupings {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<CharacterGrouping>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<CharacterGrouping>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<CharacterGroupings> for ::winrt_foundation::Collections::IVectorView<CharacterGrouping> {
    type Error = ::windows_core::Error;
    fn try_from(value: CharacterGroupings) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&CharacterGroupings> for ::winrt_foundation::Collections::IVectorView<CharacterGrouping> {
    type Error = ::windows_core::Error;
    fn try_from(value: &CharacterGroupings) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<CharacterGrouping>> for CharacterGroupings {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVectorView<CharacterGrouping>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<CharacterGrouping>> for &CharacterGroupings {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVectorView<CharacterGrouping>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVectorView<CharacterGrouping>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CharacterGroupings {}
unsafe impl ::core::marker::Sync for CharacterGroupings {}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICharacterGrouping(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICharacterGrouping {
    type Vtable = ICharacterGrouping_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfae761bb_805d_4bb0_95bb_c1f7c3e8eb8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterGrouping_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub First: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICharacterGroupings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICharacterGroupings {
    type Vtable = ICharacterGroupings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb8d20a75_d4cf_4055_80e5_ce169c226496);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterGroupings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Lookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICharacterGroupingsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICharacterGroupingsFactory {
    type Vtable = ICharacterGroupingsFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x99ea9fd9_886d_4401_9f98_69c82d4c2f78);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterGroupingsFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
