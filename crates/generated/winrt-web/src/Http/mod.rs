#[cfg(feature = "Diagnostics")]
pub mod Diagnostics;
#[cfg(feature = "Filters")]
pub mod Filters;
#[cfg(feature = "Headers")]
pub mod Headers;
#[repr(transparent)]
pub struct HttpBufferContent(::windows_core::IUnknown);
impl HttpBufferContent {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(content: Param0) -> ::windows_core::Result<HttpBufferContent> {
        Self::IHttpBufferContentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromBuffer)(::windows_core::Interface::as_raw(this), content.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpBufferContent>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBufferWithOffset<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(content: Param0, offset: u32, count: u32) -> ::windows_core::Result<HttpBufferContent> {
        Self::IHttpBufferContentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromBufferWithOffset)(::windows_core::Interface::as_raw(this), content.into_param().abi(), offset, count, result__.as_mut_ptr()).from_abi::<HttpBufferContent>(result__)
        })
    }
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows_core::Result<Headers::HttpContentHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Headers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Headers::HttpContentHeaderCollection>(result__)
        }
    }
    pub fn BufferAllAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BufferAllAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ReadAsBufferAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IBuffer, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsBufferAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IBuffer, u64>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ReadAsInputStreamAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IInputStream, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsInputStreamAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IInputStream, u64>>(result__)
        }
    }
    pub fn ReadAsStringAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::windows_core::HSTRING, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsStringAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::windows_core::HSTRING, u64>>(result__)
        }
    }
    pub fn TryComputeLength(&self, length: &mut u64) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryComputeLength)(::windows_core::Interface::as_raw(this), length, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn WriteToStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IOutputStream>>(&self, outputstream: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WriteToStreamAsync)(::windows_core::Interface::as_raw(this), outputstream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpBufferContentFactory<R, F: FnOnce(&IHttpBufferContentFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpBufferContent, IHttpBufferContentFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpBufferContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpBufferContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpBufferContent {}
impl ::core::fmt::Debug for HttpBufferContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpBufferContent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpBufferContent {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpBufferContent;{6b14a441-fba7-4bd2-af0a-839de7c295da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpBufferContent {
    type Vtable = IHttpContent_Vtbl;
    const IID: ::windows_core::GUID = <IHttpContent as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpBufferContent {
    const NAME: &'static str = "Windows.Web.Http.HttpBufferContent";
}
impl ::core::convert::From<HttpBufferContent> for ::windows_core::IUnknown {
    fn from(value: HttpBufferContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpBufferContent> for ::windows_core::IUnknown {
    fn from(value: &HttpBufferContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpBufferContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpBufferContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpBufferContent> for ::windows_core::IInspectable {
    fn from(value: HttpBufferContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpBufferContent> for ::windows_core::IInspectable {
    fn from(value: &HttpBufferContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpBufferContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpBufferContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpBufferContent> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpBufferContent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpBufferContent> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpBufferContent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for HttpBufferContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &HttpBufferContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpBufferContent> for IHttpContent {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpBufferContent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpBufferContent> for IHttpContent {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpBufferContent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IHttpContent> for HttpBufferContent {
    fn into_param(self) -> ::windows_core::Param<'a, IHttpContent> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IHttpContent> for &HttpBufferContent {
    fn into_param(self) -> ::windows_core::Param<'a, IHttpContent> {
        ::core::convert::TryInto::<IHttpContent>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpBufferContent> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpBufferContent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpBufferContent> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpBufferContent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpBufferContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpBufferContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpBufferContent {}
unsafe impl ::core::marker::Sync for HttpBufferContent {}
#[repr(transparent)]
pub struct HttpClient(::windows_core::IUnknown);
impl HttpClient {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpClient, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn DeleteAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, uri: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>(result__)
        }
    }
    pub fn GetAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, uri: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>(result__)
        }
    }
    pub fn GetWithOptionAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, uri: Param0, completionoption: HttpCompletionOption) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetWithOptionAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), completionoption, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetBufferAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, uri: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IBuffer, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetBufferAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IBuffer, HttpProgress>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetInputStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, uri: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IInputStream, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetInputStreamAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IInputStream, HttpProgress>>(result__)
        }
    }
    pub fn GetStringAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, uri: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::windows_core::HSTRING, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStringAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::windows_core::HSTRING, HttpProgress>>(result__)
        }
    }
    pub fn PostAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, IHttpContent>>(&self, uri: Param0, content: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PostAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), content.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>(result__)
        }
    }
    pub fn PutAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, IHttpContent>>(&self, uri: Param0, content: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PutAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), content.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>(result__)
        }
    }
    pub fn SendRequestAsync<'a, Param0: ::windows_core::IntoParam<'a, HttpRequestMessage>>(&self, request: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SendRequestAsync)(::windows_core::Interface::as_raw(this), request.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>(result__)
        }
    }
    pub fn SendRequestWithOptionAsync<'a, Param0: ::windows_core::IntoParam<'a, HttpRequestMessage>>(&self, request: Param0, completionoption: HttpCompletionOption) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SendRequestWithOptionAsync)(::windows_core::Interface::as_raw(this), request.into_param().abi(), completionoption, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>(result__)
        }
    }
    #[cfg(feature = "Web_Http_Headers")]
    pub fn DefaultRequestHeaders(&self) -> ::windows_core::Result<Headers::HttpRequestHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultRequestHeaders)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Headers::HttpRequestHeaderCollection>(result__)
        }
    }
    pub fn TryDeleteAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, uri: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>> {
        let this = &::windows_core::Interface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryDeleteAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>(result__)
        }
    }
    pub fn TryGetAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, uri: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>> {
        let this = &::windows_core::Interface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>(result__)
        }
    }
    pub fn TryGetAsync2<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, uri: Param0, completionoption: HttpCompletionOption) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>> {
        let this = &::windows_core::Interface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAsync2)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), completionoption, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>(result__)
        }
    }
    pub fn TryGetBufferAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, uri: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<HttpGetBufferResult, HttpProgress>> {
        let this = &::windows_core::Interface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetBufferAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<HttpGetBufferResult, HttpProgress>>(result__)
        }
    }
    pub fn TryGetInputStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, uri: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<HttpGetInputStreamResult, HttpProgress>> {
        let this = &::windows_core::Interface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetInputStreamAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<HttpGetInputStreamResult, HttpProgress>>(result__)
        }
    }
    pub fn TryGetStringAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, uri: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<HttpGetStringResult, HttpProgress>> {
        let this = &::windows_core::Interface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetStringAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<HttpGetStringResult, HttpProgress>>(result__)
        }
    }
    pub fn TryPostAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, IHttpContent>>(&self, uri: Param0, content: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>> {
        let this = &::windows_core::Interface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryPostAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), content.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>(result__)
        }
    }
    pub fn TryPutAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, IHttpContent>>(&self, uri: Param0, content: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>> {
        let this = &::windows_core::Interface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryPutAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), content.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>(result__)
        }
    }
    pub fn TrySendRequestAsync<'a, Param0: ::windows_core::IntoParam<'a, HttpRequestMessage>>(&self, request: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>> {
        let this = &::windows_core::Interface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TrySendRequestAsync)(::windows_core::Interface::as_raw(this), request.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>(result__)
        }
    }
    pub fn TrySendRequestAsync2<'a, Param0: ::windows_core::IntoParam<'a, HttpRequestMessage>>(&self, request: Param0, completionoption: HttpCompletionOption) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>> {
        let this = &::windows_core::Interface::cast::<IHttpClient2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TrySendRequestAsync2)(::windows_core::Interface::as_raw(this), request.into_param().abi(), completionoption, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<HttpRequestResult, HttpProgress>>(result__)
        }
    }
    #[cfg(feature = "Web_Http_Filters")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, Filters::IHttpFilter>>(filter: Param0) -> ::windows_core::Result<HttpClient> {
        Self::IHttpClientFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), filter.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpClient>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpClientFactory<R, F: FnOnce(&IHttpClientFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpClient, IHttpClientFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpClient {}
impl ::core::fmt::Debug for HttpClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpClient").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpClient {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpClient;{7fda1151-3574-4880-a8ba-e6b1e0061f3d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpClient {
    type Vtable = IHttpClient_Vtbl;
    const IID: ::windows_core::GUID = <IHttpClient as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpClient {
    const NAME: &'static str = "Windows.Web.Http.HttpClient";
}
impl ::core::convert::From<HttpClient> for ::windows_core::IUnknown {
    fn from(value: HttpClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpClient> for ::windows_core::IUnknown {
    fn from(value: &HttpClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpClient> for ::windows_core::IInspectable {
    fn from(value: HttpClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpClient> for ::windows_core::IInspectable {
    fn from(value: &HttpClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpClient> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpClient) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpClient> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpClient) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for HttpClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &HttpClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpClient> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpClient) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpClient> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpClient) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpClient {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpClient {}
unsafe impl ::core::marker::Sync for HttpClient {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HttpCompletionOption(pub i32);
impl HttpCompletionOption {
    pub const ResponseContentRead: Self = Self(0i32);
    pub const ResponseHeadersRead: Self = Self(1i32);
}
impl ::core::marker::Copy for HttpCompletionOption {}
impl ::core::clone::Clone for HttpCompletionOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HttpCompletionOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HttpCompletionOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for HttpCompletionOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCompletionOption").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpCompletionOption {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Web.Http.HttpCompletionOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct HttpCookie(::windows_core::IUnknown);
impl HttpCookie {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Domain(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Domain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Path(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Path)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
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
    pub fn HttpOnly(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HttpOnly)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHttpOnly(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHttpOnly)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Secure(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Secure)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetSecure(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSecure)(::windows_core::Interface::as_raw(this), value).ok() }
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
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(name: Param0, domain: Param1, path: Param2) -> ::windows_core::Result<HttpCookie> {
        Self::IHttpCookieFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), name.into_param().abi(), domain.into_param().abi(), path.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpCookie>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpCookieFactory<R, F: FnOnce(&IHttpCookieFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpCookie, IHttpCookieFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpCookie {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpCookie {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpCookie {}
impl ::core::fmt::Debug for HttpCookie {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCookie").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpCookie {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpCookie;{1f5488e2-cc2d-4779-86a7-88f10687d249})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpCookie {
    type Vtable = IHttpCookie_Vtbl;
    const IID: ::windows_core::GUID = <IHttpCookie as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpCookie {
    const NAME: &'static str = "Windows.Web.Http.HttpCookie";
}
impl ::core::convert::From<HttpCookie> for ::windows_core::IUnknown {
    fn from(value: HttpCookie) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCookie> for ::windows_core::IUnknown {
    fn from(value: &HttpCookie) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpCookie {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpCookie {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpCookie> for ::windows_core::IInspectable {
    fn from(value: HttpCookie) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCookie> for ::windows_core::IInspectable {
    fn from(value: &HttpCookie) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpCookie {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpCookie {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpCookie> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpCookie) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpCookie> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpCookie) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpCookie {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpCookie {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpCookie {}
unsafe impl ::core::marker::Sync for HttpCookie {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct HttpCookieCollection(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl HttpCookieCollection {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<HttpCookie>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<HttpCookie>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<HttpCookie>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<HttpCookie> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<HttpCookie>(result__)
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
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, HttpCookie>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<HttpCookie>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for HttpCookieCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for HttpCookieCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for HttpCookieCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for HttpCookieCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCookieCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::RuntimeType for HttpCookieCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpCookieCollection;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Web.Http.HttpCookie;{1f5488e2-cc2d-4779-86a7-88f10687d249})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for HttpCookieCollection {
    type Vtable = ::winrt_foundation::Collections::IVectorView_Vtbl<HttpCookie>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVectorView<HttpCookie> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for HttpCookieCollection {
    const NAME: &'static str = "Windows.Web.Http.HttpCookieCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpCookieCollection {
    type Item = HttpCookie;
    type IntoIter = ::winrt_foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpCookieCollection {
    type Item = HttpCookie;
    type IntoIter = ::winrt_foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<HttpCookieCollection> for ::windows_core::IUnknown {
    fn from(value: HttpCookieCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&HttpCookieCollection> for ::windows_core::IUnknown {
    fn from(value: &HttpCookieCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpCookieCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpCookieCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<HttpCookieCollection> for ::windows_core::IInspectable {
    fn from(value: HttpCookieCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&HttpCookieCollection> for ::windows_core::IInspectable {
    fn from(value: &HttpCookieCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpCookieCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpCookieCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpCookieCollection> for ::winrt_foundation::Collections::IIterable<HttpCookie> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpCookieCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpCookieCollection> for ::winrt_foundation::Collections::IIterable<HttpCookie> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpCookieCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<HttpCookie>> for HttpCookieCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<HttpCookie>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<HttpCookie>> for &HttpCookieCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<HttpCookie>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<HttpCookie>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpCookieCollection> for ::winrt_foundation::Collections::IVectorView<HttpCookie> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpCookieCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpCookieCollection> for ::winrt_foundation::Collections::IVectorView<HttpCookie> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpCookieCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<HttpCookie>> for HttpCookieCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVectorView<HttpCookie>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<HttpCookie>> for &HttpCookieCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVectorView<HttpCookie>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVectorView<HttpCookie>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for HttpCookieCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for HttpCookieCollection {}
#[repr(transparent)]
pub struct HttpCookieManager(::windows_core::IUnknown);
impl HttpCookieManager {
    pub fn SetCookie<'a, Param0: ::windows_core::IntoParam<'a, HttpCookie>>(&self, cookie: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).SetCookie)(::windows_core::Interface::as_raw(this), cookie.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCookieWithThirdParty<'a, Param0: ::windows_core::IntoParam<'a, HttpCookie>>(&self, cookie: Param0, thirdparty: bool) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).SetCookieWithThirdParty)(::windows_core::Interface::as_raw(this), cookie.into_param().abi(), thirdparty, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn DeleteCookie<'a, Param0: ::windows_core::IntoParam<'a, HttpCookie>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).DeleteCookie)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCookies<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, uri: Param0) -> ::windows_core::Result<HttpCookieCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCookies)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpCookieCollection>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpCookieManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpCookieManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpCookieManager {}
impl ::core::fmt::Debug for HttpCookieManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCookieManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpCookieManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpCookieManager;{7a431780-cd4f-4e57-a84a-5b0a53d6bb96})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpCookieManager {
    type Vtable = IHttpCookieManager_Vtbl;
    const IID: ::windows_core::GUID = <IHttpCookieManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpCookieManager {
    const NAME: &'static str = "Windows.Web.Http.HttpCookieManager";
}
impl ::core::convert::From<HttpCookieManager> for ::windows_core::IUnknown {
    fn from(value: HttpCookieManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCookieManager> for ::windows_core::IUnknown {
    fn from(value: &HttpCookieManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpCookieManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpCookieManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpCookieManager> for ::windows_core::IInspectable {
    fn from(value: HttpCookieManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpCookieManager> for ::windows_core::IInspectable {
    fn from(value: &HttpCookieManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpCookieManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpCookieManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HttpCookieManager {}
unsafe impl ::core::marker::Sync for HttpCookieManager {}
#[repr(transparent)]
pub struct HttpFormUrlEncodedContent(::windows_core::IUnknown);
impl HttpFormUrlEncodedContent {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows_core::Result<Headers::HttpContentHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Headers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Headers::HttpContentHeaderCollection>(result__)
        }
    }
    pub fn BufferAllAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BufferAllAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ReadAsBufferAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IBuffer, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsBufferAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IBuffer, u64>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ReadAsInputStreamAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IInputStream, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsInputStreamAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IInputStream, u64>>(result__)
        }
    }
    pub fn ReadAsStringAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::windows_core::HSTRING, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsStringAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::windows_core::HSTRING, u64>>(result__)
        }
    }
    pub fn TryComputeLength(&self, length: &mut u64) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryComputeLength)(::windows_core::Interface::as_raw(this), length, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn WriteToStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IOutputStream>>(&self, outputstream: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WriteToStreamAsync)(::windows_core::Interface::as_raw(this), outputstream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>>(content: Param0) -> ::windows_core::Result<HttpFormUrlEncodedContent> {
        Self::IHttpFormUrlEncodedContentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), content.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpFormUrlEncodedContent>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpFormUrlEncodedContentFactory<R, F: FnOnce(&IHttpFormUrlEncodedContentFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpFormUrlEncodedContent, IHttpFormUrlEncodedContentFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpFormUrlEncodedContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpFormUrlEncodedContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpFormUrlEncodedContent {}
impl ::core::fmt::Debug for HttpFormUrlEncodedContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpFormUrlEncodedContent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpFormUrlEncodedContent {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpFormUrlEncodedContent;{6b14a441-fba7-4bd2-af0a-839de7c295da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpFormUrlEncodedContent {
    type Vtable = IHttpContent_Vtbl;
    const IID: ::windows_core::GUID = <IHttpContent as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpFormUrlEncodedContent {
    const NAME: &'static str = "Windows.Web.Http.HttpFormUrlEncodedContent";
}
impl ::core::convert::From<HttpFormUrlEncodedContent> for ::windows_core::IUnknown {
    fn from(value: HttpFormUrlEncodedContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpFormUrlEncodedContent> for ::windows_core::IUnknown {
    fn from(value: &HttpFormUrlEncodedContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpFormUrlEncodedContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpFormUrlEncodedContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpFormUrlEncodedContent> for ::windows_core::IInspectable {
    fn from(value: HttpFormUrlEncodedContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpFormUrlEncodedContent> for ::windows_core::IInspectable {
    fn from(value: &HttpFormUrlEncodedContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpFormUrlEncodedContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpFormUrlEncodedContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpFormUrlEncodedContent> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpFormUrlEncodedContent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpFormUrlEncodedContent> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpFormUrlEncodedContent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for HttpFormUrlEncodedContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &HttpFormUrlEncodedContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpFormUrlEncodedContent> for IHttpContent {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpFormUrlEncodedContent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpFormUrlEncodedContent> for IHttpContent {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpFormUrlEncodedContent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IHttpContent> for HttpFormUrlEncodedContent {
    fn into_param(self) -> ::windows_core::Param<'a, IHttpContent> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IHttpContent> for &HttpFormUrlEncodedContent {
    fn into_param(self) -> ::windows_core::Param<'a, IHttpContent> {
        ::core::convert::TryInto::<IHttpContent>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpFormUrlEncodedContent> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpFormUrlEncodedContent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpFormUrlEncodedContent> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpFormUrlEncodedContent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpFormUrlEncodedContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpFormUrlEncodedContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpFormUrlEncodedContent {}
unsafe impl ::core::marker::Sync for HttpFormUrlEncodedContent {}
#[repr(transparent)]
pub struct HttpGetBufferResult(::windows_core::IUnknown);
impl HttpGetBufferResult {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn RequestMessage(&self) -> ::windows_core::Result<HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpRequestMessage>(result__)
        }
    }
    pub fn ResponseMessage(&self) -> ::windows_core::Result<HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResponseMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpResponseMessage>(result__)
        }
    }
    pub fn Succeeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Succeeded)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Value(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpGetBufferResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpGetBufferResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpGetBufferResult {}
impl ::core::fmt::Debug for HttpGetBufferResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpGetBufferResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpGetBufferResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpGetBufferResult;{53d08e7c-e209-404e-9a49-742d8236fd3a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpGetBufferResult {
    type Vtable = IHttpGetBufferResult_Vtbl;
    const IID: ::windows_core::GUID = <IHttpGetBufferResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpGetBufferResult {
    const NAME: &'static str = "Windows.Web.Http.HttpGetBufferResult";
}
impl ::core::convert::From<HttpGetBufferResult> for ::windows_core::IUnknown {
    fn from(value: HttpGetBufferResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpGetBufferResult> for ::windows_core::IUnknown {
    fn from(value: &HttpGetBufferResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpGetBufferResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpGetBufferResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpGetBufferResult> for ::windows_core::IInspectable {
    fn from(value: HttpGetBufferResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpGetBufferResult> for ::windows_core::IInspectable {
    fn from(value: &HttpGetBufferResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpGetBufferResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpGetBufferResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpGetBufferResult> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpGetBufferResult) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpGetBufferResult> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpGetBufferResult) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for HttpGetBufferResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &HttpGetBufferResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpGetBufferResult> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpGetBufferResult) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpGetBufferResult> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpGetBufferResult) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpGetBufferResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpGetBufferResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpGetBufferResult {}
unsafe impl ::core::marker::Sync for HttpGetBufferResult {}
#[repr(transparent)]
pub struct HttpGetInputStreamResult(::windows_core::IUnknown);
impl HttpGetInputStreamResult {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn RequestMessage(&self) -> ::windows_core::Result<HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpRequestMessage>(result__)
        }
    }
    pub fn ResponseMessage(&self) -> ::windows_core::Result<HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResponseMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpResponseMessage>(result__)
        }
    }
    pub fn Succeeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Succeeded)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Value(&self) -> ::windows_core::Result<::winrt_storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IInputStream>(result__)
        }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpGetInputStreamResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpGetInputStreamResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpGetInputStreamResult {}
impl ::core::fmt::Debug for HttpGetInputStreamResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpGetInputStreamResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpGetInputStreamResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpGetInputStreamResult;{d5d63463-13aa-4ee0-be95-a0c39fe91203})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpGetInputStreamResult {
    type Vtable = IHttpGetInputStreamResult_Vtbl;
    const IID: ::windows_core::GUID = <IHttpGetInputStreamResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpGetInputStreamResult {
    const NAME: &'static str = "Windows.Web.Http.HttpGetInputStreamResult";
}
impl ::core::convert::From<HttpGetInputStreamResult> for ::windows_core::IUnknown {
    fn from(value: HttpGetInputStreamResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpGetInputStreamResult> for ::windows_core::IUnknown {
    fn from(value: &HttpGetInputStreamResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpGetInputStreamResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpGetInputStreamResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpGetInputStreamResult> for ::windows_core::IInspectable {
    fn from(value: HttpGetInputStreamResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpGetInputStreamResult> for ::windows_core::IInspectable {
    fn from(value: &HttpGetInputStreamResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpGetInputStreamResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpGetInputStreamResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpGetInputStreamResult> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpGetInputStreamResult) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpGetInputStreamResult> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpGetInputStreamResult) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for HttpGetInputStreamResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &HttpGetInputStreamResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpGetInputStreamResult> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpGetInputStreamResult) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpGetInputStreamResult> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpGetInputStreamResult) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpGetInputStreamResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpGetInputStreamResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpGetInputStreamResult {}
unsafe impl ::core::marker::Sync for HttpGetInputStreamResult {}
#[repr(transparent)]
pub struct HttpGetStringResult(::windows_core::IUnknown);
impl HttpGetStringResult {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn RequestMessage(&self) -> ::windows_core::Result<HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpRequestMessage>(result__)
        }
    }
    pub fn ResponseMessage(&self) -> ::windows_core::Result<HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResponseMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpResponseMessage>(result__)
        }
    }
    pub fn Succeeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Succeeded)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpGetStringResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpGetStringResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpGetStringResult {}
impl ::core::fmt::Debug for HttpGetStringResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpGetStringResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpGetStringResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpGetStringResult;{9bac466d-8509-4775-b16d-8953f47a7f5f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpGetStringResult {
    type Vtable = IHttpGetStringResult_Vtbl;
    const IID: ::windows_core::GUID = <IHttpGetStringResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpGetStringResult {
    const NAME: &'static str = "Windows.Web.Http.HttpGetStringResult";
}
impl ::core::convert::From<HttpGetStringResult> for ::windows_core::IUnknown {
    fn from(value: HttpGetStringResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpGetStringResult> for ::windows_core::IUnknown {
    fn from(value: &HttpGetStringResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpGetStringResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpGetStringResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpGetStringResult> for ::windows_core::IInspectable {
    fn from(value: HttpGetStringResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpGetStringResult> for ::windows_core::IInspectable {
    fn from(value: &HttpGetStringResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpGetStringResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpGetStringResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpGetStringResult> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpGetStringResult) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpGetStringResult> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpGetStringResult) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for HttpGetStringResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &HttpGetStringResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpGetStringResult> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpGetStringResult) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpGetStringResult> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpGetStringResult) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpGetStringResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpGetStringResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpGetStringResult {}
unsafe impl ::core::marker::Sync for HttpGetStringResult {}
#[repr(transparent)]
pub struct HttpMethod(::windows_core::IUnknown);
impl HttpMethod {
    pub fn Method(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Method)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(method: Param0) -> ::windows_core::Result<HttpMethod> {
        Self::IHttpMethodFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), method.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpMethod>(result__)
        })
    }
    pub fn Delete() -> ::windows_core::Result<HttpMethod> {
        Self::IHttpMethodStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Delete)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpMethod>(result__)
        })
    }
    pub fn Get() -> ::windows_core::Result<HttpMethod> {
        Self::IHttpMethodStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Get)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpMethod>(result__)
        })
    }
    pub fn Head() -> ::windows_core::Result<HttpMethod> {
        Self::IHttpMethodStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Head)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpMethod>(result__)
        })
    }
    pub fn Options() -> ::windows_core::Result<HttpMethod> {
        Self::IHttpMethodStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Options)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpMethod>(result__)
        })
    }
    pub fn Patch() -> ::windows_core::Result<HttpMethod> {
        Self::IHttpMethodStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Patch)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpMethod>(result__)
        })
    }
    pub fn Post() -> ::windows_core::Result<HttpMethod> {
        Self::IHttpMethodStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Post)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpMethod>(result__)
        })
    }
    pub fn Put() -> ::windows_core::Result<HttpMethod> {
        Self::IHttpMethodStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Put)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpMethod>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpMethodFactory<R, F: FnOnce(&IHttpMethodFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpMethod, IHttpMethodFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHttpMethodStatics<R, F: FnOnce(&IHttpMethodStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpMethod, IHttpMethodStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpMethod {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpMethod {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpMethod {}
impl ::core::fmt::Debug for HttpMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpMethod").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpMethod {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpMethod;{728d4022-700d-4fe0-afa5-40299c58dbfd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpMethod {
    type Vtable = IHttpMethod_Vtbl;
    const IID: ::windows_core::GUID = <IHttpMethod as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpMethod {
    const NAME: &'static str = "Windows.Web.Http.HttpMethod";
}
impl ::core::convert::From<HttpMethod> for ::windows_core::IUnknown {
    fn from(value: HttpMethod) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMethod> for ::windows_core::IUnknown {
    fn from(value: &HttpMethod) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpMethod {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpMethod {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpMethod> for ::windows_core::IInspectable {
    fn from(value: HttpMethod) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMethod> for ::windows_core::IInspectable {
    fn from(value: &HttpMethod) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpMethod {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpMethod {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpMethod> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpMethod) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpMethod> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpMethod) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpMethod {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpMethod {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpMethod {}
unsafe impl ::core::marker::Sync for HttpMethod {}
#[repr(transparent)]
pub struct HttpMultipartContent(::windows_core::IUnknown);
impl HttpMultipartContent {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpMultipartContent, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows_core::Result<Headers::HttpContentHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Headers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Headers::HttpContentHeaderCollection>(result__)
        }
    }
    pub fn BufferAllAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BufferAllAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ReadAsBufferAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IBuffer, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsBufferAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IBuffer, u64>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ReadAsInputStreamAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IInputStream, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsInputStreamAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IInputStream, u64>>(result__)
        }
    }
    pub fn ReadAsStringAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::windows_core::HSTRING, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsStringAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::windows_core::HSTRING, u64>>(result__)
        }
    }
    pub fn TryComputeLength(&self, length: &mut u64) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryComputeLength)(::windows_core::Interface::as_raw(this), length, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn WriteToStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IOutputStream>>(&self, outputstream: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WriteToStreamAsync)(::windows_core::Interface::as_raw(this), outputstream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    pub fn Add<'a, Param0: ::windows_core::IntoParam<'a, IHttpContent>>(&self, content: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IHttpMultipartContent>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Add)(::windows_core::Interface::as_raw(this), content.into_param().abi()).ok() }
    }
    pub fn CreateWithSubtype<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(subtype: Param0) -> ::windows_core::Result<HttpMultipartContent> {
        Self::IHttpMultipartContentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithSubtype)(::windows_core::Interface::as_raw(this), subtype.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpMultipartContent>(result__)
        })
    }
    pub fn CreateWithSubtypeAndBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(subtype: Param0, boundary: Param1) -> ::windows_core::Result<HttpMultipartContent> {
        Self::IHttpMultipartContentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithSubtypeAndBoundary)(::windows_core::Interface::as_raw(this), subtype.into_param().abi(), boundary.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpMultipartContent>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<IHttpContent>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<IHttpContent>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<IHttpContent>>(result__)
        }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpMultipartContentFactory<R, F: FnOnce(&IHttpMultipartContentFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpMultipartContent, IHttpMultipartContentFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpMultipartContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpMultipartContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpMultipartContent {}
impl ::core::fmt::Debug for HttpMultipartContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpMultipartContent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpMultipartContent {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpMultipartContent;{6b14a441-fba7-4bd2-af0a-839de7c295da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpMultipartContent {
    type Vtable = IHttpContent_Vtbl;
    const IID: ::windows_core::GUID = <IHttpContent as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpMultipartContent {
    const NAME: &'static str = "Windows.Web.Http.HttpMultipartContent";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpMultipartContent {
    type Item = IHttpContent;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpMultipartContent {
    type Item = IHttpContent;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl ::core::convert::From<HttpMultipartContent> for ::windows_core::IUnknown {
    fn from(value: HttpMultipartContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMultipartContent> for ::windows_core::IUnknown {
    fn from(value: &HttpMultipartContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpMultipartContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpMultipartContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpMultipartContent> for ::windows_core::IInspectable {
    fn from(value: HttpMultipartContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMultipartContent> for ::windows_core::IInspectable {
    fn from(value: &HttpMultipartContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpMultipartContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpMultipartContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpMultipartContent> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpMultipartContent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpMultipartContent> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpMultipartContent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for HttpMultipartContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &HttpMultipartContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpMultipartContent> for IHttpContent {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpMultipartContent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpMultipartContent> for IHttpContent {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpMultipartContent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IHttpContent> for HttpMultipartContent {
    fn into_param(self) -> ::windows_core::Param<'a, IHttpContent> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IHttpContent> for &HttpMultipartContent {
    fn into_param(self) -> ::windows_core::Param<'a, IHttpContent> {
        ::core::convert::TryInto::<IHttpContent>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpMultipartContent> for ::winrt_foundation::Collections::IIterable<IHttpContent> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpMultipartContent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpMultipartContent> for ::winrt_foundation::Collections::IIterable<IHttpContent> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpMultipartContent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<IHttpContent>> for HttpMultipartContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<IHttpContent>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<IHttpContent>> for &HttpMultipartContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<IHttpContent>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<IHttpContent>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpMultipartContent> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpMultipartContent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpMultipartContent> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpMultipartContent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpMultipartContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpMultipartContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpMultipartContent {}
unsafe impl ::core::marker::Sync for HttpMultipartContent {}
#[repr(transparent)]
pub struct HttpMultipartFormDataContent(::windows_core::IUnknown);
impl HttpMultipartFormDataContent {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpMultipartFormDataContent, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows_core::Result<Headers::HttpContentHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Headers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Headers::HttpContentHeaderCollection>(result__)
        }
    }
    pub fn BufferAllAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BufferAllAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ReadAsBufferAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IBuffer, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsBufferAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IBuffer, u64>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ReadAsInputStreamAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IInputStream, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsInputStreamAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IInputStream, u64>>(result__)
        }
    }
    pub fn ReadAsStringAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::windows_core::HSTRING, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsStringAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::windows_core::HSTRING, u64>>(result__)
        }
    }
    pub fn TryComputeLength(&self, length: &mut u64) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryComputeLength)(::windows_core::Interface::as_raw(this), length, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn WriteToStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IOutputStream>>(&self, outputstream: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WriteToStreamAsync)(::windows_core::Interface::as_raw(this), outputstream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    pub fn Add<'a, Param0: ::windows_core::IntoParam<'a, IHttpContent>>(&self, content: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IHttpMultipartFormDataContent>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Add)(::windows_core::Interface::as_raw(this), content.into_param().abi()).ok() }
    }
    pub fn AddWithName<'a, Param0: ::windows_core::IntoParam<'a, IHttpContent>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, content: Param0, name: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IHttpMultipartFormDataContent>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddWithName)(::windows_core::Interface::as_raw(this), content.into_param().abi(), name.into_param().abi()).ok() }
    }
    pub fn AddWithNameAndFileName<'a, Param0: ::windows_core::IntoParam<'a, IHttpContent>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, content: Param0, name: Param1, filename: Param2) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IHttpMultipartFormDataContent>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddWithNameAndFileName)(::windows_core::Interface::as_raw(this), content.into_param().abi(), name.into_param().abi(), filename.into_param().abi()).ok() }
    }
    pub fn CreateWithBoundary<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(boundary: Param0) -> ::windows_core::Result<HttpMultipartFormDataContent> {
        Self::IHttpMultipartFormDataContentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithBoundary)(::windows_core::Interface::as_raw(this), boundary.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpMultipartFormDataContent>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<IHttpContent>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<IHttpContent>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<IHttpContent>>(result__)
        }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpMultipartFormDataContentFactory<R, F: FnOnce(&IHttpMultipartFormDataContentFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpMultipartFormDataContent, IHttpMultipartFormDataContentFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpMultipartFormDataContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpMultipartFormDataContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpMultipartFormDataContent {}
impl ::core::fmt::Debug for HttpMultipartFormDataContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpMultipartFormDataContent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpMultipartFormDataContent {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpMultipartFormDataContent;{6b14a441-fba7-4bd2-af0a-839de7c295da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpMultipartFormDataContent {
    type Vtable = IHttpContent_Vtbl;
    const IID: ::windows_core::GUID = <IHttpContent as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpMultipartFormDataContent {
    const NAME: &'static str = "Windows.Web.Http.HttpMultipartFormDataContent";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpMultipartFormDataContent {
    type Item = IHttpContent;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpMultipartFormDataContent {
    type Item = IHttpContent;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl ::core::convert::From<HttpMultipartFormDataContent> for ::windows_core::IUnknown {
    fn from(value: HttpMultipartFormDataContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMultipartFormDataContent> for ::windows_core::IUnknown {
    fn from(value: &HttpMultipartFormDataContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpMultipartFormDataContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpMultipartFormDataContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpMultipartFormDataContent> for ::windows_core::IInspectable {
    fn from(value: HttpMultipartFormDataContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpMultipartFormDataContent> for ::windows_core::IInspectable {
    fn from(value: &HttpMultipartFormDataContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpMultipartFormDataContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpMultipartFormDataContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpMultipartFormDataContent> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpMultipartFormDataContent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpMultipartFormDataContent> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpMultipartFormDataContent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for HttpMultipartFormDataContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &HttpMultipartFormDataContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpMultipartFormDataContent> for IHttpContent {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpMultipartFormDataContent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpMultipartFormDataContent> for IHttpContent {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpMultipartFormDataContent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IHttpContent> for HttpMultipartFormDataContent {
    fn into_param(self) -> ::windows_core::Param<'a, IHttpContent> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IHttpContent> for &HttpMultipartFormDataContent {
    fn into_param(self) -> ::windows_core::Param<'a, IHttpContent> {
        ::core::convert::TryInto::<IHttpContent>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpMultipartFormDataContent> for ::winrt_foundation::Collections::IIterable<IHttpContent> {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpMultipartFormDataContent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpMultipartFormDataContent> for ::winrt_foundation::Collections::IIterable<IHttpContent> {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpMultipartFormDataContent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<IHttpContent>> for HttpMultipartFormDataContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<IHttpContent>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<IHttpContent>> for &HttpMultipartFormDataContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<IHttpContent>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<IHttpContent>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpMultipartFormDataContent> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpMultipartFormDataContent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpMultipartFormDataContent> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpMultipartFormDataContent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpMultipartFormDataContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpMultipartFormDataContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpMultipartFormDataContent {}
unsafe impl ::core::marker::Sync for HttpMultipartFormDataContent {}
#[repr(C)]
pub struct HttpProgress {
    pub Stage: HttpProgressStage,
    pub BytesSent: u64,
    pub TotalBytesToSend: ::core::option::Option<::winrt_foundation::IReference<u64>>,
    pub BytesReceived: u64,
    pub TotalBytesToReceive: ::core::option::Option<::winrt_foundation::IReference<u64>>,
    pub Retries: u32,
}
impl ::core::clone::Clone for HttpProgress {
    fn clone(&self) -> Self {
        Self {
            Stage: self.Stage,
            BytesSent: self.BytesSent,
            TotalBytesToSend: self.TotalBytesToSend.clone(),
            BytesReceived: self.BytesReceived,
            TotalBytesToReceive: self.TotalBytesToReceive.clone(),
            Retries: self.Retries,
        }
    }
}
impl ::core::fmt::Debug for HttpProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HttpProgress").field("Stage", &self.Stage).field("BytesSent", &self.BytesSent).field("TotalBytesToSend", &self.TotalBytesToSend).field("BytesReceived", &self.BytesReceived).field("TotalBytesToReceive", &self.TotalBytesToReceive).field("Retries", &self.Retries).finish()
    }
}
unsafe impl ::windows_core::Abi for HttpProgress {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
unsafe impl ::windows_core::RuntimeType for HttpProgress {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Web.Http.HttpProgress;enum(Windows.Web.Http.HttpProgressStage;i4);u8;pinterface({61c17706-2d65-11e0-9ae8-d48564015472};u8);u8;pinterface({61c17706-2d65-11e0-9ae8-d48564015472};u8);u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(from.clone())
    }
}
impl ::core::cmp::PartialEq for HttpProgress {
    fn eq(&self, other: &Self) -> bool {
        self.Stage == other.Stage && self.BytesSent == other.BytesSent && self.TotalBytesToSend == other.TotalBytesToSend && self.BytesReceived == other.BytesReceived && self.TotalBytesToReceive == other.TotalBytesToReceive && self.Retries == other.Retries
    }
}
impl ::core::cmp::Eq for HttpProgress {}
impl ::core::default::Default for HttpProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HttpProgressStage(pub i32);
impl HttpProgressStage {
    pub const None: Self = Self(0i32);
    pub const DetectingProxy: Self = Self(10i32);
    pub const ResolvingName: Self = Self(20i32);
    pub const ConnectingToServer: Self = Self(30i32);
    pub const NegotiatingSsl: Self = Self(40i32);
    pub const SendingHeaders: Self = Self(50i32);
    pub const SendingContent: Self = Self(60i32);
    pub const WaitingForResponse: Self = Self(70i32);
    pub const ReceivingHeaders: Self = Self(80i32);
    pub const ReceivingContent: Self = Self(90i32);
}
impl ::core::marker::Copy for HttpProgressStage {}
impl ::core::clone::Clone for HttpProgressStage {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HttpProgressStage {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HttpProgressStage {
    type Abi = Self;
}
impl ::core::fmt::Debug for HttpProgressStage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpProgressStage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpProgressStage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Web.Http.HttpProgressStage;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct HttpRequestMessage(::windows_core::IUnknown);
impl HttpRequestMessage {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpRequestMessage, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Content(&self) -> ::windows_core::Result<IHttpContent> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IHttpContent>(result__)
        }
    }
    pub fn SetContent<'a, Param0: ::windows_core::IntoParam<'a, IHttpContent>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContent)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows_core::Result<Headers::HttpRequestHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Headers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Headers::HttpRequestHeaderCollection>(result__)
        }
    }
    pub fn Method(&self) -> ::windows_core::Result<HttpMethod> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Method)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpMethod>(result__)
        }
    }
    pub fn SetMethod<'a, Param0: ::windows_core::IntoParam<'a, HttpMethod>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMethod)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(result__)
        }
    }
    pub fn RequestUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetRequestUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRequestUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TransportInformation(&self) -> ::windows_core::Result<HttpTransportInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TransportInformation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpTransportInformation>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, HttpMethod>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(method: Param0, uri: Param1) -> ::windows_core::Result<HttpRequestMessage> {
        Self::IHttpRequestMessageFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), method.into_param().abi(), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpRequestMessage>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpRequestMessageFactory<R, F: FnOnce(&IHttpRequestMessageFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpRequestMessage, IHttpRequestMessageFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpRequestMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpRequestMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpRequestMessage {}
impl ::core::fmt::Debug for HttpRequestMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpRequestMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpRequestMessage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpRequestMessage;{f5762b3c-74d4-4811-b5dc-9f8b4e2f9abf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpRequestMessage {
    type Vtable = IHttpRequestMessage_Vtbl;
    const IID: ::windows_core::GUID = <IHttpRequestMessage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpRequestMessage {
    const NAME: &'static str = "Windows.Web.Http.HttpRequestMessage";
}
impl ::core::convert::From<HttpRequestMessage> for ::windows_core::IUnknown {
    fn from(value: HttpRequestMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpRequestMessage> for ::windows_core::IUnknown {
    fn from(value: &HttpRequestMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpRequestMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpRequestMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpRequestMessage> for ::windows_core::IInspectable {
    fn from(value: HttpRequestMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpRequestMessage> for ::windows_core::IInspectable {
    fn from(value: &HttpRequestMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpRequestMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpRequestMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpRequestMessage> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpRequestMessage) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpRequestMessage> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpRequestMessage) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for HttpRequestMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &HttpRequestMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpRequestMessage> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpRequestMessage) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpRequestMessage> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpRequestMessage) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpRequestMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpRequestMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpRequestMessage {}
unsafe impl ::core::marker::Sync for HttpRequestMessage {}
#[repr(transparent)]
pub struct HttpRequestResult(::windows_core::IUnknown);
impl HttpRequestResult {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn RequestMessage(&self) -> ::windows_core::Result<HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpRequestMessage>(result__)
        }
    }
    pub fn ResponseMessage(&self) -> ::windows_core::Result<HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResponseMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpResponseMessage>(result__)
        }
    }
    pub fn Succeeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Succeeded)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpRequestResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpRequestResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpRequestResult {}
impl ::core::fmt::Debug for HttpRequestResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpRequestResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpRequestResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpRequestResult;{6acf4da8-b5eb-4a35-a902-4217fbe820c5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpRequestResult {
    type Vtable = IHttpRequestResult_Vtbl;
    const IID: ::windows_core::GUID = <IHttpRequestResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpRequestResult {
    const NAME: &'static str = "Windows.Web.Http.HttpRequestResult";
}
impl ::core::convert::From<HttpRequestResult> for ::windows_core::IUnknown {
    fn from(value: HttpRequestResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpRequestResult> for ::windows_core::IUnknown {
    fn from(value: &HttpRequestResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpRequestResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpRequestResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpRequestResult> for ::windows_core::IInspectable {
    fn from(value: HttpRequestResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpRequestResult> for ::windows_core::IInspectable {
    fn from(value: &HttpRequestResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpRequestResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpRequestResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpRequestResult> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpRequestResult) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpRequestResult> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpRequestResult) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for HttpRequestResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &HttpRequestResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpRequestResult> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpRequestResult) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpRequestResult> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpRequestResult) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpRequestResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpRequestResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpRequestResult {}
unsafe impl ::core::marker::Sync for HttpRequestResult {}
#[repr(transparent)]
pub struct HttpResponseMessage(::windows_core::IUnknown);
impl HttpResponseMessage {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpResponseMessage, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Content(&self) -> ::windows_core::Result<IHttpContent> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IHttpContent>(result__)
        }
    }
    pub fn SetContent<'a, Param0: ::windows_core::IntoParam<'a, IHttpContent>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContent)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows_core::Result<Headers::HttpResponseHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Headers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Headers::HttpResponseHeaderCollection>(result__)
        }
    }
    pub fn IsSuccessStatusCode(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSuccessStatusCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ReasonPhrase(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ReasonPhrase)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetReasonPhrase<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReasonPhrase)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RequestMessage(&self) -> ::windows_core::Result<HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestMessage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpRequestMessage>(result__)
        }
    }
    pub fn SetRequestMessage<'a, Param0: ::windows_core::IntoParam<'a, HttpRequestMessage>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRequestMessage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Source(&self) -> ::windows_core::Result<HttpResponseMessageSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HttpResponseMessageSource>::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpResponseMessageSource>(result__)
        }
    }
    pub fn SetSource(&self, value: HttpResponseMessageSource) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSource)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StatusCode(&self) -> ::windows_core::Result<HttpStatusCode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HttpStatusCode>::zeroed();
            (::windows_core::Interface::vtable(this).StatusCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpStatusCode>(result__)
        }
    }
    pub fn SetStatusCode(&self, value: HttpStatusCode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStatusCode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Version(&self) -> ::windows_core::Result<HttpVersion> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HttpVersion>::zeroed();
            (::windows_core::Interface::vtable(this).Version)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpVersion>(result__)
        }
    }
    pub fn SetVersion(&self, value: HttpVersion) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVersion)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EnsureSuccessStatusCode(&self) -> ::windows_core::Result<HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EnsureSuccessStatusCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HttpResponseMessage>(result__)
        }
    }
    pub fn Create(statuscode: HttpStatusCode) -> ::windows_core::Result<HttpResponseMessage> {
        Self::IHttpResponseMessageFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), statuscode, result__.as_mut_ptr()).from_abi::<HttpResponseMessage>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpResponseMessageFactory<R, F: FnOnce(&IHttpResponseMessageFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpResponseMessage, IHttpResponseMessageFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpResponseMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpResponseMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpResponseMessage {}
impl ::core::fmt::Debug for HttpResponseMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpResponseMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpResponseMessage {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpResponseMessage;{fee200fb-8664-44e0-95d9-42696199bffc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpResponseMessage {
    type Vtable = IHttpResponseMessage_Vtbl;
    const IID: ::windows_core::GUID = <IHttpResponseMessage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpResponseMessage {
    const NAME: &'static str = "Windows.Web.Http.HttpResponseMessage";
}
impl ::core::convert::From<HttpResponseMessage> for ::windows_core::IUnknown {
    fn from(value: HttpResponseMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpResponseMessage> for ::windows_core::IUnknown {
    fn from(value: &HttpResponseMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpResponseMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpResponseMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpResponseMessage> for ::windows_core::IInspectable {
    fn from(value: HttpResponseMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpResponseMessage> for ::windows_core::IInspectable {
    fn from(value: &HttpResponseMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpResponseMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpResponseMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpResponseMessage> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpResponseMessage) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpResponseMessage> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpResponseMessage) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for HttpResponseMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &HttpResponseMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpResponseMessage> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpResponseMessage) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpResponseMessage> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpResponseMessage) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpResponseMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpResponseMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpResponseMessage {}
unsafe impl ::core::marker::Sync for HttpResponseMessage {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HttpResponseMessageSource(pub i32);
impl HttpResponseMessageSource {
    pub const None: Self = Self(0i32);
    pub const Cache: Self = Self(1i32);
    pub const Network: Self = Self(2i32);
}
impl ::core::marker::Copy for HttpResponseMessageSource {}
impl ::core::clone::Clone for HttpResponseMessageSource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HttpResponseMessageSource {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HttpResponseMessageSource {
    type Abi = Self;
}
impl ::core::fmt::Debug for HttpResponseMessageSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpResponseMessageSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpResponseMessageSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Web.Http.HttpResponseMessageSource;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HttpStatusCode(pub i32);
impl HttpStatusCode {
    pub const None: Self = Self(0i32);
    pub const Continue: Self = Self(100i32);
    pub const SwitchingProtocols: Self = Self(101i32);
    pub const Processing: Self = Self(102i32);
    pub const Ok: Self = Self(200i32);
    pub const Created: Self = Self(201i32);
    pub const Accepted: Self = Self(202i32);
    pub const NonAuthoritativeInformation: Self = Self(203i32);
    pub const NoContent: Self = Self(204i32);
    pub const ResetContent: Self = Self(205i32);
    pub const PartialContent: Self = Self(206i32);
    pub const MultiStatus: Self = Self(207i32);
    pub const AlreadyReported: Self = Self(208i32);
    pub const IMUsed: Self = Self(226i32);
    pub const MultipleChoices: Self = Self(300i32);
    pub const MovedPermanently: Self = Self(301i32);
    pub const Found: Self = Self(302i32);
    pub const SeeOther: Self = Self(303i32);
    pub const NotModified: Self = Self(304i32);
    pub const UseProxy: Self = Self(305i32);
    pub const TemporaryRedirect: Self = Self(307i32);
    pub const PermanentRedirect: Self = Self(308i32);
    pub const BadRequest: Self = Self(400i32);
    pub const Unauthorized: Self = Self(401i32);
    pub const PaymentRequired: Self = Self(402i32);
    pub const Forbidden: Self = Self(403i32);
    pub const NotFound: Self = Self(404i32);
    pub const MethodNotAllowed: Self = Self(405i32);
    pub const NotAcceptable: Self = Self(406i32);
    pub const ProxyAuthenticationRequired: Self = Self(407i32);
    pub const RequestTimeout: Self = Self(408i32);
    pub const Conflict: Self = Self(409i32);
    pub const Gone: Self = Self(410i32);
    pub const LengthRequired: Self = Self(411i32);
    pub const PreconditionFailed: Self = Self(412i32);
    pub const RequestEntityTooLarge: Self = Self(413i32);
    pub const RequestUriTooLong: Self = Self(414i32);
    pub const UnsupportedMediaType: Self = Self(415i32);
    pub const RequestedRangeNotSatisfiable: Self = Self(416i32);
    pub const ExpectationFailed: Self = Self(417i32);
    pub const UnprocessableEntity: Self = Self(422i32);
    pub const Locked: Self = Self(423i32);
    pub const FailedDependency: Self = Self(424i32);
    pub const UpgradeRequired: Self = Self(426i32);
    pub const PreconditionRequired: Self = Self(428i32);
    pub const TooManyRequests: Self = Self(429i32);
    pub const RequestHeaderFieldsTooLarge: Self = Self(431i32);
    pub const InternalServerError: Self = Self(500i32);
    pub const NotImplemented: Self = Self(501i32);
    pub const BadGateway: Self = Self(502i32);
    pub const ServiceUnavailable: Self = Self(503i32);
    pub const GatewayTimeout: Self = Self(504i32);
    pub const HttpVersionNotSupported: Self = Self(505i32);
    pub const VariantAlsoNegotiates: Self = Self(506i32);
    pub const InsufficientStorage: Self = Self(507i32);
    pub const LoopDetected: Self = Self(508i32);
    pub const NotExtended: Self = Self(510i32);
    pub const NetworkAuthenticationRequired: Self = Self(511i32);
}
impl ::core::marker::Copy for HttpStatusCode {}
impl ::core::clone::Clone for HttpStatusCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HttpStatusCode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HttpStatusCode {
    type Abi = Self;
}
impl ::core::fmt::Debug for HttpStatusCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpStatusCode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpStatusCode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Web.Http.HttpStatusCode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct HttpStreamContent(::windows_core::IUnknown);
impl HttpStreamContent {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows_core::Result<Headers::HttpContentHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Headers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Headers::HttpContentHeaderCollection>(result__)
        }
    }
    pub fn BufferAllAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BufferAllAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ReadAsBufferAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IBuffer, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsBufferAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IBuffer, u64>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ReadAsInputStreamAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IInputStream, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsInputStreamAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IInputStream, u64>>(result__)
        }
    }
    pub fn ReadAsStringAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::windows_core::HSTRING, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsStringAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::windows_core::HSTRING, u64>>(result__)
        }
    }
    pub fn TryComputeLength(&self, length: &mut u64) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryComputeLength)(::windows_core::Interface::as_raw(this), length, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn WriteToStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IOutputStream>>(&self, outputstream: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WriteToStreamAsync)(::windows_core::Interface::as_raw(this), outputstream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromInputStream<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream>>(content: Param0) -> ::windows_core::Result<HttpStreamContent> {
        Self::IHttpStreamContentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromInputStream)(::windows_core::Interface::as_raw(this), content.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpStreamContent>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpStreamContentFactory<R, F: FnOnce(&IHttpStreamContentFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpStreamContent, IHttpStreamContentFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpStreamContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpStreamContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpStreamContent {}
impl ::core::fmt::Debug for HttpStreamContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpStreamContent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpStreamContent {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpStreamContent;{6b14a441-fba7-4bd2-af0a-839de7c295da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpStreamContent {
    type Vtable = IHttpContent_Vtbl;
    const IID: ::windows_core::GUID = <IHttpContent as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpStreamContent {
    const NAME: &'static str = "Windows.Web.Http.HttpStreamContent";
}
impl ::core::convert::From<HttpStreamContent> for ::windows_core::IUnknown {
    fn from(value: HttpStreamContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpStreamContent> for ::windows_core::IUnknown {
    fn from(value: &HttpStreamContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpStreamContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpStreamContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpStreamContent> for ::windows_core::IInspectable {
    fn from(value: HttpStreamContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpStreamContent> for ::windows_core::IInspectable {
    fn from(value: &HttpStreamContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpStreamContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpStreamContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpStreamContent> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpStreamContent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpStreamContent> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpStreamContent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for HttpStreamContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &HttpStreamContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpStreamContent> for IHttpContent {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpStreamContent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpStreamContent> for IHttpContent {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpStreamContent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IHttpContent> for HttpStreamContent {
    fn into_param(self) -> ::windows_core::Param<'a, IHttpContent> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IHttpContent> for &HttpStreamContent {
    fn into_param(self) -> ::windows_core::Param<'a, IHttpContent> {
        ::core::convert::TryInto::<IHttpContent>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpStreamContent> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpStreamContent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpStreamContent> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpStreamContent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpStreamContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpStreamContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpStreamContent {}
unsafe impl ::core::marker::Sync for HttpStreamContent {}
#[repr(transparent)]
pub struct HttpStringContent(::windows_core::IUnknown);
impl HttpStringContent {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows_core::Result<Headers::HttpContentHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Headers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Headers::HttpContentHeaderCollection>(result__)
        }
    }
    pub fn BufferAllAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BufferAllAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ReadAsBufferAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IBuffer, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsBufferAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IBuffer, u64>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ReadAsInputStreamAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IInputStream, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsInputStreamAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IInputStream, u64>>(result__)
        }
    }
    pub fn ReadAsStringAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::windows_core::HSTRING, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsStringAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::windows_core::HSTRING, u64>>(result__)
        }
    }
    pub fn TryComputeLength(&self, length: &mut u64) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryComputeLength)(::windows_core::Interface::as_raw(this), length, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn WriteToStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IOutputStream>>(&self, outputstream: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WriteToStreamAsync)(::windows_core::Interface::as_raw(this), outputstream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    pub fn CreateFromString<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(content: Param0) -> ::windows_core::Result<HttpStringContent> {
        Self::IHttpStringContentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromString)(::windows_core::Interface::as_raw(this), content.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpStringContent>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStringWithEncoding<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(content: Param0, encoding: ::winrt_storage::Streams::UnicodeEncoding) -> ::windows_core::Result<HttpStringContent> {
        Self::IHttpStringContentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStringWithEncoding)(::windows_core::Interface::as_raw(this), content.into_param().abi(), encoding, result__.as_mut_ptr()).from_abi::<HttpStringContent>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStringWithEncodingAndMediaType<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(content: Param0, encoding: ::winrt_storage::Streams::UnicodeEncoding, mediatype: Param2) -> ::windows_core::Result<HttpStringContent> {
        Self::IHttpStringContentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStringWithEncodingAndMediaType)(::windows_core::Interface::as_raw(this), content.into_param().abi(), encoding, mediatype.into_param().abi(), result__.as_mut_ptr()).from_abi::<HttpStringContent>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IHttpStringContentFactory<R, F: FnOnce(&IHttpStringContentFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HttpStringContent, IHttpStringContentFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HttpStringContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpStringContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpStringContent {}
impl ::core::fmt::Debug for HttpStringContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpStringContent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpStringContent {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpStringContent;{6b14a441-fba7-4bd2-af0a-839de7c295da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpStringContent {
    type Vtable = IHttpContent_Vtbl;
    const IID: ::windows_core::GUID = <IHttpContent as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpStringContent {
    const NAME: &'static str = "Windows.Web.Http.HttpStringContent";
}
impl ::core::convert::From<HttpStringContent> for ::windows_core::IUnknown {
    fn from(value: HttpStringContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpStringContent> for ::windows_core::IUnknown {
    fn from(value: &HttpStringContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpStringContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpStringContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpStringContent> for ::windows_core::IInspectable {
    fn from(value: HttpStringContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpStringContent> for ::windows_core::IInspectable {
    fn from(value: &HttpStringContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpStringContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpStringContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpStringContent> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpStringContent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpStringContent> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpStringContent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for HttpStringContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &HttpStringContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpStringContent> for IHttpContent {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpStringContent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpStringContent> for IHttpContent {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpStringContent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IHttpContent> for HttpStringContent {
    fn into_param(self) -> ::windows_core::Param<'a, IHttpContent> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IHttpContent> for &HttpStringContent {
    fn into_param(self) -> ::windows_core::Param<'a, IHttpContent> {
        ::core::convert::TryInto::<IHttpContent>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<HttpStringContent> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpStringContent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpStringContent> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpStringContent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpStringContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpStringContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpStringContent {}
unsafe impl ::core::marker::Sync for HttpStringContent {}
#[repr(transparent)]
pub struct HttpTransportInformation(::windows_core::IUnknown);
impl HttpTransportInformation {
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ServerCertificate(&self) -> ::windows_core::Result<::winrt_security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ServerCertificate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_security::Cryptography::Certificates::Certificate>(result__)
        }
    }
    #[cfg(feature = "Networking_Sockets")]
    pub fn ServerCertificateErrorSeverity(&self) -> ::windows_core::Result<::winrt_networking::Sockets::SocketSslErrorSeverity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_networking::Sockets::SocketSslErrorSeverity>::zeroed();
            (::windows_core::Interface::vtable(this).ServerCertificateErrorSeverity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_networking::Sockets::SocketSslErrorSeverity>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerCertificateErrors(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_security::Cryptography::Certificates::ChainValidationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ServerCertificateErrors)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_security::Cryptography::Certificates::ChainValidationResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerIntermediateCertificates(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_security::Cryptography::Certificates::Certificate>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ServerIntermediateCertificates)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_security::Cryptography::Certificates::Certificate>>(result__)
        }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for HttpTransportInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpTransportInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpTransportInformation {}
impl ::core::fmt::Debug for HttpTransportInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpTransportInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpTransportInformation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.HttpTransportInformation;{70127198-c6a7-4ed0-833a-83fd8b8f178d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HttpTransportInformation {
    type Vtable = IHttpTransportInformation_Vtbl;
    const IID: ::windows_core::GUID = <IHttpTransportInformation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HttpTransportInformation {
    const NAME: &'static str = "Windows.Web.Http.HttpTransportInformation";
}
impl ::core::convert::From<HttpTransportInformation> for ::windows_core::IUnknown {
    fn from(value: HttpTransportInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpTransportInformation> for ::windows_core::IUnknown {
    fn from(value: &HttpTransportInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HttpTransportInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HttpTransportInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HttpTransportInformation> for ::windows_core::IInspectable {
    fn from(value: HttpTransportInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HttpTransportInformation> for ::windows_core::IInspectable {
    fn from(value: &HttpTransportInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HttpTransportInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HttpTransportInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HttpTransportInformation> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: HttpTransportInformation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HttpTransportInformation> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HttpTransportInformation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for HttpTransportInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &HttpTransportInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HttpTransportInformation {}
unsafe impl ::core::marker::Sync for HttpTransportInformation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HttpVersion(pub i32);
impl HttpVersion {
    pub const None: Self = Self(0i32);
    pub const Http10: Self = Self(1i32);
    pub const Http11: Self = Self(2i32);
    pub const Http20: Self = Self(3i32);
}
impl ::core::marker::Copy for HttpVersion {}
impl ::core::clone::Clone for HttpVersion {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HttpVersion {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HttpVersion {
    type Abi = Self;
}
impl ::core::fmt::Debug for HttpVersion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpVersion").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HttpVersion {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Web.Http.HttpVersion;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpBufferContentFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpBufferContentFactory {
    type Vtable = IHttpBufferContentFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc20c193_c41f_4ff7_9123_6435736eadc2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpBufferContentFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBufferWithOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows_core::RawPtr, offset: u32, count: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBufferWithOffset: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpClient(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpClient {
    type Vtable = IHttpClient_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7fda1151_3574_4880_a8ba_e6b1e0061f3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpClient_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetWithOptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, completionoption: HttpCompletionOption, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetBufferAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetBufferAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetInputStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetInputStreamAsync: usize,
    pub GetStringAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PostAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, content: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PutAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, content: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SendRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SendRequestWithOptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows_core::RawPtr, completionoption: HttpCompletionOption, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Web_Http_Headers")]
    pub DefaultRequestHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Web_Http_Headers"))]
    DefaultRequestHeaders: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpClient2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpClient2 {
    type Vtable = IHttpClient2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcdd83348_e8b7_4cec_b1b0_dc455fe72c92);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpClient2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryDeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryGetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryGetAsync2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, completionoption: HttpCompletionOption, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryGetBufferAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryGetInputStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryGetStringAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryPostAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, content: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryPutAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, content: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TrySendRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TrySendRequestAsync2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows_core::RawPtr, completionoption: HttpCompletionOption, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpClientFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpClientFactory {
    type Vtable = IHttpClientFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc30c4eca_e3fa_4f99_afb4_63cc65009462);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpClientFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Web_Http_Filters")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filter: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Web_Http_Filters"))]
    Create: usize,
}
#[repr(transparent)]
pub struct IHttpContent(::windows_core::IUnknown);
impl IHttpContent {
    #[cfg(feature = "Web_Http_Headers")]
    pub fn Headers(&self) -> ::windows_core::Result<Headers::HttpContentHeaderCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Headers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Headers::HttpContentHeaderCollection>(result__)
        }
    }
    pub fn BufferAllAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BufferAllAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ReadAsBufferAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IBuffer, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsBufferAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IBuffer, u64>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ReadAsInputStreamAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IInputStream, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsInputStreamAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IInputStream, u64>>(result__)
        }
    }
    pub fn ReadAsStringAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::windows_core::HSTRING, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsStringAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::windows_core::HSTRING, u64>>(result__)
        }
    }
    pub fn TryComputeLength(&self, length: &mut u64) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryComputeLength)(::windows_core::Interface::as_raw(this), length, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn WriteToStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IOutputStream>>(&self, outputstream: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WriteToStreamAsync)(::windows_core::Interface::as_raw(this), outputstream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::convert::From<IHttpContent> for ::windows_core::IUnknown {
    fn from(value: IHttpContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHttpContent> for ::windows_core::IUnknown {
    fn from(value: &IHttpContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IHttpContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IHttpContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IHttpContent> for ::windows_core::IInspectable {
    fn from(value: IHttpContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHttpContent> for ::windows_core::IInspectable {
    fn from(value: &IHttpContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IHttpContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IHttpContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IHttpContent> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: IHttpContent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IHttpContent> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &IHttpContent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for IHttpContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &IHttpContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IHttpContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IHttpContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHttpContent {}
impl ::core::fmt::Debug for IHttpContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHttpContent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IHttpContent {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{6b14a441-fba7-4bd2-af0a-839de7c295da}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IHttpContent {
    type Vtable = IHttpContent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b14a441_fba7_4bd2_af0a_839de7c295da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContent_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Web_Http_Headers")]
    pub Headers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Web_Http_Headers"))]
    Headers: usize,
    pub BufferAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ReadAsBufferAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ReadAsBufferAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ReadAsInputStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ReadAsInputStreamAsync: usize,
    pub ReadAsStringAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryComputeLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: *mut u64, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub WriteToStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    WriteToStreamAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCookie(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpCookie {
    type Vtable = IHttpCookie_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1f5488e2_cc2d_4779_86a7_88f10687d249);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookie_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Expires: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetExpires: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub HttpOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHttpOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Secure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetSecure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCookieFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpCookieFactory {
    type Vtable = IHttpCookieFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6a0585a9_931c_4cd1_a96d_c21701785c5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookieFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, domain: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, path: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCookieManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpCookieManager {
    type Vtable = IHttpCookieManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a431780_cd4f_4e57_a84a_5b0a53d6bb96);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookieManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCookieWithThirdParty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::windows_core::RawPtr, thirdparty: bool, result__: *mut bool) -> ::windows_core::HRESULT,
    pub DeleteCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCookies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCookies: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpFormUrlEncodedContentFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpFormUrlEncodedContentFactory {
    type Vtable = IHttpFormUrlEncodedContentFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43f0138c_2f73_4302_b5f3_eae9238a5e01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpFormUrlEncodedContentFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpGetBufferResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpGetBufferResult {
    type Vtable = IHttpGetBufferResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x53d08e7c_e209_404e_9a49_742d8236fd3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpGetBufferResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub RequestMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResponseMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Value: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpGetInputStreamResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpGetInputStreamResult {
    type Vtable = IHttpGetInputStreamResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd5d63463_13aa_4ee0_be95_a0c39fe91203);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpGetInputStreamResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub RequestMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResponseMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Value: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpGetStringResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpGetStringResult {
    type Vtable = IHttpGetStringResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9bac466d_8509_4775_b16d_8953f47a7f5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpGetStringResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub RequestMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResponseMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMethod(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpMethod {
    type Vtable = IHttpMethod_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x728d4022_700d_4fe0_afa5_40299c58dbfd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMethod_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Method: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMethodFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpMethodFactory {
    type Vtable = IHttpMethodFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3c51d10d_36d7_40f8_a86d_e759caf2f83f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMethodFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMethodStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpMethodStatics {
    type Vtable = IHttpMethodStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64d171f0_d99a_4153_8dc6_d68cc4cce317);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMethodStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Head: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Options: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Patch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Post: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Put: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMultipartContent(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpMultipartContent {
    type Vtable = IHttpMultipartContent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdf916aff_9926_4ac9_aaf1_e0d04ef09bb9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMultipartContent_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMultipartContentFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpMultipartContentFactory {
    type Vtable = IHttpMultipartContentFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7eb42e62_0222_4f20_b372_47d5db5d33b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMultipartContentFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateWithSubtype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithSubtypeAndBoundary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, boundary: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMultipartFormDataContent(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpMultipartFormDataContent {
    type Vtable = IHttpMultipartFormDataContent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64d337e2_e967_4624_b6d1_cf74604a4a42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMultipartFormDataContent_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AddWithName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows_core::RawPtr, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AddWithNameAndFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows_core::RawPtr, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, filename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMultipartFormDataContentFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpMultipartFormDataContentFactory {
    type Vtable = IHttpMultipartFormDataContentFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa04d7311_5017_4622_93a8_49b24a4fcbfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMultipartFormDataContentFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateWithBoundary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boundary: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpRequestMessage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpRequestMessage {
    type Vtable = IHttpRequestMessage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5762b3c_74d4_4811_b5dc_9f8b4e2f9abf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpRequestMessage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Web_Http_Headers")]
    pub Headers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Web_Http_Headers"))]
    Headers: usize,
    pub Method: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub RequestUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetRequestUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TransportInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpRequestMessageFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpRequestMessageFactory {
    type Vtable = IHttpRequestMessageFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5bac994e_3886_412e_aec3_52ec7f25616f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpRequestMessageFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: ::windows_core::RawPtr, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpRequestResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpRequestResult {
    type Vtable = IHttpRequestResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6acf4da8_b5eb_4a35_a902_4217fbe820c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpRequestResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub RequestMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResponseMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpResponseMessage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpResponseMessage {
    type Vtable = IHttpResponseMessage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfee200fb_8664_44e0_95d9_42696199bffc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpResponseMessage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Web_Http_Headers")]
    pub Headers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Web_Http_Headers"))]
    Headers: usize,
    pub IsSuccessStatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ReasonPhrase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetReasonPhrase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RequestMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetRequestMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HttpResponseMessageSource) -> ::windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: HttpResponseMessageSource) -> ::windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HttpStatusCode) -> ::windows_core::HRESULT,
    pub SetStatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: HttpStatusCode) -> ::windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HttpVersion) -> ::windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: HttpVersion) -> ::windows_core::HRESULT,
    pub EnsureSuccessStatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpResponseMessageFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpResponseMessageFactory {
    type Vtable = IHttpResponseMessageFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x52a8af99_f095_43da_b60f_7cfc2bc7ea2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpResponseMessageFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statuscode: HttpStatusCode, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpStreamContentFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpStreamContentFactory {
    type Vtable = IHttpStreamContentFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf3e64d9d_f725_407e_942f_0eda189809f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpStreamContentFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromInputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromInputStream: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpStringContentFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpStringContentFactory {
    type Vtable = IHttpStringContentFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x46649d5b_2e93_48eb_8e61_19677878e57f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpStringContentFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFromString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStringWithEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, encoding: ::winrt_storage::Streams::UnicodeEncoding, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStringWithEncoding: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStringWithEncodingAndMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, encoding: ::winrt_storage::Streams::UnicodeEncoding, mediatype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStringWithEncodingAndMediaType: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpTransportInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHttpTransportInformation {
    type Vtable = IHttpTransportInformation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70127198_c6a7_4ed0_833a_83fd8b8f178d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpTransportInformation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ServerCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ServerCertificate: usize,
    #[cfg(feature = "Networking_Sockets")]
    pub ServerCertificateErrorSeverity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_networking::Sockets::SocketSslErrorSeverity) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    ServerCertificateErrorSeverity: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerCertificateErrors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerCertificateErrors: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerIntermediateCertificates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerIntermediateCertificates: usize,
}
