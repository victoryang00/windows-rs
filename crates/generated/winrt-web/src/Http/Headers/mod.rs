#[repr(transparent)]
pub struct HttpCacheDirectiveHeaderValueCollection(::windows_core::IUnknown);
impl HttpCacheDirectiveHeaderValueCollection {
    pub fn MaxAge(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MaxAge)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetMaxAge<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxAge)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MaxStale(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MaxStale)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetMaxStale<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxStale)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MinFresh(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MinFresh)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetMinFresh<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMinFresh)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SharedMaxAge(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SharedMaxAge)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetSharedMaxAge<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSharedMaxAge)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi()).ok() }
    }
    pub fn TryParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<HttpNameValueHeaderValue>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<HttpNameValueHeaderValue>>(result__)
        }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<HttpNameValueHeaderValue> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<HttpNameValueHeaderValue>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<HttpNameValueHeaderValue>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<HttpNameValueHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, HttpNameValueHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, HttpNameValueHeaderValue>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, HttpNameValueHeaderValue>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, HttpNameValueHeaderValue>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<HttpNameValueHeaderValue>]) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<HttpNameValueHeaderValue>]) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpCacheDirectiveHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpCacheDirectiveHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpCacheDirectiveHeaderValueCollection {}
impl ::core::fmt::Debug for HttpCacheDirectiveHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCacheDirectiveHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpCacheDirectiveHeaderValueCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpCacheDirectiveHeaderValueCollection;{9a586b89-d5d0-4fbe-bd9d-b5b3636811b4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpCacheDirectiveHeaderValueCollection {
    type Vtable = IHttpCacheDirectiveHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = <IHttpCacheDirectiveHeaderValueCollection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpCacheDirectiveHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpCacheDirectiveHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpCacheDirectiveHeaderValueCollection {
    type Item = HttpNameValueHeaderValue;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpCacheDirectiveHeaderValueCollection {
    type Item = HttpNameValueHeaderValue;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpCacheDirectiveHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: HttpCacheDirectiveHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCacheDirectiveHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: &HttpCacheDirectiveHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpCacheDirectiveHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: HttpCacheDirectiveHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCacheDirectiveHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: &HttpCacheDirectiveHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpCacheDirectiveHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<HttpNameValueHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpCacheDirectiveHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpCacheDirectiveHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<HttpNameValueHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpCacheDirectiveHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<HttpNameValueHeaderValue>> for HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<HttpNameValueHeaderValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<HttpNameValueHeaderValue>> for &HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<HttpNameValueHeaderValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<HttpNameValueHeaderValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpCacheDirectiveHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpCacheDirectiveHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpCacheDirectiveHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpCacheDirectiveHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpCacheDirectiveHeaderValueCollection> for ::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpCacheDirectiveHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpCacheDirectiveHeaderValueCollection> for ::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpCacheDirectiveHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>> for HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>> for &HttpCacheDirectiveHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpCacheDirectiveHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpCacheDirectiveHeaderValueCollection {}
#[repr(transparent)]
pub struct HttpChallengeHeaderValue(::windows_core::IUnknown);
impl HttpChallengeHeaderValue {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Parameters(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parameters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>>(result__)
        }
    }
    pub fn Scheme(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Scheme)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Token(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Token)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CreateFromScheme<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(scheme: Param0) -> ::windows_core::Result<HttpChallengeHeaderValue> {
        Self::IHttpChallengeHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromScheme)(::windows_core::Interface::as_raw(this), scheme.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpChallengeHeaderValue>(result__)
        })
    }
    pub fn CreateFromSchemeWithToken<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(scheme: Param0, token: Param1) -> ::windows_core::Result<HttpChallengeHeaderValue> {
        Self::IHttpChallengeHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromSchemeWithToken)(::windows_core::Interface::as_raw(this), scheme.into_param().abi(), token.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpChallengeHeaderValue>(result__)
        })
    }
    pub fn Parse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0) -> ::windows_core::Result<HttpChallengeHeaderValue> {
        Self::IHttpChallengeHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpChallengeHeaderValue>(result__)
        })
    }
    pub fn TryParse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0, challengeheadervalue: &mut ::core::option::Option<HttpChallengeHeaderValue>) -> ::windows_core::Result<bool> {
        Self::IHttpChallengeHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), challengeheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpChallengeHeaderValueFactory<R, F: FnOnce(&IHttpChallengeHeaderValueFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpChallengeHeaderValue, IHttpChallengeHeaderValueFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpChallengeHeaderValueStatics<R, F: FnOnce(&IHttpChallengeHeaderValueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpChallengeHeaderValue, IHttpChallengeHeaderValueStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpChallengeHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpChallengeHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpChallengeHeaderValue {}
impl ::core::fmt::Debug for HttpChallengeHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpChallengeHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpChallengeHeaderValue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpChallengeHeaderValue;{393361af-0f7d-4820-9fdd-a2b956eeaeab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpChallengeHeaderValue {
    type Vtable = IHttpChallengeHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = <IHttpChallengeHeaderValue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpChallengeHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpChallengeHeaderValue";
}
impl ::core::convert::From<HttpChallengeHeaderValue> for ::windows_core::IUnknown {
    fn from(value: HttpChallengeHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpChallengeHeaderValue> for ::windows_core::IUnknown {
    fn from(value: &HttpChallengeHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpChallengeHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpChallengeHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpChallengeHeaderValue> for ::windows_core::IInspectable {
    fn from(value: HttpChallengeHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpChallengeHeaderValue> for ::windows_core::IInspectable {
    fn from(value: &HttpChallengeHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpChallengeHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpChallengeHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpChallengeHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpChallengeHeaderValue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpChallengeHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpChallengeHeaderValue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpChallengeHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpChallengeHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpChallengeHeaderValue {}
unsafe impl ::core::marker::Sync for HttpChallengeHeaderValue {}
#[repr(transparent)]
pub struct HttpChallengeHeaderValueCollection(::windows_core::IUnknown);
impl HttpChallengeHeaderValueCollection {
    pub fn ParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi()).ok() }
    }
    pub fn TryParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<HttpChallengeHeaderValue>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<HttpChallengeHeaderValue>>(result__)
        }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<HttpChallengeHeaderValue> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<HttpChallengeHeaderValue>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<HttpChallengeHeaderValue>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<HttpChallengeHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, HttpChallengeHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, HttpChallengeHeaderValue>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, HttpChallengeHeaderValue>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, HttpChallengeHeaderValue>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<HttpChallengeHeaderValue>]) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<HttpChallengeHeaderValue>]) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpChallengeHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpChallengeHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpChallengeHeaderValueCollection {}
impl ::core::fmt::Debug for HttpChallengeHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpChallengeHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpChallengeHeaderValueCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpChallengeHeaderValueCollection;{ca9e5f81-aee0-4353-a10b-e625babd64c2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpChallengeHeaderValueCollection {
    type Vtable = IHttpChallengeHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = <IHttpChallengeHeaderValueCollection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpChallengeHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpChallengeHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpChallengeHeaderValueCollection {
    type Item = HttpChallengeHeaderValue;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpChallengeHeaderValueCollection {
    type Item = HttpChallengeHeaderValue;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpChallengeHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: HttpChallengeHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpChallengeHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: &HttpChallengeHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpChallengeHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: HttpChallengeHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpChallengeHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: &HttpChallengeHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpChallengeHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<HttpChallengeHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpChallengeHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpChallengeHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<HttpChallengeHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpChallengeHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<HttpChallengeHeaderValue>> for HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<HttpChallengeHeaderValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<HttpChallengeHeaderValue>> for &HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<HttpChallengeHeaderValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<HttpChallengeHeaderValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpChallengeHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpChallengeHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpChallengeHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpChallengeHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpChallengeHeaderValueCollection> for ::winrt_foundation::Collections::IVector<HttpChallengeHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpChallengeHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpChallengeHeaderValueCollection> for ::winrt_foundation::Collections::IVector<HttpChallengeHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpChallengeHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<HttpChallengeHeaderValue>> for HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<HttpChallengeHeaderValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<HttpChallengeHeaderValue>> for &HttpChallengeHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<HttpChallengeHeaderValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<HttpChallengeHeaderValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpChallengeHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpChallengeHeaderValueCollection {}
#[repr(transparent)]
pub struct HttpConnectionOptionHeaderValue(::windows_core::IUnknown);
impl HttpConnectionOptionHeaderValue {
    pub fn Token(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Token)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(token: Param0) -> ::windows_core::Result<HttpConnectionOptionHeaderValue> {
        Self::IHttpConnectionOptionHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), token.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpConnectionOptionHeaderValue>(result__)
        })
    }
    pub fn Parse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0) -> ::windows_core::Result<HttpConnectionOptionHeaderValue> {
        Self::IHttpConnectionOptionHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpConnectionOptionHeaderValue>(result__)
        })
    }
    pub fn TryParse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0, connectionoptionheadervalue: &mut ::core::option::Option<HttpConnectionOptionHeaderValue>) -> ::windows_core::Result<bool> {
        Self::IHttpConnectionOptionHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), connectionoptionheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpConnectionOptionHeaderValueFactory<R, F: FnOnce(&IHttpConnectionOptionHeaderValueFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpConnectionOptionHeaderValue, IHttpConnectionOptionHeaderValueFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpConnectionOptionHeaderValueStatics<R, F: FnOnce(&IHttpConnectionOptionHeaderValueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpConnectionOptionHeaderValue, IHttpConnectionOptionHeaderValueStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpConnectionOptionHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpConnectionOptionHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpConnectionOptionHeaderValue {}
impl ::core::fmt::Debug for HttpConnectionOptionHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpConnectionOptionHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpConnectionOptionHeaderValue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpConnectionOptionHeaderValue;{cb4af27a-4e90-45eb-8dcd-fd1408f4c44f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpConnectionOptionHeaderValue {
    type Vtable = IHttpConnectionOptionHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = <IHttpConnectionOptionHeaderValue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpConnectionOptionHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpConnectionOptionHeaderValue";
}
impl ::core::convert::From<HttpConnectionOptionHeaderValue> for ::windows_core::IUnknown {
    fn from(value: HttpConnectionOptionHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpConnectionOptionHeaderValue> for ::windows_core::IUnknown {
    fn from(value: &HttpConnectionOptionHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpConnectionOptionHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpConnectionOptionHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpConnectionOptionHeaderValue> for ::windows_core::IInspectable {
    fn from(value: HttpConnectionOptionHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpConnectionOptionHeaderValue> for ::windows_core::IInspectable {
    fn from(value: &HttpConnectionOptionHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpConnectionOptionHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpConnectionOptionHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpConnectionOptionHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpConnectionOptionHeaderValue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpConnectionOptionHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpConnectionOptionHeaderValue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpConnectionOptionHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpConnectionOptionHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpConnectionOptionHeaderValue {}
unsafe impl ::core::marker::Sync for HttpConnectionOptionHeaderValue {}
#[repr(transparent)]
pub struct HttpConnectionOptionHeaderValueCollection(::windows_core::IUnknown);
impl HttpConnectionOptionHeaderValueCollection {
    pub fn ParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi()).ok() }
    }
    pub fn TryParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<HttpConnectionOptionHeaderValue>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<HttpConnectionOptionHeaderValue>>(result__)
        }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<HttpConnectionOptionHeaderValue> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<HttpConnectionOptionHeaderValue>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<HttpConnectionOptionHeaderValue>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<HttpConnectionOptionHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, HttpConnectionOptionHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, HttpConnectionOptionHeaderValue>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, HttpConnectionOptionHeaderValue>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, HttpConnectionOptionHeaderValue>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<HttpConnectionOptionHeaderValue>]) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<HttpConnectionOptionHeaderValue>]) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpConnectionOptionHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpConnectionOptionHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpConnectionOptionHeaderValueCollection {}
impl ::core::fmt::Debug for HttpConnectionOptionHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpConnectionOptionHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpConnectionOptionHeaderValueCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpConnectionOptionHeaderValueCollection;{e4f56c1d-5142-4e00-8e0f-019509337629})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpConnectionOptionHeaderValueCollection {
    type Vtable = IHttpConnectionOptionHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = <IHttpConnectionOptionHeaderValueCollection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpConnectionOptionHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpConnectionOptionHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpConnectionOptionHeaderValueCollection {
    type Item = HttpConnectionOptionHeaderValue;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpConnectionOptionHeaderValueCollection {
    type Item = HttpConnectionOptionHeaderValue;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpConnectionOptionHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: HttpConnectionOptionHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpConnectionOptionHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: &HttpConnectionOptionHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpConnectionOptionHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: HttpConnectionOptionHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpConnectionOptionHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: &HttpConnectionOptionHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpConnectionOptionHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<HttpConnectionOptionHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpConnectionOptionHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpConnectionOptionHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<HttpConnectionOptionHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpConnectionOptionHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<HttpConnectionOptionHeaderValue>> for HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<HttpConnectionOptionHeaderValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<HttpConnectionOptionHeaderValue>> for &HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<HttpConnectionOptionHeaderValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<HttpConnectionOptionHeaderValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpConnectionOptionHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpConnectionOptionHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpConnectionOptionHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpConnectionOptionHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpConnectionOptionHeaderValueCollection> for ::winrt_foundation::Collections::IVector<HttpConnectionOptionHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpConnectionOptionHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpConnectionOptionHeaderValueCollection> for ::winrt_foundation::Collections::IVector<HttpConnectionOptionHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpConnectionOptionHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<HttpConnectionOptionHeaderValue>> for HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<HttpConnectionOptionHeaderValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<HttpConnectionOptionHeaderValue>> for &HttpConnectionOptionHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<HttpConnectionOptionHeaderValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpConnectionOptionHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpConnectionOptionHeaderValueCollection {}
#[repr(transparent)]
pub struct HttpContentCodingHeaderValue(::windows_core::IUnknown);
impl HttpContentCodingHeaderValue {
    pub fn ContentCoding(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ContentCoding)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(contentcoding: Param0) -> ::windows_core::Result<HttpContentCodingHeaderValue> {
        Self::IHttpContentCodingHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), contentcoding.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpContentCodingHeaderValue>(result__)
        })
    }
    pub fn Parse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0) -> ::windows_core::Result<HttpContentCodingHeaderValue> {
        Self::IHttpContentCodingHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpContentCodingHeaderValue>(result__)
        })
    }
    pub fn TryParse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0, contentcodingheadervalue: &mut ::core::option::Option<HttpContentCodingHeaderValue>) -> ::windows_core::Result<bool> {
        Self::IHttpContentCodingHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), contentcodingheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpContentCodingHeaderValueFactory<R, F: FnOnce(&IHttpContentCodingHeaderValueFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpContentCodingHeaderValue, IHttpContentCodingHeaderValueFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpContentCodingHeaderValueStatics<R, F: FnOnce(&IHttpContentCodingHeaderValueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpContentCodingHeaderValue, IHttpContentCodingHeaderValueStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpContentCodingHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpContentCodingHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpContentCodingHeaderValue {}
impl ::core::fmt::Debug for HttpContentCodingHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpContentCodingHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpContentCodingHeaderValue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentCodingHeaderValue;{bcf7f92a-9376-4d85-bccc-9f4f9acab434})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpContentCodingHeaderValue {
    type Vtable = IHttpContentCodingHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = <IHttpContentCodingHeaderValue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpContentCodingHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentCodingHeaderValue";
}
impl ::core::convert::From<HttpContentCodingHeaderValue> for ::windows_core::IUnknown {
    fn from(value: HttpContentCodingHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentCodingHeaderValue> for ::windows_core::IUnknown {
    fn from(value: &HttpContentCodingHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpContentCodingHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpContentCodingHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpContentCodingHeaderValue> for ::windows_core::IInspectable {
    fn from(value: HttpContentCodingHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentCodingHeaderValue> for ::windows_core::IInspectable {
    fn from(value: &HttpContentCodingHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpContentCodingHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpContentCodingHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpContentCodingHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpContentCodingHeaderValue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpContentCodingHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpContentCodingHeaderValue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpContentCodingHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpContentCodingHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpContentCodingHeaderValue {}
unsafe impl ::core::marker::Sync for HttpContentCodingHeaderValue {}
#[repr(transparent)]
pub struct HttpContentCodingHeaderValueCollection(::windows_core::IUnknown);
impl HttpContentCodingHeaderValueCollection {
    pub fn ParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi()).ok() }
    }
    pub fn TryParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<HttpContentCodingHeaderValue>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<HttpContentCodingHeaderValue>>(result__)
        }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<HttpContentCodingHeaderValue> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<HttpContentCodingHeaderValue>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<HttpContentCodingHeaderValue>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<HttpContentCodingHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, HttpContentCodingHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, HttpContentCodingHeaderValue>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, HttpContentCodingHeaderValue>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, HttpContentCodingHeaderValue>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<HttpContentCodingHeaderValue>]) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<HttpContentCodingHeaderValue>]) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpContentCodingHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpContentCodingHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpContentCodingHeaderValueCollection {}
impl ::core::fmt::Debug for HttpContentCodingHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpContentCodingHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpContentCodingHeaderValueCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentCodingHeaderValueCollection;{7d221721-a6db-436e-8e83-91596192819c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpContentCodingHeaderValueCollection {
    type Vtable = IHttpContentCodingHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = <IHttpContentCodingHeaderValueCollection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpContentCodingHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentCodingHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpContentCodingHeaderValueCollection {
    type Item = HttpContentCodingHeaderValue;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpContentCodingHeaderValueCollection {
    type Item = HttpContentCodingHeaderValue;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpContentCodingHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: HttpContentCodingHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentCodingHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: &HttpContentCodingHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpContentCodingHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: HttpContentCodingHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentCodingHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: &HttpContentCodingHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpContentCodingHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<HttpContentCodingHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpContentCodingHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpContentCodingHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<HttpContentCodingHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpContentCodingHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<HttpContentCodingHeaderValue>> for HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<HttpContentCodingHeaderValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<HttpContentCodingHeaderValue>> for &HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<HttpContentCodingHeaderValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<HttpContentCodingHeaderValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpContentCodingHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpContentCodingHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpContentCodingHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpContentCodingHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpContentCodingHeaderValueCollection> for ::winrt_foundation::Collections::IVector<HttpContentCodingHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpContentCodingHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpContentCodingHeaderValueCollection> for ::winrt_foundation::Collections::IVector<HttpContentCodingHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpContentCodingHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<HttpContentCodingHeaderValue>> for HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<HttpContentCodingHeaderValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<HttpContentCodingHeaderValue>> for &HttpContentCodingHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<HttpContentCodingHeaderValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<HttpContentCodingHeaderValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpContentCodingHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpContentCodingHeaderValueCollection {}
#[repr(transparent)]
pub struct HttpContentCodingWithQualityHeaderValue(::windows_core::IUnknown);
impl HttpContentCodingWithQualityHeaderValue {
    pub fn ContentCoding(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ContentCoding)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Quality(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Quality)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn CreateFromValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(contentcoding: Param0) -> ::windows_core::Result<HttpContentCodingWithQualityHeaderValue> {
        Self::IHttpContentCodingWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromValue)(::windows_core::Interface::as_raw(this), contentcoding.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpContentCodingWithQualityHeaderValue>(result__)
        })
    }
    pub fn CreateFromValueWithQuality<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(contentcoding: Param0, quality: f64) -> ::windows_core::Result<HttpContentCodingWithQualityHeaderValue> {
        Self::IHttpContentCodingWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromValueWithQuality)(::windows_core::Interface::as_raw(this), contentcoding.into_param().abi(), quality, result__.as_mut_ptr()).from_abi::<HttpContentCodingWithQualityHeaderValue>(result__)
        })
    }
    pub fn Parse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0) -> ::windows_core::Result<HttpContentCodingWithQualityHeaderValue> {
        Self::IHttpContentCodingWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpContentCodingWithQualityHeaderValue>(result__)
        })
    }
    pub fn TryParse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0, contentcodingwithqualityheadervalue: &mut ::core::option::Option<HttpContentCodingWithQualityHeaderValue>) -> ::windows_core::Result<bool> {
        Self::IHttpContentCodingWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), contentcodingwithqualityheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpContentCodingWithQualityHeaderValueFactory<R, F: FnOnce(&IHttpContentCodingWithQualityHeaderValueFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpContentCodingWithQualityHeaderValue, IHttpContentCodingWithQualityHeaderValueFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpContentCodingWithQualityHeaderValueStatics<R, F: FnOnce(&IHttpContentCodingWithQualityHeaderValueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpContentCodingWithQualityHeaderValue, IHttpContentCodingWithQualityHeaderValueStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpContentCodingWithQualityHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpContentCodingWithQualityHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpContentCodingWithQualityHeaderValue {}
impl ::core::fmt::Debug for HttpContentCodingWithQualityHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpContentCodingWithQualityHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpContentCodingWithQualityHeaderValue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentCodingWithQualityHeaderValue;{94531cd5-8b13-4d73-8651-f76b38f88495})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpContentCodingWithQualityHeaderValue {
    type Vtable = IHttpContentCodingWithQualityHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = <IHttpContentCodingWithQualityHeaderValue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpContentCodingWithQualityHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentCodingWithQualityHeaderValue";
}
impl ::core::convert::From<HttpContentCodingWithQualityHeaderValue> for ::windows_core::IUnknown {
    fn from(value: HttpContentCodingWithQualityHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentCodingWithQualityHeaderValue> for ::windows_core::IUnknown {
    fn from(value: &HttpContentCodingWithQualityHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpContentCodingWithQualityHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpContentCodingWithQualityHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpContentCodingWithQualityHeaderValue> for ::windows_core::IInspectable {
    fn from(value: HttpContentCodingWithQualityHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentCodingWithQualityHeaderValue> for ::windows_core::IInspectable {
    fn from(value: &HttpContentCodingWithQualityHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpContentCodingWithQualityHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpContentCodingWithQualityHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpContentCodingWithQualityHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpContentCodingWithQualityHeaderValue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpContentCodingWithQualityHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpContentCodingWithQualityHeaderValue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpContentCodingWithQualityHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpContentCodingWithQualityHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpContentCodingWithQualityHeaderValue {}
unsafe impl ::core::marker::Sync for HttpContentCodingWithQualityHeaderValue {}
#[repr(transparent)]
pub struct HttpContentCodingWithQualityHeaderValueCollection(::windows_core::IUnknown);
impl HttpContentCodingWithQualityHeaderValueCollection {
    pub fn ParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi()).ok() }
    }
    pub fn TryParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<HttpContentCodingWithQualityHeaderValue>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<HttpContentCodingWithQualityHeaderValue>>(result__)
        }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<HttpContentCodingWithQualityHeaderValue> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<HttpContentCodingWithQualityHeaderValue>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<HttpContentCodingWithQualityHeaderValue>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<HttpContentCodingWithQualityHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, HttpContentCodingWithQualityHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, HttpContentCodingWithQualityHeaderValue>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, HttpContentCodingWithQualityHeaderValue>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, HttpContentCodingWithQualityHeaderValue>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<HttpContentCodingWithQualityHeaderValue>]) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<HttpContentCodingWithQualityHeaderValue>]) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpContentCodingWithQualityHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpContentCodingWithQualityHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpContentCodingWithQualityHeaderValueCollection {}
impl ::core::fmt::Debug for HttpContentCodingWithQualityHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpContentCodingWithQualityHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpContentCodingWithQualityHeaderValueCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentCodingWithQualityHeaderValueCollection;{7c0d753e-e899-4378-b5c8-412d820711cc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpContentCodingWithQualityHeaderValueCollection {
    type Vtable = IHttpContentCodingWithQualityHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = <IHttpContentCodingWithQualityHeaderValueCollection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpContentCodingWithQualityHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentCodingWithQualityHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpContentCodingWithQualityHeaderValueCollection {
    type Item = HttpContentCodingWithQualityHeaderValue;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpContentCodingWithQualityHeaderValueCollection {
    type Item = HttpContentCodingWithQualityHeaderValue;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpContentCodingWithQualityHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: HttpContentCodingWithQualityHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentCodingWithQualityHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: &HttpContentCodingWithQualityHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpContentCodingWithQualityHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: HttpContentCodingWithQualityHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentCodingWithQualityHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: &HttpContentCodingWithQualityHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpContentCodingWithQualityHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpContentCodingWithQualityHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpContentCodingWithQualityHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpContentCodingWithQualityHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue>> for HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue>> for &HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpContentCodingWithQualityHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpContentCodingWithQualityHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpContentCodingWithQualityHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpContentCodingWithQualityHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpContentCodingWithQualityHeaderValueCollection> for ::winrt_foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpContentCodingWithQualityHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpContentCodingWithQualityHeaderValueCollection> for ::winrt_foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpContentCodingWithQualityHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>> for HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>> for &HttpContentCodingWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpContentCodingWithQualityHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpContentCodingWithQualityHeaderValueCollection {}
#[repr(transparent)]
pub struct HttpContentDispositionHeaderValue(::windows_core::IUnknown);
impl HttpContentDispositionHeaderValue {
    pub fn DispositionType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DispositionType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetDispositionType<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDispositionType)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn FileName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FileName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetFileName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFileName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn FileNameStar(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FileNameStar)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetFileNameStar<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFileNameStar)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Parameters(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parameters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>>(result__)
        }
    }
    pub fn Size(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u64>>(result__)
        }
    }
    pub fn SetSize<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<u64>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSize)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(dispositiontype: Param0) -> ::windows_core::Result<HttpContentDispositionHeaderValue> {
        Self::IHttpContentDispositionHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), dispositiontype.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpContentDispositionHeaderValue>(result__)
        })
    }
    pub fn Parse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0) -> ::windows_core::Result<HttpContentDispositionHeaderValue> {
        Self::IHttpContentDispositionHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpContentDispositionHeaderValue>(result__)
        })
    }
    pub fn TryParse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0, contentdispositionheadervalue: &mut ::core::option::Option<HttpContentDispositionHeaderValue>) -> ::windows_core::Result<bool> {
        Self::IHttpContentDispositionHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), contentdispositionheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpContentDispositionHeaderValueFactory<R, F: FnOnce(&IHttpContentDispositionHeaderValueFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpContentDispositionHeaderValue, IHttpContentDispositionHeaderValueFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpContentDispositionHeaderValueStatics<R, F: FnOnce(&IHttpContentDispositionHeaderValueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpContentDispositionHeaderValue, IHttpContentDispositionHeaderValueStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpContentDispositionHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpContentDispositionHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpContentDispositionHeaderValue {}
impl ::core::fmt::Debug for HttpContentDispositionHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpContentDispositionHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpContentDispositionHeaderValue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentDispositionHeaderValue;{f2a2eedc-2629-4b49-9908-96a168e9365e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpContentDispositionHeaderValue {
    type Vtable = IHttpContentDispositionHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = <IHttpContentDispositionHeaderValue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpContentDispositionHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentDispositionHeaderValue";
}
impl ::core::convert::From<HttpContentDispositionHeaderValue> for ::windows_core::IUnknown {
    fn from(value: HttpContentDispositionHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentDispositionHeaderValue> for ::windows_core::IUnknown {
    fn from(value: &HttpContentDispositionHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpContentDispositionHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpContentDispositionHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpContentDispositionHeaderValue> for ::windows_core::IInspectable {
    fn from(value: HttpContentDispositionHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentDispositionHeaderValue> for ::windows_core::IInspectable {
    fn from(value: &HttpContentDispositionHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpContentDispositionHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpContentDispositionHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpContentDispositionHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpContentDispositionHeaderValue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpContentDispositionHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpContentDispositionHeaderValue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpContentDispositionHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpContentDispositionHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpContentDispositionHeaderValue {}
unsafe impl ::core::marker::Sync for HttpContentDispositionHeaderValue {}
#[repr(transparent)]
pub struct HttpContentHeaderCollection(::windows_core::IUnknown);
impl HttpContentHeaderCollection {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpContentHeaderCollection, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ContentDisposition(&self) -> ::windows_core::Result<HttpContentDispositionHeaderValue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentDisposition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpContentDispositionHeaderValue>(result__)
        }
    }
    pub fn SetContentDisposition<'a, Param0: ::windows_core::IntoParam<'a, HttpContentDispositionHeaderValue>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentDisposition)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ContentEncoding(&self) -> ::windows_core::Result<HttpContentCodingHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentEncoding)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpContentCodingHeaderValueCollection>(result__)
        }
    }
    pub fn ContentLanguage(&self) -> ::windows_core::Result<HttpLanguageHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentLanguage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpLanguageHeaderValueCollection>(result__)
        }
    }
    pub fn ContentLength(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u64>>(result__)
        }
    }
    pub fn SetContentLength<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<u64>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentLength)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ContentLocation(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentLocation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetContentLocation<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentLocation)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ContentMD5(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentMD5)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetContentMD5<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentMD5)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ContentRange(&self) -> ::windows_core::Result<HttpContentRangeHeaderValue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentRange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpContentRangeHeaderValue>(result__)
        }
    }
    pub fn SetContentRange<'a, Param0: ::windows_core::IntoParam<'a, HttpContentRangeHeaderValue>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentRange)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ContentType(&self) -> ::windows_core::Result<HttpMediaTypeHeaderValue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpMediaTypeHeaderValue>(result__)
        }
    }
    pub fn SetContentType<'a, Param0: ::windows_core::IntoParam<'a, HttpMediaTypeHeaderValue>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentType)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Expires(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Expires)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn SetExpires<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExpires)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn LastModified(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LastModified)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn SetLastModified<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLastModified)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    pub fn TryAppendWithoutValidation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryAppendWithoutValidation)(::windows_core::Interface::as_raw(this), name.into_param().abi(), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0, value: Param1) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Insert)(::windows_core::Interface::as_raw(this), key.into_param().abi(), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), key.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpContentHeaderCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpContentHeaderCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpContentHeaderCollection {}
impl ::core::fmt::Debug for HttpContentHeaderCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpContentHeaderCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpContentHeaderCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentHeaderCollection;{40612a44-47ae-4b7e-9124-69628b64aa18})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpContentHeaderCollection {
    type Vtable = IHttpContentHeaderCollection_Vtbl;
    const IID: ::windows_core::GUID = <IHttpContentHeaderCollection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpContentHeaderCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentHeaderCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpContentHeaderCollection {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpContentHeaderCollection {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl ::core::convert::From<HttpContentHeaderCollection> for ::windows_core::IUnknown {
    fn from(value: HttpContentHeaderCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentHeaderCollection> for ::windows_core::IUnknown {
    fn from(value: &HttpContentHeaderCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpContentHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpContentHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpContentHeaderCollection> for ::windows_core::IInspectable {
    fn from(value: HttpContentHeaderCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentHeaderCollection> for ::windows_core::IInspectable {
    fn from(value: &HttpContentHeaderCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpContentHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpContentHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpContentHeaderCollection> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpContentHeaderCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpContentHeaderCollection> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpContentHeaderCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> for HttpContentHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> for &HttpContentHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpContentHeaderCollection> for ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpContentHeaderCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpContentHeaderCollection> for ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpContentHeaderCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> for HttpContentHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> for &HttpContentHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpContentHeaderCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpContentHeaderCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpContentHeaderCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpContentHeaderCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpContentHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpContentHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpContentHeaderCollection {}
unsafe impl ::core::marker::Sync for HttpContentHeaderCollection {}
#[repr(transparent)]
pub struct HttpContentRangeHeaderValue(::windows_core::IUnknown);
impl HttpContentRangeHeaderValue {
    pub fn FirstBytePosition(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FirstBytePosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u64>>(result__)
        }
    }
    pub fn LastBytePosition(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LastBytePosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u64>>(result__)
        }
    }
    pub fn Length(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u64>>(result__)
        }
    }
    pub fn Unit(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Unit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetUnit<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUnit)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CreateFromLength(length: u64) -> ::windows_core::Result<HttpContentRangeHeaderValue> {
        Self::IHttpContentRangeHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromLength)(::windows_core::Interface::as_raw(this), length, result__.as_mut_ptr()).from_abi::<HttpContentRangeHeaderValue>(result__)
        })
    }
    pub fn CreateFromRange(from: u64, to: u64) -> ::windows_core::Result<HttpContentRangeHeaderValue> {
        Self::IHttpContentRangeHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromRange)(::windows_core::Interface::as_raw(this), from, to, result__.as_mut_ptr()).from_abi::<HttpContentRangeHeaderValue>(result__)
        })
    }
    pub fn CreateFromRangeWithLength(from: u64, to: u64, length: u64) -> ::windows_core::Result<HttpContentRangeHeaderValue> {
        Self::IHttpContentRangeHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromRangeWithLength)(::windows_core::Interface::as_raw(this), from, to, length, result__.as_mut_ptr()).from_abi::<HttpContentRangeHeaderValue>(result__)
        })
    }
    pub fn Parse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0) -> ::windows_core::Result<HttpContentRangeHeaderValue> {
        Self::IHttpContentRangeHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpContentRangeHeaderValue>(result__)
        })
    }
    pub fn TryParse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0, contentrangeheadervalue: &mut ::core::option::Option<HttpContentRangeHeaderValue>) -> ::windows_core::Result<bool> {
        Self::IHttpContentRangeHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), contentrangeheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpContentRangeHeaderValueFactory<R, F: FnOnce(&IHttpContentRangeHeaderValueFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpContentRangeHeaderValue, IHttpContentRangeHeaderValueFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpContentRangeHeaderValueStatics<R, F: FnOnce(&IHttpContentRangeHeaderValueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpContentRangeHeaderValue, IHttpContentRangeHeaderValueStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpContentRangeHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpContentRangeHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpContentRangeHeaderValue {}
impl ::core::fmt::Debug for HttpContentRangeHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpContentRangeHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpContentRangeHeaderValue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentRangeHeaderValue;{04d967d3-a4f6-495c-9530-8579fcba8aa9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpContentRangeHeaderValue {
    type Vtable = IHttpContentRangeHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = <IHttpContentRangeHeaderValue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpContentRangeHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentRangeHeaderValue";
}
impl ::core::convert::From<HttpContentRangeHeaderValue> for ::windows_core::IUnknown {
    fn from(value: HttpContentRangeHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentRangeHeaderValue> for ::windows_core::IUnknown {
    fn from(value: &HttpContentRangeHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpContentRangeHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpContentRangeHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpContentRangeHeaderValue> for ::windows_core::IInspectable {
    fn from(value: HttpContentRangeHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpContentRangeHeaderValue> for ::windows_core::IInspectable {
    fn from(value: &HttpContentRangeHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpContentRangeHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpContentRangeHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpContentRangeHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpContentRangeHeaderValue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpContentRangeHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpContentRangeHeaderValue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpContentRangeHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpContentRangeHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpContentRangeHeaderValue {}
unsafe impl ::core::marker::Sync for HttpContentRangeHeaderValue {}
#[repr(transparent)]
pub struct HttpCookiePairHeaderValue(::windows_core::IUnknown);
impl HttpCookiePairHeaderValue {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CreateFromName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(name: Param0) -> ::windows_core::Result<HttpCookiePairHeaderValue> {
        Self::IHttpCookiePairHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromName)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpCookiePairHeaderValue>(result__)
        })
    }
    pub fn CreateFromNameWithValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(name: Param0, value: Param1) -> ::windows_core::Result<HttpCookiePairHeaderValue> {
        Self::IHttpCookiePairHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromNameWithValue)(::windows_core::Interface::as_raw(this), name.into_param().abi(), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpCookiePairHeaderValue>(result__)
        })
    }
    pub fn Parse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0) -> ::windows_core::Result<HttpCookiePairHeaderValue> {
        Self::IHttpCookiePairHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpCookiePairHeaderValue>(result__)
        })
    }
    pub fn TryParse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0, cookiepairheadervalue: &mut ::core::option::Option<HttpCookiePairHeaderValue>) -> ::windows_core::Result<bool> {
        Self::IHttpCookiePairHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), cookiepairheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpCookiePairHeaderValueFactory<R, F: FnOnce(&IHttpCookiePairHeaderValueFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpCookiePairHeaderValue, IHttpCookiePairHeaderValueFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpCookiePairHeaderValueStatics<R, F: FnOnce(&IHttpCookiePairHeaderValueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpCookiePairHeaderValue, IHttpCookiePairHeaderValueStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpCookiePairHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpCookiePairHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpCookiePairHeaderValue {}
impl ::core::fmt::Debug for HttpCookiePairHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCookiePairHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpCookiePairHeaderValue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpCookiePairHeaderValue;{cbd46217-4b29-412b-bd90-b3d814ab8e1b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpCookiePairHeaderValue {
    type Vtable = IHttpCookiePairHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = <IHttpCookiePairHeaderValue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpCookiePairHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpCookiePairHeaderValue";
}
impl ::core::convert::From<HttpCookiePairHeaderValue> for ::windows_core::IUnknown {
    fn from(value: HttpCookiePairHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCookiePairHeaderValue> for ::windows_core::IUnknown {
    fn from(value: &HttpCookiePairHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpCookiePairHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpCookiePairHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpCookiePairHeaderValue> for ::windows_core::IInspectable {
    fn from(value: HttpCookiePairHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCookiePairHeaderValue> for ::windows_core::IInspectable {
    fn from(value: &HttpCookiePairHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpCookiePairHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpCookiePairHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpCookiePairHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpCookiePairHeaderValue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpCookiePairHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpCookiePairHeaderValue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpCookiePairHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpCookiePairHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpCookiePairHeaderValue {}
unsafe impl ::core::marker::Sync for HttpCookiePairHeaderValue {}
#[repr(transparent)]
pub struct HttpCookiePairHeaderValueCollection(::windows_core::IUnknown);
impl HttpCookiePairHeaderValueCollection {
    pub fn ParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi()).ok() }
    }
    pub fn TryParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<HttpCookiePairHeaderValue>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<HttpCookiePairHeaderValue>>(result__)
        }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<HttpCookiePairHeaderValue> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<HttpCookiePairHeaderValue>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<HttpCookiePairHeaderValue>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<HttpCookiePairHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, HttpCookiePairHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, HttpCookiePairHeaderValue>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, HttpCookiePairHeaderValue>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, HttpCookiePairHeaderValue>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<HttpCookiePairHeaderValue>]) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<HttpCookiePairHeaderValue>]) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpCookiePairHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpCookiePairHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpCookiePairHeaderValueCollection {}
impl ::core::fmt::Debug for HttpCookiePairHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCookiePairHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpCookiePairHeaderValueCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpCookiePairHeaderValueCollection;{f3f44350-581e-4ecc-9f59-e507d04f06e6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpCookiePairHeaderValueCollection {
    type Vtable = IHttpCookiePairHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = <IHttpCookiePairHeaderValueCollection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpCookiePairHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpCookiePairHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpCookiePairHeaderValueCollection {
    type Item = HttpCookiePairHeaderValue;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpCookiePairHeaderValueCollection {
    type Item = HttpCookiePairHeaderValue;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpCookiePairHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: HttpCookiePairHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCookiePairHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: &HttpCookiePairHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpCookiePairHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: HttpCookiePairHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCookiePairHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: &HttpCookiePairHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpCookiePairHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<HttpCookiePairHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpCookiePairHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpCookiePairHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<HttpCookiePairHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpCookiePairHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<HttpCookiePairHeaderValue>> for HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<HttpCookiePairHeaderValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<HttpCookiePairHeaderValue>> for &HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<HttpCookiePairHeaderValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<HttpCookiePairHeaderValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpCookiePairHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpCookiePairHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpCookiePairHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpCookiePairHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpCookiePairHeaderValueCollection> for ::winrt_foundation::Collections::IVector<HttpCookiePairHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpCookiePairHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpCookiePairHeaderValueCollection> for ::winrt_foundation::Collections::IVector<HttpCookiePairHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpCookiePairHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<HttpCookiePairHeaderValue>> for HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<HttpCookiePairHeaderValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<HttpCookiePairHeaderValue>> for &HttpCookiePairHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<HttpCookiePairHeaderValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<HttpCookiePairHeaderValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpCookiePairHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpCookiePairHeaderValueCollection {}
#[repr(transparent)]
pub struct HttpCredentialsHeaderValue(::windows_core::IUnknown);
impl HttpCredentialsHeaderValue {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Parameters(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parameters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>>(result__)
        }
    }
    pub fn Scheme(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Scheme)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Token(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Token)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CreateFromScheme<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(scheme: Param0) -> ::windows_core::Result<HttpCredentialsHeaderValue> {
        Self::IHttpCredentialsHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromScheme)(::windows_core::Interface::as_raw(this), scheme.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpCredentialsHeaderValue>(result__)
        })
    }
    pub fn CreateFromSchemeWithToken<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(scheme: Param0, token: Param1) -> ::windows_core::Result<HttpCredentialsHeaderValue> {
        Self::IHttpCredentialsHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromSchemeWithToken)(::windows_core::Interface::as_raw(this), scheme.into_param().abi(), token.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpCredentialsHeaderValue>(result__)
        })
    }
    pub fn Parse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0) -> ::windows_core::Result<HttpCredentialsHeaderValue> {
        Self::IHttpCredentialsHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpCredentialsHeaderValue>(result__)
        })
    }
    pub fn TryParse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0, credentialsheadervalue: &mut ::core::option::Option<HttpCredentialsHeaderValue>) -> ::windows_core::Result<bool> {
        Self::IHttpCredentialsHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), credentialsheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpCredentialsHeaderValueFactory<R, F: FnOnce(&IHttpCredentialsHeaderValueFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpCredentialsHeaderValue, IHttpCredentialsHeaderValueFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpCredentialsHeaderValueStatics<R, F: FnOnce(&IHttpCredentialsHeaderValueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpCredentialsHeaderValue, IHttpCredentialsHeaderValueStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpCredentialsHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpCredentialsHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpCredentialsHeaderValue {}
impl ::core::fmt::Debug for HttpCredentialsHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCredentialsHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpCredentialsHeaderValue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpCredentialsHeaderValue;{c34cc3cb-542e-4177-a6c7-b674ce193fbf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpCredentialsHeaderValue {
    type Vtable = IHttpCredentialsHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = <IHttpCredentialsHeaderValue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpCredentialsHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpCredentialsHeaderValue";
}
impl ::core::convert::From<HttpCredentialsHeaderValue> for ::windows_core::IUnknown {
    fn from(value: HttpCredentialsHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCredentialsHeaderValue> for ::windows_core::IUnknown {
    fn from(value: &HttpCredentialsHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpCredentialsHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpCredentialsHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpCredentialsHeaderValue> for ::windows_core::IInspectable {
    fn from(value: HttpCredentialsHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCredentialsHeaderValue> for ::windows_core::IInspectable {
    fn from(value: &HttpCredentialsHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpCredentialsHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpCredentialsHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpCredentialsHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpCredentialsHeaderValue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpCredentialsHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpCredentialsHeaderValue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpCredentialsHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpCredentialsHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpCredentialsHeaderValue {}
unsafe impl ::core::marker::Sync for HttpCredentialsHeaderValue {}
#[repr(transparent)]
pub struct HttpDateOrDeltaHeaderValue(::windows_core::IUnknown);
impl HttpDateOrDeltaHeaderValue {
    pub fn Date(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Date)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn Delta(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Delta)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn Parse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0) -> ::windows_core::Result<HttpDateOrDeltaHeaderValue> {
        Self::IHttpDateOrDeltaHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpDateOrDeltaHeaderValue>(result__)
        })
    }
    pub fn TryParse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0, dateordeltaheadervalue: &mut ::core::option::Option<HttpDateOrDeltaHeaderValue>) -> ::windows_core::Result<bool> {
        Self::IHttpDateOrDeltaHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), dateordeltaheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpDateOrDeltaHeaderValueStatics<R, F: FnOnce(&IHttpDateOrDeltaHeaderValueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpDateOrDeltaHeaderValue, IHttpDateOrDeltaHeaderValueStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpDateOrDeltaHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpDateOrDeltaHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpDateOrDeltaHeaderValue {}
impl ::core::fmt::Debug for HttpDateOrDeltaHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpDateOrDeltaHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpDateOrDeltaHeaderValue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpDateOrDeltaHeaderValue;{eafcaa6a-c4dc-49e2-a27d-043adf5867a3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpDateOrDeltaHeaderValue {
    type Vtable = IHttpDateOrDeltaHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = <IHttpDateOrDeltaHeaderValue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpDateOrDeltaHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpDateOrDeltaHeaderValue";
}
impl ::core::convert::From<HttpDateOrDeltaHeaderValue> for ::windows_core::IUnknown {
    fn from(value: HttpDateOrDeltaHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDateOrDeltaHeaderValue> for ::windows_core::IUnknown {
    fn from(value: &HttpDateOrDeltaHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpDateOrDeltaHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpDateOrDeltaHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpDateOrDeltaHeaderValue> for ::windows_core::IInspectable {
    fn from(value: HttpDateOrDeltaHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpDateOrDeltaHeaderValue> for ::windows_core::IInspectable {
    fn from(value: &HttpDateOrDeltaHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpDateOrDeltaHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpDateOrDeltaHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpDateOrDeltaHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpDateOrDeltaHeaderValue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpDateOrDeltaHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpDateOrDeltaHeaderValue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpDateOrDeltaHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpDateOrDeltaHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpDateOrDeltaHeaderValue {}
unsafe impl ::core::marker::Sync for HttpDateOrDeltaHeaderValue {}
#[repr(transparent)]
pub struct HttpExpectationHeaderValue(::windows_core::IUnknown);
impl HttpExpectationHeaderValue {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Parameters(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parameters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>>(result__)
        }
    }
    pub fn CreateFromName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(name: Param0) -> ::windows_core::Result<HttpExpectationHeaderValue> {
        Self::IHttpExpectationHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromName)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpExpectationHeaderValue>(result__)
        })
    }
    pub fn CreateFromNameWithValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(name: Param0, value: Param1) -> ::windows_core::Result<HttpExpectationHeaderValue> {
        Self::IHttpExpectationHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromNameWithValue)(::windows_core::Interface::as_raw(this), name.into_param().abi(), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpExpectationHeaderValue>(result__)
        })
    }
    pub fn Parse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0) -> ::windows_core::Result<HttpExpectationHeaderValue> {
        Self::IHttpExpectationHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpExpectationHeaderValue>(result__)
        })
    }
    pub fn TryParse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0, expectationheadervalue: &mut ::core::option::Option<HttpExpectationHeaderValue>) -> ::windows_core::Result<bool> {
        Self::IHttpExpectationHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), expectationheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpExpectationHeaderValueFactory<R, F: FnOnce(&IHttpExpectationHeaderValueFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpExpectationHeaderValue, IHttpExpectationHeaderValueFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpExpectationHeaderValueStatics<R, F: FnOnce(&IHttpExpectationHeaderValueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpExpectationHeaderValue, IHttpExpectationHeaderValueStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpExpectationHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpExpectationHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpExpectationHeaderValue {}
impl ::core::fmt::Debug for HttpExpectationHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpExpectationHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpExpectationHeaderValue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpExpectationHeaderValue;{4ce585cd-3a99-43af-a2e6-ec232fea9658})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpExpectationHeaderValue {
    type Vtable = IHttpExpectationHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = <IHttpExpectationHeaderValue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpExpectationHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpExpectationHeaderValue";
}
impl ::core::convert::From<HttpExpectationHeaderValue> for ::windows_core::IUnknown {
    fn from(value: HttpExpectationHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpExpectationHeaderValue> for ::windows_core::IUnknown {
    fn from(value: &HttpExpectationHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpExpectationHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpExpectationHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpExpectationHeaderValue> for ::windows_core::IInspectable {
    fn from(value: HttpExpectationHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpExpectationHeaderValue> for ::windows_core::IInspectable {
    fn from(value: &HttpExpectationHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpExpectationHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpExpectationHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpExpectationHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpExpectationHeaderValue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpExpectationHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpExpectationHeaderValue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpExpectationHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpExpectationHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpExpectationHeaderValue {}
unsafe impl ::core::marker::Sync for HttpExpectationHeaderValue {}
#[repr(transparent)]
pub struct HttpExpectationHeaderValueCollection(::windows_core::IUnknown);
impl HttpExpectationHeaderValueCollection {
    pub fn ParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi()).ok() }
    }
    pub fn TryParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<HttpExpectationHeaderValue>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<HttpExpectationHeaderValue>>(result__)
        }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<HttpExpectationHeaderValue> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<HttpExpectationHeaderValue>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<HttpExpectationHeaderValue>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<HttpExpectationHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, HttpExpectationHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, HttpExpectationHeaderValue>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, HttpExpectationHeaderValue>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, HttpExpectationHeaderValue>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<HttpExpectationHeaderValue>]) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<HttpExpectationHeaderValue>]) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpExpectationHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpExpectationHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpExpectationHeaderValueCollection {}
impl ::core::fmt::Debug for HttpExpectationHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpExpectationHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpExpectationHeaderValueCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpExpectationHeaderValueCollection;{e78521b3-a0e2-4ac4-9e66-79706cb9fd58})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpExpectationHeaderValueCollection {
    type Vtable = IHttpExpectationHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = <IHttpExpectationHeaderValueCollection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpExpectationHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpExpectationHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpExpectationHeaderValueCollection {
    type Item = HttpExpectationHeaderValue;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpExpectationHeaderValueCollection {
    type Item = HttpExpectationHeaderValue;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpExpectationHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: HttpExpectationHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpExpectationHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: &HttpExpectationHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpExpectationHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: HttpExpectationHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpExpectationHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: &HttpExpectationHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpExpectationHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<HttpExpectationHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpExpectationHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpExpectationHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<HttpExpectationHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpExpectationHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<HttpExpectationHeaderValue>> for HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<HttpExpectationHeaderValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<HttpExpectationHeaderValue>> for &HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<HttpExpectationHeaderValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<HttpExpectationHeaderValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpExpectationHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpExpectationHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpExpectationHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpExpectationHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpExpectationHeaderValueCollection> for ::winrt_foundation::Collections::IVector<HttpExpectationHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpExpectationHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpExpectationHeaderValueCollection> for ::winrt_foundation::Collections::IVector<HttpExpectationHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpExpectationHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<HttpExpectationHeaderValue>> for HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<HttpExpectationHeaderValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<HttpExpectationHeaderValue>> for &HttpExpectationHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<HttpExpectationHeaderValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<HttpExpectationHeaderValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpExpectationHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpExpectationHeaderValueCollection {}
#[repr(transparent)]
pub struct HttpLanguageHeaderValueCollection(::windows_core::IUnknown);
impl HttpLanguageHeaderValueCollection {
    pub fn ParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi()).ok() }
    }
    pub fn TryParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<::winrt_globalization::Language>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<::winrt_globalization::Language>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<::winrt_globalization::Language>>(result__)
        }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<::winrt_globalization::Language> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<::winrt_globalization::Language>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<::winrt_globalization::Language>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<::winrt_globalization::Language>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_globalization::Language>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<::winrt_globalization::Language>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_globalization::Language>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_globalization::Language>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<::winrt_globalization::Language>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_globalization::Language>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<::winrt_globalization::Language>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_globalization::Language>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<::winrt_globalization::Language>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<::winrt_globalization::Language>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_globalization::Language>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<::winrt_globalization::Language>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<::winrt_globalization::Language>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<::winrt_globalization::Language>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<::winrt_globalization::Language>]) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<::winrt_globalization::Language>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<::winrt_globalization::Language>]) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<::winrt_globalization::Language>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpLanguageHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpLanguageHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpLanguageHeaderValueCollection {}
impl ::core::fmt::Debug for HttpLanguageHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpLanguageHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpLanguageHeaderValueCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpLanguageHeaderValueCollection;{9ebd7ca3-8219-44f6-9902-8c56dfd3340c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpLanguageHeaderValueCollection {
    type Vtable = IHttpLanguageHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = <IHttpLanguageHeaderValueCollection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpLanguageHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpLanguageHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpLanguageHeaderValueCollection {
    type Item = ::winrt_globalization::Language;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpLanguageHeaderValueCollection {
    type Item = ::winrt_globalization::Language;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpLanguageHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: HttpLanguageHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpLanguageHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: &HttpLanguageHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpLanguageHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: HttpLanguageHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpLanguageHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: &HttpLanguageHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
impl ::core::convert::TryFrom<HttpLanguageHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<::winrt_globalization::Language> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpLanguageHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
impl ::core::convert::TryFrom<&HttpLanguageHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<::winrt_globalization::Language> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpLanguageHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_globalization::Language>> for HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_globalization::Language>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_globalization::Language>> for &HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_globalization::Language>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<::winrt_globalization::Language>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpLanguageHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpLanguageHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpLanguageHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpLanguageHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
impl ::core::convert::TryFrom<HttpLanguageHeaderValueCollection> for ::winrt_foundation::Collections::IVector<::winrt_globalization::Language> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpLanguageHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
impl ::core::convert::TryFrom<&HttpLanguageHeaderValueCollection> for ::winrt_foundation::Collections::IVector<::winrt_globalization::Language> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpLanguageHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<::winrt_globalization::Language>> for HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<::winrt_globalization::Language>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<::winrt_globalization::Language>> for &HttpLanguageHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<::winrt_globalization::Language>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<::winrt_globalization::Language>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpLanguageHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpLanguageHeaderValueCollection {}
#[repr(transparent)]
pub struct HttpLanguageRangeWithQualityHeaderValue(::windows_core::IUnknown);
impl HttpLanguageRangeWithQualityHeaderValue {
    pub fn LanguageRange(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).LanguageRange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Quality(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Quality)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn CreateFromLanguageRange<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(languagerange: Param0) -> ::windows_core::Result<HttpLanguageRangeWithQualityHeaderValue> {
        Self::IHttpLanguageRangeWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromLanguageRange)(::windows_core::Interface::as_raw(this), languagerange.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpLanguageRangeWithQualityHeaderValue>(result__)
        })
    }
    pub fn CreateFromLanguageRangeWithQuality<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(languagerange: Param0, quality: f64) -> ::windows_core::Result<HttpLanguageRangeWithQualityHeaderValue> {
        Self::IHttpLanguageRangeWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromLanguageRangeWithQuality)(::windows_core::Interface::as_raw(this), languagerange.into_param().abi(), quality, result__.as_mut_ptr()).from_abi::<HttpLanguageRangeWithQualityHeaderValue>(result__)
        })
    }
    pub fn Parse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0) -> ::windows_core::Result<HttpLanguageRangeWithQualityHeaderValue> {
        Self::IHttpLanguageRangeWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpLanguageRangeWithQualityHeaderValue>(result__)
        })
    }
    pub fn TryParse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0, languagerangewithqualityheadervalue: &mut ::core::option::Option<HttpLanguageRangeWithQualityHeaderValue>) -> ::windows_core::Result<bool> {
        Self::IHttpLanguageRangeWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), languagerangewithqualityheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpLanguageRangeWithQualityHeaderValueFactory<R, F: FnOnce(&IHttpLanguageRangeWithQualityHeaderValueFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpLanguageRangeWithQualityHeaderValue, IHttpLanguageRangeWithQualityHeaderValueFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpLanguageRangeWithQualityHeaderValueStatics<R, F: FnOnce(&IHttpLanguageRangeWithQualityHeaderValueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpLanguageRangeWithQualityHeaderValue, IHttpLanguageRangeWithQualityHeaderValueStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpLanguageRangeWithQualityHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpLanguageRangeWithQualityHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpLanguageRangeWithQualityHeaderValue {}
impl ::core::fmt::Debug for HttpLanguageRangeWithQualityHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpLanguageRangeWithQualityHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpLanguageRangeWithQualityHeaderValue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpLanguageRangeWithQualityHeaderValue;{7256e102-0080-4db4-a083-7de7b2e5ba4c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpLanguageRangeWithQualityHeaderValue {
    type Vtable = IHttpLanguageRangeWithQualityHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = <IHttpLanguageRangeWithQualityHeaderValue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpLanguageRangeWithQualityHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpLanguageRangeWithQualityHeaderValue";
}
impl ::core::convert::From<HttpLanguageRangeWithQualityHeaderValue> for ::windows_core::IUnknown {
    fn from(value: HttpLanguageRangeWithQualityHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpLanguageRangeWithQualityHeaderValue> for ::windows_core::IUnknown {
    fn from(value: &HttpLanguageRangeWithQualityHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpLanguageRangeWithQualityHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpLanguageRangeWithQualityHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpLanguageRangeWithQualityHeaderValue> for ::windows_core::IInspectable {
    fn from(value: HttpLanguageRangeWithQualityHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpLanguageRangeWithQualityHeaderValue> for ::windows_core::IInspectable {
    fn from(value: &HttpLanguageRangeWithQualityHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpLanguageRangeWithQualityHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpLanguageRangeWithQualityHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpLanguageRangeWithQualityHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpLanguageRangeWithQualityHeaderValue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpLanguageRangeWithQualityHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpLanguageRangeWithQualityHeaderValue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpLanguageRangeWithQualityHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpLanguageRangeWithQualityHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpLanguageRangeWithQualityHeaderValue {}
unsafe impl ::core::marker::Sync for HttpLanguageRangeWithQualityHeaderValue {}
#[repr(transparent)]
pub struct HttpLanguageRangeWithQualityHeaderValueCollection(::windows_core::IUnknown);
impl HttpLanguageRangeWithQualityHeaderValueCollection {
    pub fn ParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi()).ok() }
    }
    pub fn TryParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<HttpLanguageRangeWithQualityHeaderValue>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<HttpLanguageRangeWithQualityHeaderValue>>(result__)
        }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<HttpLanguageRangeWithQualityHeaderValue> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<HttpLanguageRangeWithQualityHeaderValue>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<HttpLanguageRangeWithQualityHeaderValue>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<HttpLanguageRangeWithQualityHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, HttpLanguageRangeWithQualityHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, HttpLanguageRangeWithQualityHeaderValue>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, HttpLanguageRangeWithQualityHeaderValue>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, HttpLanguageRangeWithQualityHeaderValue>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<HttpLanguageRangeWithQualityHeaderValue>]) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<HttpLanguageRangeWithQualityHeaderValue>]) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpLanguageRangeWithQualityHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpLanguageRangeWithQualityHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpLanguageRangeWithQualityHeaderValueCollection {}
impl ::core::fmt::Debug for HttpLanguageRangeWithQualityHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpLanguageRangeWithQualityHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpLanguageRangeWithQualityHeaderValueCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpLanguageRangeWithQualityHeaderValueCollection;{885d5abd-4b4f-480a-89ce-8aedcee6e3a0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpLanguageRangeWithQualityHeaderValueCollection {
    type Vtable = IHttpLanguageRangeWithQualityHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = <IHttpLanguageRangeWithQualityHeaderValueCollection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpLanguageRangeWithQualityHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpLanguageRangeWithQualityHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpLanguageRangeWithQualityHeaderValueCollection {
    type Item = HttpLanguageRangeWithQualityHeaderValue;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpLanguageRangeWithQualityHeaderValueCollection {
    type Item = HttpLanguageRangeWithQualityHeaderValue;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpLanguageRangeWithQualityHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: HttpLanguageRangeWithQualityHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpLanguageRangeWithQualityHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: &HttpLanguageRangeWithQualityHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpLanguageRangeWithQualityHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: HttpLanguageRangeWithQualityHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpLanguageRangeWithQualityHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: &HttpLanguageRangeWithQualityHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpLanguageRangeWithQualityHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpLanguageRangeWithQualityHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue>> for HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue>> for &HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpLanguageRangeWithQualityHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpLanguageRangeWithQualityHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpLanguageRangeWithQualityHeaderValueCollection> for ::winrt_foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpLanguageRangeWithQualityHeaderValueCollection> for ::winrt_foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>> for HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>> for &HttpLanguageRangeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpLanguageRangeWithQualityHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpLanguageRangeWithQualityHeaderValueCollection {}
#[repr(transparent)]
pub struct HttpMediaTypeHeaderValue(::windows_core::IUnknown);
impl HttpMediaTypeHeaderValue {
    pub fn CharSet(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CharSet)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetCharSet<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCharSet)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MediaType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).MediaType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetMediaType<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMediaType)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Parameters(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parameters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(mediatype: Param0) -> ::windows_core::Result<HttpMediaTypeHeaderValue> {
        Self::IHttpMediaTypeHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), mediatype.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpMediaTypeHeaderValue>(result__)
        })
    }
    pub fn Parse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0) -> ::windows_core::Result<HttpMediaTypeHeaderValue> {
        Self::IHttpMediaTypeHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpMediaTypeHeaderValue>(result__)
        })
    }
    pub fn TryParse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0, mediatypeheadervalue: &mut ::core::option::Option<HttpMediaTypeHeaderValue>) -> ::windows_core::Result<bool> {
        Self::IHttpMediaTypeHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), mediatypeheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpMediaTypeHeaderValueFactory<R, F: FnOnce(&IHttpMediaTypeHeaderValueFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpMediaTypeHeaderValue, IHttpMediaTypeHeaderValueFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpMediaTypeHeaderValueStatics<R, F: FnOnce(&IHttpMediaTypeHeaderValueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpMediaTypeHeaderValue, IHttpMediaTypeHeaderValueStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpMediaTypeHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpMediaTypeHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpMediaTypeHeaderValue {}
impl ::core::fmt::Debug for HttpMediaTypeHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpMediaTypeHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpMediaTypeHeaderValue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpMediaTypeHeaderValue;{16b28533-e728-4fcb-bdb0-08a431a14844})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpMediaTypeHeaderValue {
    type Vtable = IHttpMediaTypeHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = <IHttpMediaTypeHeaderValue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpMediaTypeHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpMediaTypeHeaderValue";
}
impl ::core::convert::From<HttpMediaTypeHeaderValue> for ::windows_core::IUnknown {
    fn from(value: HttpMediaTypeHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMediaTypeHeaderValue> for ::windows_core::IUnknown {
    fn from(value: &HttpMediaTypeHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpMediaTypeHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpMediaTypeHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpMediaTypeHeaderValue> for ::windows_core::IInspectable {
    fn from(value: HttpMediaTypeHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMediaTypeHeaderValue> for ::windows_core::IInspectable {
    fn from(value: &HttpMediaTypeHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpMediaTypeHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpMediaTypeHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpMediaTypeHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpMediaTypeHeaderValue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpMediaTypeHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpMediaTypeHeaderValue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpMediaTypeHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpMediaTypeHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpMediaTypeHeaderValue {}
unsafe impl ::core::marker::Sync for HttpMediaTypeHeaderValue {}
#[repr(transparent)]
pub struct HttpMediaTypeWithQualityHeaderValue(::windows_core::IUnknown);
impl HttpMediaTypeWithQualityHeaderValue {
    pub fn CharSet(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CharSet)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetCharSet<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCharSet)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MediaType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).MediaType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetMediaType<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMediaType)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Parameters(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parameters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>>(result__)
        }
    }
    pub fn Quality(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Quality)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn SetQuality<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<f64>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetQuality)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CreateFromMediaType<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(mediatype: Param0) -> ::windows_core::Result<HttpMediaTypeWithQualityHeaderValue> {
        Self::IHttpMediaTypeWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromMediaType)(::windows_core::Interface::as_raw(this), mediatype.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpMediaTypeWithQualityHeaderValue>(result__)
        })
    }
    pub fn CreateFromMediaTypeWithQuality<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(mediatype: Param0, quality: f64) -> ::windows_core::Result<HttpMediaTypeWithQualityHeaderValue> {
        Self::IHttpMediaTypeWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromMediaTypeWithQuality)(::windows_core::Interface::as_raw(this), mediatype.into_param().abi(), quality, result__.as_mut_ptr()).from_abi::<HttpMediaTypeWithQualityHeaderValue>(result__)
        })
    }
    pub fn Parse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0) -> ::windows_core::Result<HttpMediaTypeWithQualityHeaderValue> {
        Self::IHttpMediaTypeWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpMediaTypeWithQualityHeaderValue>(result__)
        })
    }
    pub fn TryParse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0, mediatypewithqualityheadervalue: &mut ::core::option::Option<HttpMediaTypeWithQualityHeaderValue>) -> ::windows_core::Result<bool> {
        Self::IHttpMediaTypeWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), mediatypewithqualityheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpMediaTypeWithQualityHeaderValueFactory<R, F: FnOnce(&IHttpMediaTypeWithQualityHeaderValueFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpMediaTypeWithQualityHeaderValue, IHttpMediaTypeWithQualityHeaderValueFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpMediaTypeWithQualityHeaderValueStatics<R, F: FnOnce(&IHttpMediaTypeWithQualityHeaderValueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpMediaTypeWithQualityHeaderValue, IHttpMediaTypeWithQualityHeaderValueStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpMediaTypeWithQualityHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpMediaTypeWithQualityHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpMediaTypeWithQualityHeaderValue {}
impl ::core::fmt::Debug for HttpMediaTypeWithQualityHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpMediaTypeWithQualityHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpMediaTypeWithQualityHeaderValue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpMediaTypeWithQualityHeaderValue;{188d5e32-76be-44a0-b1cd-2074bded2dde})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpMediaTypeWithQualityHeaderValue {
    type Vtable = IHttpMediaTypeWithQualityHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = <IHttpMediaTypeWithQualityHeaderValue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpMediaTypeWithQualityHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpMediaTypeWithQualityHeaderValue";
}
impl ::core::convert::From<HttpMediaTypeWithQualityHeaderValue> for ::windows_core::IUnknown {
    fn from(value: HttpMediaTypeWithQualityHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMediaTypeWithQualityHeaderValue> for ::windows_core::IUnknown {
    fn from(value: &HttpMediaTypeWithQualityHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpMediaTypeWithQualityHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpMediaTypeWithQualityHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpMediaTypeWithQualityHeaderValue> for ::windows_core::IInspectable {
    fn from(value: HttpMediaTypeWithQualityHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMediaTypeWithQualityHeaderValue> for ::windows_core::IInspectable {
    fn from(value: &HttpMediaTypeWithQualityHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpMediaTypeWithQualityHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpMediaTypeWithQualityHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpMediaTypeWithQualityHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpMediaTypeWithQualityHeaderValue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpMediaTypeWithQualityHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpMediaTypeWithQualityHeaderValue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpMediaTypeWithQualityHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpMediaTypeWithQualityHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpMediaTypeWithQualityHeaderValue {}
unsafe impl ::core::marker::Sync for HttpMediaTypeWithQualityHeaderValue {}
#[repr(transparent)]
pub struct HttpMediaTypeWithQualityHeaderValueCollection(::windows_core::IUnknown);
impl HttpMediaTypeWithQualityHeaderValueCollection {
    pub fn ParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi()).ok() }
    }
    pub fn TryParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<HttpMediaTypeWithQualityHeaderValue>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<HttpMediaTypeWithQualityHeaderValue>>(result__)
        }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<HttpMediaTypeWithQualityHeaderValue> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<HttpMediaTypeWithQualityHeaderValue>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<HttpMediaTypeWithQualityHeaderValue>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<HttpMediaTypeWithQualityHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, HttpMediaTypeWithQualityHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, HttpMediaTypeWithQualityHeaderValue>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, HttpMediaTypeWithQualityHeaderValue>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, HttpMediaTypeWithQualityHeaderValue>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<HttpMediaTypeWithQualityHeaderValue>]) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<HttpMediaTypeWithQualityHeaderValue>]) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpMediaTypeWithQualityHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpMediaTypeWithQualityHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpMediaTypeWithQualityHeaderValueCollection {}
impl ::core::fmt::Debug for HttpMediaTypeWithQualityHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpMediaTypeWithQualityHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpMediaTypeWithQualityHeaderValueCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpMediaTypeWithQualityHeaderValueCollection;{3c0c6b73-1342-4587-a056-18d02ff67165})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpMediaTypeWithQualityHeaderValueCollection {
    type Vtable = IHttpMediaTypeWithQualityHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = <IHttpMediaTypeWithQualityHeaderValueCollection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpMediaTypeWithQualityHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpMediaTypeWithQualityHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpMediaTypeWithQualityHeaderValueCollection {
    type Item = HttpMediaTypeWithQualityHeaderValue;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpMediaTypeWithQualityHeaderValueCollection {
    type Item = HttpMediaTypeWithQualityHeaderValue;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpMediaTypeWithQualityHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: HttpMediaTypeWithQualityHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMediaTypeWithQualityHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: &HttpMediaTypeWithQualityHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpMediaTypeWithQualityHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: HttpMediaTypeWithQualityHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMediaTypeWithQualityHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: &HttpMediaTypeWithQualityHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpMediaTypeWithQualityHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpMediaTypeWithQualityHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue>> for HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue>> for &HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpMediaTypeWithQualityHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpMediaTypeWithQualityHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpMediaTypeWithQualityHeaderValueCollection> for ::winrt_foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpMediaTypeWithQualityHeaderValueCollection> for ::winrt_foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>> for HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>> for &HttpMediaTypeWithQualityHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpMediaTypeWithQualityHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpMediaTypeWithQualityHeaderValueCollection {}
#[repr(transparent)]
pub struct HttpMethodHeaderValueCollection(::windows_core::IUnknown);
impl HttpMethodHeaderValueCollection {
    pub fn ParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi()).ok() }
    }
    pub fn TryParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<super::HttpMethod>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<super::HttpMethod>>(result__)
        }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<super::HttpMethod> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<super::HttpMethod>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<super::HttpMethod>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<super::HttpMethod>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, super::HttpMethod>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, super::HttpMethod>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, super::HttpMethod>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, super::HttpMethod>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<super::HttpMethod>]) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<super::HttpMethod>]) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpMethodHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpMethodHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpMethodHeaderValueCollection {}
impl ::core::fmt::Debug for HttpMethodHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpMethodHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpMethodHeaderValueCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpMethodHeaderValueCollection;{43bc3ff4-6119-4adf-938c-34bfffcf92ed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpMethodHeaderValueCollection {
    type Vtable = IHttpMethodHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = <IHttpMethodHeaderValueCollection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpMethodHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpMethodHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpMethodHeaderValueCollection {
    type Item = super::HttpMethod;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpMethodHeaderValueCollection {
    type Item = super::HttpMethod;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpMethodHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: HttpMethodHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMethodHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: &HttpMethodHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpMethodHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: HttpMethodHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMethodHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: &HttpMethodHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpMethodHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<super::HttpMethod> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpMethodHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpMethodHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<super::HttpMethod> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpMethodHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<super::HttpMethod>> for HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<super::HttpMethod>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<super::HttpMethod>> for &HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<super::HttpMethod>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<super::HttpMethod>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpMethodHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpMethodHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpMethodHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpMethodHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpMethodHeaderValueCollection> for ::winrt_foundation::Collections::IVector<super::HttpMethod> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpMethodHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpMethodHeaderValueCollection> for ::winrt_foundation::Collections::IVector<super::HttpMethod> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpMethodHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<super::HttpMethod>> for HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<super::HttpMethod>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<super::HttpMethod>> for &HttpMethodHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<super::HttpMethod>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<super::HttpMethod>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpMethodHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpMethodHeaderValueCollection {}
#[repr(transparent)]
pub struct HttpNameValueHeaderValue(::windows_core::IUnknown);
impl HttpNameValueHeaderValue {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CreateFromName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(name: Param0) -> ::windows_core::Result<HttpNameValueHeaderValue> {
        Self::IHttpNameValueHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromName)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpNameValueHeaderValue>(result__)
        })
    }
    pub fn CreateFromNameWithValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(name: Param0, value: Param1) -> ::windows_core::Result<HttpNameValueHeaderValue> {
        Self::IHttpNameValueHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromNameWithValue)(::windows_core::Interface::as_raw(this), name.into_param().abi(), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpNameValueHeaderValue>(result__)
        })
    }
    pub fn Parse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0) -> ::windows_core::Result<HttpNameValueHeaderValue> {
        Self::IHttpNameValueHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpNameValueHeaderValue>(result__)
        })
    }
    pub fn TryParse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0, namevalueheadervalue: &mut ::core::option::Option<HttpNameValueHeaderValue>) -> ::windows_core::Result<bool> {
        Self::IHttpNameValueHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), namevalueheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpNameValueHeaderValueFactory<R, F: FnOnce(&IHttpNameValueHeaderValueFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpNameValueHeaderValue, IHttpNameValueHeaderValueFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpNameValueHeaderValueStatics<R, F: FnOnce(&IHttpNameValueHeaderValueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpNameValueHeaderValue, IHttpNameValueHeaderValueStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpNameValueHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpNameValueHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpNameValueHeaderValue {}
impl ::core::fmt::Debug for HttpNameValueHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpNameValueHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpNameValueHeaderValue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpNameValueHeaderValue;{d8ba7463-5b9a-4d1b-93f9-aa5b44ecfddf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpNameValueHeaderValue {
    type Vtable = IHttpNameValueHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = <IHttpNameValueHeaderValue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpNameValueHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpNameValueHeaderValue";
}
impl ::core::convert::From<HttpNameValueHeaderValue> for ::windows_core::IUnknown {
    fn from(value: HttpNameValueHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpNameValueHeaderValue> for ::windows_core::IUnknown {
    fn from(value: &HttpNameValueHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpNameValueHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpNameValueHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpNameValueHeaderValue> for ::windows_core::IInspectable {
    fn from(value: HttpNameValueHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpNameValueHeaderValue> for ::windows_core::IInspectable {
    fn from(value: &HttpNameValueHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpNameValueHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpNameValueHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpNameValueHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpNameValueHeaderValue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpNameValueHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpNameValueHeaderValue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpNameValueHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpNameValueHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpNameValueHeaderValue {}
unsafe impl ::core::marker::Sync for HttpNameValueHeaderValue {}
#[repr(transparent)]
pub struct HttpProductHeaderValue(::windows_core::IUnknown);
impl HttpProductHeaderValue {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Version(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Version)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CreateFromName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(productname: Param0) -> ::windows_core::Result<HttpProductHeaderValue> {
        Self::IHttpProductHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromName)(::windows_core::Interface::as_raw(this), productname.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpProductHeaderValue>(result__)
        })
    }
    pub fn CreateFromNameWithVersion<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(productname: Param0, productversion: Param1) -> ::windows_core::Result<HttpProductHeaderValue> {
        Self::IHttpProductHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromNameWithVersion)(::windows_core::Interface::as_raw(this), productname.into_param().abi(), productversion.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpProductHeaderValue>(result__)
        })
    }
    pub fn Parse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0) -> ::windows_core::Result<HttpProductHeaderValue> {
        Self::IHttpProductHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpProductHeaderValue>(result__)
        })
    }
    pub fn TryParse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0, productheadervalue: &mut ::core::option::Option<HttpProductHeaderValue>) -> ::windows_core::Result<bool> {
        Self::IHttpProductHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), productheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpProductHeaderValueFactory<R, F: FnOnce(&IHttpProductHeaderValueFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpProductHeaderValue, IHttpProductHeaderValueFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpProductHeaderValueStatics<R, F: FnOnce(&IHttpProductHeaderValueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpProductHeaderValue, IHttpProductHeaderValueStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpProductHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpProductHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpProductHeaderValue {}
impl ::core::fmt::Debug for HttpProductHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpProductHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpProductHeaderValue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpProductHeaderValue;{f4feee03-ebd4-4160-b9ff-807c5183b6e6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpProductHeaderValue {
    type Vtable = IHttpProductHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = <IHttpProductHeaderValue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpProductHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpProductHeaderValue";
}
impl ::core::convert::From<HttpProductHeaderValue> for ::windows_core::IUnknown {
    fn from(value: HttpProductHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpProductHeaderValue> for ::windows_core::IUnknown {
    fn from(value: &HttpProductHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpProductHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpProductHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpProductHeaderValue> for ::windows_core::IInspectable {
    fn from(value: HttpProductHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpProductHeaderValue> for ::windows_core::IInspectable {
    fn from(value: &HttpProductHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpProductHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpProductHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpProductHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpProductHeaderValue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpProductHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpProductHeaderValue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpProductHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpProductHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpProductHeaderValue {}
unsafe impl ::core::marker::Sync for HttpProductHeaderValue {}
#[repr(transparent)]
pub struct HttpProductInfoHeaderValue(::windows_core::IUnknown);
impl HttpProductInfoHeaderValue {
    pub fn Product(&self) -> ::windows_core::Result<HttpProductHeaderValue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Product)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpProductHeaderValue>(result__)
        }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CreateFromComment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(productcomment: Param0) -> ::windows_core::Result<HttpProductInfoHeaderValue> {
        Self::IHttpProductInfoHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromComment)(::windows_core::Interface::as_raw(this), productcomment.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpProductInfoHeaderValue>(result__)
        })
    }
    pub fn CreateFromNameWithVersion<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(productname: Param0, productversion: Param1) -> ::windows_core::Result<HttpProductInfoHeaderValue> {
        Self::IHttpProductInfoHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromNameWithVersion)(::windows_core::Interface::as_raw(this), productname.into_param().abi(), productversion.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpProductInfoHeaderValue>(result__)
        })
    }
    pub fn Parse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0) -> ::windows_core::Result<HttpProductInfoHeaderValue> {
        Self::IHttpProductInfoHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpProductInfoHeaderValue>(result__)
        })
    }
    pub fn TryParse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0, productinfoheadervalue: &mut ::core::option::Option<HttpProductInfoHeaderValue>) -> ::windows_core::Result<bool> {
        Self::IHttpProductInfoHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), productinfoheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpProductInfoHeaderValueFactory<R, F: FnOnce(&IHttpProductInfoHeaderValueFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpProductInfoHeaderValue, IHttpProductInfoHeaderValueFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpProductInfoHeaderValueStatics<R, F: FnOnce(&IHttpProductInfoHeaderValueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpProductInfoHeaderValue, IHttpProductInfoHeaderValueStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpProductInfoHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpProductInfoHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpProductInfoHeaderValue {}
impl ::core::fmt::Debug for HttpProductInfoHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpProductInfoHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpProductInfoHeaderValue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpProductInfoHeaderValue;{1b1a8732-4c35-486a-966f-646489198e4d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpProductInfoHeaderValue {
    type Vtable = IHttpProductInfoHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = <IHttpProductInfoHeaderValue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpProductInfoHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpProductInfoHeaderValue";
}
impl ::core::convert::From<HttpProductInfoHeaderValue> for ::windows_core::IUnknown {
    fn from(value: HttpProductInfoHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpProductInfoHeaderValue> for ::windows_core::IUnknown {
    fn from(value: &HttpProductInfoHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpProductInfoHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpProductInfoHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpProductInfoHeaderValue> for ::windows_core::IInspectable {
    fn from(value: HttpProductInfoHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpProductInfoHeaderValue> for ::windows_core::IInspectable {
    fn from(value: &HttpProductInfoHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpProductInfoHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpProductInfoHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpProductInfoHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpProductInfoHeaderValue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpProductInfoHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpProductInfoHeaderValue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpProductInfoHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpProductInfoHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpProductInfoHeaderValue {}
unsafe impl ::core::marker::Sync for HttpProductInfoHeaderValue {}
#[repr(transparent)]
pub struct HttpProductInfoHeaderValueCollection(::windows_core::IUnknown);
impl HttpProductInfoHeaderValueCollection {
    pub fn ParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi()).ok() }
    }
    pub fn TryParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<HttpProductInfoHeaderValue>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<HttpProductInfoHeaderValue>>(result__)
        }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<HttpProductInfoHeaderValue> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<HttpProductInfoHeaderValue>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<HttpProductInfoHeaderValue>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<HttpProductInfoHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, HttpProductInfoHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, HttpProductInfoHeaderValue>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, HttpProductInfoHeaderValue>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, HttpProductInfoHeaderValue>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<HttpProductInfoHeaderValue>]) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<HttpProductInfoHeaderValue>]) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpProductInfoHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpProductInfoHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpProductInfoHeaderValueCollection {}
impl ::core::fmt::Debug for HttpProductInfoHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpProductInfoHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpProductInfoHeaderValueCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpProductInfoHeaderValueCollection;{877df74a-d69b-44f8-ad4f-453af9c42ed0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpProductInfoHeaderValueCollection {
    type Vtable = IHttpProductInfoHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = <IHttpProductInfoHeaderValueCollection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpProductInfoHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpProductInfoHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpProductInfoHeaderValueCollection {
    type Item = HttpProductInfoHeaderValue;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpProductInfoHeaderValueCollection {
    type Item = HttpProductInfoHeaderValue;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpProductInfoHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: HttpProductInfoHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpProductInfoHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: &HttpProductInfoHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpProductInfoHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: HttpProductInfoHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpProductInfoHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: &HttpProductInfoHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpProductInfoHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<HttpProductInfoHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpProductInfoHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpProductInfoHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<HttpProductInfoHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpProductInfoHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<HttpProductInfoHeaderValue>> for HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<HttpProductInfoHeaderValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<HttpProductInfoHeaderValue>> for &HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<HttpProductInfoHeaderValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<HttpProductInfoHeaderValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpProductInfoHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpProductInfoHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpProductInfoHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpProductInfoHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpProductInfoHeaderValueCollection> for ::winrt_foundation::Collections::IVector<HttpProductInfoHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpProductInfoHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpProductInfoHeaderValueCollection> for ::winrt_foundation::Collections::IVector<HttpProductInfoHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpProductInfoHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<HttpProductInfoHeaderValue>> for HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<HttpProductInfoHeaderValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<HttpProductInfoHeaderValue>> for &HttpProductInfoHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<HttpProductInfoHeaderValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<HttpProductInfoHeaderValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpProductInfoHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpProductInfoHeaderValueCollection {}
#[repr(transparent)]
pub struct HttpRequestHeaderCollection(::windows_core::IUnknown);
impl HttpRequestHeaderCollection {
    pub fn Accept(&self) -> ::windows_core::Result<HttpMediaTypeWithQualityHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Accept)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpMediaTypeWithQualityHeaderValueCollection>(result__)
        }
    }
    pub fn AcceptEncoding(&self) -> ::windows_core::Result<HttpContentCodingWithQualityHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AcceptEncoding)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpContentCodingWithQualityHeaderValueCollection>(result__)
        }
    }
    pub fn AcceptLanguage(&self) -> ::windows_core::Result<HttpLanguageRangeWithQualityHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AcceptLanguage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpLanguageRangeWithQualityHeaderValueCollection>(result__)
        }
    }
    pub fn Authorization(&self) -> ::windows_core::Result<HttpCredentialsHeaderValue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Authorization)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpCredentialsHeaderValue>(result__)
        }
    }
    pub fn SetAuthorization<'a, Param0: ::windows_core::IntoParam<'a, HttpCredentialsHeaderValue>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAuthorization)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CacheControl(&self) -> ::windows_core::Result<HttpCacheDirectiveHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CacheControl)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpCacheDirectiveHeaderValueCollection>(result__)
        }
    }
    pub fn Connection(&self) -> ::windows_core::Result<HttpConnectionOptionHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Connection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpConnectionOptionHeaderValueCollection>(result__)
        }
    }
    pub fn Cookie(&self) -> ::windows_core::Result<HttpCookiePairHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Cookie)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpCookiePairHeaderValueCollection>(result__)
        }
    }
    pub fn Date(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Date)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn SetDate<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDate)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Expect(&self) -> ::windows_core::Result<HttpExpectationHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Expect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpExpectationHeaderValueCollection>(result__)
        }
    }
    pub fn From(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).From)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetFrom<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFrom)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Networking")]
    pub fn Host(&self) -> ::windows_core::Result<::winrt_networking::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Host)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_networking::HostName>(result__)
        }
    }
    #[cfg(feature = "Networking")]
    pub fn SetHost<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_networking::HostName>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHost)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IfModifiedSince(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IfModifiedSince)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn SetIfModifiedSince<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIfModifiedSince)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IfUnmodifiedSince(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IfUnmodifiedSince)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn SetIfUnmodifiedSince<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIfUnmodifiedSince)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MaxForwards(&self) -> ::windows_core::Result<::winrt_foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MaxForwards)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<u32>>(result__)
        }
    }
    pub fn SetMaxForwards<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<u32>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxForwards)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ProxyAuthorization(&self) -> ::windows_core::Result<HttpCredentialsHeaderValue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProxyAuthorization)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpCredentialsHeaderValue>(result__)
        }
    }
    pub fn SetProxyAuthorization<'a, Param0: ::windows_core::IntoParam<'a, HttpCredentialsHeaderValue>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProxyAuthorization)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Referer(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Referer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetReferer<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReferer)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TransferEncoding(&self) -> ::windows_core::Result<HttpTransferCodingHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TransferEncoding)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpTransferCodingHeaderValueCollection>(result__)
        }
    }
    pub fn UserAgent(&self) -> ::windows_core::Result<HttpProductInfoHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UserAgent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpProductInfoHeaderValueCollection>(result__)
        }
    }
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    pub fn TryAppendWithoutValidation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryAppendWithoutValidation)(::windows_core::Interface::as_raw(this), name.into_param().abi(), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0, value: Param1) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Insert)(::windows_core::Interface::as_raw(this), key.into_param().abi(), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), key.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpRequestHeaderCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpRequestHeaderCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpRequestHeaderCollection {}
impl ::core::fmt::Debug for HttpRequestHeaderCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpRequestHeaderCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpRequestHeaderCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpRequestHeaderCollection;{af40329b-b544-469b-86b9-ac3d466fea36})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpRequestHeaderCollection {
    type Vtable = IHttpRequestHeaderCollection_Vtbl;
    const IID: ::windows_core::GUID = <IHttpRequestHeaderCollection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpRequestHeaderCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpRequestHeaderCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpRequestHeaderCollection {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpRequestHeaderCollection {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl ::core::convert::From<HttpRequestHeaderCollection> for ::windows_core::IUnknown {
    fn from(value: HttpRequestHeaderCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpRequestHeaderCollection> for ::windows_core::IUnknown {
    fn from(value: &HttpRequestHeaderCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpRequestHeaderCollection> for ::windows_core::IInspectable {
    fn from(value: HttpRequestHeaderCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpRequestHeaderCollection> for ::windows_core::IInspectable {
    fn from(value: &HttpRequestHeaderCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpRequestHeaderCollection> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpRequestHeaderCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpRequestHeaderCollection> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpRequestHeaderCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> for HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> for &HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpRequestHeaderCollection> for ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpRequestHeaderCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpRequestHeaderCollection> for ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpRequestHeaderCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> for HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> for &HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpRequestHeaderCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpRequestHeaderCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpRequestHeaderCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpRequestHeaderCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpRequestHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpRequestHeaderCollection {}
unsafe impl ::core::marker::Sync for HttpRequestHeaderCollection {}
#[repr(transparent)]
pub struct HttpResponseHeaderCollection(::windows_core::IUnknown);
impl HttpResponseHeaderCollection {
    pub fn Age(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Age)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetAge<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAge)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Allow(&self) -> ::windows_core::Result<HttpMethodHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Allow)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpMethodHeaderValueCollection>(result__)
        }
    }
    pub fn CacheControl(&self) -> ::windows_core::Result<HttpCacheDirectiveHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CacheControl)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpCacheDirectiveHeaderValueCollection>(result__)
        }
    }
    pub fn Connection(&self) -> ::windows_core::Result<HttpConnectionOptionHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Connection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpConnectionOptionHeaderValueCollection>(result__)
        }
    }
    pub fn Date(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Date)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn SetDate<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDate)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Location(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Location)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetLocation<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLocation)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ProxyAuthenticate(&self) -> ::windows_core::Result<HttpChallengeHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProxyAuthenticate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpChallengeHeaderValueCollection>(result__)
        }
    }
    pub fn RetryAfter(&self) -> ::windows_core::Result<HttpDateOrDeltaHeaderValue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RetryAfter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpDateOrDeltaHeaderValue>(result__)
        }
    }
    pub fn SetRetryAfter<'a, Param0: ::windows_core::IntoParam<'a, HttpDateOrDeltaHeaderValue>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRetryAfter)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TransferEncoding(&self) -> ::windows_core::Result<HttpTransferCodingHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TransferEncoding)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpTransferCodingHeaderValueCollection>(result__)
        }
    }
    pub fn WwwAuthenticate(&self) -> ::windows_core::Result<HttpChallengeHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WwwAuthenticate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpChallengeHeaderValueCollection>(result__)
        }
    }
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    pub fn TryAppendWithoutValidation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryAppendWithoutValidation)(::windows_core::Interface::as_raw(this), name.into_param().abi(), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0, value: Param1) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Insert)(::windows_core::Interface::as_raw(this), key.into_param().abi(), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), key.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpResponseHeaderCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpResponseHeaderCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpResponseHeaderCollection {}
impl ::core::fmt::Debug for HttpResponseHeaderCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpResponseHeaderCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpResponseHeaderCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpResponseHeaderCollection;{7a990969-fa3f-41ed-aac6-bf957975c16b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpResponseHeaderCollection {
    type Vtable = IHttpResponseHeaderCollection_Vtbl;
    const IID: ::windows_core::GUID = <IHttpResponseHeaderCollection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpResponseHeaderCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpResponseHeaderCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpResponseHeaderCollection {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpResponseHeaderCollection {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl ::core::convert::From<HttpResponseHeaderCollection> for ::windows_core::IUnknown {
    fn from(value: HttpResponseHeaderCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpResponseHeaderCollection> for ::windows_core::IUnknown {
    fn from(value: &HttpResponseHeaderCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpResponseHeaderCollection> for ::windows_core::IInspectable {
    fn from(value: HttpResponseHeaderCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpResponseHeaderCollection> for ::windows_core::IInspectable {
    fn from(value: &HttpResponseHeaderCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpResponseHeaderCollection> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpResponseHeaderCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpResponseHeaderCollection> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpResponseHeaderCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> for HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> for &HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpResponseHeaderCollection> for ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpResponseHeaderCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpResponseHeaderCollection> for ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpResponseHeaderCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> for HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> for &HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpResponseHeaderCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpResponseHeaderCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpResponseHeaderCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpResponseHeaderCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpResponseHeaderCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpResponseHeaderCollection {}
unsafe impl ::core::marker::Sync for HttpResponseHeaderCollection {}
#[repr(transparent)]
pub struct HttpTransferCodingHeaderValue(::windows_core::IUnknown);
impl HttpTransferCodingHeaderValue {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Parameters(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parameters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<HttpNameValueHeaderValue>>(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0) -> ::windows_core::Result<HttpTransferCodingHeaderValue> {
        Self::IHttpTransferCodingHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpTransferCodingHeaderValue>(result__)
        })
    }
    pub fn Parse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0) -> ::windows_core::Result<HttpTransferCodingHeaderValue> {
        Self::IHttpTransferCodingHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpTransferCodingHeaderValue>(result__)
        })
    }
    pub fn TryParse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0, transfercodingheadervalue: &mut ::core::option::Option<HttpTransferCodingHeaderValue>) -> ::windows_core::Result<bool> {
        Self::IHttpTransferCodingHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), transfercodingheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpTransferCodingHeaderValueFactory<R, F: FnOnce(&IHttpTransferCodingHeaderValueFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpTransferCodingHeaderValue, IHttpTransferCodingHeaderValueFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpTransferCodingHeaderValueStatics<R, F: FnOnce(&IHttpTransferCodingHeaderValueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpTransferCodingHeaderValue, IHttpTransferCodingHeaderValueStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpTransferCodingHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpTransferCodingHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpTransferCodingHeaderValue {}
impl ::core::fmt::Debug for HttpTransferCodingHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpTransferCodingHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpTransferCodingHeaderValue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpTransferCodingHeaderValue;{436f32f9-3ded-42bd-b38a-5496a2511ce6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpTransferCodingHeaderValue {
    type Vtable = IHttpTransferCodingHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = <IHttpTransferCodingHeaderValue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpTransferCodingHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpTransferCodingHeaderValue";
}
impl ::core::convert::From<HttpTransferCodingHeaderValue> for ::windows_core::IUnknown {
    fn from(value: HttpTransferCodingHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpTransferCodingHeaderValue> for ::windows_core::IUnknown {
    fn from(value: &HttpTransferCodingHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpTransferCodingHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpTransferCodingHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpTransferCodingHeaderValue> for ::windows_core::IInspectable {
    fn from(value: HttpTransferCodingHeaderValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpTransferCodingHeaderValue> for ::windows_core::IInspectable {
    fn from(value: &HttpTransferCodingHeaderValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpTransferCodingHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpTransferCodingHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpTransferCodingHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpTransferCodingHeaderValue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpTransferCodingHeaderValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpTransferCodingHeaderValue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpTransferCodingHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpTransferCodingHeaderValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpTransferCodingHeaderValue {}
unsafe impl ::core::marker::Sync for HttpTransferCodingHeaderValue {}
#[repr(transparent)]
pub struct HttpTransferCodingHeaderValueCollection(::windows_core::IUnknown);
impl HttpTransferCodingHeaderValueCollection {
    pub fn ParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi()).ok() }
    }
    pub fn TryParseAdd<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, input: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParseAdd)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<HttpTransferCodingHeaderValue>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<HttpTransferCodingHeaderValue>>(result__)
        }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<HttpTransferCodingHeaderValue> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<HttpTransferCodingHeaderValue>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<HttpTransferCodingHeaderValue>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<HttpTransferCodingHeaderValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, HttpTransferCodingHeaderValue>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, HttpTransferCodingHeaderValue>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, HttpTransferCodingHeaderValue>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, HttpTransferCodingHeaderValue>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<HttpTransferCodingHeaderValue>]) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<HttpTransferCodingHeaderValue>]) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpTransferCodingHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpTransferCodingHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpTransferCodingHeaderValueCollection {}
impl ::core::fmt::Debug for HttpTransferCodingHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpTransferCodingHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpTransferCodingHeaderValueCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpTransferCodingHeaderValueCollection;{202c8c34-2c03-49b8-9665-73e27cb2fc79})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpTransferCodingHeaderValueCollection {
    type Vtable = IHttpTransferCodingHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = <IHttpTransferCodingHeaderValueCollection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpTransferCodingHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpTransferCodingHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpTransferCodingHeaderValueCollection {
    type Item = HttpTransferCodingHeaderValue;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpTransferCodingHeaderValueCollection {
    type Item = HttpTransferCodingHeaderValue;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<HttpTransferCodingHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: HttpTransferCodingHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpTransferCodingHeaderValueCollection> for ::windows_core::IUnknown {
    fn from(value: &HttpTransferCodingHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpTransferCodingHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: HttpTransferCodingHeaderValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpTransferCodingHeaderValueCollection> for ::windows_core::IInspectable {
    fn from(value: &HttpTransferCodingHeaderValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpTransferCodingHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<HttpTransferCodingHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpTransferCodingHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpTransferCodingHeaderValueCollection> for ::winrt_foundation::Collections::IIterable<HttpTransferCodingHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpTransferCodingHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<HttpTransferCodingHeaderValue>> for HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<HttpTransferCodingHeaderValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<HttpTransferCodingHeaderValue>> for &HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<HttpTransferCodingHeaderValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<HttpTransferCodingHeaderValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpTransferCodingHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpTransferCodingHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpTransferCodingHeaderValueCollection> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpTransferCodingHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpTransferCodingHeaderValueCollection> for ::winrt_foundation::Collections::IVector<HttpTransferCodingHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpTransferCodingHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpTransferCodingHeaderValueCollection> for ::winrt_foundation::Collections::IVector<HttpTransferCodingHeaderValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpTransferCodingHeaderValueCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<HttpTransferCodingHeaderValue>> for HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<HttpTransferCodingHeaderValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<HttpTransferCodingHeaderValue>> for &HttpTransferCodingHeaderValueCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<HttpTransferCodingHeaderValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<HttpTransferCodingHeaderValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpTransferCodingHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpTransferCodingHeaderValueCollection {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCacheDirectiveHeaderValueCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpCacheDirectiveHeaderValueCollection {
    type Vtable = IHttpCacheDirectiveHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a586b89_d5d0_4fbe_bd9d_b5b3636811b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCacheDirectiveHeaderValueCollection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MaxAge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetMaxAge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MaxStale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetMaxStale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MinFresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetMinFresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SharedMaxAge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetSharedMaxAge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpChallengeHeaderValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpChallengeHeaderValue {
    type Vtable = IHttpChallengeHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x393361af_0f7d_4820_9fdd_a2b956eeaeab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpChallengeHeaderValue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Parameters: usize,
    pub Scheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Token: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpChallengeHeaderValueCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpChallengeHeaderValueCollection {
    type Vtable = IHttpChallengeHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca9e5f81_aee0_4353_a10b_e625babd64c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpChallengeHeaderValueCollection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpChallengeHeaderValueFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpChallengeHeaderValueFactory {
    type Vtable = IHttpChallengeHeaderValueFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc452c451_d99c_40aa_9399_90eeb98fc613);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpChallengeHeaderValueFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFromScheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFromSchemeWithToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, token: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpChallengeHeaderValueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpChallengeHeaderValueStatics {
    type Vtable = IHttpChallengeHeaderValueStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf3d38a72_fc01_4d01_a008_fcb7c459d635);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpChallengeHeaderValueStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, challengeheadervalue: *mut ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpConnectionOptionHeaderValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpConnectionOptionHeaderValue {
    type Vtable = IHttpConnectionOptionHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb4af27a_4e90_45eb_8dcd_fd1408f4c44f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpConnectionOptionHeaderValue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Token: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpConnectionOptionHeaderValueCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpConnectionOptionHeaderValueCollection {
    type Vtable = IHttpConnectionOptionHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe4f56c1d_5142_4e00_8e0f_019509337629);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpConnectionOptionHeaderValueCollection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpConnectionOptionHeaderValueFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpConnectionOptionHeaderValueFactory {
    type Vtable = IHttpConnectionOptionHeaderValueFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd93ccc1e_0b7d_4c3f_a58d_a2a1bdeabc0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpConnectionOptionHeaderValueFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpConnectionOptionHeaderValueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpConnectionOptionHeaderValueStatics {
    type Vtable = IHttpConnectionOptionHeaderValueStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaaa75d37_a946_4b1f_85af_48b68b3c50bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpConnectionOptionHeaderValueStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, connectionoptionheadervalue: *mut ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentCodingHeaderValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpContentCodingHeaderValue {
    type Vtable = IHttpContentCodingHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbcf7f92a_9376_4d85_bccc_9f4f9acab434);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingHeaderValue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ContentCoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentCodingHeaderValueCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpContentCodingHeaderValueCollection {
    type Vtable = IHttpContentCodingHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d221721_a6db_436e_8e83_91596192819c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingHeaderValueCollection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentCodingHeaderValueFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpContentCodingHeaderValueFactory {
    type Vtable = IHttpContentCodingHeaderValueFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc53d2bd7_332b_4350_8510_2e67a2289a5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingHeaderValueFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentcoding: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentCodingHeaderValueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpContentCodingHeaderValueStatics {
    type Vtable = IHttpContentCodingHeaderValueStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x94d8602e_f9bf_42f7_aa46_ed272a41e212);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingHeaderValueStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, contentcodingheadervalue: *mut ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentCodingWithQualityHeaderValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpContentCodingWithQualityHeaderValue {
    type Vtable = IHttpContentCodingWithQualityHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x94531cd5_8b13_4d73_8651_f76b38f88495);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingWithQualityHeaderValue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ContentCoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Quality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentCodingWithQualityHeaderValueCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpContentCodingWithQualityHeaderValueCollection {
    type Vtable = IHttpContentCodingWithQualityHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7c0d753e_e899_4378_b5c8_412d820711cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingWithQualityHeaderValueCollection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentCodingWithQualityHeaderValueFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpContentCodingWithQualityHeaderValueFactory {
    type Vtable = IHttpContentCodingWithQualityHeaderValueFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc45eee1a_c553_46fc_ade2_d75c1d53df7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingWithQualityHeaderValueFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFromValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentcoding: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFromValueWithQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentcoding: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, quality: f64, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentCodingWithQualityHeaderValueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpContentCodingWithQualityHeaderValueStatics {
    type Vtable = IHttpContentCodingWithQualityHeaderValueStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8c9357c_8f89_4801_8e75_4c9abfc3de71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingWithQualityHeaderValueStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, contentcodingwithqualityheadervalue: *mut ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentDispositionHeaderValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpContentDispositionHeaderValue {
    type Vtable = IHttpContentDispositionHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2a2eedc_2629_4b49_9908_96a168e9365e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentDispositionHeaderValue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DispositionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDispositionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FileNameStar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetFileNameStar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Parameters: usize,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentDispositionHeaderValueFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpContentDispositionHeaderValueFactory {
    type Vtable = IHttpContentDispositionHeaderValueFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9915bbc4_456c_4e81_8295_b2ab3cbcf545);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentDispositionHeaderValueFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispositiontype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentDispositionHeaderValueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpContentDispositionHeaderValueStatics {
    type Vtable = IHttpContentDispositionHeaderValueStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x29c56067_5a37_46e4_b074_c5177d69ca66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentDispositionHeaderValueStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, contentdispositionheadervalue: *mut ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentHeaderCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpContentHeaderCollection {
    type Vtable = IHttpContentHeaderCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x40612a44_47ae_4b7e_9124_69628b64aa18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentHeaderCollection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ContentDisposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetContentDisposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContentEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContentLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContentLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetContentLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContentLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetContentLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ContentMD5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ContentMD5: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetContentMD5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetContentMD5: usize,
    pub ContentRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetContentRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Expires: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetExpires: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LastModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetLastModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TryAppendWithoutValidation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentRangeHeaderValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpContentRangeHeaderValue {
    type Vtable = IHttpContentRangeHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04d967d3_a4f6_495c_9530_8579fcba8aa9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentRangeHeaderValue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FirstBytePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LastBytePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Unit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentRangeHeaderValueFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpContentRangeHeaderValueFactory {
    type Vtable = IHttpContentRangeHeaderValueFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3f5bd691_a03c_4456_9a6f_ef27ecd03cae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentRangeHeaderValueFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFromLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: u64, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFromRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, from: u64, to: u64, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFromRangeWithLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, from: u64, to: u64, length: u64, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentRangeHeaderValueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpContentRangeHeaderValueStatics {
    type Vtable = IHttpContentRangeHeaderValueStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80a346ca_174c_4fae_821c_134cd294aa38);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentRangeHeaderValueStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, contentrangeheadervalue: *mut ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCookiePairHeaderValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpCookiePairHeaderValue {
    type Vtable = IHttpCookiePairHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcbd46217_4b29_412b_bd90_b3d814ab8e1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookiePairHeaderValue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCookiePairHeaderValueCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpCookiePairHeaderValueCollection {
    type Vtable = IHttpCookiePairHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf3f44350_581e_4ecc_9f59_e507d04f06e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookiePairHeaderValueCollection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCookiePairHeaderValueFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpCookiePairHeaderValueFactory {
    type Vtable = IHttpCookiePairHeaderValueFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x635e326f_146f_4f56_aa21_2cb7d6d58b1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookiePairHeaderValueFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFromName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFromNameWithValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCookiePairHeaderValueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpCookiePairHeaderValueStatics {
    type Vtable = IHttpCookiePairHeaderValueStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e866d48_06af_4462_8158_99388d5dca81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookiePairHeaderValueStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, cookiepairheadervalue: *mut ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCredentialsHeaderValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpCredentialsHeaderValue {
    type Vtable = IHttpCredentialsHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc34cc3cb_542e_4177_a6c7_b674ce193fbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCredentialsHeaderValue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Parameters: usize,
    pub Scheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Token: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCredentialsHeaderValueFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpCredentialsHeaderValueFactory {
    type Vtable = IHttpCredentialsHeaderValueFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf21d9e91_4d1c_4182_bfd1_34470a62f950);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCredentialsHeaderValueFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFromScheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFromSchemeWithToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, token: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCredentialsHeaderValueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpCredentialsHeaderValueStatics {
    type Vtable = IHttpCredentialsHeaderValueStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa69b2be6_ce8c_4443_a35a_1b727b131036);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCredentialsHeaderValueStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, credentialsheadervalue: *mut ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDateOrDeltaHeaderValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpDateOrDeltaHeaderValue {
    type Vtable = IHttpDateOrDeltaHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeafcaa6a_c4dc_49e2_a27d_043adf5867a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDateOrDeltaHeaderValue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Date: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Delta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDateOrDeltaHeaderValueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpDateOrDeltaHeaderValueStatics {
    type Vtable = IHttpDateOrDeltaHeaderValueStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7c2659a8_6672_4e90_9a9a_f39766f7f576);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDateOrDeltaHeaderValueStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, dateordeltaheadervalue: *mut ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpExpectationHeaderValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpExpectationHeaderValue {
    type Vtable = IHttpExpectationHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4ce585cd_3a99_43af_a2e6_ec232fea9658);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpExpectationHeaderValue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Parameters: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpExpectationHeaderValueCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpExpectationHeaderValueCollection {
    type Vtable = IHttpExpectationHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe78521b3_a0e2_4ac4_9e66_79706cb9fd58);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpExpectationHeaderValueCollection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpExpectationHeaderValueFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpExpectationHeaderValueFactory {
    type Vtable = IHttpExpectationHeaderValueFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4ea275cb_d53e_4868_8856_1e21a5030dc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpExpectationHeaderValueFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFromName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFromNameWithValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpExpectationHeaderValueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpExpectationHeaderValueStatics {
    type Vtable = IHttpExpectationHeaderValueStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3019abe2_cfe5_473b_a57f_fba5b14eb257);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpExpectationHeaderValueStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, expectationheadervalue: *mut ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpLanguageHeaderValueCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpLanguageHeaderValueCollection {
    type Vtable = IHttpLanguageHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9ebd7ca3_8219_44f6_9902_8c56dfd3340c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpLanguageHeaderValueCollection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpLanguageRangeWithQualityHeaderValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpLanguageRangeWithQualityHeaderValue {
    type Vtable = IHttpLanguageRangeWithQualityHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7256e102_0080_4db4_a083_7de7b2e5ba4c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpLanguageRangeWithQualityHeaderValue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LanguageRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Quality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpLanguageRangeWithQualityHeaderValueCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpLanguageRangeWithQualityHeaderValueCollection {
    type Vtable = IHttpLanguageRangeWithQualityHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x885d5abd_4b4f_480a_89ce_8aedcee6e3a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpLanguageRangeWithQualityHeaderValueCollection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpLanguageRangeWithQualityHeaderValueFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpLanguageRangeWithQualityHeaderValueFactory {
    type Vtable = IHttpLanguageRangeWithQualityHeaderValueFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7bb83970_780f_4c83_9fe4_dc3087f6bd55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpLanguageRangeWithQualityHeaderValueFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFromLanguageRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languagerange: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFromLanguageRangeWithQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languagerange: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, quality: f64, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpLanguageRangeWithQualityHeaderValueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpLanguageRangeWithQualityHeaderValueStatics {
    type Vtable = IHttpLanguageRangeWithQualityHeaderValueStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2541e146_f308_46f5_b695_42f54024ec68);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpLanguageRangeWithQualityHeaderValueStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, languagerangewithqualityheadervalue: *mut ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMediaTypeHeaderValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpMediaTypeHeaderValue {
    type Vtable = IHttpMediaTypeHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x16b28533_e728_4fcb_bdb0_08a431a14844);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeHeaderValue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CharSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCharSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Parameters: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMediaTypeHeaderValueFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpMediaTypeHeaderValueFactory {
    type Vtable = IHttpMediaTypeHeaderValueFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbed747a8_cd17_42dd_9367_ab9c5b56dd7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeHeaderValueFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediatype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMediaTypeHeaderValueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpMediaTypeHeaderValueStatics {
    type Vtable = IHttpMediaTypeHeaderValueStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe04d83df_1d41_4d8c_a2de_6fd2ed87399b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeHeaderValueStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, mediatypeheadervalue: *mut ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMediaTypeWithQualityHeaderValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpMediaTypeWithQualityHeaderValue {
    type Vtable = IHttpMediaTypeWithQualityHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x188d5e32_76be_44a0_b1cd_2074bded2dde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeWithQualityHeaderValue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CharSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCharSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Parameters: usize,
    pub Quality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMediaTypeWithQualityHeaderValueCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpMediaTypeWithQualityHeaderValueCollection {
    type Vtable = IHttpMediaTypeWithQualityHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3c0c6b73_1342_4587_a056_18d02ff67165);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeWithQualityHeaderValueCollection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMediaTypeWithQualityHeaderValueFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpMediaTypeWithQualityHeaderValueFactory {
    type Vtable = IHttpMediaTypeWithQualityHeaderValueFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c6d20f4_9457_44e6_a323_d122b958780b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeWithQualityHeaderValueFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFromMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediatype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFromMediaTypeWithQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediatype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, quality: f64, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMediaTypeWithQualityHeaderValueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpMediaTypeWithQualityHeaderValueStatics {
    type Vtable = IHttpMediaTypeWithQualityHeaderValueStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b070cd9_b560_4fc8_9835_7e6c0a657b24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeWithQualityHeaderValueStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, mediatypewithqualityheadervalue: *mut ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMethodHeaderValueCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpMethodHeaderValueCollection {
    type Vtable = IHttpMethodHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43bc3ff4_6119_4adf_938c_34bfffcf92ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMethodHeaderValueCollection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpNameValueHeaderValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpNameValueHeaderValue {
    type Vtable = IHttpNameValueHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8ba7463_5b9a_4d1b_93f9_aa5b44ecfddf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpNameValueHeaderValue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpNameValueHeaderValueFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpNameValueHeaderValueFactory {
    type Vtable = IHttpNameValueHeaderValueFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x770e2267_cbf8_4736_a925_93fbe10c7ca8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpNameValueHeaderValueFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFromName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFromNameWithValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpNameValueHeaderValueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpNameValueHeaderValueStatics {
    type Vtable = IHttpNameValueHeaderValueStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xffd4030f_1130_4152_8659_256909a9d115);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpNameValueHeaderValueStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, namevalueheadervalue: *mut ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpProductHeaderValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpProductHeaderValue {
    type Vtable = IHttpProductHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf4feee03_ebd4_4160_b9ff_807c5183b6e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductHeaderValue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpProductHeaderValueFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpProductHeaderValueFactory {
    type Vtable = IHttpProductHeaderValueFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x611aa4f5_82bc_42fb_977b_dc00536e5e86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductHeaderValueFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFromName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFromNameWithVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, productversion: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpProductHeaderValueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpProductHeaderValueStatics {
    type Vtable = IHttpProductHeaderValueStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x90c33e29_befc_4337_be62_49f097975f53);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductHeaderValueStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, productheadervalue: *mut ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpProductInfoHeaderValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpProductInfoHeaderValue {
    type Vtable = IHttpProductInfoHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1b1a8732_4c35_486a_966f_646489198e4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductInfoHeaderValue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Product: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Comment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpProductInfoHeaderValueCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpProductInfoHeaderValueCollection {
    type Vtable = IHttpProductInfoHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x877df74a_d69b_44f8_ad4f_453af9c42ed0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductInfoHeaderValueCollection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpProductInfoHeaderValueFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpProductInfoHeaderValueFactory {
    type Vtable = IHttpProductInfoHeaderValueFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24220fbe_eabe_4464_b460_ec010b7c41e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductInfoHeaderValueFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFromComment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productcomment: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFromNameWithVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, productversion: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpProductInfoHeaderValueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpProductInfoHeaderValueStatics {
    type Vtable = IHttpProductInfoHeaderValueStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdb7fd857_327a_4e73_81e5_7059a302b042);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductInfoHeaderValueStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, productinfoheadervalue: *mut ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpRequestHeaderCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpRequestHeaderCollection {
    type Vtable = IHttpRequestHeaderCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaf40329b_b544_469b_86b9_ac3d466fea36);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpRequestHeaderCollection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Accept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AcceptEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AcceptLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Authorization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAuthorization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CacheControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Cookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Date: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Expect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub From: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Networking")]
    pub Host: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    Host: usize,
    #[cfg(feature = "Networking")]
    pub SetHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    SetHost: usize,
    pub IfModifiedSince: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetIfModifiedSince: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IfUnmodifiedSince: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetIfUnmodifiedSince: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MaxForwards: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetMaxForwards: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ProxyAuthorization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetProxyAuthorization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Referer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetReferer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TransferEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UserAgent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TryAppendWithoutValidation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpResponseHeaderCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpResponseHeaderCollection {
    type Vtable = IHttpResponseHeaderCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a990969_fa3f_41ed_aac6_bf957975c16b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpResponseHeaderCollection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Age: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Allow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CacheControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Date: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ProxyAuthenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RetryAfter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetRetryAfter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TransferEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub WwwAuthenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TryAppendWithoutValidation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpTransferCodingHeaderValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpTransferCodingHeaderValue {
    type Vtable = IHttpTransferCodingHeaderValue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x436f32f9_3ded_42bd_b38a_5496a2511ce6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpTransferCodingHeaderValue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Parameters: usize,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpTransferCodingHeaderValueCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpTransferCodingHeaderValueCollection {
    type Vtable = IHttpTransferCodingHeaderValueCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x202c8c34_2c03_49b8_9665_73e27cb2fc79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpTransferCodingHeaderValueCollection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpTransferCodingHeaderValueFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpTransferCodingHeaderValueFactory {
    type Vtable = IHttpTransferCodingHeaderValueFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb62dffc_e361_4f08_8e4f_c9e723de703b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpTransferCodingHeaderValueFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpTransferCodingHeaderValueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpTransferCodingHeaderValueStatics {
    type Vtable = IHttpTransferCodingHeaderValueStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6ab8892a_1a98_4d32_a906_7470a9875ce5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpTransferCodingHeaderValueStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, transfercodingheadervalue: *mut ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
}
